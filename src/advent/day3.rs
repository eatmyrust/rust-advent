use std::{collections::HashSet, fs};

use super::{AdventDay, Parse};

pub struct NewDay3Puzzle {}

pub struct Day3Puzzle {
    parsed_input: Vec<String>,
}

impl Parse for NewDay3Puzzle {
    fn parse_input(&mut self, input_path: &str) -> Box<dyn AdventDay> {
        let puzzle_input = fs::read_to_string(input_path).unwrap();

        let rucksack_compartments = parse_rucksacks(&puzzle_input);

        Box::new(Day3Puzzle {
            parsed_input: rucksack_compartments,
        })
    }
}

fn parse_rucksacks(input: &str) -> Vec<String> {
    input
        .split("\n")
        .map(|rucksack| String::from(rucksack))
        .collect::<Vec<String>>()
}

impl AdventDay for Day3Puzzle {
    fn solve_first_puzzle(&self) -> String {
        calculate_sum_of_priorities_for_items_to_reorganize(&self.parsed_input).to_string()
    }

    fn solve_second_puzzle(&self) -> String {
        calculate_sum_of_priorities_of_badges(&self.parsed_input).to_string()
    }
}

fn get_char_value(character: char) -> u32 {
    if character.is_uppercase() {
        return character as u32 - 38;
    }
    character as u32 - 96
}

fn split_string_in_half<'a>(string_to_split: &'a str) -> (&'a str, &'a str) {
    let halfway_point_of_sting = string_to_split.len() / 2;
    let first_half = &string_to_split[..halfway_point_of_sting];
    let second_half = &string_to_split[halfway_point_of_sting..];

    (first_half, second_half)
}

fn find_intersection_between_strings(strings: &[&str]) -> char {
    let mut strings_iter = strings.iter();
    let first_string = strings_iter.next().unwrap();
    let mut running_intersection: HashSet<char> = first_string.chars().collect();
    for string in strings {
        running_intersection.retain(|&e| string.contains(e))
    }
    *running_intersection.iter().next().unwrap()
}

fn calculate_sum_of_priorities_for_items_to_reorganize(rucksacks: &Vec<String>) -> u32 {
    rucksacks
        .iter()
        .map(|e| {
            let rucksack_compartments = split_string_in_half(e);
            let item_to_reorganize = find_intersection_between_strings(&[
                &rucksack_compartments.0,
                &rucksack_compartments.1,
            ]);
            get_char_value(item_to_reorganize)
        })
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn calculate_sum_of_priorities_of_badges(rucksacks: &Vec<String>) -> u32 {
    let mut accumulator = 0;
    for (index, rucksack) in rucksacks.iter().enumerate().filter(|e| e.0 % 3 == 0) {
        let badge = find_intersection_between_strings(&[
            rucksack,
            &rucksacks[index + 1],
            &rucksacks[index + 2],
        ]);
        accumulator += get_char_value(badge);
    }
    accumulator
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn a_has_val_1() {
        let expected = 1;

        let actual = get_char_value('a');

        assert_eq!(actual, expected)
    }

    #[test]
    fn z_has_val_26() {
        let expected = 26;

        let actual = get_char_value('z');

        assert_eq!(actual, expected)
    }

    #[test]
    fn uppercase_a_has_val_27() {
        let expected = 27;

        let actual = get_char_value('A');

        assert_eq!(actual, expected)
    }

    #[test]
    fn uppercase_z_has_val_52() {
        let expected = 52;

        let actual = get_char_value('Z');

        assert_eq!(actual, expected)
    }

    #[test]
    fn string_can_be_split_in_half() {
        let expected = ("vJrwpWtwJgWr", "hcsFMMfFFhFp");

        let actual = split_string_in_half("vJrwpWtwJgWrhcsFMMfFFhFp");

        assert_eq!(actual, expected)
    }

    #[test]
    fn intersection_between_strings_is_p() {
        let expected = 'p';

        let actual = find_intersection_between_strings(&["vJrwpWtwJgWr", "hcsFMMfFFhFp"]);

        assert_eq!(actual, expected)
    }

    #[test]
    fn intersection_between_strings_is_uppercase_l() {
        let expected = 'L';

        let actual = find_intersection_between_strings(&["jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"]);

        assert_eq!(actual, expected)
    }

    #[test]
    fn intersection_between_strings_is_r() {
        let expected = 'r';

        let actual = find_intersection_between_strings(&[
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ]);

        assert_eq!(actual, expected)
    }

    #[test]
    fn intersection_between_strings_is_uppercase_z() {
        let expected = 'Z';

        let actual = find_intersection_between_strings(&[
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]);

        assert_eq!(actual, expected)
    }

    #[test]
    fn sum_of_priorities_for_items_to_reorganize_is_157() {
        let expected = 157;

        let rucksacks = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];
        let actual = calculate_sum_of_priorities_for_items_to_reorganize(&rucksacks);

        assert_eq!(actual, expected)
    }

    #[test]
    fn sum_of_priorities_of_badges_is_70() {
        let expected = 70;

        let rucksacks = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];
        let actual = calculate_sum_of_priorities_of_badges(&rucksacks);

        assert_eq!(actual, expected)
    }

    #[test]
    fn parse_input() {
        let expected = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let actual = parse_rucksacks(input);

        assert_eq!(actual, expected)
    }
}
