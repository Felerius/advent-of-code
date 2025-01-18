use std::{collections::VecDeque, mem, str};

use itertools::Itertools;
use utils::hash::{FastHashCollectionExt, FastHashMap};

pub(crate) fn run(input: &str) -> (u64, String) {
    let (adj, wire_vals) = parse_graph(input);
    (part1(&adj, wire_vals), part2(&adj))
}

fn part1(adj: &FastHashMap<Wire, Vec<Gate>>, mut wire_vals: FastHashMap<Wire, bool>) -> u64 {
    let mut ready: VecDeque<_> = wire_vals
        .keys()
        .flat_map(|wire| {
            adj[wire]
                .iter()
                .filter(|gate| gate.left == *wire && wire_vals.contains_key(&gate.right))
        })
        .copied()
        .collect();
    while let Some(gate) = ready.pop_front() {
        let left = wire_vals[&gate.left];
        let right = wire_vals[&gate.right];
        let val = match gate.operation {
            Operation::And => left & right,
            Operation::Or => left | right,
            Operation::Xor => left ^ right,
        };
        wire_vals.insert(gate.output, val);

        for &next in adj.get(&gate.output).into_iter().flatten() {
            if wire_vals.contains_key(&next.left) && wire_vals.contains_key(&next.right) {
                ready.push_back(next);
            }
        }
    }

    (0..)
        .map_while(|i| {
            wire_vals
                .get(&Wire::output(i))
                .map(|&val| u64::from(val) << i)
        })
        .fold(0, |a, b| a | b)
}

/// Part 2 solver
///
/// ## Assumptions
///
/// - The input is a ripple carry adder
/// - The adder for the least significant bit is correct
/// - Each swap occurred within the wires of a single-bit adder
///
/// ## 1-bit Adder
///
/// Each adder from the second bit onward contains these five gates:
///
/// - `x_i XOR y_i -> S_i`
/// - `x_i AND y_i -> T_i`
/// - `S_i XOR C_(i - 1) -> z_i`
/// - `S_i AND C_(i - 1) -> U_i`
/// - `T_i OR U_i -> C_i`
///
/// where `x_i` and `y_i` are the `i`-th input bits, `z_i` the `i`-th output
/// bit, `C_i` the `i`-th carry bit, and `S_i`, `T_i`, and `U_i` are temporary
/// wires.
///
/// ## Fixing Each Adder
///
/// Given that we fixed the previous adder, we know which wire is `C_(i - 1)`
/// and can thus identify the first four gates. We can also find the last gate
/// by looking for the only OR gate involving output wires from the first four
/// gates.
///
/// Instead of finding the two swapped wires, we can instead just reconstruct
/// the correct output wires for all gates. The gate for `z_i` is obvious. We
/// can identify `T_i` and `U_i` by them only being used in one gate, and assign
/// them to their two gates in a way that causes the least changed output wires.
/// Lastly, we can differentiate between `S_i` and `C_i` by seeing that the
/// former is used in gates involving `C_(i - 1)`.
fn part2(adj: &FastHashMap<Wire, Vec<Gate>>) -> String {
    let (_, carry0_gate) = get_xor_and_gates(adj, Wire::left_input(0));
    let mut carry = carry0_gate.output;
    let mut fixed_outputs = FastHashMap::new();
    let mut bit = 0;

    while !carry.is_output() {
        bit += 1;
        let (si_gate, mut ti_gate) = get_xor_and_gates(adj, Wire::left_input(bit));
        let (zi_gate, mut ui_gate) = get_xor_and_gates(adj, carry);

        let zi = Wire::output(bit);
        let si = if zi_gate.left == carry {
            zi_gate.right
        } else {
            zi_gate.left
        };
        let (ti, ui) = [si_gate, ti_gate, zi_gate, ui_gate]
            .into_iter()
            .map(|gate| gate.output)
            .filter(|wire| adj.get(wire).is_some_and(|a| a.len() == 1))
            .collect_tuple()
            .unwrap();

        fixed_outputs.insert(si_gate, si);
        fixed_outputs.insert(zi_gate, zi);
        if ti_gate.output == ui || ui_gate.output == ti {
            mem::swap(&mut ti_gate, &mut ui_gate);
        }
        fixed_outputs.insert(ti_gate, ti);
        fixed_outputs.insert(ui_gate, ui);

        let ci_gate = adj[&ti][0];
        carry = [si_gate, ti_gate, zi_gate, ui_gate, ci_gate]
            .into_iter()
            .map(|gate| gate.output)
            .find(|wire| ![si, ti, ui, zi].contains(wire))
            .unwrap();
        fixed_outputs.insert(ci_gate, carry);
    }

    fixed_outputs
        .iter()
        .filter(|&(gate, &output)| gate.output != output)
        .map(|(_, output)| output.as_str())
        .sorted()
        .join(",")
}

fn get_xor_and_gates(adj: &FastHashMap<Wire, Vec<Gate>>, wire: Wire) -> (Gate, Gate) {
    let (mut gate1, mut gate2) = adj[&wire].iter().copied().collect_tuple().unwrap();
    if gate1.operation != Operation::Xor {
        (gate1, gate2) = (gate2, gate1);
    }
    debug_assert_eq!(gate1.operation, Operation::Xor);
    debug_assert_eq!(gate2.operation, Operation::And);
    (gate1, gate2)
}

fn parse_graph(input: &str) -> (FastHashMap<Wire, Vec<Gate>>, FastHashMap<Wire, bool>) {
    let mut lines = input.lines();
    let input = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| (Wire::from_str(&line[..3]), line.ends_with("1")))
        .collect();

    let mut adj = FastHashMap::<_, Vec<_>>::new();
    for line in lines {
        let (left, operation, right, _, output) =
            line.split_ascii_whitespace().collect_tuple().unwrap();
        let operation = match operation {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => panic!("Unknown operation: {operation:?}"),
        };
        let gate = Gate {
            operation,
            left: Wire::from_str(left),
            right: Wire::from_str(right),
            output: Wire::from_str(output),
        };

        adj.entry(gate.left).or_default().push(gate);
        adj.entry(gate.right).or_default().push(gate);
    }

    (adj, input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Wire([u8; 3]);

impl Wire {
    fn from_str(s: impl AsRef<[u8]>) -> Self {
        let s = s.as_ref();
        assert_eq!(s.len(), 3);
        Self([s[0], s[1], s[2]])
    }

    fn left_input(i: u8) -> Self {
        debug_assert!(i < 100);
        Self([b'x', b'0' + (i / 10), b'0' + (i % 10)])
    }

    fn output(i: u8) -> Self {
        debug_assert!(i < 100);
        Self([b'z', b'0' + (i / 10), b'0' + (i % 10)])
    }

    fn is_output(self) -> bool {
        self.0[0] == b'z'
    }

    fn as_str(&self) -> &str {
        str::from_utf8(&self.0).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Gate {
    operation: Operation,
    left: Wire,
    right: Wire,
    output: Wire,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const LARGE_INPUT: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn part1_small() {
        let (adj, wire_vals) = parse_graph(SMALL_INPUT);
        assert_eq!(part1(&adj, wire_vals), 4);
    }

    #[test]
    fn part1_large() {
        let (adj, wire_vals) = parse_graph(LARGE_INPUT);
        assert_eq!(part1(&adj, wire_vals), 2024);
    }
}
