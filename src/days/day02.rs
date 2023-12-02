use crate::utils::{Problem, Solution};
use std::cmp::max;

pub struct Day02;

impl Day02 {
    fn check_rules(&self, colour: &str) -> bool {
        let mut splitted = colour.split_whitespace();
        let nbr = splitted.next().unwrap();
        let colour = splitted.next().unwrap();
        let nbr = nbr.parse::<u32>().unwrap();
        match colour {
            "red" => nbr <= 12,
            "green" => nbr <= 13,
            "blue" => nbr <= 14,
            _ => false,
        }
    }
}

impl Problem for Day02 {
    fn part_one(&self, input: &str) -> Solution {
        Solution::U32(
            input
                .lines()
                .map(|game| {
                    let mut test = game.split(":");
                    let game_nbr = test.next().unwrap().split_whitespace().nth(1).unwrap();
                    let game = test.next().unwrap().split(';').all(|set| {
                        set.split(',')
                            .map(|colour| self.check_rules(colour))
                            .all(|x| x == true)
                    });
                    if game {
                        game_nbr.parse::<u32>().unwrap()
                    } else {
                        0
                    }
                })
                .sum(),
        )
    }
    fn part_two(&self, input: &str) -> Solution {
        Solution::U32(
            input
                .lines()
                .map(|game| {
                    game.split(':')
                        .skip(1)
                        .next()
                        .unwrap()
                        .split(";")
                        .fold([0, 0, 0], |acc, set| {
                            let balls = set.trim().split(", ");
                            let mut prev = acc;
                            for ball in balls {
                                let mut splitted = ball.split_whitespace();
                                let nbr = splitted.next().unwrap();
                                let colour = splitted.next().unwrap();
                                println!("{} {}", nbr, colour);
                                let nbr = nbr.parse::<u32>().unwrap();
                                prev = match colour {
                                    "red" => [max(prev[0], nbr), prev[1], prev[2]],
                                    "green" => [prev[0], max(prev[1], nbr), prev[2]],
                                    "blue" => [prev[0], prev[1], max(prev[2], nbr)],
                                    _ => prev,
                                };
                                // println!("{:?}", prev);
                            }
                            prev
                        })
                        .iter()
                        .product::<u32>()
                })
                .sum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    }

    #[test]
    fn test_part_one() {
        let day = Day02 {};
        assert_eq!(
            day.part_one(input()).to_string(),
            Solution::U32(8).to_string()
        );
    }

    #[test]
    fn test_part_two() {
        let day = Day02 {};
        assert_eq!(
            day.part_two(input()).to_string(),
            Solution::U32(2286).to_string()
        );
    }
}
