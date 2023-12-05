use std::{error::Error, fs};

use super::super::{AdventDay, Parse};

const SPELLED_OUT_NUMBERS_NUMERIC_EQUIVALENT: &[(&str, &str); 9] = &[
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

pub struct NewDay1Puzzle {}

pub struct Day1Puzzle {
    parsed_input: Vec<String>,
}

impl Parse for NewDay1Puzzle {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>> {
        let puzzle_input = fs::read_to_string(input_path)?;

        let parsed_input = puzzle_input
            .split("\n")
            .map(String::from)
            .collect::<Vec<_>>();

        Ok(Box::new(Day1Puzzle { parsed_input }))
    }
}

impl AdventDay for Day1Puzzle {
    fn solve_first_puzzle(&self) -> String {
        extract_numbers_from_strings_and_find_sum(self.parsed_input.iter()).to_string()
    }

    fn solve_second_puzzle(&self) -> String {
        let input_with_spelled_out_numbers_replaced = self
            .parsed_input
            .iter()
            .map(|x| convert_spelled_out_numbers_to_numeric(x))
            .collect::<Vec<String>>();
        extract_numbers_from_strings_and_find_sum(input_with_spelled_out_numbers_replaced.iter())
            .to_string()
    }
}

fn extract_first_number_from_string(string: &str) -> &str {
    string.matches(char::is_numeric).next().unwrap()
}

fn extract_last_number_from_string(string: &str) -> &str {
    string.matches(char::is_numeric).next_back().unwrap()
}

fn extract_first_and_last_number_into_int(string: &str) -> u32 {
    let mut first_number = extract_first_number_from_string(string).to_string();
    let last_number = extract_last_number_from_string(string);
    first_number.push_str(last_number);
    first_number.parse::<u32>().unwrap()
}

fn extract_numbers_from_strings_and_find_sum<'a>(strings: impl Iterator<Item = &'a String>) -> u32 {
    strings
        .map(|x| extract_first_and_last_number_into_int(&x))
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn search_for_first_spelled_out_number<'a>(
    string_to_search: &str,
    number_to_search_for: &(&'a str, &'a str),
    current_first_spelled_out_number: (&'a str, &'a str, usize),
) -> (&'a str, &'a str, usize) {
    let maybe_found_index = string_to_search.find(number_to_search_for.0);
    if let Some(found_index) = maybe_found_index {
        if found_index < current_first_spelled_out_number.2 {
            return (number_to_search_for.0, number_to_search_for.1, found_index);
        }
    }
    current_first_spelled_out_number
}

fn search_for_last_spelled_out_number<'a>(
    string_to_search: &str,
    number_to_search_for: &(&'a str, &'a str),
    current_last_spelled_out_number: (&'a str, &'a str, usize),
) -> (&'a str, &'a str, usize) {
    let maybe_found_index = string_to_search.rfind(number_to_search_for.0);
    if let Some(found_index) = maybe_found_index {
        if found_index > current_last_spelled_out_number.2 {
            return (number_to_search_for.0, number_to_search_for.1, found_index);
        }
    }
    current_last_spelled_out_number
}

fn find_first_and_last_spelled_out_number(input: &str) -> ((&str, &str), (&str, &str)) {
    let mut first_spelled_out_number = ("one", "1", input.len());
    let mut last_spelled_out_number = ("one", "1", 0);
    for number_to_find in SPELLED_OUT_NUMBERS_NUMERIC_EQUIVALENT {
        first_spelled_out_number =
            search_for_first_spelled_out_number(input, number_to_find, first_spelled_out_number);
        last_spelled_out_number =
            search_for_last_spelled_out_number(input, number_to_find, last_spelled_out_number)
    }
    (
        (first_spelled_out_number.0, first_spelled_out_number.1),
        (last_spelled_out_number.0, last_spelled_out_number.1),
    )
}

fn convert_spelled_out_numbers_to_numeric(string: &str) -> String {
    let (first_spelled_out_number, last_spelled_out_number) =
        find_first_and_last_spelled_out_number(string);
    let string_with_first_spelled_out_number_replaced =
        string.replace(first_spelled_out_number.0, first_spelled_out_number.1);
    let string_with_last_spelled_out_number_replaced =
        string.replace(last_spelled_out_number.0, last_spelled_out_number.1);
    let combined_replaced_strings = format!("{string_with_first_spelled_out_number_replaced}{string_with_last_spelled_out_number_replaced}");
    combined_replaced_strings
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn first_number_from_string_is_1() {
        let expected = "1";

        let input = "1abc2";
        let actual = extract_first_number_from_string(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn first_number_from_string_is_3() {
        let expected = "3";

        let input = "pqr3stu8vwx";
        let actual = extract_first_number_from_string(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn first_number_from_string_is_7() {
        let expected = "7";

        let input = "treb7uchet";
        let actual = extract_first_number_from_string(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn last_number_from_string_is_2() {
        let expected = "2";

        let input = "1abc2";
        let actual = extract_last_number_from_string(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn last_number_from_string_is_8() {
        let expected = "8";

        let input = "pqr3stu8vwx";
        let actual = extract_last_number_from_string(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn last_number_from_string_is_7() {
        let expected = "7";

        let input = "treb7uchet";
        let actual = extract_last_number_from_string(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn first_and_last_number_from_string_is_12() {
        let expected: u32 = 12;

        let input = "1abc2";
        let actual = extract_first_and_last_number_into_int(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn first_and_last_number_from_string_is_38() {
        let expected: u32 = 38;

        let input = "pqr3stu8vwx";
        let actual = extract_first_and_last_number_into_int(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn first_and_last_number_from_string_is_77() {
        let expected: u32 = 77;

        let input = "treb7uchet";
        let actual = extract_first_and_last_number_into_int(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn extract_numbers_and_find_sum_is_142() {
        let expected: u32 = 142;

        let input = vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        ];
        let actual = extract_numbers_from_strings_and_find_sum(input.iter());

        assert_eq!(expected, actual)
    }

    #[test]
    fn search_for_spelled_out_number_happy_1() {
        let expected: (&str, &str, usize) = ("two", "2", 0);

        let actual =
            search_for_first_spelled_out_number("two1nine", &("two", "2"), ("one", "1", 8));

        assert_eq!(expected, actual);
    }

    #[test]
    fn find_first_spelled_out_number_happy_1() {
        let expected = (("two", "2"), ("nine", "9"));

        let actual = find_first_and_last_spelled_out_number("two1nine");

        assert_eq!(expected, actual);
    }

    #[test]
    fn find_first_spelled_out_number_edge_1() {
        let expected = (("two", "2"), ("four", "4"));

        // even though two and one overlap, function should identify two
        let input = "xtwone3four";
        let actual = find_first_and_last_spelled_out_number(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn find_last_spelled_out_number_edge_1() {
        let expected = (("one", "1"), ("eight", "8"));

        // even though one and eight overlap, function should identify both
        let input = "zoneight234";
        let actual = find_first_and_last_spelled_out_number(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn convert_spelled_out_numbers_to_numeric_2_9() {
        let expected = "21ninetwo19";

        let input = "two1nine";
        let actual = convert_spelled_out_numbers_to_numeric(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn convert_spelled_out_numbers_to_numeric_8_3() {
        let expected = "8wothreeeightwo3";

        let input = "eightwothree";
        let actual = convert_spelled_out_numbers_to_numeric(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn convert_spelled_out_numbers_to_numeric_1_3() {
        let expected = "abc12threexyzabcone23xyz";

        let input = "abcone2threexyz";
        let actual = convert_spelled_out_numbers_to_numeric(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn convert_spelled_out_numbers_to_numeric_2_4() {
        let expected = "x2ne3fourxtwone34";

        let input = "xtwone3four";
        let actual = convert_spelled_out_numbers_to_numeric(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn convert_spelled_out_numbers_to_numeric_9_7() {
        let expected = "49eightseven24nineeight72";

        let input = "4nineeightseven2";
        let actual = convert_spelled_out_numbers_to_numeric(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn convert_spelled_out_numbers_to_numeric_1() {
        let expected = "z1ight234zon8234";

        let input = "zoneight234";
        let actual = convert_spelled_out_numbers_to_numeric(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn convert_spelled_out_numbers_to_numeric_6() {
        let expected = "7pqrst6teen7pqrst6teen";

        let input = "7pqrstsixteen";
        let actual = convert_spelled_out_numbers_to_numeric(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn convert_spelled_out_numbers_to_numeric_two_occurences() {
        let expected = "414nineeightvxxjdthreeeight41fournine8vxxjdthree8";

        let input = "41fournineeightvxxjdthreeeight";
        let actual = convert_spelled_out_numbers_to_numeric(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn convert_spelled_out_numbers_to_numeric_overlapping_replacement() {
        let expected = "419ightvxx41nin8vxx";

        let input = "41nineightvxx";
        let actual = convert_spelled_out_numbers_to_numeric(input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn extract_numbers_and_find_sum_is_281() {
        let expected: u32 = 281;

        let input = vec![
            String::from("21ninetwo19"),
            String::from("8wothreeeightwo3"),
            String::from("abc12threexyzabcone23xyz"),
            String::from("x2ne3fourxtwone34"),
            String::from("49eightseven24nineeight72"),
            String::from("z1ight234zon8234"),
            String::from("7pqrst6teen7pqrst6teen"),
        ];
        let actual = extract_numbers_from_strings_and_find_sum(input.iter());

        assert_eq!(expected, actual)
    }

    #[test]
    fn extract_numbers_and_find_sum_with_edge_cases() {
        let expected: u32 = 377;

        let input = vec![
            String::from("21ninetwo19"),
            String::from("8wothreeeightwo3"),
            String::from("abc12threexyzabcone23xyz"),
            String::from("x2ne3fourxtwone34"),
            String::from("49eightseven24nineeight72"),
            String::from("z1ight234zon8234"),
            String::from("7pqrst6teen7pqrst6teen"),
            String::from("414nineeightvxxjdthreeeight41fournine8vxxjdthree8"),
            String::from("419ightvxx41nin8vxx"),
        ];
        let actual = extract_numbers_from_strings_and_find_sum(input.iter());

        assert_eq!(expected, actual)
    }
}
