mod day1;
mod util;

use clap::{ArgAction, Parser, Subcommand};
use day1::{day1_part1, day1_part2};
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
    command: Option<Commands>,

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

    match args.command {
        Some(Commands::Day1_1) => day1_part1(input),
        Some(Commands::Day1_2) => day1_part2(input),
        None => {}
    }
}
