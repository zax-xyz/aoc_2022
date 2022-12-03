use crate::util::read_lines;
use itertools::Itertools;
use std::collections::HashSet;

fn priorities(line: &str) -> HashSet<u32> {
    line.bytes()
        .map(|b| {
            (b as i32
                + if b.is_ascii_uppercase() {
                    27 - b'A' as i32
                } else {
                    1 - b'a' as i32
                }) as u32
        })
        .collect()
}

pub fn part1() -> u32 {
    read_lines("inputs/day03.txt")
        .unwrap()
        .map(|line| {
            let unwrapped_line = line.unwrap();
            let (first, second) = unwrapped_line.split_at(unwrapped_line.len() / 2);
            priorities(first)
                .intersection(&priorities(second))
                .sum::<u32>()
        })
        .sum()
}

pub fn part2() -> u32 {
    read_lines("inputs/day03.txt")
        .unwrap()
        .chunks(3)
        .into_iter()
        .map(|mut lines| -> u32 {
            let initial = priorities(&lines.next().unwrap().ok().unwrap());
            lines
                .fold(initial, |s, line| {
                    s.intersection(&priorities(&line.unwrap()))
                        .copied()
                        .collect()
                })
                .iter()
                .sum()
        })
        .sum()
}
