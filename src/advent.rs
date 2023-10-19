pub mod day1;
pub mod day2;
pub mod day3;

pub trait Parse {
    fn parse_input(&mut self, input_path: &str) -> Box<dyn AdventDay>;
}

pub trait AdventDay {
    fn solve_first_puzzle(&self) -> String;
    fn solve_second_puzzle(&self) -> String;
}
