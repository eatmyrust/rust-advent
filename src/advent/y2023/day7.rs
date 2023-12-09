use std::{collections::HashMap, error::Error, fs, str::FromStr};

use super::super::{AdventDay, Parse};

const FIVE_OF_A_KIND_SCORE: u32 = 10000000;
const FOUR_OF_A_KIND_SCORE: u32 = 9000000;
const FULL_HOUSE_SCORE: u32 = 8000000;
const THREE_OF_A_KIND_SCORE: u32 = 7000000;
const TWO_PAIR_SCORE: u32 = 6000000;
const ONE_PAIR_SCORE: u32 = 5000000;
const HIGH_CARD_SCORE: u32 = 4000000;

pub struct NewDay7Puzzle {}

pub struct Day7Puzzle {
    part_one_parsed_input: Vec<Hand>,
    part_two_parsed_input: Vec<Hand>,
}

impl Parse for NewDay7Puzzle {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>> {
        let puzzle_input = fs::read_to_string(input_path)?;

        let part_one_parsed_input = parse_hands(&puzzle_input, false);
        let part_two_parsed_input = parse_hands(&puzzle_input, true);

        Ok(Box::new(Day7Puzzle {
            part_one_parsed_input,
            part_two_parsed_input,
        }))
    }
}

fn extract_character_counts_from_hand(hand: &str) -> HashMap<char, u32> {
    let mut char_counts = HashMap::new();
    for char in hand.chars() {
        *char_counts.entry(char).or_insert(0) += 1;
    }
    char_counts
}

fn parse_hand_and_bid(hand_and_bid_str: &str) -> (&str, &str) {
    let mut hand_and_bid = hand_and_bid_str.split(" ");
    let hand = hand_and_bid.next().unwrap();
    let bid = hand_and_bid.next().unwrap();
    (hand, bid)
}

fn parse_cards_in_hand(hand_str: &str) -> Vec<Card> {
    let mut cards = vec![];
    for card in hand_str.chars() {
        cards.push(Card::from_str(&card.to_string()).unwrap())
    }
    cards
}

fn calculate_hand_score(char_counts: &HashMap<char, u32>) -> u32 {
    let mut pair_counts: HashMap<u32, u32> = HashMap::new();
    for char_count in char_counts.values() {
        *pair_counts.entry(*char_count).or_insert(0) += 1;
    }
    if let Some(_) = pair_counts.get(&5) {
        return FIVE_OF_A_KIND_SCORE;
    }
    if let Some(_) = pair_counts.get(&4) {
        return FOUR_OF_A_KIND_SCORE;
    }
    let pair_count_keys = pair_counts.keys().collect::<Vec<_>>();
    if pair_count_keys.contains(&&3) && pair_count_keys.contains(&&2) {
        return FULL_HOUSE_SCORE;
    }
    if pair_count_keys.contains(&&3) {
        return THREE_OF_A_KIND_SCORE;
    }
    if let Some(pair_count) = pair_counts.get(&2) {
        if *pair_count == 2 {
            return TWO_PAIR_SCORE;
        }
        return ONE_PAIR_SCORE;
    }
    HIGH_CARD_SCORE
}

fn calculate_hand_score_with_jokers(mut char_counts: HashMap<char, u32>) -> u32 {
    let j_count = char_counts.remove(&'J').unwrap();
    let mut hand_score = 0;
    if char_counts.keys().len() == 0 {
        return FIVE_OF_A_KIND_SCORE;
    }
    for (key, _) in char_counts.iter() {
        let mut mutable_char_counts = char_counts.clone();
        *mutable_char_counts.get_mut(key).unwrap() += j_count;
        let new_hand_score = calculate_hand_score(&mutable_char_counts);
        hand_score = if new_hand_score > hand_score {
            new_hand_score
        } else {
            hand_score
        }
    }
    hand_score
}

fn calculate_character_position_additional_score(hand: &Vec<Card>, part_two: bool) -> u32 {
    let mut additional_score = 0;
    let position_multiplier = HashMap::from([(1, 1), (2, 15), (3, 211), (4, 2955), (5, 41370)]);
    for (i, card) in hand.iter().rev().enumerate() {
        let mut card_value = card.0;
        if part_two && card.0 == 11 {
            card_value = 1
        }
        additional_score += (position_multiplier.get(&(i as u32 + 1)).unwrap()) * card_value
    }
    additional_score
}

fn parse_hands(hands_str: &str, part_two: bool) -> Vec<Hand> {
    let mut hands = hands_str
        .split("\n")
        .map(|x| {
            let (hand, bid) = parse_hand_and_bid(x);
            let cards = parse_cards_in_hand(hand);
            let char_counts = extract_character_counts_from_hand(hand);
            let hand_score: u32;
            if part_two && char_counts.get(&'J').is_some() {
                hand_score = calculate_hand_score_with_jokers(char_counts);
            } else {
                hand_score = calculate_hand_score(&char_counts);
            }
            let card_position_additional_score =
                calculate_character_position_additional_score(&cards, part_two);
            Hand {
                cards,
                value: hand_score + card_position_additional_score,
                bid: bid.parse::<u32>().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    hands.sort();
    hands
}

impl AdventDay for Day7Puzzle {
    fn solve_first_puzzle(&self) -> String {
        calculate_total_winnings(&self.part_one_parsed_input).to_string()
    }

    fn solve_second_puzzle(&self) -> String {
        calculate_total_winnings(&self.part_two_parsed_input).to_string()
    }
}

#[derive(Debug, PartialEq)]
struct ParseCardError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card(u32);

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maybe_int = s.parse::<u32>();
        if let Ok(parsed_int) = maybe_int {
            return Ok(Card(parsed_int));
        }
        if s == "T" {
            return Ok(Card(10));
        } else if s == "J" {
            return Ok(Card(11));
        } else if s == "Q" {
            return Ok(Card(12));
        } else if s == "K" {
            return Ok(Card(13));
        } else if s == "A" {
            return Ok(Card(14));
        }
        Err(ParseCardError)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    value: u32,
    cards: Vec<Card>,
    bid: u32,
}

fn calculate_total_winnings(hands: &Vec<Hand>) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(i, x)| (i as u32 + 1) * x.bid)
        .reduce(|acc, e| acc + e)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_character_counts_from_hand_1() {
        let expected = HashMap::from([('A', 5)]);

        let actual = extract_character_counts_from_hand("AAAAA");

        assert_eq!(expected, actual);
    }

    #[test]
    fn extract_character_counts_from_hand_2() {
        let expected = HashMap::from([('3', 2), ('2', 1), ('T', 1), ('K', 1)]);

        let actual = extract_character_counts_from_hand("32T3K");

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_hand_score_1() {
        let expected = FIVE_OF_A_KIND_SCORE;

        let input = extract_character_counts_from_hand("AAAAA");
        let actual = calculate_hand_score(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_hand_score_2() {
        let expected = FOUR_OF_A_KIND_SCORE;

        let input = extract_character_counts_from_hand("AA8AA");
        let actual = calculate_hand_score(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_hand_score_3() {
        let expected = FULL_HOUSE_SCORE;

        let input = extract_character_counts_from_hand("23332");
        let actual = calculate_hand_score(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_hand_score_4() {
        let expected = THREE_OF_A_KIND_SCORE;

        let input = extract_character_counts_from_hand("TTT98");
        let actual = calculate_hand_score(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_hand_score_5() {
        let expected = TWO_PAIR_SCORE;

        let input = extract_character_counts_from_hand("23432");
        let actual = calculate_hand_score(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_hand_score_6() {
        let expected = ONE_PAIR_SCORE;

        let input = extract_character_counts_from_hand("A23A4");
        let actual = calculate_hand_score(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_hand_score_7() {
        let expected = HIGH_CARD_SCORE;

        let input = extract_character_counts_from_hand("23456");
        let actual = calculate_hand_score(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_hands_1() {
        let expected: Vec<Hand> = vec![
            Hand {
                value: 5132188,
                cards: vec![Card(3), Card(2), Card(10), Card(3), Card(13)],
                bid: 765,
            },
            Hand {
                value: 6569856,
                cards: vec![Card(13), Card(10), Card(11), Card(11), Card(10)],
                bid: 220,
            },
            Hand {
                value: 6577603,
                cards: vec![Card(13), Card(13), Card(6), Card(7), Card(7)],
                bid: 28,
            },
            Hand {
                value: 7429700,
                cards: vec![Card(10), Card(5), Card(5), Card(11), Card(5)],
                bid: 684,
            },
            Hand {
                value: 7534611,
                cards: vec![Card(12), Card(12), Card(12), Card(11), Card(14)],
                bid: 483,
            },
        ];

        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let actual = parse_hands(input, false);

        assert_eq!(expected, actual)
    }

    #[test]
    fn parse_hands_2() {
        let expected: Vec<Hand> = vec![
            Hand {
                value: 5132188,
                cards: vec![Card(3), Card(2), Card(10), Card(3), Card(13)],
                bid: 765,
            },
            Hand {
                value: 6577603,
                cards: vec![Card(13), Card(13), Card(6), Card(7), Card(7)],
                bid: 28,
            },
            Hand {
                value: 7332544,
                cards: vec![Card(7), Card(14), Card(7), Card(7), Card(2)],
                bid: 167,
            },
            Hand {
                value: 9330776,
                cards: vec![Card(7), Card(13), Card(13), Card(11), Card(13)],
                bid: 628,
            },
            Hand {
                value: 9429550,
                cards: vec![Card(10), Card(5), Card(5), Card(11), Card(5)],
                bid: 684,
            },
            Hand {
                value: 9534461,
                cards: vec![Card(12), Card(12), Card(12), Card(11), Card(14)],
                bid: 483,
            },
            Hand {
                value: 9567596,
                cards: vec![Card(13), Card(10), Card(11), Card(11), Card(10)],
                bid: 220,
            },
            Hand {
                value: 10044552,
                cards: vec![Card(11), Card(11), Card(11), Card(11), Card(11)],
                bid: 91,
            },
            Hand {
                value: 10620790,
                cards: vec![Card(14), Card(14), Card(11), Card(11), Card(14)],
                bid: 235,
            },
        ];

        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
7KKJK 628
AAJJA 235
JJJJJ 91
7A772 167";
        let actual = parse_hands(input, true);

        assert_eq!(expected, actual)
    }

    #[test]
    fn calculate_total_winnings_1() {
        let expected = 6440;

        let hands_str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let input = parse_hands(hands_str, false);
        let actual = calculate_total_winnings(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_total_winnings_2() {
        let expected = 5905;

        let hands_str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let input = parse_hands(hands_str, true);
        let actual = calculate_total_winnings(&input);

        assert_eq!(expected, actual);
    }
}
