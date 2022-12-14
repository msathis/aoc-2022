use std::cmp;
use std::collections::{HashSet};
use crate::Problem;

enum Direction { R, L, U, D }

struct Move {
    direction: Direction,
    count: u32,
}

impl Move {
    pub fn new(lines: &str) -> Self {
        let parts: Vec<&str> = lines.split_whitespace().collect();
        let direction = match parts[0] {
            "R" => Direction::R,
            "L" => Direction::L,
            "U" => Direction::U,
            "D" => Direction::D,
            _ => panic!("Invalid direction")
        };

        Move {
            direction,
            count: parts[1].parse::<u32>().unwrap(),
        }
    }
}

pub struct Problem9 {
    moves: Vec<Move>,
}

fn distance(x: i32, y: i32, t_pos: (i32, i32)) -> u32 {
    let x_dis: u32 = (x - t_pos.0).abs() as u32;
    let y_dis: u32 = (y - t_pos.1).abs() as u32;
    return cmp::max(x_dis, y_dis);
}

fn get_position(from: (i32, i32), to: (i32, i32)) -> (i32, i32) {
    let mut delta_x: i32 = to.0 - from.0;
    let mut delta_y: i32 = to.1 - from.1;
    if (delta_x.abs() <= 2) && (delta_y.abs() <= 2) {
        delta_x = delta_x.clamp(-1, 1);
        delta_y = delta_y.clamp(-1, 1);
    } else if delta_x.abs() == 2 && delta_y == 0 {
        delta_x = delta_x.clamp(-1, 1);
    } else if delta_x == 0 && delta_y.abs() == 2 {
        delta_y = delta_y.clamp(-1, 1);
    }
    (delta_x, delta_y)
}

impl Problem for Problem9 {
    fn new(input: &str) -> Self where Self: Sized {
        let moves = input.lines().map(|l| Move::new(l)).collect();
        Problem9 { moves }
    }

    fn solve_part_1(&self) -> String {
        let mut set = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        let mut t_pos = (0, 0);

        set.insert(t_pos);
        for m in &self.moves {
            for _ in 0..m.count {
                match m.direction {
                    Direction::R => {
                        x += 1;
                    }
                    Direction::L => {
                        x -= 1;
                    }
                    Direction::U => {
                        y -= 1;
                    }
                    Direction::D => {
                        y += 1;
                    }
                }
                if distance(x, y, t_pos) > 1 {
                    let delta = get_position(t_pos, (x, y));
                    t_pos.0 += delta.0;
                    t_pos.1 += delta.1;
                    set.insert(t_pos);
                }
            }
        }
        format!("{}", set.len())
    }

    fn solve_part_2(&self) -> String {
        let mut set = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        let mut t_pos = (0, 0);

        set.insert(t_pos);
        for m in &self.moves {
            for _ in 0..m.count {
                match m.direction {
                    Direction::R => {
                        x += 1;
                    }
                    Direction::L => {
                        x -= 1;
                    }
                    Direction::U => {
                        y -= 1;
                    }
                    Direction::D => {
                        y += 1;
                    }
                }
                if distance(x, y, t_pos) > 1 {
                    let delta = get_position(t_pos, (x, y));
                    t_pos.0 += delta.0;
                    t_pos.1 += delta.1;
                    set.insert(t_pos);
                    println!("{},{} -> {:?}", x, y, t_pos);
                }
            }
        }
        format!("{}", set.len())
    }
}