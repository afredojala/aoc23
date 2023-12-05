use crate::utils::{Problem, Solution};
use std::collections::HashSet;

pub struct Day05;

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn contains(&self, value: u64) -> bool {
        value >= self.start && value <= self.end
    }

    fn contains_range(&self, range: &Range) -> bool {
        self.contains(range.start) && self.contains(range.end)
    }

    fn overlaps(&self, range: &Range) -> bool {
        self.contains(range.start) || self.contains(range.end)
    }

    fn get_overlap(&self, range: &Range) -> Range {
        Range::new(self.start.max(range.start), self.end.min(range.end))
    }

    fn get_non_overlap(&self, range: &Range) -> Vec<Range> {
        let mut non_overlaps: Vec<Range> = Vec::new();
        if self.start < range.start {
            non_overlaps.push(Range::new(self.start, range.start - 1));
        }
        if self.end > range.end {
            non_overlaps.push(Range::new(range.end + 1, self.end));
        }
        non_overlaps
    }

    fn min(&self) -> u64 {
        self.start.min(self.end)
    }
}

impl Problem for Day05 {
    fn part_two(&self, input: &str) -> Solution {
        let groups = input.split("\n\n").collect::<Vec<&str>>();

        let seed_range = groups[0].split(": ").collect::<Vec<&str>>()[1]
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
            .chunks(2)
            .map(|x| Range::new(x[0], x[0] + x[1] - 1))
            .collect::<Vec<Range>>();

        let mut locations: Vec<Range> = Vec::new();

        for range in seed_range.iter() {
            let mut group_ranges: Vec<Range> = Vec::new();
            group_ranges.push(range.clone());
            for group in groups[1..].iter() {
                let mut new_group_ranges: Vec<Range> = Vec::new();
                while let Some(group_range) = group_ranges.pop() {
                    let mut lines: Vec<&str> = group.lines().collect(); // Skip the header
                    lines.reverse();
                    let mut line_ranges: Vec<Range> = Vec::new();
                    line_ranges.push(group_range.clone());

                    let mut changes = false;
                    while let Some(line) = lines.pop() {
                        // println!("Line empty: {:?}", lines.is_empty());
                        let mut stop = false;
                        while let Some(line_range) = line_ranges.pop() {
                            // println!("Line ranges: {:?}", line_ranges);
                            // println!("Line Range: {:?}", line_range);
                            let numbers = line
                                .split_whitespace()
                                .map(|x| x.parse::<u64>().unwrap())
                                .collect::<Vec<u64>>();
                            let destination_number = numbers[0];
                            let source_number = numbers[1];
                            let range = numbers[2];
                            let range = Range::new(source_number, source_number + range - 1);
                            if range.contains_range(&line_range) {
                                let new_range = Range::new(
                                    line_range.start + destination_number - source_number,
                                    line_range.end + destination_number - source_number,
                                );
                                new_group_ranges.push(new_range);
                                stop = true;
                                changes = true;
                                break;
                            } else if range.overlaps(&line_range) {
                                let overlap = range.get_overlap(&line_range);
                                new_group_ranges.push(overlap);
                                let non_overlaps = range.get_non_overlap(&line_range);
                                for over in non_overlaps.iter() {
                                    line_ranges.push(over.clone());
                                }
                                changes = true;
                            }
                        }
                        if stop {
                            break;
                        } else {
                            line_ranges.push(group_range.clone());
                        }
                    }
                    if !changes {
                        new_group_ranges.push(group_range);
                    }
                }
                group_ranges = new_group_ranges;
            }
            locations.append(&mut group_ranges);
        }
        locations.sort_by(|a, b| a.min().cmp(&b.min()));
        Solution::U64(locations[0].min())
    }
    fn part_one(&self, input: &str) -> Solution {
        let groups = input.split("\n\n").collect::<Vec<&str>>();

        let seeds = groups[0].split(": ").collect::<Vec<&str>>()[1]
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let mut locations: Vec<_> = Vec::new();
        for seed in seeds.iter() {
            let mut current_val = *seed;
            for group in groups[1..].iter() {
                let mut lines = group.lines().skip(1); // Skip the header
                loop {
                    let next_line = lines.next();
                    if next_line.is_none() {
                        break;
                    }
                    let numbers = next_line
                        .unwrap()
                        .split_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    let destination_number = numbers[0];
                    let source_number = numbers[1];
                    let range = numbers[2];
                    if current_val >= source_number && current_val < source_number + range {
                        current_val = current_val + destination_number - source_number;
                        break;
                    }
                }
            }
            locations.push(current_val);
        }
        // gets minimum value of the vector locations

        Solution::U64(locations.iter().min().unwrap().clone() as u64)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    fn input() -> String {
        String::from(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        )
    }

    #[test]
    fn test_part_one() {
        let day = Day05 {};
        assert_eq!(day.part_one(&input()), Solution::U64(35));
    }

    #[test]
    fn test_part_two() {
        let day = Day05 {};
        assert_eq!(day.part_two(&input()), Solution::U64(46));
    }
}
