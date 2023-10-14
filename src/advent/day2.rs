use super::AdventDay;

pub struct Day2Puzzle {
    parsed_input: Vec<u32>,
}

impl Day2Puzzle {
    pub fn new() -> Day2Puzzle {
        Day2Puzzle {
            parsed_input: vec![],
        }
    }
}

impl AdventDay for Day2Puzzle {
    fn parse_input(&mut self, input_path: &str) {
        self.parsed_input = vec![]
    }

    fn solve_first_puzzle(&self) -> String {
        String::from("hello world")
    }

    fn solve_second_puzzle(&self) -> String {
        String::from("hello world")
    }
}
