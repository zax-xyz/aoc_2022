use crate::util::read_lines;

pub fn part1() -> u32 {
    read_lines("inputs/day02.txt")
        .map(|line| {
            if let Some((theirs, ours)) = line.split_once(' ') {
                let theirs_num = theirs.bytes().next().unwrap() - b'A' + 1;
                let ours_num = ours.bytes().next().unwrap() - b'X' + 1;
                let res = (ours_num as i8 - theirs_num as i8).rem_euclid(3);

                ours_num as u32
                    + match res {
                        2 => 0,
                        1 => 6,
                        0 => 3,
                        _ => panic!("bad nums"),
                    }
            } else {
                panic!("could not parse line: {}", line)
            }
        })
        .sum()
}

pub fn part2() -> u32 {
    read_lines("inputs/day02.txt")
        .map(|line| -> u32 {
            let (theirs, ours) = line.split_once(' ').unwrap();
            let theirs_num = theirs.bytes().next().unwrap() - b'A';
            let ours_num = ours.bytes().next().unwrap() - b'X';

            (1 + ((theirs_num + ours_num + 2) % 3) + ours_num * 3).into()
        })
        .sum()
}
