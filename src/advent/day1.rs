use std::fs;

use super::AdventDay;

pub struct Day1Puzzle {
    parsed_input: Vec<u32>,
}

impl Day1Puzzle {
    pub fn new() -> Day1Puzzle {
        Day1Puzzle {
            parsed_input: vec![],
        }
    }
}

impl AdventDay for Day1Puzzle {
    fn parse_input(&mut self, input_path: &str) {
        let puzzle_input = fs::read_to_string(input_path).unwrap();

        let individual_elf_calories: Vec<u32> = puzzle_input
            .split("\n\n")
            .map(|elf_calories| {
                elf_calories
                    .split("\n")
                    .map(|item| item.parse::<u32>().unwrap())
                    .reduce(|acc, e| acc + e)
                    .unwrap()
            })
            .collect();

        self.parsed_input = individual_elf_calories;
    }

    fn solve_first_puzzle(&self) -> String {
        self.parsed_input.iter().max().unwrap().to_string()
    }

    fn solve_second_puzzle(&self) -> String {
        let mut copied_calories = self.parsed_input.clone();
        copied_calories.sort();
        let total_calories: u32 = copied_calories[self.parsed_input.len() - 3..].iter().sum();
        total_calories.to_string()
    }
}
