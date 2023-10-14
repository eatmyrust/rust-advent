mod advent;
use std::collections::HashMap;

use advent::day1::Day1Puzzle;
use advent::day2::Day2Puzzle;
use advent::AdventDay;

pub struct CLIParams {
    pub day_to_run: String,
    pub input_path: String,
}

impl CLIParams {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<CLIParams, &'static str> {
        args.next();

        let day_to_run = match args.next() {
            Some(day) => day,
            None => return Err("Day to run not provided!"),
        };

        let input_path = match args.next() {
            Some(input_path) => input_path,
            None => return Err("Path to puzzle input not provided!"),
        };

        return Ok(CLIParams {
            day_to_run,
            input_path,
        });
    }
}

fn collect_puzzles() -> HashMap<&'static str, Box<dyn AdventDay>> {
    let mut puzzle_days: HashMap<&'static str, Box<dyn AdventDay>> = HashMap::new();

    puzzle_days.insert("day1", Box::new(Day1Puzzle::new()));
    puzzle_days.insert("day2", Box::new(Day2Puzzle::new()));
    puzzle_days
}

pub fn run_advent_day(cli_params: &CLIParams) -> () {
    let mut puzzle_days = collect_puzzles();
    let mut puzzle_to_run = puzzle_days.remove(&*cli_params.day_to_run).unwrap();
    puzzle_to_run.parse_input(&*cli_params.input_path);
    println!("Part 1: {}", puzzle_to_run.solve_first_puzzle());
    println!("Part 2: {}", puzzle_to_run.solve_second_puzzle());
}
