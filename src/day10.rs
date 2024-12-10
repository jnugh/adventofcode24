use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

type Position = (usize, usize);

#[derive(Debug)]
struct Map {
    nodes: HashMap<Position, MapNode>,
}

#[derive(Debug)]
struct MapNode {
    position: Position,
    edges: Vec<MapEdge>,
    height: u8,
}

#[derive(Clone, Debug)]
struct MapEdge {
    dst: Position,
    slope: i8,
}

pub fn day10_part1(input: String) -> usize {
    let map = Map::from_input(input);

    map.count_paths(false)
}

pub fn day10_part2(input: String) -> usize {
    let map = Map::from_input(input);

    map.count_paths(true)
}

impl Map {
    fn from_input(input: String) -> Self {
        let height = input.trim().lines().count();
        let width = input.split("\n").nth(0).unwrap().chars().count();
        let input = input.replace("\n", "");

        let mut nodes = HashMap::new();

        for x in 0..width {
            for y in 0..height {
                let i = y * width + x;
                let position = (x, y);

                nodes.insert(
                    position,
                    MapNode {
                        edges: Vec::new(),
                        height: (input.chars().nth(i).unwrap() as usize - '0' as usize) as u8,
                        position,
                    },
                );
            }
        }

        for x in 0..width {
            for y in 0..height {
                let node = nodes.get(&(x, y)).unwrap();
                let mut edges = Vec::new();
                let start_height = node.height;

                if x > 0 {
                    let other_node = nodes.get(&(x - 1, y)).unwrap();
                    edges.push(MapEdge {
                        dst: other_node.position,
                        slope: other_node.height as i8 - start_height as i8,
                    });
                }
                if x < width - 1 {
                    let other_node = nodes.get(&(x + 1, y)).unwrap();
                    edges.push(MapEdge {
                        dst: other_node.position,
                        slope: other_node.height as i8 - start_height as i8,
                    });
                }
                if y > 0 {
                    let other_node = nodes.get(&(x, y - 1)).unwrap();
                    edges.push(MapEdge {
                        dst: other_node.position,
                        slope: other_node.height as i8 - start_height as i8,
                    });
                }
                if y < height - 1 {
                    let other_node = nodes.get(&(x, y + 1)).unwrap();
                    edges.push(MapEdge {
                        dst: other_node.position,
                        slope: other_node.height as i8 - start_height as i8,
                    });
                }

                nodes.get_mut(&(x, y)).unwrap().edges = edges;
            }
        }

        Map { nodes }
    }

    fn count_paths(&self, ignore_duplicates: bool) -> usize {
        let starts = self.nodes.values().filter(|node| node.height == 0);
        let paths = starts.map(|node| {
            self.count_paths_starting_at(
                node,
                ignore_duplicates,
                Rc::new(RefCell::new(HashSet::new())),
            )
        });

        paths.sum()
    }

    fn count_paths_starting_at(
        &self,
        start: &MapNode,
        ignore_duplicates: bool,
        visited_tops: Rc<RefCell<HashSet<Position>>>,
    ) -> usize {
        {
            let mut visited_tops = visited_tops.borrow_mut();
            if start.height == 9 && (ignore_duplicates || !visited_tops.contains(&start.position)) {
                visited_tops.insert(start.position);
                return 1;
            }
        }

        let valid_destinations = start
            .edges
            .clone()
            .into_iter()
            .filter(|edge| edge.slope == 1);
        valid_destinations
            .map(|edge| {
                self.count_paths_starting_at(
                    self.nodes.get(&edge.dst).unwrap(),
                    ignore_duplicates,
                    visited_tops.clone(),
                )
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::day10::{day10_part1, day10_part2};

    #[test]
    fn test_part1() {
        assert_eq!(
            36,
            day10_part1(
                r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#
                .to_string()
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            81,
            day10_part2(
                r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#
                .to_string()
            )
        );
    }
}
