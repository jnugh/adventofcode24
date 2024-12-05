use std::{collections::HashMap, iter::zip};

pub fn day1_part1(input: String) -> usize {
    let (mut list1, mut list2) = parse(input);
    list1.sort_unstable();
    list2.sort_unstable();

    let mut total_distance = 0;

    for (value1, value2) in zip(list1, list2) {
        total_distance += value1.abs_diff(value2)
    }

    total_distance
}

pub fn day1_part2(input: String) -> usize {
    let (list1, list2) = parse(input);

    let mut counts = HashMap::<usize, usize>::new();
    for value in list2 {
        counts.insert(value, counts.get(&value).unwrap_or(&0) + 1);
    }

    let mut score: usize = 0;
    for value in list1 {
        score += value * counts.get(&value).unwrap_or(&0);
    }

    score
}

fn parse(input: String) -> (Vec<usize>, Vec<usize>) {
    let mut list1 = Vec::<usize>::new();
    let mut list2 = Vec::<usize>::new();

    let lines = input.trim().split("\n");
    for line in lines {
        let values = line.split("   ").collect::<Vec<&str>>();

        list1.push(values[0].parse().unwrap());
        list2.push(values[1].parse().unwrap());
    }

    (list1, list2)
}
