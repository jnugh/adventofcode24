use indicatif::ProgressIterator;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use itertools::Itertools;

type Computer = String;

struct LanParty {
    connections: HashMap<Computer, HashSet<Computer>>,
}

#[derive(Clone)]
struct ComputerGroup {
    computers: Vec<Computer>,
}

impl Eq for ComputerGroup {}

impl PartialEq for ComputerGroup {
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.computers.clone();
        let mut b = other.computers.clone();
        a.sort_unstable();
        b.sort_unstable();

        a.eq(&b)
    }
}

impl Hash for ComputerGroup {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut computers = self.computers.clone();
        computers.sort_unstable();

        computers.hash(state);
    }
}

impl FromIterator<String> for ComputerGroup {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self {
            computers: Vec::from_iter(iter),
        }
    }
}

impl LanParty {
    fn from_input(input: &str) -> Self {
        let mut connections: HashMap<Computer, HashSet<Computer>> = HashMap::new();
        let connection_tuples: Vec<(&str, &str)> = input
            .trim()
            .lines()
            .flat_map(|line| line.split("-").collect_tuple())
            .collect();

        for (c1, c2) in connection_tuples {
            if !connections.contains_key(c1) {
                connections.insert(c1.to_string(), HashSet::new());
            }
            connections.get_mut(c1).unwrap().insert(c2.to_string());
            if !connections.contains_key(c2) {
                connections.insert(c2.to_string(), HashSet::new());
            }
            connections.get_mut(c2).unwrap().insert(c1.to_string());
        }

        Self { connections }
    }

    fn get_interconnected_computer_groups(
        &self,
        start: &Computer,
        len: usize,
    ) -> HashSet<ComputerGroup> {
        if let Some(connected_computers) = self.connections.get(start) {
            let mut groups: Vec<ComputerGroup> = Vec::new();
            for connected_computer in connected_computers {
                groups.push(ComputerGroup::from_iter([
                    start.to_string(),
                    connected_computer.to_string(),
                ]));
            }

            for _ in 2..len {
                let mut new_groups: HashSet<ComputerGroup> = HashSet::new();
                for group in &groups {
                    let intersection = group
                        .computers
                        .iter()
                        .flat_map(|computer| self.connections.get(computer))
                        .cloned()
                        .reduce(|acc, e| HashSet::from_iter(acc.intersection(&e).cloned()))
                        .unwrap_or_default();
                    new_groups.extend(intersection.iter().map(|new_item| {
                        let mut res = group.clone();
                        res.computers.push(new_item.clone());
                        res
                    }));
                }
                groups = new_groups.iter().cloned().collect();
            }

            HashSet::from_iter(groups)
        } else {
            HashSet::default()
        }
    }
}

pub fn day23_part1(input: String) -> usize {
    let lan_parts = LanParty::from_input(&input);

    let mut result: HashSet<ComputerGroup> = HashSet::new();

    for computer in lan_parts
        .connections
        .keys()
        .filter(|computer| computer.starts_with("t"))
    {
        result.extend(lan_parts.get_interconnected_computer_groups(computer, 3));
    }

    result.len()
}

pub fn day23_part2(input: String) -> String {
    let lan_parts = LanParty::from_input(&input);

    let mut largest_group_count = 3;
    let mut largest_group = None;
    let mut seen_before: HashSet<Computer> = HashSet::new();

    for computer in lan_parts.connections.keys().progress() {
        if seen_before.contains(computer) {
            continue;
        }

        let mut largest_group_by_computer_count = 3;
        let mut largest_group_by_computer = None;
        loop {
            let res = lan_parts
                .get_interconnected_computer_groups(computer, largest_group_by_computer_count);
            seen_before.extend(res.iter().flat_map(|group| group.computers.clone()));

            if res.is_empty() {
                break;
            }
            largest_group_by_computer_count += 1;
            largest_group_by_computer = Some(res);
        }
        if largest_group_by_computer_count > largest_group_count {
            largest_group_count = largest_group_by_computer_count;
            largest_group = largest_group_by_computer;
        }
    }

    let largest_group = largest_group.unwrap();
    let mut largest_group = Vec::from_iter(&largest_group.iter().next().unwrap().computers);
    largest_group.sort_unstable();
    largest_group.iter().join(",")
}

#[cfg(test)]
mod test {
    use crate::day23::{day23_part1, day23_part2};

    #[test]
    fn test_part1() {
        assert_eq!(
            7,
            day23_part1(
                r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#
                    .to_string()
            )
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            "co,de,ka,ta".to_string(),
            day23_part2(
                r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#
                    .to_string()
            )
        )
    }
}
