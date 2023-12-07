mod advent;
use std::collections::HashMap;
use std::error::Error;

use advent::y2022;
use advent::y2023;
use advent::Parse;

pub struct CLIParams {
    pub year_of_puzzle: String,
    pub day_to_run: String,
    pub input_path: String,
}

impl CLIParams {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<CLIParams, &'static str> {
        args.next();

        let year_of_puzzle = match args.next() {
            Some(year) => year,
            None => return Err("Year of puzzle not provided!"),
        };

        let day_to_run = match args.next() {
            Some(day) => day,
            None => return Err("Day to run not provided!"),
        };

        let input_path = match args.next() {
            Some(input_path) => input_path,
            None => return Err("Path to puzzle input not provided!"),
        };

        return Ok(CLIParams {
            year_of_puzzle,
            day_to_run,
            input_path,
        });
    }
}

fn collect_puzzles() -> HashMap<&'static str, HashMap<&'static str, Box<dyn Parse>>> {
    let mut puzzle_days_2022: HashMap<&'static str, Box<dyn Parse>> = HashMap::new();
    let mut puzzle_days_2023: HashMap<&'static str, Box<dyn Parse>> = HashMap::new();

    puzzle_days_2022.insert("day1", Box::new(y2022::NewDay1Puzzle {}));
    puzzle_days_2022.insert("day2", Box::new(y2022::NewDay2Puzzle {}));
    puzzle_days_2022.insert("day3", Box::new(y2022::NewDay3Puzzle {}));
    puzzle_days_2022.insert("day4", Box::new(y2022::NewDay4Puzzle {}));
    puzzle_days_2022.insert("day5", Box::new(y2022::NewDay5Puzzle {}));
    puzzle_days_2023.insert("day1", Box::new(y2023::NewDay1Puzzle {}));
    puzzle_days_2023.insert("day2", Box::new(y2023::NewDay2Puzzle {}));
    puzzle_days_2023.insert("day3", Box::new(y2023::NewDay3Puzzle {}));
    puzzle_days_2023.insert("day4", Box::new(y2023::NewDay4Puzzle {}));
    puzzle_days_2023.insert("day5", Box::new(y2023::NewDay5Puzzle {}));
    puzzle_days_2023.insert("day6", Box::new(y2023::NewDay6Puzzle {}));

    let puzzle_days = HashMap::from([("2022", puzzle_days_2022), ("2023", puzzle_days_2023)]);
    puzzle_days
}

pub fn run_advent_day(cli_params: &CLIParams) -> Result<(), Box<dyn Error>> {
    let mut puzzle_days = collect_puzzles();
    let puzzle_to_run = puzzle_days
        .remove(&*cli_params.year_of_puzzle)
        .ok_or("Specified year has not been implemented yet")?
        .remove(&*cli_params.day_to_run)
        .ok_or("Specified day to run has not been implemented for that year")?;
    let advent_day = puzzle_to_run.parse_input(&*cli_params.input_path)?;
    println!("Part 1: {}", advent_day.solve_first_puzzle());
    println!("Part 2: {}", advent_day.solve_second_puzzle());

    Ok(())
}
