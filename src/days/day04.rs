use crate::utils::{Problem, Solution};
use std::cmp::min;
use std::collections::{HashMap, HashSet};

pub struct Day04;

impl Problem for Day04 {
    fn part_one(&self, input: &str) -> Solution {
        let lines = input.lines().collect::<Vec<&str>>();
        let mut sum = 0;
        for line in lines {
            let mut split = line.split("|");
            let winning_nbr: HashSet<u32> = split
                .next()
                .unwrap()
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            sum += split.next().unwrap().split_whitespace().fold(0, |acc, x| {
                if winning_nbr.contains(&x.parse::<u32>().unwrap()) {
                    if acc == 0 {
                        1
                    } else {
                        acc * 2
                    }
                } else {
                    acc
                }
            });
            println!("{:?}", sum);
        }

        Solution::U32(sum)
    }

    fn part_two(&self, input: &str) -> Solution {
        let lines = input.lines().collect::<Vec<&str>>();
        let max_lines = lines.len();
        let mut extra_cards: HashMap<usize, usize> = (0..max_lines).map(|x| (x, 1)).collect();
        for (i, line) in lines.iter().enumerate() {
            let mut split = line.split("|");
            let winning_nbr: HashSet<u32> = split
                .next()
                .unwrap()
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            let nbr_winning_cards = split.next().unwrap().split_whitespace().fold(0, |acc, x| {
                if winning_nbr.contains(&x.parse::<u32>().unwrap()) {
                    acc + 1
                } else {
                    acc
                }
            });
            let start_idx = min(i + 1, max_lines);
            for j in start_idx..min(start_idx + nbr_winning_cards, max_lines) {
                if j >= max_lines {
                    break;
                }
                match extra_cards.get(&j) {
                    Some(x) => {
                        extra_cards.insert(j, x + extra_cards.get(&i).unwrap());
                    }
                    None => {
                        extra_cards.insert(j, 1);
                    }
                }
            }
        }
        let sum = extra_cards.values().sum::<usize>();

        Solution::U32(sum as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> String {
        String::from(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        )
    }

    #[test]
    fn test_part_one() {
        let day = Day04 {};
        assert_eq!(day.part_one(&input()).to_string(), String::from("13"));
    }

    #[test]
    fn test_part_two() {
        let day = Day04 {};
        assert_eq!(day.part_two(&input()).to_string(), String::from("30"));
    }
}
