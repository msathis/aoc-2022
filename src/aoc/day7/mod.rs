use std::collections::HashMap;
use crate::Problem;

pub struct Problem7 {
    sizes: HashMap<String, usize>
}

impl Problem for Problem7 {
    fn new(input: &str) -> Self where Self: Sized {
        let mut paths = vec![];
        let mut sizes = HashMap::new();

        input.lines().for_each(|l| {
            let bytes = l.as_bytes();
            match bytes[0] {
                b'$' => {
                    match bytes[2] {
                        b'c' => {
                            match bytes[5] {
                                b'.' => { paths.pop(); }
                                _ => {
                                    paths.push(&l[5..]);
                                    sizes.insert(paths.join(""), 0 as usize);
                                }
                            }
                        }
                        _ => {} // Let's skip ls command
                    }
                }
                b'd' => {} // Let's skip dir for now
                _ => {
                    let size = l.split_whitespace().next().unwrap().parse::<usize>().unwrap();
                    for i in (1..paths.len() + 1).rev() {
                        *sizes.get_mut(&paths[0..i].join("")).unwrap() += size;
                    }
                }
            }
        });
        Problem7 {
            sizes
        }
    }

    fn solve_part_1(&self) -> String {
        let sum: usize = self.sizes.values().into_iter().filter(|v| **v <= 100000_usize).sum();
        format!("{}", sum)
    }

    fn solve_part_2(&self) -> String {
        let root_size = self.sizes.get("/").unwrap();
        let max = root_size - 40000000;
        let sum: &usize = self.sizes.values().into_iter().filter(|v| **v >= max).min().unwrap();
        format!("{}", sum)
    }
}