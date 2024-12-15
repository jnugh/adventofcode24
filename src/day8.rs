use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Sub},
};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Position> for i32 {
    type Output = Position;

    fn mul(self, rhs: Position) -> Self::Output {
        Position {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
struct Map {
    antennas: HashMap<char, HashSet<Position>>,
    antinodes: HashSet<Position>,

    width: i32,
    height: i32,
}

impl Map {
    fn parse_input(input: String) -> Self {
        let mut antennas: HashMap<char, HashSet<Position>> = HashMap::new();
        for (y, line) in input.split("\n").enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char != '.' {
                    if let Some(positions) = antennas.get_mut(&char) {
                        positions.insert(Position {
                            x: x as i32,
                            y: y as i32,
                        });
                    } else {
                        antennas.insert(
                            char,
                            HashSet::from_iter(vec![Position {
                                x: x as i32,
                                y: y as i32,
                            }]),
                        );
                    }
                }
            }
        }

        Self {
            antennas,
            antinodes: HashSet::new(),

            width: input.split("\n").nth(0).unwrap().chars().count() as i32,
            height: input.trim().split("\n").count() as i32,
        }
    }

    fn place_antinodes(&mut self, repeating_pattern: bool) {
        for antenna_positions in self.antennas.values() {
            for antenna_pair in antenna_positions.iter().permutations(2) {
                let antenna1 = *antenna_pair[0];
                let antenna2 = *antenna_pair[1];

                let distance = antenna2 - antenna1;

                if repeating_pattern {
                    for i in 0..self.width.max(self.height) {
                        let antinode1 = antenna1 - (i * distance);
                        let antinode2 = antenna2 + (i * distance);

                        if self.check_bounds(antinode1) {
                            self.antinodes.insert(antinode1);
                        }
                        if self.check_bounds(antinode2) {
                            self.antinodes.insert(antinode2);
                        }
                    }
                } else {
                    let antinode1 = antenna1 - distance;
                    let antinode2 = antenna2 + distance;

                    if self.check_bounds(antinode1) {
                        self.antinodes.insert(antinode1);
                    }
                    if self.check_bounds(antinode2) {
                        self.antinodes.insert(antinode2);
                    }
                }
            }
        }
    }

    fn check_bounds(&self, position: Position) -> bool {
        position.x >= 0 && position.x < self.width && position.y >= 0 && position.y < self.height
    }
}

pub fn day8_part1(input: String) -> usize {
    let mut map = Map::parse_input(input);

    map.place_antinodes(false);

    map.antinodes.len()
}

pub fn day8_part2(input: String) -> usize {
    let mut map = Map::parse_input(input);

    map.place_antinodes(true);

    map.antinodes.len()
}

#[cfg(test)]
mod test {
    use crate::day8::{day8_part1, day8_part2};

    #[test]
    fn test_part1() {
        assert_eq!(
            14,
            day8_part1(
                r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#
                    .to_string()
            )
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            34,
            day8_part2(
                r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#
                    .to_string()
            )
        )
    }
}
