use crate::util::read_lines;
use itertools::Itertools;

fn contains(interval1: (u32, u32), interval2: (u32, u32)) -> bool {
    interval1.0 <= interval2.0 && interval1.1 >= interval2.1
}

pub fn part1() -> u32 {
    read_lines("inputs/day04.txt")
        .filter(|line| {
            let (first, second) = line
                .split(',')
                .map(|p| -> (u32, u32) {
                    p.split('-').map(|n| str::parse::<u32>(n).ok().unwrap()).next_tuple().unwrap()
                })
                .next_tuple()
                .unwrap();

            contains(first, second) || contains(second, first)
        })
        .count()
        .try_into()
        .unwrap()
}

pub fn part2() -> u32 {
    0
}
