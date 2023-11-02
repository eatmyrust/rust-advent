mod advent;
use std::collections::HashMap;
use std::error::Error;

use advent::day1::NewDay1Puzzle;
use advent::day2::NewDay2Puzzle;
use advent::day3::NewDay3Puzzle;
use advent::day4::NewDay4Puzzle;
use advent::Parse;

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

fn collect_puzzles() -> HashMap<&'static str, Box<dyn Parse>> {
    let mut puzzle_days: HashMap<&'static str, Box<dyn Parse>> = HashMap::new();

    puzzle_days.insert("day1", Box::new(NewDay1Puzzle {}));
    puzzle_days.insert("day2", Box::new(NewDay2Puzzle {}));
    puzzle_days.insert("day3", Box::new(NewDay3Puzzle {}));
    puzzle_days.insert("day4", Box::new(NewDay4Puzzle {}));
    puzzle_days
}

pub fn run_advent_day(cli_params: &CLIParams) -> Result<(), Box<dyn Error>> {
    let mut puzzle_days = collect_puzzles();
    let puzzle_to_run = puzzle_days
        .remove(&*cli_params.day_to_run)
        .ok_or("Specified day to run has not been implemented yet")?;
    let advent_day = puzzle_to_run.parse_input(&*cli_params.input_path)?;
    println!("Part 1: {}", advent_day.solve_first_puzzle());
    println!("Part 2: {}", advent_day.solve_second_puzzle());

    Ok(())
}
