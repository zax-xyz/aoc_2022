use crate::util::read_lines;
use itertools::Itertools;

type Interval = (u32, u32);
type Pair = (Interval, Interval);

fn pairs() -> impl Iterator<Item = Pair> {
    read_lines("inputs/day04.txt").map(|line| -> Pair {
        line.split(',')
            .map(|p| -> Interval {
                p.split('-')
                    .map(|n| str::parse::<u32>(n).ok().unwrap())
                    .next_tuple()
                    .unwrap()
            })
            .next_tuple()
            .unwrap()
    })
}

fn contains(interval1: &Interval, interval2: &Interval) -> bool {
    interval1.0 <= interval2.0 && interval1.1 >= interval2.1
}

pub fn part1() -> u32 {
    pairs()
        .filter(|(first, second)| contains(first, second) || contains(second, first))
        .count()
        .try_into()
        .unwrap()
}

pub fn part2() -> u32 {
    pairs()
        .filter(|(first, second)| first.0 <= second.1 && first.1 >= second.0)
        .count()
        .try_into()
        .unwrap()
}
