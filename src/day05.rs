use crate::util::read_lines;
use itertools::Itertools;
use std::collections::VecDeque;

fn parse_line_into_stacks(line: &str, stacks: &mut Vec<VecDeque<char>>) {
    line.chars()
        .chunks(4)
        .into_iter()
        .enumerate()
        .for_each(|(i, mut x)| {
            let crate_ = x.nth(1).unwrap();
            if crate_ != ' ' {
                stacks
                    .get_mut(i)
                    .unwrap()
                    .push_front(crate_);
            }
        });
}

pub fn part1() -> String {
    let mut lines = read_lines("inputs/day05.txt");
    let line = lines.next().unwrap();
    let mut stacks = vec![VecDeque::<char>::new(); (line.len() + 1) / 4];
    parse_line_into_stacks(&line, &mut stacks);

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        parse_line_into_stacks(&line, &mut stacks);
    }

    lines.for_each(|line| {
        let parts: Vec<_> = line
            .split(' ')
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, n)| str::parse::<usize>(n).ok().unwrap())
            .collect();

        if let [qty, from, to] = &parts[..] {
            (0..*qty).for_each(|_| {
                let crate_from = stacks.get_mut(*from - 1).unwrap().pop_back().unwrap();
                stacks
                    .get_mut(*to - 1)
                    .unwrap()
                    .push_back(crate_from);
            })
        }
    });

    stacks.iter().map(|stack| stack.back().unwrap()).join("")
}

pub fn part2() -> String {
    "".to_string()
}
