use std::collections::HashMap;
use crate::Problem;

pub struct Problem6 {
    chars: Vec<char>,
}

impl Problem for Problem6 {
    fn new(input: &str) -> Self where Self: Sized {
        let line = input.lines().next().unwrap();
        Problem6 {
            chars: line.chars().collect()
        }
    }

    fn solve_part_1(&self) -> String {
        let res = self.solve(4);
        format!("{}", res)
    }

    fn solve_part_2(&self) -> String {
        let res = self.solve(14);
        format!("{}", res)
    }
}

impl Problem6 {
    fn solve(&self, window_size: usize) -> i32 {
        let mut map: HashMap<&char, u32> = HashMap::new();
        for i in 0..self.chars.len() {
            let count = map.get(&self.chars[i]).or(Some(&0)).unwrap() + 1;
            map.insert(&self.chars[i], count);

            if i >= window_size {
                let val = map.get(&self.chars[i - window_size]).unwrap() - 1;
                map.insert(&self.chars[i - window_size], val);

                if val == 0 {
                    map.remove(&self.chars[i - window_size]);
                }
            }

            if map.len() == window_size {
                return (i + 1) as i32;
            }
        }
        -1
    }
}