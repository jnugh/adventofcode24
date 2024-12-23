use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

struct Onsen {
    towel_prefixes: HashMap<char, Vec<String>>,
    designs: Vec<String>,
    impossible_designs: RefCell<HashSet<String>>,
    possible_designs: RefCell<HashMap<String, usize>>,
}

impl Onsen {
    fn from_input(input: String) -> Self {
        let parts: Vec<&str> = input.trim().split("\n\n").collect();

        let available_towels: Vec<&str> = parts[0].split(", ").collect();
        let mut towel_prefixes: HashMap<char, Vec<String>> = HashMap::new();

        for available_towel in available_towels {
            let prefix: char = available_towel.chars().nth(0).unwrap();
            if let Some(existing_list) = towel_prefixes.get_mut(&prefix) {
                existing_list.push(available_towel.to_string());
            } else {
                towel_prefixes.insert(prefix, vec![available_towel.to_string()]);
            }
        }

        let designs = parts[1].lines().map(|l| l.to_string()).collect();

        Self {
            towel_prefixes,
            designs,
            impossible_designs: RefCell::new(HashSet::new()),
            possible_designs: RefCell::new(HashMap::new()),
        }
    }

    fn count_possible_designs(&self) -> usize {
        self.designs
            .iter()
            .filter(|design| self.count_combinations(design) > 0)
            .count()
    }

    fn count_possible_design_positions(&self) -> usize {
        self.designs
            .iter()
            .map(|design| self.count_combinations(design))
            .sum()
    }

    fn count_combinations(&self, design: &str) -> usize {
        if self
            .impossible_designs
            .borrow()
            .contains(&design.to_string())
        {
            return 0;
        }
        if let Some(result) = self.possible_designs.borrow().get(design) {
            return *result;
        }
        if let Some(prefix) = design.chars().nth(0) {
            if let Some(towels) = self.towel_prefixes.get(&prefix) {
                let c = towels
                    .iter()
                    .filter(|towel| design.starts_with(*towel))
                    .sorted_by(|a, b| a.len().cmp(&b.len()).reverse())
                    .map(|towel| self.count_combinations(&design[towel.len()..]))
                    .sum();
                if c > 0 {
                    self.possible_designs
                        .borrow_mut()
                        .insert(design.to_string(), c);
                    c
                } else {
                    self.impossible_designs
                        .borrow_mut()
                        .insert(design.to_string());
                    0
                }
            } else {
                self.impossible_designs
                    .borrow_mut()
                    .insert(design.to_string());
                0
            }
        } else {
            1
        }
    }
}

pub fn day19_part1(input: String) -> usize {
    let onsen = Onsen::from_input(input);

    onsen.count_possible_designs()
}

pub fn day19_part2(input: String) -> usize {
    let onsen = Onsen::from_input(input);

    onsen.count_possible_design_positions()
}

#[cfg(test)]
mod test {
    use crate::day19::{day19_part1, day19_part2};

    #[test]
    fn test_part1() {
        assert_eq!(
            6,
            day19_part1(
                r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#
                    .to_string()
            )
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            16,
            day19_part2(
                r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#
                    .to_string()
            )
        )
    }
}
