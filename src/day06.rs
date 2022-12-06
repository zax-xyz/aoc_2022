use crate::util::read_lines;
use std::collections::HashSet;

pub fn part1() -> usize {
    read_lines("inputs/day06.txt")
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
        .into_iter()
        .find(|(_, w)| HashSet::<&char>::from_iter(w.into_iter()).len() == 4)
        .unwrap()
        .0
        + 4
}

pub fn part2() -> u32 {
    0
}
