use std::{
    collections::HashSet,
    ops::Add,
    thread::{self, JoinHandle},
};

use log::{info, warn};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

type Position = (isize, isize);

#[derive(Debug)]
struct Map {
    obstacles: HashSet<Position>,

    guard_position: Option<Position>,
    guard_start_position: Position,
    guard_direction: Direction,
    guard_start_direction: Direction,

    width: isize,
    height: isize,

    visited: HashSet<Position>,
}

impl Direction {
    fn get_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    fn rotate(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        let offset = rhs.get_offset();
        (self.0 + offset.0, self.1 + offset.1)
    }
}

impl Map {
    fn parse_input(input: String) -> Self {
        let mut obstacles = HashSet::<Position>::new();
        let mut guard_position: Option<Position> = None;
        let mut width = 0;

        let lines = input.split("\n");
        let height = lines.clone().count() as isize;

        for (row_index, row) in lines.enumerate() {
            if width == 0 {
                width = row.len() as isize;
            }
            let row_index = row_index as isize;
            for (col_index, value) in row.chars().enumerate() {
                let col_index = col_index as isize;
                if value == '#' {
                    obstacles.insert((col_index, row_index));
                }
                if value == '^' {
                    guard_position = Some((col_index, row_index));
                }
            }
        }

        Self {
            obstacles,
            visited: HashSet::<Position>::new(),
            guard_position,
            guard_start_position: guard_position.unwrap(),
            guard_direction: Direction::North,
            guard_start_direction: Direction::North,

            height,
            width,
        }
    }

    fn simulate_all_steps(&mut self) {
        let max_iterations = 999_999_999;

        for i in 0..max_iterations {
            if let Some(guard_position) = self.guard_position {
                let new_position = guard_position + self.guard_direction;

                if !self.check_bounds(new_position) {
                    self.guard_position = None;
                    info!("Done after {i} iterations");
                    return;
                } else if self.obstacles.contains(&new_position) {
                    self.guard_direction = self.guard_direction.rotate();
                } else {
                    self.visited.insert(new_position);
                    self.guard_position = Some(new_position);
                }
            }
        }

        warn!("Abort after {max_iterations} iterations");
    }

    fn check_bounds(&self, position: Position) -> bool {
        position.0 >= 0 && position.1 >= 0 && position.0 < self.width && position.1 < self.height
    }

    fn check_for_loops(&self) -> usize {
        let mut threads: Vec<JoinHandle<usize>> = Vec::with_capacity(self.height as usize);

        for y in 0..self.height {
            let template = Map {
                guard_direction: self.guard_direction,
                guard_position: self.guard_position,
                guard_start_direction: self.guard_start_direction,
                guard_start_position: self.guard_start_position,
                height: self.height,
                obstacles: self.obstacles.clone(),
                visited: HashSet::new(),
                width: self.width,
            };
            let thread = thread::spawn(move || {
                let mut sum: usize = 0;

                for x in 0..template.width {
                    let mut obstacles = template.obstacles.clone();
                    obstacles.insert((x, y));

                    let mut simulation = Map {
                        guard_direction: template.guard_direction,
                        guard_position: template.guard_position,
                        guard_start_direction: template.guard_start_direction,
                        guard_start_position: template.guard_start_position,
                        height: template.height,
                        obstacles,
                        visited: HashSet::new(),
                        width: template.width,
                    };

                    if simulation.check_on_loop_path() {
                        sum += 1;
                    }
                }
                sum
            });
            threads.push(thread);
        }

        let mut total = 0;
        for thread in threads {
            total += thread.join().unwrap();
        }
        total
    }

    fn render(&self) {
        for y in 0..self.height {
            let mut output = String::new();
            for x in 0..self.width {
                if self.guard_start_position == (x, y) {
                    output += match self.guard_start_direction {
                        Direction::North => "↑",
                        Direction::East => "→",
                        Direction::South => "↓",
                        Direction::West => "←",
                    };
                } else if self.obstacles.contains(&(x, y)) {
                    output += "#";
                } else if self.visited.contains(&(x, y)) {
                    output += "X";
                } else {
                    output += ".";
                }
            }
            println!("{}", output);
        }
        println!("-----");
    }

    fn check_on_loop_path(&mut self) -> bool {
        let max_iterations: u64 = 999_999_999_999_999;
        let mut visit_directions: HashSet<(Position, Direction)> = HashSet::new();

        for _ in 0..max_iterations {
            if let Some(guard_position) = self.guard_position {
                let new_position = guard_position + self.guard_direction;

                if !self.check_bounds(new_position) {
                    self.guard_position = None;
                    return false;
                } else if self.obstacles.contains(&new_position) {
                    self.guard_direction = self.guard_direction.rotate();
                } else if visit_directions.contains(&(new_position, self.guard_direction)) {
                    return true;
                } else {
                    visit_directions.insert((new_position, self.guard_direction));

                    self.visited.insert(new_position);
                    self.guard_position = Some(new_position);
                }
            }
        }

        false
    }
}

pub fn day6_part1(input: String) -> usize {
    let mut map = Map::parse_input(input);

    map.simulate_all_steps();
    map.render();

    map.visited.len()
}
pub fn day6_part2(input: String) -> usize {
    let map = Map::parse_input(input);

    map.check_for_loops()
}

#[cfg(test)]
mod tests {
    use crate::day6::{day6_part1, day6_part2};

    #[test]
    fn test_part1() {
        assert_eq!(
            41,
            day6_part1(
                r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#
                    .to_string()
            )
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            6,
            day6_part2(
                r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#
                    .to_string()
            )
        )
    }
}
