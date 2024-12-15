use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
};

#[derive(Debug)]
enum Ordering {
    Before(usize),
    After(usize),
}

#[derive(Debug)]
struct PrintingInstructions {
    orders: HashMap<usize, Vec<Ordering>>,
    updates: Vec<Vec<usize>>,
}

struct SortablePage<'a> {
    page: usize,
    orders: &'a HashMap<usize, Vec<Ordering>>,
}

impl PartialEq for SortablePage<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.page == other.page
    }
}

impl Ord for SortablePage<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.orders.get(&self.page) {
            Some(rules) => {
                for rule in rules {
                    match rule {
                        Ordering::After(after) if *after == other.page => {
                            return std::cmp::Ordering::Less;
                        }
                        Ordering::Before(after) if *after == other.page => {
                            return std::cmp::Ordering::Greater;
                        }
                        _ => {}
                    }
                }
                std::cmp::Ordering::Equal
            }
            None => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for SortablePage<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SortablePage<'_> {}

impl PrintingInstructions {
    fn from_input(input: String) -> Self {
        let parts: Vec<&str> = input.trim().split("\n\n").collect();
        let orderings = parts[0].split("\n");
        let updates = parts[1].split("\n");

        let mut order_map = HashMap::<usize, Vec<Ordering>>::new();
        let mut updates_list = Vec::<Vec<usize>>::new();

        for ordering in orderings {
            let numbers = ordering
                .split("|")
                .map(|n| n.parse::<usize>())
                .collect::<Result<Vec<usize>, ParseIntError>>()
                .unwrap();

            if let Some(existing) = order_map.get_mut(&numbers[0]) {
                existing.push(Ordering::Before(numbers[1]));
            } else {
                order_map.insert(numbers[0], vec![Ordering::Before(numbers[1])]);
            }
            if let Some(existing) = order_map.get_mut(&numbers[1]) {
                existing.push(Ordering::After(numbers[0]));
            } else {
                order_map.insert(numbers[1], vec![Ordering::After(numbers[0])]);
            }
        }

        for update in updates {
            let numbers = update
                .split(",")
                .map(|n| n.parse::<usize>())
                .collect::<Result<Vec<usize>, ParseIntError>>()
                .unwrap();

            updates_list.push(numbers);
        }

        Self {
            orders: order_map,
            updates: updates_list,
        }
    }

    fn get_valid_updates(&self) -> Vec<Vec<usize>> {
        self.updates
            .clone()
            .into_iter()
            .filter(|update| self.is_valid_update(update))
            .collect()
    }

    fn get_fixed_updates(&self) -> Vec<Vec<usize>> {
        self.updates
            .clone()
            .into_iter()
            .filter(|update| !self.is_valid_update(update))
            .map(|update| self.fix_ordering(&update))
            .collect()
    }

    fn is_valid_update(&self, update: &Vec<usize>) -> bool {
        let mut forbidden_following: HashSet<usize> = HashSet::new();
        for number in update {
            if forbidden_following.contains(number) {
                return false;
            }
            let rules = self.orders.get(number);
            if let Some(rules) = rules {
                for rule in rules {
                    if let Ordering::After(after) = rule {
                        forbidden_following.insert(*after);
                    }
                }
            }
        }

        true
    }

    fn fix_ordering(&self, update: &[usize]) -> Vec<usize> {
        let mut result: Vec<SortablePage<'_>> = update
            .iter()
            .map(|page| SortablePage {
                page: *page,
                orders: &self.orders,
            })
            .collect();

        result.sort();

        result.into_iter().map(|p| p.page).collect()
    }
}

pub fn day5_part1(input: String) -> usize {
    let instructions = PrintingInstructions::from_input(input);

    let valid_updates = instructions.get_valid_updates();
    valid_updates
        .into_iter()
        .map(|numbers| numbers[numbers.len() / 2])
        .sum()
}

pub fn day5_part2(input: String) -> usize {
    let instructions = PrintingInstructions::from_input(input);

    let valid_updates = instructions.get_fixed_updates();

    valid_updates
        .into_iter()
        .map(|numbers| numbers[numbers.len() / 2])
        .sum()
}

#[cfg(test)]
mod test {
    use crate::day5::{day5_part1, day5_part2};

    #[test]
    fn test_part1() {
        assert_eq!(
            143,
            day5_part1(
                r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#
                .to_string()
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            123,
            day5_part2(
                r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#
                .to_string()
            )
        );
    }
}
