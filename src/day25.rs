type KeyOrLock = Vec<u8>;

#[derive(Debug)]
struct Locksmith {
    locks: Vec<KeyOrLock>,
    keys: Vec<KeyOrLock>,
}

impl Locksmith {
    fn from_input(input: &str) -> Self {
        let items = input.split("\n\n");
        let mut locks: Vec<Vec<u8>> = Vec::new();
        let mut keys: Vec<Vec<u8>> = Vec::new();

        for key_or_lock in items {
            let pattern = key_or_lock.replace("\n", "");

            if key_or_lock.lines().nth(0).unwrap() == "#####" {
                let mut lock = Vec::with_capacity(5);
                for col in 0..5 {
                    for row in 1..7 {
                        if pattern.as_bytes()[row * 5 + col] == b'.' {
                            lock.push((row - 1) as u8);
                            break;
                        }
                    }
                }
                locks.push(lock);
            } else {
                let mut key = Vec::with_capacity(5);
                for col in 0..5 {
                    for row in (0..6).rev() {
                        if pattern.as_bytes()[row * 5 + col] == b'.' {
                            key.push((5 - row) as u8);
                            break;
                        }
                    }
                }
                keys.push(key);
            }
        }

        Self { locks, keys }
    }

    fn find_possible_combinations(&self) -> Vec<(KeyOrLock, Vec<KeyOrLock>)> {
        self.locks
            .iter()
            .map(|lock| {
                let keys: Vec<KeyOrLock> = self
                    .keys
                    .iter()
                    .filter(|key| Locksmith::key_fits(lock, key))
                    .cloned()
                    .collect();
                (lock.clone(), keys)
            })
            .collect()
    }

    fn key_fits(lock: &KeyOrLock, key: &KeyOrLock) -> bool {
        lock.iter()
            .zip(key)
            .all(|(lock_part, key_part)| lock_part + key_part < 6)
    }
}

pub fn day25_part1(input: String) -> usize {
    let locksmith = Locksmith::from_input(&input);

    locksmith
        .find_possible_combinations()
        .iter()
        .map(|(_lock, keys)| keys.len())
        .sum()
}

pub fn day25_part2(_input: String) -> usize {
    panic!("There is nothing to do!")
}

#[cfg(test)]
mod test {
    use crate::day25::day25_part1;

    #[test]
    fn test_part1() {
        assert_eq!(
            3,
            day25_part1(
                r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#
                    .to_string()
            )
        )
    }
}
