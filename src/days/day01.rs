use crate::utils::{Problem, Solution};
pub struct Day01;

fn convert_number_to_string(number_string: &str) -> String {
    match number_string {
        "one" => "1".to_string(),
        "two" => "2".to_string(),
        "three" => "3".to_string(),
        "four" => "4".to_string(),
        "five" => "5".to_string(),
        "six" => "6".to_string(),
        "seven" => "7".to_string(),
        "eight" => "8".to_string(),
        "nine" => "9".to_string(),
        _ => number_string.to_string(),
    }
}

impl Problem for Day01 {
    fn part_one(&self, input: &str) -> Solution {
        let lines = input.lines();
        let results: u32 = lines
            .map(|line| {
                u32::from_str_radix(
                    &format!(
                        "{}{}",
                        line.chars().find(|&line| line.is_digit(10)).unwrap(),
                        line.chars().rev().find(|&line| line.is_digit(10)).unwrap(),
                    ),
                    10,
                )
                .unwrap()
            })
            .sum();
        return Solution::U32(results);
    }
    fn part_two(&self, input: &str) -> Solution {
        let all_splits = vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ];
        Solution::U32(
            input
                .lines()
                .map(|line| {
                    u32::from_str_radix(
                        &format!(
                            "{}{}",
                            convert_number_to_string(
                                all_splits
                                    .iter()
                                    .rev()
                                    .map(|word| (
                                        word,
                                        match line.find(word) {
                                            Some(x) => x,
                                            None => 1000,
                                        }
                                    ))
                                    .min_by(|x, y| x.1.cmp(&y.1))
                                    .unwrap()
                                    .0
                            ),
                            convert_number_to_string(
                                all_splits
                                    .iter()
                                    .rev()
                                    .map(|word| (
                                        word,
                                        match line.rfind(*word) {
                                            Some(x) => x + word.len(),
                                            None => 0,
                                        }
                                    ))
                                    .max_by(|x, y| x.1.cmp(&y.1))
                                    .unwrap()
                                    .0
                            ),
                        ),
                        10,
                    )
                    .unwrap()
                })
                .sum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input_1() -> String {
        String::from(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        )
    }

    fn test_input_2() -> String {
        String::from(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
one77ninetwoseventhreedqljsvj5oneightv",
        )
    }

    #[test]
    fn test_part_one() {
        let input_1 = test_input_1();
        let input_2 = test_input_2();

        let problem = Day01 {};
        let test_sol1 = problem.part_one(&input_1);
        assert_eq!(test_sol1.to_string(), "142");

        let test_sol2 = problem.part_two(&input_2);
        assert_eq!(test_sol2.to_string(), "299");
    }
}
