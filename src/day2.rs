use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};

use log::error;

struct Report {
    levels: Vec<usize>,
}

#[derive(Debug)]
enum ReportParsingError {
    ParseIntError,
}

impl From<ParseIntError> for ReportParsingError {
    fn from(value: ParseIntError) -> Self {
        error!("Failed to parse input: {value}");
        Self::ParseIntError
    }
}

impl Report {
    fn is_safe(&self, enable_problem_dampener: bool) -> bool {
        assert!(self.levels.len() > 1);

        let is_ascending = self.levels[1] > self.levels[0];
        let mut i = 0;

        for pair in self.levels.clone().into_iter().tuple_windows::<(_, _)>() {
            if is_ascending && pair.1 < pair.0 {
                if enable_problem_dampener {
                    return self.is_save_with_problem_dampener(i);
                } else {
                    return false;
                }
            }
            if !is_ascending && pair.1 > pair.0 {
                if enable_problem_dampener {
                    return self.is_save_with_problem_dampener(i);
                } else {
                    return false;
                }
            }
            let distance = pair.0.abs_diff(pair.1);
            if distance < 1 || distance > 3 {
                if enable_problem_dampener {
                    return self.is_save_with_problem_dampener(i);
                } else {
                    return false;
                }
            }
            i += 1;
        }

        true
    }

    fn is_save_with_problem_dampener(&self, index: usize) -> bool {
        (index > 0 && self.is_safe_with_removal(index - 1))
            || self.is_safe_with_removal(index)
            || self.is_safe_with_removal(index + 1)
    }

    fn is_safe_with_removal(&self, index: usize) -> bool {
        if self.levels.len() <= index {
            return false;
        }
        let mut new_levels = self.levels.clone();
        new_levels.remove(index);

        Report { levels: new_levels }.is_safe(false)
    }
}

impl FromStr for Report {
    type Err = ReportParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split(" ")
            .map(|v| v.parse::<usize>())
            .collect::<Result<Vec<usize>, ParseIntError>>()?;
        Ok(Self { levels })
    }
}

pub fn day2_part1(input: String) -> usize {
    let reports = parse_input(input).unwrap();

    let result = reports
        .into_iter()
        .filter(|report| report.is_safe(false))
        .count();

    result
}

pub fn day2_part2(input: String) -> usize {
    let reports = parse_input(input).unwrap();

    let result = reports
        .into_iter()
        .filter(|report| report.is_safe(true))
        .count();

    result
}

fn parse_input(input: String) -> Result<Vec<Report>, ReportParsingError> {
    let reports = input.trim().split("\n");
    let reports = reports.map(Report::from_str);

    reports.collect()
}
