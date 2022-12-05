use crate::Problem;

pub struct Problem2 {
    strategy: Vec<(String, String)>,
}

impl Problem for Problem2 {
    fn new(input: &str) -> Self {
        let data: Vec<(String, String)> = input
            .lines()
            .map(|line| {
                let chars: Vec<String> = line.split_whitespace().map(|word| word.to_string()).collect();
                (chars[0].clone(), chars[1].clone())
            })
            .collect(); // Collect the iterator of Vec<(String, String)> into a Vec<Vec<(String, String)>>
        Problem2 { strategy: data }
    }

    fn solve_part_1(&self) -> String {
        let total: u32 = self.strategy.clone().into_iter().map(|row: (String, String)| {
            let choice_point: u32 = if row.1 == "X" { 1 } else if row.1 == "Y" { 2 } else { 3 };
            let win_point: u32 = match (row.0.as_ref(), row.1.as_ref()) {
                ("A", "Z") => 0,
                ("A", "Y") => 6,
                ("B", "Z") => 6,
                ("B", "X") => 0,
                ("C", "X") => 6,
                ("C", "Y") => 0,
                (_, _) => 3,
            };
            choice_point + win_point
        }).sum();
        format!("{}", total)
    }

    fn solve_part_2(&self) -> String {
        let total: u32 = self.strategy.clone().into_iter().map(|row: (String, String)| {
            let choice_point: u32 = match (row.0.as_ref(), row.1.as_ref()) {
                ("A", "X") => 3,
                ("A", "Z") => 2,
                ("A", "Y") => 1,
                ("B", "X") => 1,
                ("B", "Y") => 2,
                ("B", "Z") => 3,
                ("C", "X") => 2,
                ("C", "Y") => 3,
                ("C", "Z") => 1,
                _ => panic!("Invalid combo")
            };
            let win_point: u32 = if row.1 == "X" { 0 } else if row.1 == "Y" { 3 } else { 6 };
            choice_point + win_point
        }).sum();
        format!("{}", total)
    }
}
