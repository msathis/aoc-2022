use std::cmp::max;
use crate::Problem;

pub struct Problem8 {
    grid: Vec<Vec<u32>>,
}

impl Problem for Problem8 {
    fn new(input: &str) -> Self where Self: Sized {
        let grid = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
        Problem8 { grid }
    }

    fn solve_part_1(&self) -> String {
        let mut visible = self.grid.len() * 4 - 4;
        for i in 1..self.grid.len() - 1 {
            for j in 1..self.grid[0].len() - 1 {
                if self.is_visible(i, j) {
                    visible += 1;
                }
            }
        }
        format!("{}", visible)
    }

    fn solve_part_2(&self) -> String {
        let mut max_score = 0;
        for i in 1..self.grid.len() - 1 {
            for j in 1..self.grid[0].len() - 1 {
                max_score = max(max_score, self.scenic_score(i, j));
            }
        }
        format!("{}", max_score)
    }
}

impl Problem8 {
    fn is_visible(&self, i: usize, j: usize) -> bool {
        let curr = self.grid[i][j];
        let mut left = true;
        let mut right = true;
        let mut top = true;
        let mut bottom = true;

        for k in 0..i {
            if curr <= self.grid[k][j] {
                top = false;
            }
        }

        for k in 0..j {
            if curr <= self.grid[i][k] {
                left = false;
            }
        }

        for k in (i + 1)..self.grid.len() {
            if curr <= self.grid[k][j] {
                right = false;
            }
        }

        for k in (j + 1)..self.grid[0].len() {
            if curr <= self.grid[i][k] {
                bottom = false;
            }
        }
        left || top || right || bottom
    }

    fn scenic_score(&self, i: usize, j: usize) -> u32 {
        let curr = self.grid[i][j];
        let mut left = 0_u32;
        let mut right = 0_u32;
        let mut top = 0_u32;
        let mut bottom = 0_u32;

        for k in (0..i).rev() {
            top += 1;
            if curr <= self.grid[k][j] {
                break;
            }
        }

        for k in (0..j).rev() {
            left += 1;
            if curr <= self.grid[i][k] {
                break;
            }
        }

        for k in (i + 1)..self.grid.len() {
            right += 1;
            if curr <= self.grid[k][j] {
                break;
            }
        }

        for k in (j + 1)..self.grid[0].len() {
            bottom += 1;
            if curr <= self.grid[i][k] {
                break;
            }
        }
        left * top * right * bottom
    }
}