use std::fmt::{Display, Error, Formatter};
use std::fs;
pub fn read_input(day: &u8) -> String {
    let path = format!("input/day{}.txt", day);
    fs::read_to_string(path).expect("Something went wrong reading the file")
}

#[derive(Debug, Clone)]
pub enum Solution {
    U32(u32),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Solution::U32(value) => write!(f, "{}", value),
        }
    }
}

impl From<u32> for Solution {
    fn from(value: u32) -> Self {
        Solution::U32(value)
    }
}
pub trait Problem {
    fn part_one(&self, input: &str) -> Solution;
    fn part_two(&self, input: &str) -> Solution;
}
