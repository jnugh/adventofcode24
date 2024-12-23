use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

struct Race {
    map: String,
    path: Vec<Position>,
    position_steps: HashMap<Position, usize>,

    start: Position,
    end: Position,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Wall,
    Empty,
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
struct Cheat {
    time_saved: usize,
}

impl Race {
    fn from_input(input: &str) -> Self {
        let height = input.trim().lines().count();
        let width = input.lines().nth(0).unwrap_or("").chars().count();
        let map = input.trim().replace("\n", "");

        let start_idx = map.find("S").expect("There is no start");
        let end_idx = map.find("E").expect("There is no end");

        let start = Position {
            x: start_idx % width,
            y: start_idx / width,
        };
        let end = Position {
            x: end_idx % width,
            y: end_idx / width,
        };

        let mut result = Race {
            map,
            path: Vec::new(),
            position_steps: HashMap::new(),
            start,
            end,
            width,
            height,
        };

        result.parse_path();

        result
    }

    fn parse_path(&mut self) {
        let mut position = self.start;
        self.position_steps.insert(position, 0);
        self.path.push(position);
        let mut visited = HashSet::new();
        visited.insert(position);

        while self.get_tile(position) != Tile::End {
            position = position + self.get_empty_direction(position, &visited);
            visited.insert(position);

            self.position_steps.insert(position, self.path.len());
            self.path.push(position);
        }
    }

    fn get_tile(&self, position: Position) -> Tile {
        match self.map.as_bytes()[position.y * self.height + position.x] {
            b'#' => Tile::Wall,
            b'S' => Tile::Start,
            b'E' => Tile::End,
            b'.' => Tile::Empty,
            t => panic!("Unknown tile type {}", t),
        }
    }

    fn get_empty_direction(&self, position: Position, visited: &HashSet<Position>) -> Direction {
        if position.y > 0
            && !visited.contains(&Position {
                x: position.x,
                y: position.y - 1,
            })
            && self.get_tile(Position {
                x: position.x,
                y: position.y - 1,
            }) != Tile::Wall
        {
            Direction::North
        } else if position.x < self.width - 1
            && !visited.contains(&Position {
                x: position.x + 1,
                y: position.y,
            })
            && self.get_tile(Position {
                x: position.x + 1,
                y: position.y,
            }) != Tile::Wall
        {
            Direction::East
        } else if position.y < self.height - 1
            && !visited.contains(&Position {
                x: position.x,
                y: position.y + 1,
            })
            && self.get_tile(Position {
                x: position.x,
                y: position.y + 1,
            }) != Tile::Wall
        {
            Direction::South
        } else if position.x > 0
            && !visited.contains(&Position {
                x: position.x - 1,
                y: position.y,
            })
            && self.get_tile(Position {
                x: position.x - 1,
                y: position.y,
            }) != Tile::Wall
        {
            Direction::West
        } else {
            self.print();
            panic!("There is no way out: {:?}", position)
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = Position { x, y };
                if self.start == position {
                    print!("S");
                } else if self.end == position {
                    print!("E");
                } else if self.position_steps.contains_key(&position) {
                    print!("X");
                } else if self.get_tile(position) == Tile::Wall {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }

    fn get_cheats(&self, len: usize) -> Vec<Cheat> {
        let mut cheats = Vec::new();

        for position in &self.path {
            let mut interesting_cheats = Vec::with_capacity(len ^ 2);
            for x in -(len as isize)..=len as isize {
                if (position.x as isize) < -x || x > 0 && (position.x + x as usize) >= self.width {
                    continue;
                }
                for y in -(len as isize)..=len as isize {
                    if (position.y as isize) < -y
                        || y > 0 && (position.y + y as usize) >= self.height
                    {
                        continue;
                    }
                    if (x.abs() + y.abs()) as usize > len {
                        continue;
                    }
                    let end_position = Position {
                        x: (position.x as isize + x) as usize,
                        y: (position.y as isize + y) as usize,
                    };
                    if self.get_tile(end_position) != Tile::Wall {
                        interesting_cheats.push((*position, end_position));
                    }
                }
            }

            for (start, end) in interesting_cheats {
                if let Some(start_steps) = self.position_steps.get(&start) {
                    if let Some(end_steps) = self.position_steps.get(&end) {
                        let distance = end.x.abs_diff(start.x) + end.y.abs_diff(start.y);
                        if *end_steps > (*start_steps + distance) {
                            cheats.push(Cheat {
                                time_saved: end_steps - start_steps - distance,
                            })
                        }
                    }
                }
            }
        }

        cheats
    }
}

pub fn day20_part1(input: String) -> usize {
    let race = Race::from_input(&input);

    let cheats = race.get_cheats(2);

    cheats
        .iter()
        .filter(|cheat| cheat.time_saved >= 100)
        .count()
}

pub fn day20_part2(input: String) -> usize {
    let race = Race::from_input(&input);

    let cheats = race.get_cheats(20);

    cheats
        .iter()
        .filter(|cheat| cheat.time_saved >= 100)
        .count()
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::Race;

    #[test]
    fn test_part1() {
        let race = Race::from_input(
            r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#,
        );
        let cheats = race.get_cheats(2);
        let counted = cheats
            .iter()
            .map(|cheat| (cheat.time_saved, cheat))
            .into_group_map();

        assert_eq!(14, counted.get(&2).unwrap_or(&vec![]).iter().count());
        assert_eq!(14, counted.get(&4).unwrap_or(&vec![]).iter().count());
        assert_eq!(2, counted.get(&6).unwrap_or(&vec![]).iter().count());
        assert_eq!(4, counted.get(&8).unwrap_or(&vec![]).iter().count());
        assert_eq!(2, counted.get(&10).unwrap_or(&vec![]).iter().count());
        assert_eq!(3, counted.get(&12).unwrap_or(&vec![]).iter().count());
        assert_eq!(1, counted.get(&20).unwrap_or(&vec![]).iter().count());
        assert_eq!(1, counted.get(&36).unwrap_or(&vec![]).iter().count());
        assert_eq!(1, counted.get(&38).unwrap_or(&vec![]).iter().count());
        assert_eq!(1, counted.get(&40).unwrap_or(&vec![]).iter().count());
        assert_eq!(1, counted.get(&64).unwrap_or(&vec![]).iter().count());
    }

    #[test]
    fn test_part2() {
        let race = Race::from_input(
            r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#,
        );
        let cheats = race.get_cheats(20);
        let counted = cheats
            .iter()
            .filter(|cheat| cheat.time_saved >= 50)
            .map(|cheat| (cheat.time_saved, cheat))
            .into_group_map();

        assert_eq!(32, counted.get(&50).unwrap_or(&vec![]).iter().count());
        assert_eq!(31, counted.get(&52).unwrap_or(&vec![]).iter().count());
        assert_eq!(29, counted.get(&54).unwrap_or(&vec![]).iter().count());
        assert_eq!(39, counted.get(&56).unwrap_or(&vec![]).iter().count());
        assert_eq!(25, counted.get(&58).unwrap_or(&vec![]).iter().count());
        assert_eq!(23, counted.get(&60).unwrap_or(&vec![]).iter().count());
        assert_eq!(20, counted.get(&62).unwrap_or(&vec![]).iter().count());
        assert_eq!(19, counted.get(&64).unwrap_or(&vec![]).iter().count());
        assert_eq!(12, counted.get(&66).unwrap_or(&vec![]).iter().count());
        assert_eq!(14, counted.get(&68).unwrap_or(&vec![]).iter().count());
        assert_eq!(12, counted.get(&70).unwrap_or(&vec![]).iter().count());
        assert_eq!(22, counted.get(&72).unwrap_or(&vec![]).iter().count());
        assert_eq!(4, counted.get(&74).unwrap_or(&vec![]).iter().count());
        assert_eq!(3, counted.get(&76).unwrap_or(&vec![]).iter().count());
    }
}
