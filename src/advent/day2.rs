use super::{AdventDay, Parse};

pub struct NewDay2Puzzle {}

pub struct Day2Puzzle {
    parsed_input: Vec<u32>,
}

impl Parse for NewDay2Puzzle {
    fn parse_input(&mut self, input_path: &str) -> Box<dyn AdventDay> {
        Box::new(Day2Puzzle {
            parsed_input: vec![],
        })
    }
}

impl AdventDay for Day2Puzzle {
    fn solve_first_puzzle(&self) -> String {
        String::from("hello world")
    }

    fn solve_second_puzzle(&self) -> String {
        String::from("hello world")
    }
}
