use std::error::Error;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub trait Parse {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>>;
}

pub trait AdventDay {
    fn solve_first_puzzle(&self) -> String;
    fn solve_second_puzzle(&self) -> String;
}
