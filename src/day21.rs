use std::{collections::HashMap, ops::Add, vec};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

struct Vector {
    direction: Direction,
    len: usize,
}

impl Add<Vector> for Position {
    type Output = Position;

    fn add(self, rhs: Vector) -> Self::Output {
        match rhs.direction {
            Direction::Right => Position {
                x: self.x + rhs.len as isize,
                y: self.y,
            },
            Direction::Left => Position {
                x: self.x - rhs.len as isize,
                y: self.y,
            },
            Direction::Up => Position {
                x: self.x,
                y: self.y - rhs.len as isize,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + rhs.len as isize,
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Keypad {
    keys: HashMap<char, Position>,
    position: Position,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_key(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

impl Keypad {
    fn from_input(input: &str) -> Self {
        let mut keys = HashMap::new();
        let rows = input.lines().enumerate();
        let mut position: Option<Position> = None;
        for (y, row) in rows {
            for (x, char) in row.chars().enumerate() {
                keys.insert(
                    char,
                    Position {
                        x: x as isize,
                        y: y as isize,
                    },
                );

                if char == 'A' {
                    position = Some(Position {
                        x: x as isize,
                        y: y as isize,
                    });
                }
            }
        }

        let position = position.unwrap_or(Position { x: 0, y: 0 });
        Self { keys, position }
    }

    fn create_num_pad() -> Self {
        Self::from_input(
            r#"789
456
123
 0A"#,
        )
    }

    fn create_direction_pad() -> Self {
        Self::from_input(
            r#" ^A
<v>"#,
        )
    }

    fn get_steps(&self, key: char) -> Vec<Direction> {
        let target = self.keys.get(&key).unwrap();

        let x_direction = if target.x > self.position.x {
            Direction::Right
        } else {
            Direction::Left
        };
        let y_direction = if target.y > self.position.y {
            Direction::Down
        } else {
            Direction::Up
        };
        let x_steps = target.x.abs_diff(self.position.x);
        let y_steps = target.y.abs_diff(self.position.y);

        let empty_position = self.keys.get(&' ').unwrap();

        let intermediate_x = self.position
            + Vector {
                direction: x_direction,
                len: x_steps,
            };
        let intermediate_y = self.position
            + Vector {
                direction: y_direction,
                len: y_steps,
            };

        if intermediate_x == *empty_position {
            let mut path = vec![y_direction; y_steps];
            path.extend(vec![x_direction; x_steps]);
            path
        } else if intermediate_y == *empty_position || x_direction == Direction::Left {
            let mut path = vec![x_direction; x_steps];
            path.extend(vec![y_direction; y_steps]);
            path
        } else {
            let mut path = vec![y_direction; y_steps];
            path.extend(vec![x_direction; x_steps]);
            path
        }
    }

    fn move_to_key(&mut self, key: char) {
        self.position = *self.keys.get(&key).unwrap()
    }
}

#[derive(Debug)]
struct KeypadChain {
    num_pad: Keypad,
    indirections: usize,

    cache: HashMap<(Vec<char>, usize), usize>,
}

#[derive(Debug)]
struct CodeEntry {
    code: String,
    keypads: KeypadChain,
}

impl CodeEntry {
    fn from_input(input: &str, indirections: usize) -> Self {
        let code = input.trim().to_string();
        Self {
            code,
            keypads: KeypadChain::from_indirections(indirections),
        }
    }

    fn input_len(&mut self) -> usize {
        let mut sum = 0;
        for button in self.code.chars() {
            sum += self.keypads.press(button);
        }

        sum
    }

    fn code_number(&self) -> usize {
        self.code.replace("A", "").parse().unwrap()
    }
}

impl KeypadChain {
    fn from_indirections(indirections: usize) -> Self {
        let num_pad: Keypad = Keypad::create_num_pad();

        Self {
            num_pad,
            indirections,
            cache: HashMap::new(),
        }
    }

    fn press(&mut self, key: char) -> usize {
        let mut path: Vec<char> = self
            .num_pad
            .get_steps(key)
            .iter()
            .map(|dir| dir.get_key())
            .collect();
        self.num_pad.move_to_key(key);
        path.push('A');

        self.resolve_paths2(&path, self.indirections)
    }

    fn resolve_paths2(&mut self, path: &[char], n: usize) -> usize {
        if let Some(result) = self.cache.get(&(path.to_vec(), n)) {
            return *result;
        }
        if n == 0 {
            path.len()
        } else {
            let mut keypad = Keypad::create_direction_pad();
            let mut sum = 0;
            let keys = path.iter().flat_map(|key| {
                let result = keypad
                    .get_steps(*key)
                    .iter()
                    .map(|direction| direction.get_key())
                    .chain(vec!['A'])
                    .collect::<Vec<_>>();
                keypad.move_to_key(*key);
                result
            });

            let mut series = vec![];
            for key in keys {
                series.push(key);
                if key == 'A' {
                    sum += self.resolve_paths2(&series, n - 1);
                    series = vec![];
                }
            }

            self.cache.insert((path.to_vec(), n), sum);
            sum
        }
    }
}

pub fn day21_part1(input: String) -> usize {
    let mut code_entries = input
        .trim()
        .lines()
        .map(|line| CodeEntry::from_input(line, 2));

    let mut sum = 0;
    for mut code_entry in &mut code_entries {
        sum += code_entry.input_len() * code_entry.code_number();
    }

    sum
}

pub fn day21_part2(input: String) -> usize {
    let mut code_entries = input
        .trim()
        .lines()
        .map(|line| CodeEntry::from_input(line, 25));

    let mut sum = 0;
    for mut code_entry in &mut code_entries {
        sum += code_entry.input_len() * code_entry.code_number();
    }

    sum
}

#[cfg(test)]
mod test {
    use crate::day21::{day21_part1, KeypadChain};

    #[test]
    fn test_zero_indirection() {
        let mut keypad_chain = KeypadChain::from_indirections(0);

        let result: usize = [
            keypad_chain.press('0'),
            keypad_chain.press('2'),
            keypad_chain.press('9'),
            keypad_chain.press('A'),
        ]
        .iter()
        .sum();

        assert_eq!("<A^A>^^AvvvA".len(), result);
    }

    #[test]
    fn test_one_indirection() {
        let mut keypad_chain = KeypadChain::from_indirections(1);

        let result: usize = [
            keypad_chain.press('0'),
            keypad_chain.press('2'),
            keypad_chain.press('9'),
            keypad_chain.press('A'),
        ]
        .iter()
        .sum();
        assert_eq!("v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len(), result,);
    }

    #[test]
    fn test_two_indirections() {
        let mut keypad_chain = KeypadChain::from_indirections(2);

        let result: usize = [
            keypad_chain.press('0'),
            keypad_chain.press('2'),
            keypad_chain.press('9'),
            keypad_chain.press('A'),
        ]
        .iter()
        .sum();
        assert_eq!(
            "<vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A".len(),
            result,
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            126384,
            day21_part1(
                r#"029A
980A
179A
456A
379A"#
                    .to_string()
            )
        )
    }

    #[test]
    fn test_part1_input2() {
        let mut keypad_chain = KeypadChain::from_indirections(2);

        let result: usize = [
            keypad_chain.press('9'),
            keypad_chain.press('8'),
            keypad_chain.press('0'),
            keypad_chain.press('A'),
        ]
        .iter()
        .sum();
        assert_eq!(
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len(),
            result,
        );
    }

    #[test]
    fn test_part1_input3() {
        let mut keypad_chain = KeypadChain::from_indirections(2);

        let result: usize = [
            keypad_chain.press('1'),
            keypad_chain.press('7'),
            keypad_chain.press('9'),
            keypad_chain.press('A'),
        ]
        .iter()
        .sum();
        assert_eq!(
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len(),
            result,
        );
    }

    #[test]
    fn test_part1_input4() {
        let mut keypad_chain = KeypadChain::from_indirections(2);

        let result: usize = [
            keypad_chain.press('4'),
            keypad_chain.press('5'),
            keypad_chain.press('6'),
            keypad_chain.press('A'),
        ]
        .iter()
        .sum();
        assert_eq!(
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len(),
            result,
        );
    }

    #[test]
    fn test_part1_input5() {
        let mut keypad_chain = KeypadChain::from_indirections(2);

        let result: usize = [
            keypad_chain.press('3'),
            keypad_chain.press('7'),
            keypad_chain.press('9'),
            keypad_chain.press('A'),
        ]
        .iter()
        .sum();
        assert_eq!(
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len(),
            result
        );
    }
}
