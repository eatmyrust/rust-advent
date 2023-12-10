use std::{error::Error, fs};

use super::super::{AdventDay, Parse};

pub struct NewDay9Puzzle {}

pub struct Day9Puzzle {
    parsed_input: Vec<Vec<i64>>,
}

impl Parse for NewDay9Puzzle {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>> {
        let puzzle_input = fs::read_to_string(input_path)?;

        let parsed_input = puzzle_input
            .split("\n")
            .map(|line| {
                line.split(" ")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Box::new(Day9Puzzle { parsed_input }))
    }
}

impl AdventDay for Day9Puzzle {
    fn solve_first_puzzle(&self) -> String {
        self.parsed_input
            .iter()
            .map(|x| predict_value_of_sequence(x, PredictionDirection::Future))
            .reduce(|acc, e| acc + e)
            .unwrap()
            .to_string()
    }

    fn solve_second_puzzle(&self) -> String {
        self.parsed_input
            .iter()
            .map(|x| predict_value_of_sequence(x, PredictionDirection::Past))
            .reduce(|acc, e| acc + e)
            .unwrap()
            .to_string()
    }
}

enum PredictionDirection {
    Past,
    Future,
}

fn predict_value_of_sequence(
    sequence: &Vec<i64>,
    prediction_direction: PredictionDirection,
) -> i64 {
    let mut sequences_of_differences = vec![sequence.clone()];
    let mut sequence_of_differences_all_zero = false;

    while !sequence_of_differences_all_zero {
        let mut next_sequence_of_differences = vec![];
        let most_recently_calculated_sequence_of_differences =
            &sequences_of_differences[sequences_of_differences.len() - 1];
        for (i, x) in most_recently_calculated_sequence_of_differences
            .iter()
            .enumerate()
        {
            if i == most_recently_calculated_sequence_of_differences.len() - 1 {
                break;
            }
            next_sequence_of_differences
                .push(most_recently_calculated_sequence_of_differences[i + 1] - x)
        }
        sequence_of_differences_all_zero = next_sequence_of_differences.iter().all(|x| x == &0);
        sequences_of_differences.push(next_sequence_of_differences);
    }

    sequences_of_differences.reverse();
    let mut predicted_values = vec![0];
    for i in 0..sequences_of_differences.len() - 1 {
        let next_sequence_predicated_value_difference =
            predicted_values[predicted_values.len() - 1];
        let next_sequence = &sequences_of_differences[i + 1];
        match prediction_direction {
            PredictionDirection::Future => predicted_values.push(
                next_sequence[next_sequence.len() - 1] + next_sequence_predicated_value_difference,
            ),
            PredictionDirection::Past => {
                predicted_values.push(next_sequence[0] - next_sequence_predicated_value_difference)
            }
        }
    }
    predicted_values[predicted_values.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_next_value_of_sequence_1() {
        let expected = 18;

        let input = vec![0, 3, 6, 9, 12, 15];
        let actual = predict_value_of_sequence(&input, PredictionDirection::Future);

        assert_eq!(expected, actual)
    }
}
