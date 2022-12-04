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
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            priorities(first)
                .intersection(&priorities(second))
                .sum::<u32>()
        })
        .sum()
}

pub fn part2() -> u32 {
    read_lines("inputs/day03.txt")
        .chunks(3)
        .into_iter()
        .map(|lines| -> u32 {
            lines
                .map(|line| priorities(&line))
                .reduce(|accum, p| accum.intersection(&p).copied().collect())
                .unwrap()
                .iter()
                .sum()
        })
        .sum()
}
