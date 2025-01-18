use anyhow::Result;

pub(crate) fn run(input: &str) -> Result<(String, String)> {
    let init: Vec<_> = input.bytes().map(|b| b - b'0').collect();
    let part1 = checksum(generate(272, &init));
    let part2 = checksum(generate(35_651_584, &init));
    Ok((to_string(part1), to_string(part2)))
}

fn generate(len: usize, init: &[u8]) -> Vec<u8> {
    let mut data = init.to_vec();
    while data.len() <= len {
        let n = data.len();
        data.resize(2 * n + 1, 0);
        for i in 0..n {
            data[2 * n - i] = 1 - data[i];
        }
    }

    data.truncate(len);
    data
}

fn checksum(mut data: Vec<u8>) -> Vec<u8> {
    while data.len() % 2 == 0 {
        for i in 0..data.len() / 2 {
            data[i] = u8::from(data[2 * i] == data[2 * i + 1]);
        }
        data.truncate(data.len() / 2);
    }

    data
}

fn to_string(mut data: Vec<u8>) -> String {
    for c in &mut data {
        *c += b'0';
    }
    String::from_utf8(data).unwrap()
}
