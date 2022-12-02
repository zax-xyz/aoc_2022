use crate::util::read_lines;

pub fn day02() {
    let score: i32 = read_lines("inputs/day02.txt")
        .expect("could not open file")
        .map(|line| {
            if let Some((theirs, ours)) = line.as_ref().unwrap().split_once(' ') {
                let theirs_num = theirs.bytes().next().unwrap() as i32 - 'A' as i32 + 1;
                let ours_num = ours.bytes().next().unwrap() as i32 - b'X' as i32 + 1;
                let res = (ours_num - theirs_num).rem_euclid(3);

                ours_num + match res {
                    2 => 0,
                    1 => 6,
                    0 => 3,
                    _ => panic!("bad nums")
                }
            } else {
                panic!("could not parse line: {}", line.unwrap())
            }
        })
        .sum();

    println!("{}", score);
}
