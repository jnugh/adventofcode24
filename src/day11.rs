use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    numbers: HashMap<usize, usize>,
}

impl Game {
    fn from_input(input: String) -> Self {
        let numbers = input.trim().split(" ").map(|n| n.parse::<usize>().unwrap());

        let mut number_counts = HashMap::<usize, usize>::new();
        for number in numbers {
            number_counts.insert(number, number_counts.get(&number).unwrap_or(&0) + 1);
        }

        Game {
            numbers: number_counts,
        }
    }

    fn step(&self) -> Game {
        let mut new_numbers: HashMap<usize, usize> = HashMap::new();
        for (number, count) in &self.numbers {
            if *number == 0 {
                insert_or_add(&mut new_numbers, 1, *count);
            } else if number.to_string().len() % 2 == 0 {
                let len = number.to_string().len();
                let n1: usize = number.to_string()[0..len / 2].parse().unwrap();
                let n2: usize = number.to_string()[len / 2..len].parse().unwrap();

                insert_or_add(&mut new_numbers, n1, *count);
                insert_or_add(&mut new_numbers, n2, *count);
            } else {
                insert_or_add(&mut new_numbers, number * 2024, *count);
            }
        }

        Game {
            numbers: new_numbers,
        }
    }

    fn count_numbers(&self) -> usize {
        self.numbers.values().sum()
    }
}

fn insert_or_add(map: &mut HashMap<usize, usize>, key: usize, count: usize) {
    map.insert(key, map.get(&key).unwrap_or(&0) + count);
}

pub fn day11_part1(input: String) -> usize {
    let mut game = Game::from_input(input);

    for _ in 0..25 {
        game = game.step();
    }

    game.count_numbers()
}

pub fn day11_part2(input: String) -> usize {
    let mut game = Game::from_input(input);

    for _ in 0..75 {
        game = game.step();
    }

    game.count_numbers()
}

#[cfg(test)]
mod test {
    use crate::day11::day11_part1;

    #[test]
    fn test_part1() {
        assert_eq!(55312, day11_part1("125 17".to_string()));
    }
}
