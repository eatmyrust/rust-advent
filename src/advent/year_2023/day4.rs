use std::{collections::HashSet, error::Error, fs};

use super::super::{AdventDay, Parse};

pub struct NewDay4Puzzle {}

pub struct Day4Puzzle {
    parsed_input: Vec<Card>,
}

impl Parse for NewDay4Puzzle {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>> {
        let puzzle_input = fs::read_to_string(input_path)?;

        let parsed_input = parse_input_into_cards(&puzzle_input);

        Ok(Box::new(Day4Puzzle { parsed_input }))
    }
}

fn parse_numbers_into_hashset(numbers: &str) -> HashSet<u32> {
    HashSet::<_>::from_iter(
        numbers
            .split(" ")
            .filter(|x| x != &"")
            .map(|x| x.trim().parse::<u32>().unwrap()),
    )
}

fn parse_card_components(card_line: &str) -> (&str, &str) {
    let mut split_card_line = card_line.split(": ").map(|x| x.split(" | ")).flatten();
    split_card_line.next();
    let winning_numbers = split_card_line.next().unwrap();
    let card_numbers = split_card_line.next().unwrap();
    (winning_numbers, card_numbers)
}

fn parse_input_into_cards(input: &str) -> Vec<Card> {
    input
        .split("\n")
        .map(|x| {
            let (winning_numbers, card_numbers) = parse_card_components(x);
            let winning_numbers = parse_numbers_into_hashset(winning_numbers);
            let card_numbers = parse_numbers_into_hashset(card_numbers);
            Card::new(winning_numbers, card_numbers)
        })
        .collect::<Vec<_>>()
}

impl AdventDay for Day4Puzzle {
    fn solve_first_puzzle(&self) -> String {
        calculate_sum_of_card_worths(&self.parsed_input).to_string()
    }

    fn solve_second_puzzle(&self) -> String {
        calculate_count_of_cards(&self.parsed_input).to_string()
    }
}

#[derive(Debug, PartialEq)]
struct Card {
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

impl Card {
    fn new(winning_numbers: HashSet<u32>, card_numbers: HashSet<u32>) -> Card {
        Card {
            winning_numbers,
            card_numbers,
        }
    }

    fn calculate_worth(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .fold(0, |acc, _| {
                if acc == 0 {
                    return acc + 1;
                }
                acc * 2
            })
    }

    fn calculate_matches(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .fold(0, |acc, _| acc + 1)
    }
}

fn calculate_sum_of_card_worths(cards: &Vec<Card>) -> u32 {
    cards
        .iter()
        .map(Card::calculate_worth)
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn calculate_count_of_cards(cards: &Vec<Card>) -> u32 {
    let mut card_counts = vec![1; cards.len()];
    for (index, card) in cards.iter().enumerate() {
        let card_matches = card.calculate_matches();
        for j in 1..card_matches + 1 {
            card_counts[index + j as usize] += 1 * card_counts[index];
        }
    }
    card_counts.into_iter().reduce(|acc, e| acc + e).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers_into_hashset_1() {
        let expected = HashSet::from([41, 48, 83, 86, 17]);

        let actual = parse_numbers_into_hashset("41 48 83 86 17");

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_numbers_into_hashset_2() {
        let expected = HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]);

        let actual = parse_numbers_into_hashset("83 86  6 31 17  9 48 53");

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_card_worth_1() {
        let expected = 8;

        let winning_numbers = parse_numbers_into_hashset("41 48 83 86 17");
        let card_numbers = parse_numbers_into_hashset("83 86  6 31 17  9 48 53");
        let actual = Card::new(winning_numbers, card_numbers).calculate_worth();

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_card_matches_1() {
        let expected = 4;

        let winning_numbers = parse_numbers_into_hashset("41 48 83 86 17");
        let card_numbers = parse_numbers_into_hashset("83 86  6 31 17  9 48 53");
        let actual = Card::new(winning_numbers, card_numbers).calculate_matches();

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_sum_of_card_worths_1() {
        let expected = 13;

        let cards = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let input = parse_input_into_cards(cards);
        let actual = calculate_sum_of_card_worths(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_count_of_cards_1() {
        let expected = 30;

        let cards = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let input = parse_input_into_cards(cards);
        let actual = calculate_count_of_cards(&input);

        assert_eq!(expected, actual);
    }
}
