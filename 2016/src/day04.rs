use anyhow::{Context, Result};

const ROOM_ROTATIONS: [&[u8]; 26] = [
    b"northpole-object-storage",
    b"mnqsgonkd-naidbs-rsnqzfd",
    b"lmprfnmjc-mzhcar-qrmpyec",
    b"kloqemlib-lygbzq-pqloxdb",
    b"jknpdlkha-kxfayp-opknwca",
    b"ijmockjgz-jwezxo-nojmvbz",
    b"hilnbjify-ivdywn-mniluay",
    b"ghkmaihex-hucxvm-lmhktzx",
    b"fgjlzhgdw-gtbwul-klgjsyw",
    b"efikygfcv-fsavtk-jkfirxv",
    b"dehjxfebu-erzusj-ijehqwu",
    b"cdgiwedat-dqytri-hidgpvt",
    b"bcfhvdczs-cpxsqh-ghcfous",
    b"abegucbyr-bowrpg-fgbentr",
    b"zadftbaxq-anvqof-efadmsq",
    b"yzcesazwp-zmupne-dezclrp",
    b"xybdrzyvo-yltomd-cdybkqo",
    b"wxacqyxun-xksnlc-bcxajpn",
    b"vwzbpxwtm-wjrmkb-abwziom",
    b"uvyaowvsl-viqlja-zavyhnl",
    b"tuxznvurk-uhpkiz-yzuxgmk",
    b"stwymutqj-tgojhy-xytwflj",
    b"rsvxltspi-sfnigx-wxsveki",
    b"qruwksroh-remhfw-vwrudjh",
    b"pqtvjrqng-qdlgev-uvqtcig",
    b"opsuiqpmf-pckfdu-tupsbhf",
];

pub(crate) fn run(input: &str) -> Result<(u32, u16)> {
    let mut part1 = 0;
    let mut part2 = None;
    for line in input.lines() {
        let line = line.as_bytes();
        let len = line.len();
        let cnt = line[..len - 11]
            .iter()
            .filter(|&&c| c != b'-')
            .fold([0; 26], |mut cnt, &c| {
                cnt[usize::from(c - b'a')] += 1;
                cnt
            });

        let mut coc = [0; 32];
        for c in cnt {
            coc[c] += 1;
        }
        for i in (1..coc.len()).rev() {
            coc[i - 1] += coc[i];
        }

        let mut sorted = [0; 26];
        for (i, ci) in cnt.into_iter().enumerate().rev() {
            coc[ci] -= 1;
            sorted[coc[ci]] = b'a' + i as u8;
        }

        if sorted[..5] == line[len - 6..len - 1] {
            let sector_id = 100 * u16::from(line[len - 10] - b'0')
                + 10 * u16::from(line[len - 9] - b'0')
                + u16::from(line[len - 8] - b'0');
            part1 += u32::from(sector_id);
            if &line[..len - 11] == ROOM_ROTATIONS[usize::from(sector_id % 26)] {
                part2 = Some(sector_id);
            }
        }
    }

    Ok((part1, part2.context("room for part2 not found")?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]
northpole-object-storage-000[oetra]";

    #[test]
    fn part1() {
        assert_eq!(run(INPUT).unwrap().0, 123 + 987 + 404);
    }
}
