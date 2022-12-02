use crate::util::read_lines;

pub fn part1() -> u32 {
    read_lines("inputs/day02.txt")
        .expect("could not open file")
        .map(|line| {
            if let Some((theirs, ours)) = line.as_ref().unwrap().split_once(' ') {
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
                panic!("could not parse line: {}", line.unwrap())
            }
        })
        .sum()
}

pub fn part2() -> u32 {
    read_lines("inputs/day02.txt")
        .expect("could not open file")
        .map(|line| {
            if let Some((theirs, ours)) = line.as_ref().unwrap().split_once(' ') {
                let theirs_num: u32 = (theirs.bytes().next().unwrap() - b'A').into();

                1 + match ours {
                    "X" => (theirs_num + 2) % 3,
                    "Y" => theirs_num + 3,
                    "Z" => ((theirs_num + 1) % 3) + 6,
                    _ => panic!("bad key"),
                }
            } else {
                panic!("could not parse line: {}", line.unwrap())
            }
        })
        .sum()
}