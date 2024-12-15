mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
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
use day2::{day2_part1, day2_part2};
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

    let result = match args.command {
        Commands::Day1_1 => day1_part1(input),
        Commands::Day1_2 => day1_part2(input),
        Commands::Day2_1 => day2_part1(input),
        Commands::Day2_2 => day2_part2(input),
        Commands::Day3_1 => day3_part1(input),
        Commands::Day3_2 => day3_part2(input),
        Commands::Day4_1 => day4_part1(input),
        Commands::Day4_2 => day4_part2(input),
        Commands::Day5_1 => day5_part1(input),
        Commands::Day5_2 => day5_part2(input),
        Commands::Day6_1 => day6_part1(input),
        Commands::Day6_2 => day6_part2(input),
        Commands::Day7_1 => day7_part1(input),
        Commands::Day7_2 => day7_part2(input),
        Commands::Day8_1 => day8_part1(input),
        Commands::Day8_2 => day8_part2(input),
        Commands::Day9_1 => day9_part1(input),
        Commands::Day9_2 => day9_part2(input),
        Commands::Day10_1 => day10_part1(input),
        Commands::Day10_2 => day10_part2(input),
        Commands::Day11_1 => day11_part1(input),
        Commands::Day11_2 => day11_part2(input),
        Commands::Day12_1 => day12_part1(input),
        Commands::Day12_2 => day12_part2(input),
        Commands::Day13_1 => day13_part1(input),
        Commands::Day13_2 => day13_part2(input),
        Commands::Day14_1 => day14_part1(input),
        Commands::Day14_2 => day14_part2(input),
    };

    println!("Result: {result}")
}
