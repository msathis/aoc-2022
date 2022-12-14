use crate::Problem;

pub struct Problem10 {
    commands: Vec<Option<i32>>,
}

impl Problem for Problem10 {
    fn new(input: &str) -> Self where Self: Sized {
        let commands = input.lines().map(|l| {
            match l.split_whitespace().next().unwrap() {
                "noop" => None,
                _ => Some(l.split_whitespace().skip(1).next().unwrap().parse::<i32>().unwrap())
            }
        }).collect();
        Problem10 {
            commands
        }
    }

    fn solve_part_1(&self) -> String {
        let mut x = 1;
        let mut strength: i32 = 0;
        let mut cycles = 0;
        self.commands.iter().for_each(|op| {
            cycles += 1;
            if cycles == 19 || cycles == 59 || cycles == 99 || cycles == 139 || cycles == 179 || cycles == 219 {
                strength += (cycles + 1) as i32 * x;
            }
            if op.is_some() {
                x += op.unwrap();
                cycles += 1;

                if cycles == 19 || cycles == 59 || cycles == 99 || cycles == 139 || cycles == 179 || cycles == 219 {
                    strength += (cycles + 1) as i32 * x;
                }
            }
        });
        format!("{}", strength)
    }

    fn solve_part_2(&self) -> String {
        format!("")
    }
}