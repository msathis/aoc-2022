extern crate core;

mod aoc;

use std::env;
use std::fs;
use aoc::day1::Problem1;
use aoc::day2::Problem2;
use aoc::day3::Problem3;
use aoc::day4::Problem4;
use crate::aoc::day10::Problem10;
use crate::aoc::day5::Problem5;
use crate::aoc::day6::Problem6;
use crate::aoc::day7::Problem7;
use crate::aoc::day8::Problem8;
use crate::aoc::day9::Problem9;

trait Problem {
    fn new(input: &str) -> Self where Self: Sized;
    fn solve_part_1(&self) -> String;
    fn solve_part_2(&self) -> String;
}


fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the user provided a problem number as the first command line argument
    if args.len() > 1 {
        let problem_number = args[1].parse::<u32>().unwrap();

        // Call the function to solve the specified problem
        solve_problem(problem_number);
    } else {
        // If no problem number was provided, print an error message
        println!("Error: Please specify a problem number as the first argument.");
    }
}

fn solve_problem(problem_number: u32) {
    let input = fs::read_to_string(format!("src/aoc/day{}/input.txt", problem_number))
        .expect("Failed to read input file");

    let problem: Box<dyn Problem> = match problem_number {
        1 => Box::new(Problem1::new(&input)),
        2 => Box::new(Problem2::new(&input)),
        3 => Box::new(Problem3::new(&input)),
        4 => Box::new(Problem4::new(&input)),
        5 => Box::new(Problem5::new(&input)),
        6 => Box::new(Problem6::new(&input)),
        7 => Box::new(Problem7::new(&input)),
        8 => Box::new(Problem8::new(&input)),
        9 => Box::new(Problem9::new(&input)),
        10 => Box::new(Problem10::new(&input)),
        // Add more cases here to solve more problems...
        _ => panic!("Invalid problem number"),
    };

    // Solve the first part of the problem
    let solution_1 = problem.solve_part_1();

    // Solve the second part of the problem
    let solution_2 = problem.solve_part_2();

    // Print the solutions to the console
    println!("Part 1 solution: {}", solution_1);
    println!("Part 2 solution: {}", solution_2);
}