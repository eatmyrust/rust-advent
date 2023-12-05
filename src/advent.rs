use std::error::Error;

pub mod y2022;
pub mod y2023;

pub trait Parse {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>>;
}

pub trait AdventDay {
    fn solve_first_puzzle(&self) -> String;
    fn solve_second_puzzle(&self) -> String;
}
