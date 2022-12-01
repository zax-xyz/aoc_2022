use std::fs;

pub fn part1() -> u32 {
    fs::read_to_string("inputs/day01p1.txt")
        .expect("Reading file failed :(")
        .lines()
        .collect::<Vec<&str>>()
        .split(|line| line.len() == 0)
        .map(|group| group.iter().map(|l| str::parse::<u32>(l).unwrap()).sum())
        .max()
        .unwrap()
}

pub fn part2() -> u32 {
    let mut calories = fs::read_to_string("inputs/day01p1.txt")
        .expect("Reading file failed :(")
        .lines()
        .collect::<Vec<&str>>()
        .split(|line| line.len() == 0)
        .map(|group| group.iter().map(|l| str::parse::<u32>(l).unwrap()).sum())
        .collect::<std::collections::BinaryHeap<u32>>();

    (0..3).map(|_| calories.pop().unwrap()).sum()
}
