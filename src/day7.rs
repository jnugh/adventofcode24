#[derive(Clone, Debug)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

struct Calibration {
    result: usize,
    numbers: Vec<usize>,
    operators: Option<Vec<Operator>>,

    count_of_one: usize,

    solvable: Option<bool>,
}

impl Calibration {
    fn parse_input(input: &str) -> Self {
        let parts: Vec<&str> = input.split(":").collect();
        let result: usize = parts[0].parse().unwrap();
        let numbers: Vec<usize> = parts[1]
            .trim()
            .split(" ")
            .map(|n| n.trim().parse().unwrap())
            .collect();

        let count_of_one = numbers.clone().into_iter().filter(|n| *n == 1).count();

        Self {
            result,
            numbers,
            count_of_one,
            operators: None,
            solvable: None,
        }
    }

    fn calculate_result(&self, operators: &[Operator]) -> usize {
        let mut result = self.numbers[0];
        for (index, num) in self.numbers.clone().into_iter().skip(1).enumerate() {
            let operator = &operators[index];
            match operator {
                Operator::Add => {
                    result += num;
                }
                Operator::Multiply => {
                    result *= num;
                }
                Operator::Concat => {
                    result = 10_usize.pow((num as f64).log10().floor() as u32 + 1) * result + num;
                }
            }
        }

        result
    }

    fn solve(&mut self, allow_concat: bool) {
        let mut operators = vec![Operator::Add; self.numbers.len() - 1];
        operators.fill(Operator::Add);

        let result = self.solve_recursive(allow_concat, operators, 0);
        self.solvable = Some(result.is_some());
        self.operators = result;
    }

    fn solve_recursive(
        &self,
        allow_concat: bool,
        operators: Vec<Operator>,
        pos: usize,
    ) -> Option<Vec<Operator>> {
        let current = self.calculate_result(&operators);
        if current == self.result {
            return Some(operators);
        }
        if current > self.result + self.count_of_one {
            return None;
        }
        for i in pos..operators.len() {
            let mut new_operators = operators.clone();
            new_operators[i] = Operator::Multiply;

            let recursive_result = self.solve_recursive(allow_concat, new_operators.clone(), i + 1);
            if recursive_result.is_some() {
                return recursive_result;
            }

            if allow_concat {
                new_operators[i] = Operator::Concat;

                let recursive_result = self.solve_recursive(allow_concat, new_operators, i + 1);
                if recursive_result.is_some() {
                    return recursive_result;
                }
            }
        }

        None
    }
}

fn parse_complete_input(input: String) -> Vec<Calibration> {
    input
        .trim()
        .split("\n")
        .map(Calibration::parse_input)
        .collect()
}

pub fn day7_part1(input: String) -> usize {
    let mut calibrations = parse_complete_input(input);

    let mut result = 0;

    for calibration in calibrations.iter_mut() {
        calibration.solve(false);

        if calibration.solvable == Some(true) {
            result += calibration.result;
        }
    }

    result
}

pub fn day7_part2(input: String) -> usize {
    let mut calibrations = parse_complete_input(input);

    let mut result = 0;

    for calibration in calibrations.iter_mut() {
        calibration.solve(true);

        if calibration.solvable == Some(true) {
            result += calibration.result;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use crate::day7::{day7_part1, day7_part2};

    use super::{Calibration, Operator};

    #[test]
    fn test_part1() {
        assert_eq!(
            3749,
            day7_part1(
                r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#
                    .to_string()
            )
        )
    }

    #[test]
    fn concat() {
        let test = Calibration {
            count_of_one: 0,
            numbers: vec![123, 45],
            operators: Some(vec![Operator::Concat]),
            result: 12345,
            solvable: Some(true),
        };

        assert_eq!(12345, test.calculate_result(&[Operator::Concat]));
    }

    #[test]
    fn concat_div10() {
        let test = Calibration {
            count_of_one: 0,
            numbers: vec![120, 45],
            operators: Some(vec![Operator::Concat]),
            result: 12045,
            solvable: Some(true),
        };

        assert_eq!(12045, test.calculate_result(&[Operator::Concat]));
    }

    #[test]
    fn concat_small() {
        let test = Calibration {
            count_of_one: 0,
            numbers: vec![120, 1],
            operators: Some(vec![Operator::Concat]),
            result: 12045,
            solvable: Some(true),
        };

        assert_eq!(1201, test.calculate_result(&[Operator::Concat]));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            11387,
            day7_part2(
                r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#
                    .to_string()
            )
        )
    }
}
