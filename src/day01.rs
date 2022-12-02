extern crate itertools;

use crate::util::read_lines;
use itertools::Itertools;
use std::collections::BinaryHeap;

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
    let mut calories_heap = BinaryHeap::<i32>::new();

    let groups = read_lines("inputs/day01p1.txt")
        .expect("Reading file failed :(")
        .group_by(|line| line.as_ref().unwrap().len() == 0);

    let mut calories = groups
        .into_iter()
        // explicitly filters out blank lines before trying to parse
        .filter_map(|(_, group)| {
            group
                .map(|l| {
                    (l.as_ref().unwrap().len() > 0).then(|| str::parse::<i32>(&l.unwrap()).unwrap())
                })
                .sum::<Option<i32>>()
        });

    (0..3).for_each(|_| calories_heap.push(-calories.next().unwrap()));
    calories.for_each(|cal| {
        calories_heap.push(-cal);
        calories_heap.pop();
    });

    calories_heap.iter().map(|cal| -cal).sum::<i32>() as u32
}
