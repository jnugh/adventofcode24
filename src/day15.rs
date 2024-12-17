use std::{collections::HashSet, fmt::Display};

use log::warn;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::North,
            '>' => Self::East,
            'v' => Self::South,
            '<' => Self::West,
            _ => panic!("Unknown direction: {}", value),
        }
    }
}

struct Warehouse {
    robot: Position,
    boxes: HashSet<Position>,
    walls: HashSet<Position>,

    steps: Vec<Direction>,

    width: usize,
    height: usize,
    large_boxes: bool,
}

impl Warehouse {
    fn from_input(input: &str, wide_layout: bool) -> Self {
        let parts: Vec<&str> = input.trim().split("\n\n").collect();
        let map = parts[0];
        let steps: Vec<Direction> = parts[1]
            .replace("\n", "")
            .chars()
            .map(Direction::from)
            .rev()
            .collect();
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot_position = Position { x: 0, y: 0 };

        for (y, line) in map.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let x = if wide_layout { x * 2 } else { x };
                match char {
                    '#' => {
                        walls.insert(Position { x, y });
                    }
                    'O' => {
                        boxes.insert(Position { x, y });
                    }
                    '@' => {
                        robot_position = Position { x, y };
                    }
                    '.' => {}
                    _ => {
                        panic!("Invalid map input: {}", char);
                    }
                }
            }
        }

        Self {
            robot: robot_position,
            width: walls.iter().map(|wall| wall.x).max().unwrap_or(0),
            height: walls.iter().map(|wall| wall.y).max().unwrap_or(0),
            boxes,
            walls,
            steps,
            large_boxes: wide_layout,
        }
    }

    fn simulate(&mut self) {
        while let Some(direction) = self.steps.pop() {
            let mut hit_wall = false;
            let mut boxes_to_move: Vec<Position> = Vec::new();
            let mut positions = vec![self.robot.clone()];
            let mut visited = HashSet::<Position>::new();
            while let Some(position) = positions.pop() {
                visited.insert(position.clone());
                if let Some(next_position) = self.get_next_position(&position, direction) {
                    if self.has_wall(&next_position) {
                        hit_wall = true;
                        break;
                    }

                    let affected_box_positions = self.get_affected_box_positions(&next_position);
                    for affected_position in affected_box_positions.iter() {
                        if self.boxes.contains(affected_position) {
                            boxes_to_move.push(affected_position.clone());
                        }
                        if !visited.contains(affected_position) {
                            positions.push(affected_position.clone());
                        }
                    }
                }
            }

            if !hit_wall {
                for item in boxes_to_move.iter() {
                    self.boxes.remove(item);
                }
                for item in boxes_to_move.iter() {
                    self.boxes
                        .insert(self.get_next_position(item, direction).unwrap());
                }
                self.robot = self.get_next_position(&self.robot, direction).unwrap();
            }
        }
    }

    fn has_wall(&self, position: &Position) -> bool {
        self.walls.contains(position)
            || (position.x > 0
                && self.large_boxes
                && self.walls.contains(&Position {
                    x: position.x - 1,
                    y: position.y,
                }))
    }

    fn get_affected_box_positions(&self, position: &Position) -> Vec<Position> {
        if self.large_boxes {
            if self.boxes.contains(position) {
                return vec![
                    position.clone(),
                    Position {
                        x: position.x + 1,
                        y: position.y,
                    },
                ];
            } else if position.x > 0
                && self.boxes.contains(&Position {
                    x: position.x - 1,
                    y: position.y,
                })
            {
                return vec![
                    position.clone(),
                    Position {
                        x: position.x - 1,
                        y: position.y,
                    },
                ];
            }
        } else if self.boxes.contains(position) {
            return vec![position.clone()];
        }

        vec![]
    }

    fn get_next_position(&self, position: &Position, direction: Direction) -> Option<Position> {
        match direction {
            Direction::North if position.y > 0 => Some(Position {
                x: position.x,
                y: position.y - 1,
            }),
            Direction::East if position.x < self.width => Some(Position {
                x: position.x + 1,
                y: position.y,
            }),
            Direction::South if position.y < self.height => Some(Position {
                x: position.x,
                y: position.y + 1,
            }),
            Direction::West if position.x > 0 => Some(Position {
                x: position.x - 1,
                y: position.y,
            }),
            _ => {
                warn!("OOB: {}, {:?}", position, direction);
                None
            }
        }
    }

    fn calculate_gps_coordinates(&self) -> usize {
        self.boxes.iter().map(|pos| pos.x + 100 * pos.y).sum()
    }

    fn print(&self) {
        for y in 0..=self.height {
            for x in 0..=self.width {
                let position = Position { x, y };
                if self.walls.contains(&position)
                    || (x > 0 && self.large_boxes && self.walls.contains(&Position { x: x - 1, y }))
                {
                    print!("#");
                } else if !self.large_boxes && self.boxes.contains(&position) {
                    print!("O");
                } else if self.large_boxes && self.boxes.contains(&position) {
                    print!("[");
                } else if self.large_boxes
                    && x > 0
                    && self.boxes.contains(&Position { x: x - 1, y })
                {
                    print!("]");
                } else if self.robot == position {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub fn day15_part1(input: String) -> usize {
    let mut warehouse = Warehouse::from_input(&input, false);

    warehouse.simulate();
    warehouse.print();

    warehouse.calculate_gps_coordinates()
}

pub fn day15_part2(input: String) -> usize {
    let mut warehouse = Warehouse::from_input(&input, true);

    warehouse.simulate();
    warehouse.print();

    warehouse.calculate_gps_coordinates()
}

#[cfg(test)]
mod test {
    use crate::day15::{day15_part1, day15_part2};

    #[test]
    fn test_part1() {
        assert_eq!(
            10092,
            day15_part1(
                r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#
                    .to_string()
            )
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            9021,
            day15_part2(
                r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#
                    .to_string()
            )
        )
    }
}
