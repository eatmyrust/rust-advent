use std::fs;

use super::{AdventDay, Parse};

pub struct NewDay1Puzzle {}

pub struct Day1Puzzle {
    parsed_input: Vec<u32>,
}

impl Parse for NewDay1Puzzle {
    fn parse_input(&self, input_path: &str) -> Box<dyn AdventDay> {
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

        Box::new(Day1Puzzle {
            parsed_input: individual_elf_calories,
        })
    }
}

impl AdventDay for Day1Puzzle {
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
