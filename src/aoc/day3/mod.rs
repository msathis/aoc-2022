use crate::Problem;
use array_tool::vec::Intersect;

pub struct Problem3 {
    data: Vec<Vec<char>>,
}

impl Problem for Problem3 {
    fn new(input: &str) -> Self where Self: Sized {
        let data: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        Problem3 { data }
    }

    fn solve_part_1(&self) -> String {
        let sum: u32 = self.data.iter().map(|chars: &Vec<char>| {
            let mut chunks = chars.chunks(chars.len() / 2);
            let chunk1 = chunks.next().unwrap().to_vec();
            let chunk2 = chunks.next().unwrap().to_vec();
            let common: Vec<char> = chunk1.intersect(chunk2);
            Self::get_priority_sum(common)
        }).sum();
        format!("{}", sum)
    }

    fn solve_part_2(&self) -> String {
        let sum: u32 = self.data.chunks(3).map(|chunks| {
            let chunk1 = chunks[0].to_vec();
            let chunk2 = chunks[1].to_vec();
            let chunk3 = chunks[2].to_vec();
            let common: Vec<char> = chunk1.intersect(chunk2).intersect(chunk3);
            Self::get_priority_sum(common)
        }).sum();
        format!("{}", sum)
    }
}

impl Problem3 {
    fn get_priority_sum(common: Vec<char>) -> u32 {
        let sum = common.iter().map(|e: &char| {
            if e.is_ascii_lowercase() {
                *e as u32 - 96_u32
            } else {
                *e as u32 - 38_u32
            }
        }).sum::<u32>();
        sum
    }
}