use crate::utils::{Problem, Solution};
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

pub struct Day03;

// #[derive(Debug)]
// pub struct Number {
//     value: u32,
//     start_idx: usize,
//     end_idx: usize,
// }

// impl Number {
//     fn new(value: u32, start_idx: usize, end_idx: usize) -> Self {
//         Self {
//             value,
//             start_idx,
//             end_idx,
//         }
//     }
// }

impl Problem for Day03 {
    fn part_one(&self, input: &str) -> Solution {
        let lines: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut special_chars_coords: HashSet<(usize, usize)> = HashSet::new();
        let mut numbers: HashMap<(usize, usize, usize), u32> = HashMap::new();
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                let c = lines[i][j];
                if c != '.' && !c.is_digit(10) {
                    special_chars_coords.insert((i, j));
                }
            }
        }

        for special_char_chord in special_chars_coords.iter() {
            let start_i = max(special_char_chord.0 as i32 - 1, 0) as usize;
            let end_i = min(special_char_chord.0 + 1, lines.len() - 1);
            let start_j = max(special_char_chord.1 as i32 - 1, 0) as usize;
            let end_j = min(special_char_chord.1 + 1, lines[0].len() - 1);

            for i in start_i..=end_i {
                for j in start_j..=end_j {
                    if i == special_char_chord.0 && j == special_char_chord.1 {
                        continue;
                    }
                    let c = lines[i][j];
                    match c {
                        '0'..='9' => {
                            let mut start_idx = j;
                            let mut end_idx = j;
                            let mut before = String::new();
                            for prev in lines[i][0..j].iter().rev() {
                                if !prev.is_digit(10) {
                                    break;
                                }
                                start_idx -= 1;
                                before = format!("{}{}", prev, before);
                            }
                            let mut after = String::new();
                            for next in lines[i][j + 1..].iter() {
                                if !next.is_digit(10) {
                                    break;
                                }
                                end_idx += 1;
                                after = format!("{}{}", after, next);
                            }
                            let number = format!("{}{}{}", before, c, after);
                            println!("{} {} {} {}", i, start_idx, end_idx, number);
                            numbers.insert((i, start_idx, end_idx), number.parse::<u32>().unwrap());
                        }
                        _ => {}
                    }
                }
            }
        }
        return Solution::U32(numbers.values().sum());
    }
    fn part_two(&self, input: &str) -> Solution {
        let lines: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut special_chars_coords: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                let c = lines[i][j];
                if c == '*' {
                    special_chars_coords.insert((i, j));
                }
            }
        }
        let mut sum = 0;
        for special_char_chord in special_chars_coords.iter() {
            let start_i = max(special_char_chord.0 as i32 - 1, 0) as usize;
            let end_i = min(special_char_chord.0 + 1, lines.len() - 1);
            let start_j = max(special_char_chord.1 as i32 - 1, 0) as usize;
            let end_j = min(special_char_chord.1 + 1, lines[0].len() - 1);

            let mut matches: HashSet<(usize, usize, usize, u32)> = HashSet::new();
            for i in start_i..=end_i {
                for j in start_j..=end_j {
                    if i == special_char_chord.0 && j == special_char_chord.1 {
                        continue;
                    }
                    let c = lines[i][j];
                    match c {
                        '0'..='9' => {
                            let mut start_idx = j;
                            let mut end_idx = j;
                            let mut before = String::new();
                            for prev in lines[i][0..j].iter().rev() {
                                if !prev.is_digit(10) {
                                    break;
                                }
                                start_idx -= 1;
                                before = format!("{}{}", prev, before);
                            }
                            let mut after = String::new();
                            for next in lines[i][j + 1..].iter() {
                                if !next.is_digit(10) {
                                    break;
                                }
                                end_idx += 1;
                                after = format!("{}{}", after, next);
                            }
                            let number = format!("{}{}{}", before, c, after);
                            let number = number.parse::<u32>().unwrap();
                            matches.insert((i, start_idx, end_idx, number));
                        }
                        _ => {}
                    }
                }
            }
            if matches.len() > 1 {
                let test = matches.iter().fold(1, |acc, (_, _, _, f)| acc * f);
                sum += test;
            }
        }
        return Solution::U32(sum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
    }

    #[test]
    fn test_part_one() {
        let day = Day03 {};
        assert_eq!(
            day.part_one(input()).to_string(),
            Solution::U32(4361).to_string()
        );
    }

    #[test]
    fn test_part_two() {
        let day = Day03 {};
        assert_eq!(
            day.part_two(input()).to_string(),
            Solution::U32(467835).to_string()
        );
    }
}
