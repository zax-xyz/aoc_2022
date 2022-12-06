use crate::util::read_lines;
use std::collections::HashSet;

pub fn part1() -> usize {
    start_of_packet_end(4)
}

pub fn part2() -> usize {
    start_of_packet_end(14)
}

fn start_of_packet_end(len: usize) -> usize {
    read_lines("inputs/day06.txt")
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .enumerate()
        .into_iter()
        .find(|(_, w)| HashSet::<&char>::from_iter(w.into_iter()).len() == len)
        .unwrap()
        .0
        + len
}
