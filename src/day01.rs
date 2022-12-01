extern crate itertools;

use itertools::Itertools;
use std::collections::BinaryHeap;
use std::fs;

pub fn part1() -> u32 {
    fs::read_to_string("inputs/day01p1.txt")
        .expect("Reading file failed :(")
        .lines()
        .group_by(|line| line.len() == 0)
        .into_iter()
        // assumes parsing only fails on the blank lines
        .filter_map(|(_, group)| group.map(|l| str::parse::<u32>(l).ok()).sum())
        .max()
        .unwrap()
}

pub fn part2() -> u32 {
    let mut calories = fs::read_to_string("inputs/day01p1.txt")
        .expect("Reading file failed :(")
        .lines()
        .group_by(|line| line.len() == 0)
        .into_iter()
        // explicitly filters out blank lines before trying to parse
        .filter_map(|(_, group)| {
            group
                .map(|l| (l.len() > 0).then(|| str::parse::<u32>(l).unwrap()))
                .sum()
        })
        .collect::<BinaryHeap<u32>>();

    (0..3).map(|_| calories.pop().unwrap()).sum()
}
