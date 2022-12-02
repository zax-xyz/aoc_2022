extern crate itertools;

use itertools::Itertools;
use std::collections::BinaryHeap;
use crate::util::read_lines;

pub fn part1() -> u32 {
    read_lines("inputs/day01p1.txt")
        .expect("Reading file failed :(")
        .group_by(|line| line.as_ref().unwrap().len() == 0)
        .into_iter()
        // assumes parsing only fails on the blank lines
        .filter_map(|(_, group)| group.map(|l| str::parse::<u32>(&l.unwrap()).ok()).sum())
        .max()
        .unwrap()
}

pub fn part2() -> u32 {
    let mut calories = read_lines("inputs/day01p1.txt")
        .expect("Reading file failed :(")
        .group_by(|line| line.as_ref().unwrap().len() == 0)
        .into_iter()
        // explicitly filters out blank lines before trying to parse
        .filter_map(|(_, group)| {
            group
                .map(|l| (l.as_ref().unwrap().len() > 0).then(|| str::parse::<u32>(&l.unwrap()).unwrap()))
                .sum()
        })
        .collect::<BinaryHeap<u32>>();

    (0..3).map(|_| calories.pop().unwrap()).sum()
}
