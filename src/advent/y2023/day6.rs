use std::{error::Error, fs};

use super::super::{AdventDay, Parse};

pub struct NewDay6Puzzle {}

pub struct Day6Puzzle {
    allocated_times: Vec<u64>,
    distance_records: Vec<u64>,
}

impl Parse for NewDay6Puzzle {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>> {
        let puzzle_input = fs::read_to_string(input_path)?;

        let mut split_lines = puzzle_input.split("\n");
        let allocated_times = parse_input_line(split_lines.next().unwrap());
        let distance_records = parse_input_line(split_lines.next().unwrap());

        Ok(Box::new(Day6Puzzle {
            allocated_times,
            distance_records,
        }))
    }
}

fn parse_input_line(input_line: &str) -> Vec<u64> {
    let mut label_and_values = input_line.split(":");
    label_and_values.next();
    let values = label_and_values.next().unwrap();
    values
        .split(" ")
        .filter(|x| x != &"")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

impl AdventDay for Day6Puzzle {
    fn solve_first_puzzle(&self) -> String {
        calculate_margin_of_error(&self.allocated_times, &self.distance_records).to_string()
    }

    fn solve_second_puzzle(&self) -> String {
        let single_time = self
            .allocated_times
            .iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let single_record = self
            .distance_records
            .iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        calculate_number_of_different_winning_charge_times(single_record, single_time).to_string()
    }
}

fn find_lowest_charge_time_to_beat_record_distance(
    record_distance: u64,
    allocated_time: u64,
) -> u64 {
    for i in 0..allocated_time {
        if i * (allocated_time - i) > record_distance {
            return i;
        }
    }
    0
}

fn find_highest_charge_time_to_beat_record_distance(
    record_distance: u64,
    allocated_time: u64,
) -> u64 {
    let allocated_time_range = 0..allocated_time;
    for i in allocated_time_range.rev() {
        if i * (allocated_time - i) > record_distance {
            return i;
        }
    }
    0
}

fn calculate_number_of_different_winning_charge_times(
    record_distance: u64,
    allocated_time: u64,
) -> u64 {
    let lowest_charge_time =
        find_lowest_charge_time_to_beat_record_distance(record_distance, allocated_time);
    let highest_charge_time =
        find_highest_charge_time_to_beat_record_distance(record_distance, allocated_time);
    highest_charge_time - lowest_charge_time + 1
}

fn calculate_margin_of_error(allocated_times: &Vec<u64>, distance_records: &Vec<u64>) -> u64 {
    allocated_times
        .iter()
        .enumerate()
        .map(|(i, x)| calculate_number_of_different_winning_charge_times(distance_records[i], *x))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_margin_of_error_1() {
        let expected = 288;

        let allocated_times = vec![7, 15, 30];
        let distance_records = vec![9, 40, 200];
        let actual = calculate_margin_of_error(&allocated_times, &distance_records);

        assert_eq!(expected, actual);
    }
}
