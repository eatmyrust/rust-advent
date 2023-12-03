use std::{error::Error, fs};

use super::super::{AdventDay, Parse};

pub struct NewDay3Puzzle {}

pub struct Day3Puzzle {
    parsed_input: Vec<Vec<String>>,
}

impl Parse for NewDay3Puzzle {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>> {
        let puzzle_input = fs::read_to_string(input_path)?;

        let parsed_input = puzzle_input
            .split("\n")
            .map(|line| line.chars().map(String::from).collect::<Vec<String>>())
            .collect::<Vec<_>>();

        Ok(Box::new(Day3Puzzle { parsed_input }))
    }
}

impl AdventDay for Day3Puzzle {
    fn solve_first_puzzle(&self) -> String {
        extract_part_numbers_to_sum(&self.parsed_input, false)
            .into_iter()
            .reduce(|acc, e| acc + e)
            .unwrap()
            .to_string()
    }

    fn solve_second_puzzle(&self) -> String {
        let gear_part_numbers = extract_part_numbers_to_sum(&self.parsed_input, true);
        let mut running_total = 0;

        for (index, part_number) in gear_part_numbers.iter().enumerate() {
            if index % 2 == 0 {
                running_total += part_number * gear_part_numbers[index + 1]
            }
        }
        running_total.to_string()
    }
}

fn combine_split_part_number_descending(split_number: &Vec<String>, start_index: usize) -> String {
    let mut digits_before_index = String::new();
    if start_index == 0 {
        return digits_before_index;
    }
    let mut current_index = start_index;
    while current_index > 0 {
        current_index -= 1;
        let current_value = &split_number[current_index];
        if !current_value.parse::<u32>().is_ok() {
            break;
        }
        digits_before_index.push_str(current_value);
    }

    digits_before_index.chars().rev().collect::<String>()
}

fn combine_split_part_number_ascending(split_number: &Vec<String>, start_index: usize) -> String {
    let mut digits_after_index = String::new();
    if start_index == split_number.len() - 1 {
        return digits_after_index;
    }
    let mut current_index = start_index;
    while current_index < split_number.len() - 1 {
        current_index += 1;
        let current_value = &split_number[current_index];
        if !current_value.parse::<u32>().is_ok() {
            break;
        }
        digits_after_index.push_str(current_value);
    }

    digits_after_index
}

fn combine_split_part_number(split_number: &Vec<String>, index_of_digit_in_number: usize) -> u32 {
    let digits_after_index =
        combine_split_part_number_ascending(&split_number, index_of_digit_in_number);
    let digits_before_index =
        combine_split_part_number_descending(&split_number, index_of_digit_in_number);

    format!(
        "{digits_before_index}{}{digits_after_index}",
        split_number[index_of_digit_in_number]
    )
    .parse::<u32>()
    .unwrap()
}

fn extract_indices_of_numbers_adjacent_to_symbol(
    engine_schematic: &Vec<Vec<String>>,
    symbol_index: (usize, usize),
) -> Vec<(usize, usize)> {
    let left_index = symbol_index.0.checked_sub(1);
    let right_index = if symbol_index.0 < engine_schematic[symbol_index.1].len() - 1 {
        Some(symbol_index.0 + 1)
    } else {
        None
    };
    let above_index = symbol_index.1.checked_sub(1);
    let below_index = if symbol_index.1 < engine_schematic.len() - 1 {
        Some(symbol_index.1 + 1)
    } else {
        None
    };
    let coordinates_to_check = [
        (left_index, above_index),
        (Some(symbol_index.0), above_index),
        (right_index, above_index),
        (left_index, Some(symbol_index.1)),
        (right_index, Some(symbol_index.1)),
        (left_index, below_index),
        (Some(symbol_index.0), below_index),
        (right_index, below_index),
    ];
    coordinates_to_check
        .iter()
        .filter(|x| {
            x.0.is_some()
                && x.1.is_some()
                && engine_schematic[x.1.unwrap()][x.0.unwrap()]
                    .parse::<u32>()
                    .is_ok()
        })
        .map(|x| (x.0.unwrap(), x.1.unwrap()))
        .collect::<Vec<_>>()
}

fn find_part_indices_adjacent_to_symbol(
    engine_schematic: &Vec<Vec<String>>,
    find_gears: bool,
) -> Vec<(usize, usize)> {
    let mut part_locations = vec![];
    for (line_number, line) in engine_schematic.iter().enumerate() {
        for (index, schematic_item) in line.iter().enumerate() {
            if schematic_item.parse::<u32>().is_err() && schematic_item != &String::from(".") {
                let mut indices_of_numbers = extract_indices_of_numbers_adjacent_to_symbol(
                    engine_schematic,
                    (index, line_number),
                );
                if find_gears {
                    if schematic_item == &String::from("*") {
                        let deduplicated_indices_of_numbers =
                            deduplicate_part_indices(indices_of_numbers);
                        indices_of_numbers = if deduplicated_indices_of_numbers.len() == 2 {
                            deduplicated_indices_of_numbers
                        } else {
                            vec![]
                        };
                        part_locations.append(&mut indices_of_numbers);
                    } else {
                        continue;
                    }
                }
                part_locations.append(&mut indices_of_numbers);
            }
        }
    }
    part_locations
}

fn deduplicate_part_indices(mut part_indices: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut indices_to_remove = vec![];
    let mut visited = vec![];
    for (outer_index, i) in part_indices.iter().enumerate() {
        visited.push(outer_index);
        for (index, j) in part_indices.iter().enumerate() {
            if i.1 == j.1
                && i.0.abs_diff(j.0) <= 1
                && i != j
                && !visited.contains(&index)
                && !indices_to_remove.contains(&index)
            {
                indices_to_remove.push(index)
            }
        }
    }
    indices_to_remove.sort();
    indices_to_remove.reverse();
    for i in indices_to_remove {
        part_indices.remove(i);
    }
    part_indices
}

fn extract_part_numbers_to_sum(engine_schematic: &Vec<Vec<String>>, find_gears: bool) -> Vec<u32> {
    let part_indices = find_part_indices_adjacent_to_symbol(engine_schematic, find_gears);
    let deduplicated_part_indices = deduplicate_part_indices(part_indices);
    let mut part_numbers_to_sum = vec![];
    for part_index in deduplicated_part_indices {
        let part_number = combine_split_part_number(&engine_schematic[part_index.1], part_index.0);
        part_numbers_to_sum.push(part_number);
    }
    part_numbers_to_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combine_split_part_number_descending_1() {
        let expected = String::from("14");

        let actual = combine_split_part_number_descending(
            &vec![
                String::from("1"),
                String::from("4"),
                String::from("6"),
                String::from("7"),
                String::from("5"),
            ],
            2,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn combine_split_part_number_ascending_1() {
        let expected = String::from("75");

        let actual = combine_split_part_number_ascending(
            &vec![
                String::from("1"),
                String::from("4"),
                String::from("6"),
                String::from("7"),
                String::from("5"),
            ],
            2,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn combine_split_part_number_1() {
        let expected: u32 = 14675;

        let actual = combine_split_part_number(
            &vec![
                String::from("1"),
                String::from("4"),
                String::from("6"),
                String::from("7"),
                String::from("5"),
            ],
            2,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn combine_split_part_number_descending_2() {
        let expected = String::from("14");

        let actual = combine_split_part_number_descending(
            &vec![
                String::from("."),
                String::from("."),
                String::from("."),
                String::from("1"),
                String::from("4"),
                String::from("6"),
                String::from("7"),
                String::from("5"),
            ],
            5,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn combine_split_part_number_ascending_2() {
        let expected = String::from("75");

        let actual = combine_split_part_number_ascending(
            &vec![
                String::from("1"),
                String::from("4"),
                String::from("6"),
                String::from("7"),
                String::from("5"),
                String::from("."),
                String::from("."),
                String::from("."),
            ],
            2,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn combine_split_part_number_2() {
        let expected: u32 = 14675;

        let actual = combine_split_part_number(
            &vec![
                String::from("."),
                String::from("."),
                String::from("."),
                String::from("."),
                String::from("1"),
                String::from("4"),
                String::from("6"),
                String::from("7"),
                String::from("5"),
                String::from("."),
                String::from("."),
                String::from("."),
                String::from("."),
            ],
            6,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn find_adjacent_part_indices_1() {
        let expected = vec![(2, 0), (2, 2), (3, 2)];

        let input = "\
467..114..
...*......
..35..633."
            .split("\n")
            .map(|line| line.chars().map(String::from).collect::<Vec<String>>())
            .collect::<Vec<_>>();
        let actual = find_part_indices_adjacent_to_symbol(&input, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn deduplicate_part_indices_1() {
        let expected = vec![(2, 0), (2, 2)];

        let actual = deduplicate_part_indices(vec![(2, 0), (2, 2), (3, 2)]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn extract_part_numbers_to_sum_1() {
        let expected = vec![467, 35, 633, 617, 592, 664, 755, 598];

        let input = "\
467..114..
...*......
..35..633.
......#.*.
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .split("\n")
            .map(|line| line.chars().map(String::from).collect::<Vec<String>>())
            .collect::<Vec<_>>();
        let actual = extract_part_numbers_to_sum(&input, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn extract_part_numbers_to_sum_2() {
        let expected = vec![467, 35, 755, 598];

        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .split("\n")
            .map(|line| line.chars().map(String::from).collect::<Vec<String>>())
            .collect::<Vec<_>>();
        let actual = extract_part_numbers_to_sum(&input, true);

        assert_eq!(expected, actual);
    }
}
