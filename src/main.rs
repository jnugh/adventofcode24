mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

use clap::{ArgAction, Parser, Subcommand};
use day1::{day1_part1, day1_part2};
use day10::{day10_part1, day10_part2};
use day11::{day11_part1, day11_part2};
use day12::{day12_part1, day12_part2};
use day13::{day13_part1, day13_part2};
use day14::{day14_part1, day14_part2};
use day15::{day15_part1, day15_part2};
use day16::{day16_part1, day16_part2};
use day17::{day17_part1, day17_part2};
use day18::{day18_part1, day18_part2};
use day19::{day19_part1, day19_part2};
use day2::{day2_part1, day2_part2};
use day20::{day20_part1, day20_part2};
use day21::{day21_part1, day21_part2};
use day22::{day22_part1, day22_part2};
use day23::{day23_part1, day23_part2};
use day24::{day24_part1, day24_part2};
use day3::{day3_part1, day3_part2};
use day4::{day4_part1, day4_part2};
use day5::{day5_part1, day5_part2};
use day6::{day6_part1, day6_part2};
use day7::{day7_part1, day7_part2};
use day8::{day8_part1, day8_part2};
use day9::{day9_part1, day9_part2};
use std::path::PathBuf;
use util::read_input_or_crash;

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Silence all output
    #[structopt(short = 'q', long = "quiet")]
    quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = 'v', long = "verbose", action = ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,

    /// The puzzle input
    #[arg(long = "input", short = 'i')]
    input: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Day 1: Historian Hysteria: part 1
    Day1_1,
    /// Day 1: Historian Hysteria: part 2
    Day1_2,
    /// Day 2: Red-Nosed Reports: part 1
    Day2_1,
    /// Day 2: Red-Nosed Reports: part 2
    Day2_2,
    /// Day 3: Mull It Over: part 1
    Day3_1,
    /// Day 3: Mull It Over: part 2
    Day3_2,
    /// Day 4: Ceres Search: part 1
    Day4_1,
    /// Day 4: Ceres Search: part 2
    Day4_2,
    /// Day 5: Print Queue: part 1
    Day5_1,
    /// Day 5: Print Queue: part 2
    Day5_2,
    /// Day 6: Guard Gallivant: part 1
    Day6_1,
    /// Day 6: Guard Gallivant: part 2
    Day6_2,
    /// Day 7: Bridge Repair: part 1
    Day7_1,
    /// Day 7: Bridge Repair: part 2
    Day7_2,
    ///Day 8: Resonant Collinearity: part 1
    Day8_1,
    ///Day 8: Resonant Collinearity: part 2
    Day8_2,
    ///Day 9: Disk Fragmenter: part 1
    Day9_1,
    ///Day 9: Disk Fragmenter: part 2
    Day9_2,
    ///Day 10: Hoof It: part 1
    Day10_1,
    ///Day 10: Hoof It: part 2
    Day10_2,
    ///Day 11: Plutonian Pebbles: part 1
    Day11_1,
    ///Day 11: Plutonian Pebbles: part 2
    Day11_2,
    ///Day 12: Garden Groups: part 1
    Day12_1,
    ///Day 12: Garden Groups: part 2
    Day12_2,
    ///Day 13: Claw Contraption: part 1
    Day13_1,
    ///Day 13: Claw Contraption: part 2
    Day13_2,
    ///Day 14: Restroom Redoubt: part 1
    Day14_1,
    ///Day 14: Restroom Redoubt: part 2
    Day14_2,
    ///Day 15: Warehouse Woes: part 1
    Day15_1,
    ///Day 15: Warehouse Woes: part 2
    Day15_2,
    ///Day 16: Reindeer Maze: part 1
    Day16_1,
    ///Day 16: Reindeer Maze: part 2
    Day16_2,
    ///Day 17: Chronospatial Computer: part 1
    Day17_1,
    ///Day 17: Chronospatial Computer: part 2
    Day17_2,
    ///Day 18: RAM Run: part 1
    Day18_1,
    ///Day 18: RAM Run: part 2
    Day18_2,
    ///Day 19: Linen Layout: part 1
    Day19_1,
    ///Day 19: Linen Layout: part 2
    Day19_2,
    ///Day 20: Race Condition: part 1
    Day20_1,
    ///Day 20: Race Condition: part 2
    Day20_2,
    ///Day 21: Keypad Conundrum: part 1
    Day21_1,
    ///Day 21: Keypad Conundrum: part 2
    Day21_2,
    ///Day 22: Monkey Market: part 1
    Day22_1,
    ///Day 22: Monkey Market: part 2
    Day22_2,
    ///Day 23: LAN Party: part 1
    Day23_1,
    ///Day 23: LAN Party: part 2
    Day23_2,
    ///Day 24: Crossed Wires: part 1
    Day24_1,
    ///Day 24: Crossed Wires: part 2
    Day24_2,
}

fn main() {
    let args = Args::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(args.quiet)
        .verbosity(args.verbose as usize)
        .init()
        .unwrap();

    let input = read_input_or_crash(args.input);

    let result: String = match args.command {
        Commands::Day1_1 => day1_part1(input).to_string(),
        Commands::Day1_2 => day1_part2(input).to_string(),
        Commands::Day2_1 => day2_part1(input).to_string(),
        Commands::Day2_2 => day2_part2(input).to_string(),
        Commands::Day3_1 => day3_part1(input).to_string(),
        Commands::Day3_2 => day3_part2(input).to_string(),
        Commands::Day4_1 => day4_part1(input).to_string(),
        Commands::Day4_2 => day4_part2(input).to_string(),
        Commands::Day5_1 => day5_part1(input).to_string(),
        Commands::Day5_2 => day5_part2(input).to_string(),
        Commands::Day6_1 => day6_part1(input).to_string(),
        Commands::Day6_2 => day6_part2(input).to_string(),
        Commands::Day7_1 => day7_part1(input).to_string(),
        Commands::Day7_2 => day7_part2(input).to_string(),
        Commands::Day8_1 => day8_part1(input).to_string(),
        Commands::Day8_2 => day8_part2(input).to_string(),
        Commands::Day9_1 => day9_part1(input).to_string(),
        Commands::Day9_2 => day9_part2(input).to_string(),
        Commands::Day10_1 => day10_part1(input).to_string(),
        Commands::Day10_2 => day10_part2(input).to_string(),
        Commands::Day11_1 => day11_part1(input).to_string(),
        Commands::Day11_2 => day11_part2(input).to_string(),
        Commands::Day12_1 => day12_part1(input).to_string(),
        Commands::Day12_2 => day12_part2(input).to_string(),
        Commands::Day13_1 => day13_part1(input).to_string(),
        Commands::Day13_2 => day13_part2(input).to_string(),
        Commands::Day14_1 => day14_part1(input).to_string(),
        Commands::Day14_2 => day14_part2(input).to_string(),
        Commands::Day15_1 => day15_part1(input).to_string(),
        Commands::Day15_2 => day15_part2(input).to_string(),
        Commands::Day16_1 => day16_part1(input).to_string(),
        Commands::Day16_2 => day16_part2(input).to_string(),
        Commands::Day17_1 => day17_part1(input),
        Commands::Day17_2 => day17_part2(input).to_string(),
        Commands::Day18_1 => day18_part1(input).to_string(),
        Commands::Day18_2 => day18_part2(input),
        Commands::Day19_1 => day19_part1(input).to_string(),
        Commands::Day19_2 => day19_part2(input).to_string(),
        Commands::Day20_1 => day20_part1(input).to_string(),
        Commands::Day20_2 => day20_part2(input).to_string(),
        Commands::Day21_1 => day21_part1(input).to_string(),
        Commands::Day21_2 => day21_part2(input).to_string(),
        Commands::Day22_1 => day22_part1(input).to_string(),
        Commands::Day22_2 => day22_part2(input).to_string(),
        Commands::Day23_1 => day23_part1(input).to_string(),
        Commands::Day23_2 => day23_part2(input).to_string(),
        Commands::Day24_1 => day24_part1(input).to_string(),
        Commands::Day24_2 => day24_part2(input),
    };

    println!("Result: {result}")
}
