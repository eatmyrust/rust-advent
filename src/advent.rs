pub mod day1;
pub mod day2;

pub trait AdventDay {
    fn parse_input(&mut self, input_path: &str);
    fn solve_first_puzzle(&self) -> String;
    fn solve_second_puzzle(&self) -> String;
}
