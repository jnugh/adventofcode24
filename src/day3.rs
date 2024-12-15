use regex::Regex;

pub fn day3_part1(input: String) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for mul in re.captures_iter(&input) {
        let (_, [number1, number2]) = mul.extract();
        sum += number1.parse::<usize>().unwrap() * number2.parse::<usize>().unwrap();
    }

    sum
}
pub fn day3_part2(input: String) -> usize {
    let dos: Vec<usize> = input.match_indices("do()").map(|(pos, _)| pos).collect();
    let donts: Vec<usize> = input.match_indices("don't()").map(|(pos, _)| pos).collect();

    let mut disabled_ranges = Vec::<(usize, usize)>::new();
    let mut last_do_index = 0;
    for dont_position in donts {
        let mut dont_end: Option<usize> = None;
        for (i, doit) in dos.iter().enumerate().skip(last_do_index) {
            if *doit > dont_position {
                last_do_index = i;
                dont_end = Some(*doit);
                break;
            }
        }

        disabled_ranges.push(match dont_end {
            Some(end) => (dont_position, end),
            None => (dont_position, input.len()),
        });
    }

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    let input_str = input.as_str();
    for mul in re.captures_iter(input_str) {
        let (complete_mul, [number1, number2]) = mul.extract();
        let offset = complete_mul.as_ptr() as usize - input_str.as_ptr() as usize;
        if disabled_ranges
            .clone()
            .into_iter()
            .any(|(start, end)| start < offset && end > offset)
        {
            continue;
        }
        sum += number1.parse::<usize>().unwrap() * number2.parse::<usize>().unwrap();
    }

    sum
}

#[cfg(test)]
mod test {
    use crate::day3::{day3_part1, day3_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(
            161,
            day3_part1(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .to_string()
            )
        )
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            48,
            day3_part2(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                    .to_string()
            )
        )
    }
}
