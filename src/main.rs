mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod util;

use clap::{ArgAction, Parser, Subcommand};
use day1::{day1_part1, day1_part2};
use day2::{day2_part1, day2_part2};
use day3::{day3_part1, day3_part2};
use day4::{day4_part1, day4_part2};
use day5::{day5_part1, day5_part2};
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
    };

    println!("Result: {result}")
}
