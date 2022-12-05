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
                stacks.get_mut(i).unwrap().push_front(crate_);
            }
        });
}

fn parse_file() -> (
    Vec<VecDeque<char>>,
    impl Iterator<Item = (usize, usize, usize)>,
) {
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

    (
        stacks,
        lines.map(|line| {
            let mut parts = line
                .split(' ')
                .enumerate()
                .filter(|(i, _)| i % 2 == 1)
                .map(|(_, n)| str::parse::<usize>(n).ok().unwrap());

            (
                parts.next().unwrap(),
                parts.next().unwrap() - 1,
                parts.next().unwrap() - 1,
            )
        }),
    )
}

pub fn part1() -> String {
    let (mut stacks, lines) = parse_file();

    lines.for_each(|(qty, from, to)| {
        (0..qty).for_each(|_| {
            let crate_from = stacks.get_mut(from).unwrap().pop_back().unwrap();
            stacks.get_mut(to).unwrap().push_back(crate_from);
        })
    });

    stacks.iter().map(|stack| stack.back().unwrap()).join("")
}

pub fn part2() -> String {
    let (mut stacks, lines) = parse_file();

    lines.for_each(|(qty, from, to)| {
        let from_stack = stacks.get_mut(from).unwrap();
        let mut temp_stack = Vec::new();
        (0..qty).for_each(|_| {
            temp_stack.push(from_stack.pop_back().unwrap());
        });

        let to_stack = stacks.get_mut(to).unwrap();
        (0..qty).for_each(|_| {
            to_stack.push_back(temp_stack.pop().unwrap());
        });
    });

    stacks.iter().map(|stack| stack.back().unwrap()).join("")
}
