use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Memory {
    corrupted_memory_areas: HashSet<Position>,
    falling_bytes: Vec<Position>,
    width: usize,
    height: usize,
    position: Position,
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split(",").collect();

        Self {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
        }
    }
}

impl Memory {
    fn from_input(input: String, width: usize, height: usize) -> Self {
        Memory {
            corrupted_memory_areas: HashSet::new(),
            falling_bytes: input.trim().lines().map(Position::from).collect(),
            height,
            width,
            position: Position {
                x: width - 1,
                y: height - 1,
            },
        }
    }

    fn drop_bytes(&mut self, bytes: usize) {
        for i in 0..bytes {
            self.drop_byte(i);
        }
    }

    fn drop_byte(&mut self, i: usize) {
        if let Some(position) = self.falling_bytes.get(i) {
            self.corrupted_memory_areas.insert(*position);
        }
    }

    fn shortest_path(&self) -> usize {
        let mut paths: HashMap<Position, usize> = HashMap::new();
        let mut visited: HashSet<Position> = HashSet::new();
        paths.insert(self.position, 0);

        let target = Position { x: 0, y: 0 };

        loop {
            if let Some(next) = paths
                .clone()
                .iter()
                .sorted_by(|path1, path2| path1.1.cmp(path2.1))
                .nth(0)
            {
                visited.insert(*next.0);
                paths.remove(next.0);
                if *next.0 == target {
                    return *next.1;
                } else {
                    if next.0.x > 0 {
                        let mut new_item = *next.0;
                        new_item.x -= 1;
                        if !visited.contains(&new_item)
                            && !self.corrupted_memory_areas.contains(&new_item)
                        {
                            paths.insert(new_item, next.1 + 1);
                        }
                    }
                    if next.0.y > 0 {
                        let mut new_item = *next.0;
                        new_item.y -= 1;
                        if !visited.contains(&new_item)
                            && !self.corrupted_memory_areas.contains(&new_item)
                        {
                            paths.insert(new_item, next.1 + 1);
                        }
                    }
                    if next.0.x < self.width - 1 {
                        let mut new_item = *next.0;
                        new_item.x += 1;
                        if !visited.contains(&new_item)
                            && !self.corrupted_memory_areas.contains(&new_item)
                        {
                            paths.insert(new_item, next.1 + 1);
                        }
                    }
                    if next.0.y < self.height - 1 {
                        let mut new_item = *next.0;
                        new_item.y += 1;
                        if !visited.contains(&new_item)
                            && !self.corrupted_memory_areas.contains(&new_item)
                        {
                            paths.insert(new_item, next.1 + 1);
                        }
                    }
                }
            } else {
                return usize::MAX;
            }
        }
    }
}

pub fn day18_part1(input: String) -> usize {
    execute_part1(input, 71, 71, 1024)
}

fn execute_part1(input: String, width: usize, height: usize, bytes: usize) -> usize {
    let mut memory = Memory::from_input(input, width, height);
    memory.drop_bytes(bytes);

    memory.shortest_path()
}

pub fn day18_part2(input: String) -> String {
    execute_part2(input, 71, 71)
}

fn execute_part2(input: String, width: usize, height: usize) -> String {
    let memory: Memory = Memory::from_input(input, width, height);
    let max = memory.falling_bytes.len();
    bin_search(&memory, 0, max - 1)
}

fn bin_search(memory: &Memory, min: usize, max: usize) -> String {
    if min == max {
        let result = memory.falling_bytes.get(min).unwrap();
        return format!("{},{}", result.x, result.y);
    }
    let mid = min + (max - min) / 2;
    let mut test_memory = memory.clone();

    test_memory.drop_bytes(mid);
    if test_memory.shortest_path() == usize::MAX {
        if max - min == 1 {
            let result = memory.falling_bytes.get(min).unwrap();
            return format!("{},{}", result.x, result.y);
        }
        bin_search(memory, min, mid)
    } else {
        if max - min == 1 {
            let mut test_memory = memory.clone();
            test_memory.drop_bytes(max);
            let result_idx = if test_memory.shortest_path() == usize::MAX {
                min
            } else {
                max
            };

            let result = memory.falling_bytes.get(result_idx).unwrap();
            return format!("{},{}", result.x, result.y);
        }
        bin_search(memory, mid, max)
    }
}

#[cfg(test)]
mod test {
    use crate::day18::{execute_part1, execute_part2};

    #[test]
    fn test_part1() {
        assert_eq!(
            22,
            execute_part1(
                r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#
                    .to_string(),
                7,
                7,
                12
            )
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            "6,1".to_string(),
            execute_part2(
                r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#
                    .to_string(),
                7,
                7,
            )
        )
    }
}
