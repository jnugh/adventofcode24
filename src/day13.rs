const A_PRESS_TOKENS: usize = 3;
const B_PRESS_TOKENS: usize = 1;

type Direction = (usize, usize);

enum Button {
    A,
    B,
}

#[derive(Debug)]
struct GameMachine {
    a_direction: Direction,
    b_direction: Direction,

    price_position: Direction,
}

impl GameMachine {
    fn from_input(input: &str, incorporate_unit_conversion_error: bool) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        Self {
            a_direction: GameMachine::parse_direction(lines[0], "+", false),
            b_direction: GameMachine::parse_direction(lines[1], "+", false),
            price_position: GameMachine::parse_direction(
                lines[2],
                "=",
                incorporate_unit_conversion_error,
            ),
        }
    }

    fn parse_direction(
        input: &str,
        coord_delimiter: &str,
        incorporate_unit_conversion_error: bool,
    ) -> Direction {
        let coordinates: Vec<usize> = input
            .split(": ")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|coordinate| {
                coordinate
                    .trim()
                    .split(coord_delimiter)
                    .nth(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    + match incorporate_unit_conversion_error {
                        true => 10000000000000,
                        false => 0,
                    }
            })
            .collect();

        (coordinates[0], coordinates[1])
    }

    fn get_minimum_tokens(&self) -> Option<usize> {
        let required_a_tokens = (self.price_position.0 / self.a_direction.0)
            .max(self.price_position.1 / self.a_direction.1)
            * A_PRESS_TOKENS;
        let required_b_tokens = (self.price_position.0 / self.b_direction.0)
            .max(self.price_position.1 / self.b_direction.1)
            * B_PRESS_TOKENS;

        let cheapest_button = match required_a_tokens.cmp(&required_b_tokens) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Button::A,
            std::cmp::Ordering::Greater => Button::B,
        };

        let cheap_direction = match cheapest_button {
            Button::A => self.a_direction,
            Button::B => self.b_direction,
        };
        let other_direction = match cheapest_button {
            Button::A => self.b_direction,
            Button::B => self.a_direction,
        };

        let start_count = (self.price_position.0 / cheap_direction.0)
            .min(self.price_position.1 / cheap_direction.1);

        let mut tests = (0..=start_count).rev();

        while let Some(cheap_presses) = tests.next() {
            let required_expensive_press =
                (self.price_position.0 - cheap_presses * cheap_direction.0) / other_direction.0;

            let diff = (
                self.price_position.0 as isize
                    - (cheap_presses * cheap_direction.0
                        + required_expensive_press * other_direction.0)
                        as isize,
                self.price_position.1 as isize
                    - (cheap_presses * cheap_direction.1
                        + required_expensive_press * other_direction.1)
                        as isize,
            );

            if diff == (0, 0) {
                return Some(match cheapest_button {
                    Button::A => {
                        cheap_presses * A_PRESS_TOKENS + required_expensive_press * B_PRESS_TOKENS
                    }
                    Button::B => {
                        cheap_presses * B_PRESS_TOKENS + required_expensive_press * A_PRESS_TOKENS
                    }
                });
            }

            let spread = diff.0.abs_diff(diff.1);
            let reduce = (spread / cheap_direction.0).min(spread / cheap_direction.1) / 100;
            if reduce > 1 {
                tests.nth(reduce - 1);
            }
        }

        None
    }
}

#[derive(Debug)]
struct Arcade {
    game_machines: Vec<GameMachine>,
}

impl Arcade {
    fn from_input(input: &str, incorporate_unit_conversion_error: bool) -> Self {
        Self {
            game_machines: input
                .split("\n\n")
                .map(|l| GameMachine::from_input(l, incorporate_unit_conversion_error))
                .collect(),
        }
    }

    fn get_minimum_tokens(&self) -> usize {
        self.game_machines
            .iter()
            .map(|machine| machine.get_minimum_tokens().unwrap_or(0))
            .sum()
    }
}

pub fn day13_part1(input: String) -> usize {
    let arcade = Arcade::from_input(&input, false);

    arcade.get_minimum_tokens()
}

pub fn day13_part2(input: String) -> usize {
    let arcade = Arcade::from_input(&input, true);

    arcade.get_minimum_tokens()
}

#[cfg(test)]
mod test {
    use crate::day13::{day13_part1, day13_part2};

    #[test]
    fn test_part1() {
        assert_eq!(
            480,
            day13_part1(
                r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#
                    .to_string()
            )
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            875318608908,
            day13_part2(
                r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#
                    .to_string()
            )
        )
    }
}
