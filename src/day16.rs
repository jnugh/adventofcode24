use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    vec,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Step {
    Move,
    RotateClockwise,
    RotateCounterClockwise,
}

#[derive(Debug)]
enum Field {
    Wall,
    End,
    Empty,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Maze {
    map: String,
    start: Position,
    position: Position,
    direction: Direction,
    score: usize,
    target: Position,

    width: usize,
    height: usize,

    steps: Vec<Step>,
    visited_positions: HashSet<Position>,
}

struct Solver {
    mazes: Vec<Maze>,
    visited: HashMap<(Position, Direction), usize>,
}

impl PartialOrd for Maze {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Maze {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
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

impl Solver {
    fn solve(&mut self) -> Vec<Maze> {
        let mut matches = Vec::new();
        let mut best_score = None;
        while let Some(maze) = self.mazes.pop() {
            if let Some(best_score) = best_score {
                if best_score < maze.score {
                    break;
                }
            }
            if let Some(visited_score) = self.visited.get(&(maze.position, maze.direction)) {
                if *visited_score < maze.score {
                    continue;
                }
            }
            self.visited
                .insert((maze.position, maze.direction), maze.score);
            let next_position = maze.get_next_position();
            match next_position {
                Field::End => {
                    let mut result = maze.clone();
                    result.score += 1;
                    result.steps.push(Step::Move);
                    result.visited_positions.insert(result.target);
                    best_score = Some(result.score);
                    matches.push(result);
                }
                Field::Empty => {
                    let mut step = maze.clone();
                    step.score += 1;
                    step.steps.push(Step::Move);
                    step.position = step.position + step.direction;
                    step.visited_positions.insert(step.position);
                    self.mazes.push(step);
                }
                Field::Wall => {}
            }

            let mut rotated1 = maze.clone();
            rotated1.direction = rotated1.direction.rotate();
            rotated1.score += 1000;
            rotated1.steps.push(Step::RotateClockwise);

            let mut rotated2 = maze.clone();
            rotated2.direction = rotated2.direction.rotate_counter_clockwise();
            rotated2.score += 1000;
            rotated2.steps.push(Step::RotateCounterClockwise);

            self.mazes.push(rotated1);
            self.mazes.push(rotated2);
            self.mazes.sort_unstable();
        }

        matches
    }
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn rotate_counter_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

impl Maze {
    fn from_input(input: &str) -> Self {
        let lines = input.trim().lines();
        let height = lines.clone().count();
        let width = lines
            .into_iter()
            .nth(0)
            .map(|l| l.chars().count())
            .unwrap_or(0);
        let map = input.replace("\n", "");
        let start_index = map.find('S').unwrap();
        let end_index = map.find('E').unwrap();
        let position = Position {
            x: start_index % width,
            y: start_index / height,
        };
        let target = Position {
            x: end_index % width,
            y: end_index / height,
        };

        Maze {
            position,
            start: position,
            direction: Direction::East,
            score: 0,
            target,
            map,
            height,
            width,
            steps: vec![],
            visited_positions: HashSet::from_iter(vec![position]),
        }
    }

    fn get_next_position(&self) -> Field {
        let next_position = self.position + self.direction;
        self.get_position(next_position)
    }

    fn get_position(&self, position: Position) -> Field {
        match self.map.chars().nth(position.y * self.height + position.x) {
            Some('.') | Some('S') | None => Field::Empty,
            Some('#') => Field::Wall,
            Some('E') => Field::End,
            Some(x) => panic!("Unknown field type {}", x),
        }
    }
}

pub fn day16_part1(input: String) -> usize {
    let maze = Maze::from_input(&input);
    let mut solver = Solver {
        mazes: vec![maze],
        visited: HashMap::new(),
    };

    solver.solve()[0].score
}

pub fn day16_part2(input: String) -> usize {
    let maze = Maze::from_input(&input);
    let mut solver = Solver {
        mazes: vec![maze],
        visited: HashMap::new(),
    };

    let mut positions: HashSet<Position> = HashSet::new();

    let solved_mazes = solver.solve();
    for maze in solved_mazes {
        for visited in maze.visited_positions {
            positions.insert(visited);
        }
    }

    positions.len()
}

#[cfg(test)]
mod test {
    use crate::day16::{day16_part1, day16_part2};

    #[test]
    fn test_part1_1() {
        assert_eq!(
            7036,
            day16_part1(
                r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#
                    .to_string()
            )
        )
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(
            11048,
            day16_part1(
                r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#
                    .to_string()
            )
        )
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(
            45,
            day16_part2(
                r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#
                    .to_string()
            )
        )
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(
            64,
            day16_part2(
                r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#
                    .to_string()
            )
        )
    }
}
