use std::{sync::Arc, thread};

use itertools::Itertools;

#[derive(Clone)]
struct Market {
    buyer_seeds: Vec<usize>,
    buyer_prices: Vec<Vec<u8>>,
    buyer_price_changes: Vec<Vec<i8>>,
    iterations: usize,
}

impl Market {
    fn from_input(input: &str) -> Self {
        let buyer_seeds: Vec<usize> = input.trim().lines().map(|l| l.parse().unwrap()).collect();
        let count = buyer_seeds.len();

        Self {
            buyer_seeds,
            buyer_prices: Vec::with_capacity(count),
            buyer_price_changes: Vec::with_capacity(count),
            iterations: 0,
        }
    }

    fn get_seeds_after(&mut self, iterations: usize) -> Vec<usize> {
        self.iterations = iterations;
        let mut end_seeds = Vec::with_capacity(self.buyer_seeds.len());
        for buyer in &self.buyer_seeds {
            let mut result = *buyer;
            let mut prices = Vec::with_capacity(iterations + 1);
            prices.push((*buyer % 10) as u8);

            for _ in 0..iterations {
                result = next_number(result);
                prices.push((result % 10) as u8);
            }

            self.buyer_price_changes.push(
                prices
                    .iter()
                    .tuple_windows::<(_, _)>()
                    .map(|(n1, n2)| (*n2 as i8) - (*n1 as i8))
                    .collect(),
            );
            self.buyer_prices.push(prices);
            end_seeds.push(result);
        }
        end_seeds
    }

    fn get_best_prefix(&self) -> usize {
        let mut max = 0;
        let market = Arc::new(self.clone());
        let mut handles = Vec::new();
        for n1 in -9..=9 {
            let market = Arc::clone(&market);
            handles.push(thread::spawn(move || market.get_best_prefix_n1(n1)));
        }

        for handle in handles {
            max = max.max(handle.join().unwrap());
        }
        max
    }

    fn get_best_prefix_n1(&self, n1: i8) -> usize {
        let mut max = 0;
        for n2 in -9..=9 {
            if n1 + n2 < -9 || n1 + n2 > 9 {
                continue;
            }
            for n3 in -9..=9 {
                if n1 + n2 + n3 < -9 || n1 + n2 + n3 > 9 {
                    continue;
                }
                for n4 in -9..=9 {
                    if n1 + n2 + n3 + n4 < -9 || n1 + n2 + n3 + n4 > 9 {
                        continue;
                    }

                    let needle = [n1, n2, n3, n4];
                    let combination_result: usize = self
                        .buyer_price_changes
                        .iter()
                        .map(|price_changes| {
                            price_changes.windows(4).position(|window| window == needle)
                        })
                        .zip(&self.buyer_prices)
                        .map(|(position, prices)| match position {
                            Some(position) if position < self.iterations - 4 => {
                                prices[position + 4] as usize
                            }
                            _ => 0,
                        })
                        .sum();

                    max = max.max(combination_result);
                }
            }
        }
        max
    }
}

fn next_number(secret: usize) -> usize {
    let secret = prune(mix(secret * 64, secret));
    let secret = prune(mix(secret / 32, secret));

    prune(mix(secret * 2048, secret))
}

fn mix(n: usize, secret: usize) -> usize {
    n ^ secret
}

fn prune(n: usize) -> usize {
    n % 16777216
}

pub fn day22_part1(input: String) -> usize {
    let mut market = Market::from_input(&input);

    market.get_seeds_after(2000).iter().sum()
}

pub fn day22_part2(input: String) -> usize {
    let mut market = Market::from_input(&input);

    market.get_seeds_after(2000);
    market.get_best_prefix()
}

#[cfg(test)]
mod test {
    use crate::day22::{day22_part1, day22_part2, next_number};

    #[test]
    fn test_rng() {
        assert_eq!(15887950, next_number(123));
        assert_eq!(16495136, next_number(15887950));

        assert_eq!(527345, next_number(16495136));
        assert_eq!(704524, next_number(527345));
        assert_eq!(1553684, next_number(704524));
        assert_eq!(12683156, next_number(1553684));
        assert_eq!(11100544, next_number(12683156));
        assert_eq!(12249484, next_number(11100544));
        assert_eq!(7753432, next_number(12249484));
        assert_eq!(5908254, next_number(7753432));
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            37327623,
            day22_part1(
                r#"1
10
100
2024"#
                    .to_string()
            )
        );
    }

    #[test]
    #[ignore = "slow"]
    fn test_part2() {
        assert_eq!(
            23,
            day22_part2(
                r#"1
2
3
2024"#
                    .to_string()
            )
        )
    }
}
