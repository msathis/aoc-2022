use crate::Problem;
use itertools::Itertools;

pub struct Problem1 {
    elves: Vec<Vec<u32>>,
}

impl Problem for Problem1 {
    fn new(input: &str) -> Self {
        let elves: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.parse().ok())
            .fold(vec![vec![]], |mut acc, item| {
                if item == None {
                    acc.push(vec![]);
                } else {
                    acc.last_mut().unwrap().push(item.unwrap());
                }
                acc
            });
        Problem1 { elves }
    }

    fn solve_part_1(&self) -> String {
        let max_calories: u32 = self.elves
            .iter()
            .map(|group| group.iter().sum())
            .max()
            .unwrap();
        format!("{}", max_calories)
    }

    fn solve_part_2(&self) -> String {
        let mut sums: Vec<u32> = self.elves
            .iter()
            .map(|group| group.iter().sum())
            .collect();
        let max_calories: u32 = sums
            .iter()
            .sorted_by_key(|&v| u32::MAX - v)
            .take(3)
            .sum::<u32>();
        format!("{}", max_calories)
    }
}