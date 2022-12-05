use crate::Problem;

pub struct Problem4 {
    data: Vec<Vec<Range>>,
}

struct Range {
    from: u32,
    to: u32,
}

impl Range {
    fn new(input: &str) -> Self {
        let splits: Vec<u32> = input.split("-").map(|e| e.parse::<u32>().unwrap()).collect();
        Range {
            from: splits[0],
            to: splits[1],
        }
    }

    fn contains(&self, range: &Range) -> bool {
        if self.from <= range.from && self.to >= range.to {
            return true;
        } else if range.from <= self.from && range.to >= self.to {
            return true;
        }
        false
    }

    fn overlaps(&self, range: &Range) -> bool {
        if self.to >= range.from && self.from <= range.from {
            return true;
        } else if self.from >= range.from && range.to >= self.from {
            return true;
        }
        false
    }
}

impl Problem for Problem4 {
    fn new(input: &str) -> Self where Self: Sized {
        let data: Vec<Vec<Range>> = input.lines().map(|line| {
            line.split(",").map(|l| Range::new(l)).collect()
        }).collect();
        Problem4 { data }
    }

    fn solve_part_1(&self) -> String {
        let count = self.data.iter().filter(|r| r[0].contains(&r[1])).count();
        format!("{}", count)
    }

    fn solve_part_2(&self) -> String {
        let count = self.data.iter().filter(|r| r[0].overlaps(&r[1])).count();
        format!("{}", count)
    }
}