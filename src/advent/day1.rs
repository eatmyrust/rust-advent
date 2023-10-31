use std::{error::Error, fs};

use super::{AdventDay, Parse};

pub struct NewDay1Puzzle {}

pub struct Day1Puzzle {
    parsed_input: Vec<u32>,
}

impl Parse for NewDay1Puzzle {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>> {
        let puzzle_input = fs::read_to_string(input_path)?;

        let individual_elf_calories = parse_elf_calories(&puzzle_input)?;

        Ok(Box::new(Day1Puzzle {
            parsed_input: individual_elf_calories,
        }))
    }
}

fn parse_elf_calories(elf_calories: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    elf_calories
        .split("\n\n")
        .map(add_individual_elf_calories)
        .collect::<Result<Vec<u32>, _>>()
}

fn add_individual_elf_calories(elf_calories: &str) -> Result<u32, Box<dyn Error>> {
    let mut err = Ok(());
    let possible_elf_calories = elf_calories
        .split("\n")
        .map(|item| item.parse::<u32>())
        .scan(&mut err, until_err)
        .reduce(|acc, e| acc + e);

    err?;
    if let Some(added_elf_calories) = possible_elf_calories {
        return Ok(added_elf_calories);
    }
    Err("No calories to add were provided!".into())
}

fn until_err<T, E>(err: &mut &mut Result<(), E>, item: Result<T, E>) -> Option<T> {
    match item {
        Ok(item) => Some(item),
        Err(e) => {
            **err = Err(e);
            None
        }
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day_1_parse_input() -> Result<(), Box<dyn Error>> {
        let expected = vec![15, 10, 20];

        let input = "\
5
7
3

9
1

10
2
8";
        let actual = parse_elf_calories(&input)?;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn day_1_parse_input_all_sections_invalid() {
        let input = "\
2
hello
1

this
is
5

6
not
valid";
        let actual = parse_elf_calories(&input);

        assert!(actual.is_err());
    }

    #[test]
    fn day_1_parse_input_some_sections_invalid() {
        let input = "\
2
5
1

4
this
is
5

6
9
30";
        let actual = parse_elf_calories(&input);

        assert!(actual.is_err());
    }
}
