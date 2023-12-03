mod days;
mod utils;

use days::{day01, day02, day03};
use utils::Problem;

enum Day {
    Day01(day01::Day01),
    Day02(day02::Day02),
    Day03(day03::Day03),
}

impl Day {
    fn part_one(&self, input: &str) -> utils::Solution {
        match self {
            Day::Day01(day) => day.part_one(input),
            Day::Day02(day) => day.part_one(input),
            Day::Day03(day) => day.part_one(input),
        }
    }
    fn part_two(&self, input: &str) -> utils::Solution {
        match self {
            Day::Day01(day) => day.part_two(input),
            Day::Day02(day) => day.part_two(input),
            Day::Day03(day) => day.part_two(input),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let day = args[1].parse::<u8>().unwrap();

    solve(day);
}

fn solve(day: u8) {
    let input = utils::read_input(&day);
    let problem = match day {
        1 => Day::Day01(day01::Day01),
        2 => Day::Day02(day02::Day02),
        3 => Day::Day03(day03::Day03),
        _ => panic!("Day {} not implemented yet", day),
    };
    let sol_1 = problem.part_one(&input);
    println!("Solution 1: {}", sol_1);
    let sol_2 = problem.part_two(&input);
    println!("Solution 2: {}", sol_2);
}
