use std::collections::HashSet;

type Position = (usize, usize);

#[derive(Debug)]
struct Plot {
    area: usize,
    perimeter: usize,
    sides: usize,
}

#[derive(Debug)]
struct Map {
    data: String,
    visited_nodes: HashSet<Position>,
    plots: Vec<Plot>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Default, Debug)]
struct SharedSides {
    sides: Vec<(Direction, Position, Position)>,
}

impl SharedSides {
    fn add(&mut self, direction: Direction, position: Position) {
        let mut extended = false;
        for (existing_direction, start, end) in &mut self.sides {
            if *existing_direction != direction {
                continue;
            }
            match direction {
                Direction::North | Direction::South => {
                    if start.1 != position.1 {
                        continue;
                    }
                    if start.0.abs_diff(position.0) == 1 {
                        start.0 = start.0.min(position.0);
                        extended = true;
                    }
                    if end.0.abs_diff(position.0) == 1 {
                        end.0 = end.0.max(position.0);
                        extended = true;
                    }
                }
                Direction::East | Direction::West => {
                    if start.0 != position.0 {
                        continue;
                    }
                    if start.1.abs_diff(position.1) == 1 {
                        start.1 = start.1.min(position.1);
                        extended = true;
                    }
                    if end.1.abs_diff(position.1) == 1 {
                        end.1 = end.1.max(position.1);
                        extended = true;
                    }
                }
            };

            if extended {
                return;
            }
        }

        self.sides.push((direction, position, position));
    }
}

impl Map {
    fn parse_input(input: String) -> Self {
        let input = input.trim();
        let height = input.split("\n").count();
        let width = input
            .split("\n")
            .take(1)
            .map(|l| l.chars().count())
            .nth(0)
            .unwrap();

        Map {
            data: input.replace("\n", ""),
            height,
            width,
            plots: Vec::new(),
            visited_nodes: HashSet::new(),
        }
    }

    fn parse_plots(&mut self) {
        for x in 1..=self.width {
            for y in 1..=self.height {
                if self.visited_nodes.contains(&(x, y)) {
                    continue;
                }

                let plot = self.parse_plot((x, y));
                self.plots.push(plot);
            }
        }
    }

    fn parse_plot(&mut self, position: Position) -> Plot {
        let crop_type = self.get_crop_type(position);
        let mut positions = vec![position];
        let mut local_visited: HashSet<Position> = HashSet::new();

        let mut perimeter = 0;
        let mut area = 0;
        let mut sides = SharedSides::default();

        let mut top_left: Position = (usize::MAX, usize::MAX);
        let mut bottom_right: Position = (usize::MIN, usize::MIN);

        while let Some(current_position) = positions.pop() {
            let current_crop_type = self.get_crop_type(current_position);
            if current_crop_type != crop_type {
                perimeter += 1;
                continue;
            }

            if local_visited.contains(&current_position) {
                continue;
            }
            local_visited.insert(current_position);

            if self.visited_nodes.contains(&current_position) {
                continue;
            }
            area += 1;
            top_left.0 = top_left.0.min(current_position.0);
            top_left.1 = top_left.1.min(current_position.1);
            bottom_right.0 = bottom_right.0.max(current_position.0);
            bottom_right.1 = bottom_right.1.max(current_position.1);

            self.visited_nodes.insert(current_position);
            if current_position.0 > 0 {
                positions.push((current_position.0 - 1, current_position.1));
            } else {
                perimeter += 1;
            }
            if current_position.1 > 0 {
                positions.push((current_position.0, current_position.1 - 1));
            } else {
                perimeter += 1;
            }
            if current_position.0 < self.width {
                positions.push((current_position.0 + 1, current_position.1));
            } else {
                perimeter += 1;
            }
            if current_position.1 < self.height {
                positions.push((current_position.0, current_position.1 + 1));
            } else {
                perimeter += 1;
            }
        }

        for x in top_left.0..=bottom_right.0 {
            for y in top_left.1..=bottom_right.1 {
                if self.get_crop_type((x, y)) != crop_type || !local_visited.contains(&(x, y)) {
                    continue;
                }

                if self.get_crop_type((x, y - 1)) != crop_type {
                    sides.add(Direction::North, (x, y));
                }
                if self.get_crop_type((x, y + 1)) != crop_type {
                    sides.add(Direction::South, (x, y));
                }
                if self.get_crop_type((x - 1, y)) != crop_type {
                    sides.add(Direction::West, (x, y));
                }
                if self.get_crop_type((x + 1, y)) != crop_type {
                    sides.add(Direction::East, (x, y));
                }
            }
        }

        Plot {
            perimeter,
            area,
            sides: sides.sides.len(),
        }
    }

    fn get_crop_type(&self, position: Position) -> char {
        if position.0 == 0 || position.1 == 0 || position.0 > self.width || position.1 > self.height
        {
            '.'
        } else {
            self.data
                .chars()
                .nth((position.1 - 1) * self.width + (position.0 - 1))
                .unwrap_or('.')
        }
    }

    fn calculate_fence_cost(&self) -> usize {
        self.plots
            .iter()
            .map(|plot| plot.area * plot.perimeter)
            .sum()
    }

    fn calculate_fence_cost_discounted(&self) -> usize {
        self.plots.iter().map(|plot| plot.area * plot.sides).sum()
    }
}

pub fn day12_part1(input: String) -> usize {
    let mut map = Map::parse_input(input);
    map.parse_plots();

    map.calculate_fence_cost()
}

pub fn day12_part2(input: String) -> usize {
    let mut map = Map::parse_input(input);
    map.parse_plots();

    map.calculate_fence_cost_discounted()
}

#[cfg(test)]
mod test {
    use crate::day12::{day12_part1, day12_part2};

    #[test]
    fn test_part1_1() {
        assert_eq!(
            140,
            day12_part1(
                r#"AAAA
BBCD
BBCC
EEEC"#
                    .to_string()
            )
        );
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(
            772,
            day12_part1(
                r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#
                    .to_string()
            )
        );
    }

    #[test]
    fn test_part1_larger() {
        assert_eq!(
            1930,
            day12_part1(
                r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#
                    .to_string()
            )
        );
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(
            80,
            day12_part2(
                r#"AAAA
BBCD
BBCC
EEEC"#
                    .to_string()
            )
        );
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(
            436,
            day12_part2(
                r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#
                    .to_string()
            )
        );
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(
            368,
            day12_part2(
                r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#
                    .to_string()
            )
        );
    }

    #[test]
    fn test_part2_4() {
        assert_eq!(
            236,
            day12_part2(
                r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#
                    .to_string()
            )
        );
    }

    #[test]
    fn test_part2_larger() {
        assert_eq!(
            1206,
            day12_part2(
                r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#
                    .to_string()
            )
        );
    }
}
