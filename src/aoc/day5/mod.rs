use std::collections::VecDeque;
use itertools::Itertools;
use crate::Problem;

type Move = (usize, usize, usize);

pub struct Problem5 {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

impl Problem for Problem5 {
    fn new(input: &str) -> Self where Self: Sized {
        let stack_size = 9;
        let mut stacks = vec![VecDeque::new(); stack_size];
        input.lines().take(stack_size).for_each(|l| {
            let mut elements = [None; 9];
            for (i, p) in (1..l.len()).step_by(4).enumerate() {
                let c = l.as_bytes()[p];
                elements[i] = if c == b' ' { None } else { Some(c as char) };
            }
            for (i, e) in elements.iter().enumerate() {
                if e.is_some() {
                    stacks[i].push_back(e.unwrap());
                }
            }
        });
        let moves = input.lines().skip(10).map(|l| l.split_whitespace().map(|w| w.parse::<usize>().ok()).filter(|num| num.is_some()).map(|e| e.unwrap())
            .collect_tuple::<Move>().unwrap()).collect();
        Problem5 {
            stacks,
            moves,
        }
    }

    fn solve_part_1(&self) -> String {
        let mut stacks = self.stacks.clone();
        for (count, from, to) in self.moves.clone() {
            for _ in 0..count {
                let elem = stacks[from - 1].pop_front().unwrap();
                stacks[to - 1].push_front(elem);
            }
        }

        let res = stacks.iter().map(|s| s.front().unwrap()).collect::<String>();

        format!("{}", res)
    }

    fn solve_part_2(&self) -> String {
        let mut stacks = self.stacks.clone();
        for (count, from, to) in self.moves.clone() {
            let mut vec: Vec<char> = vec![];
            for _ in 0..count {
                let elem = stacks[from - 1].pop_front().unwrap();
                vec.push(elem);
            }
            let vec: Vec<&char> = vec.iter().rev().collect();

            for e in vec {
                stacks[to - 1].push_front(*e);
            }
        }

        let res = stacks.iter().map(|s| s.front().unwrap()).collect::<String>();

        format!("{}", res)
    }
}