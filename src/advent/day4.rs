use std::{error::Error, fs};

use super::{AdventDay, Parse};

pub struct NewDay4Puzzle {}

pub struct Day4Puzzle {
    parsed_input: Vec<(CleaningAssignment, CleaningAssignment)>,
}

impl Parse for NewDay4Puzzle {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>> {
        let puzzle_input = fs::read_to_string(input_path).unwrap();

        let parsed_input = parse_cleaning_assignments(&puzzle_input)?;
        Ok(Box::new(Day4Puzzle { parsed_input }))
    }
}

fn parse_cleaning_range(cleaning_range_string: &str) -> Result<CleaningAssignment, Box<dyn Error>> {
    let mut cleaning_zone_start_and_end = cleaning_range_string.split("-");
    let start_zone: u32 = cleaning_zone_start_and_end
        .next()
        .ok_or("Cleaning zone did not contain start or end zone.")?
        .parse()?;
    let end_zone: u32 = cleaning_zone_start_and_end
        .next()
        .ok_or("Cleaning zone did not contain start or end zone.")?
        .parse()?;
    Ok(CleaningAssignment::new(start_zone, end_zone))
}

fn parse_cleaning_assignments(
    input: &str,
) -> Result<Vec<(CleaningAssignment, CleaningAssignment)>, Box<dyn Error>> {
    input
        .split("\n")
        .map(|x| {
            let mut cleaning_range_iter = x.split(",");
            let first_cleaning_range = parse_cleaning_range(
                cleaning_range_iter
                    .next()
                    .ok_or("Cleaning assignments not separated by comma")?,
            )?;
            let second_cleaning_range = parse_cleaning_range(
                cleaning_range_iter
                    .next()
                    .ok_or("Cleaning assignments not separated by comma")?,
            )?;
            Ok((first_cleaning_range, second_cleaning_range))
        })
        .collect::<Result<Vec<(CleaningAssignment, CleaningAssignment)>, _>>()
}

impl AdventDay for Day4Puzzle {
    fn solve_first_puzzle(&self) -> String {
        count_overlapping_cleaning_assignments(&self.parsed_input, true).to_string()
    }

    fn solve_second_puzzle(&self) -> String {
        count_overlapping_cleaning_assignments(&self.parsed_input, false).to_string()
    }
}

#[derive(Debug, PartialEq)]
struct CleaningAssignment {
    start_zone: u32,
    end_zone: u32,
}

impl CleaningAssignment {
    fn new(start_zone: u32, end_zone: u32) -> CleaningAssignment {
        CleaningAssignment {
            start_zone,
            end_zone,
        }
    }

    fn contains_or_contained_in(&self, other: &CleaningAssignment) -> bool {
        (&self.start_zone <= &other.start_zone && &self.end_zone >= &other.end_zone)
            || (&self.start_zone >= &other.start_zone && &self.end_zone <= &other.end_zone)
    }

    fn partial_overlap(&self, other: &CleaningAssignment) -> bool {
        (&self.start_zone >= &other.start_zone && &self.start_zone <= &other.end_zone)
            || (&self.end_zone >= &other.start_zone && &self.end_zone <= &other.end_zone)
            || (self.contains_or_contained_in(&other))
    }
}

fn count_overlapping_cleaning_assignments(
    cleaning_assingments_to_compare: &Vec<(CleaningAssignment, CleaningAssignment)>,
    complete_overlap: bool,
) -> u32 {
    cleaning_assingments_to_compare
        .iter()
        .filter(|&x| {
            if complete_overlap {
                x.0.contains_or_contained_in(&x.1)
            } else {
                x.0.partial_overlap(&x.1)
            }
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_4_cleaning_assignment_2_4_does_not_contain_6_8() {
        assert!(
            !CleaningAssignment::new(2, 4).contains_or_contained_in(&CleaningAssignment::new(6, 8))
        );
    }

    #[test]
    fn day_4_cleaning_assignment_2_3_does_not_contain_4_5() {
        assert!(
            !CleaningAssignment::new(2, 3).contains_or_contained_in(&CleaningAssignment::new(4, 5))
        );
    }

    #[test]
    fn day_4_cleaning_assignment_5_7_does_not_contain_7_9() {
        assert!(
            !CleaningAssignment::new(5, 7).contains_or_contained_in(&CleaningAssignment::new(7, 9))
        );
    }

    #[test]
    fn day_4_cleaning_assignment_2_8_contains_3_7() {
        assert!(
            CleaningAssignment::new(2, 8).contains_or_contained_in(&CleaningAssignment::new(3, 7))
        );
    }

    #[test]
    fn day_4_cleaning_assignment_6_6_contained_in_4_6() {
        assert!(
            CleaningAssignment::new(6, 6).contains_or_contained_in(&CleaningAssignment::new(4, 6))
        );
    }

    #[test]
    fn day_4_cleaning_assignment_2_6_does_not_contain_4_8() {
        assert!(
            !CleaningAssignment::new(2, 6).contains_or_contained_in(&CleaningAssignment::new(4, 8))
        );
    }

    #[test]
    fn day_4_count_fully_overlapping_cleaning_assignments() {
        let expected = 2;

        let input = vec![
            (CleaningAssignment::new(2, 4), CleaningAssignment::new(6, 8)),
            (CleaningAssignment::new(2, 3), CleaningAssignment::new(4, 5)),
            (CleaningAssignment::new(5, 7), CleaningAssignment::new(7, 9)),
            (CleaningAssignment::new(2, 8), CleaningAssignment::new(3, 7)),
            (CleaningAssignment::new(6, 6), CleaningAssignment::new(4, 6)),
            (CleaningAssignment::new(2, 6), CleaningAssignment::new(4, 8)),
        ];
        let actual = count_overlapping_cleaning_assignments(&input, true);

        assert_eq!(actual, expected);
    }

    #[test]
    fn day_4_count_partially_overlapping_cleaning_assignments() {
        let expected = 4;

        let input = vec![
            (CleaningAssignment::new(2, 4), CleaningAssignment::new(6, 8)),
            (CleaningAssignment::new(2, 3), CleaningAssignment::new(4, 5)),
            (CleaningAssignment::new(5, 7), CleaningAssignment::new(7, 9)),
            (CleaningAssignment::new(2, 8), CleaningAssignment::new(3, 7)),
            (CleaningAssignment::new(6, 6), CleaningAssignment::new(4, 6)),
            (CleaningAssignment::new(2, 6), CleaningAssignment::new(4, 8)),
        ];
        let actual = count_overlapping_cleaning_assignments(&input, false);

        assert_eq!(actual, expected);
    }

    #[test]
    fn day_4_parse_cleaning_range_start_2_end_4() -> Result<(), Box<dyn Error>> {
        let expected = CleaningAssignment {
            start_zone: 2,
            end_zone: 4,
        };

        let actual = parse_cleaning_range("2-4")?;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn day_4_parse_cleaning_range_start_6_end_8() -> Result<(), Box<dyn Error>> {
        let expected = CleaningAssignment {
            start_zone: 6,
            end_zone: 8,
        };

        let actual = parse_cleaning_range("6-8")?;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn day_4_parse_cleaning_range_start_6_end_6() -> Result<(), Box<dyn Error>> {
        let expected = CleaningAssignment {
            start_zone: 6,
            end_zone: 6,
        };

        let actual = parse_cleaning_range("6-6")?;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn day_4_parse_cleaning_range() -> Result<(), Box<dyn Error>> {
        let expected = CleaningAssignment::new(8, 14);

        let actual = parse_cleaning_range("8-14")?;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn day_4_parse_cleaning_range_invalid_start_and_end() {
        let actual = parse_cleaning_range("foo-bar");

        assert!(actual.is_err())
    }

    #[test]
    fn day_4_parse_cleaning_range_empty_start() {
        let actual = parse_cleaning_range("-13");

        assert!(actual.is_err())
    }

    #[test]
    fn day_4_parse_cleaning_range_empty_end() {
        let actual = parse_cleaning_range("2-");

        assert!(actual.is_err())
    }

    #[test]
    fn day_4_parse_cleaning_range_empty_start_and_end() {
        let actual = parse_cleaning_range("-");

        assert!(actual.is_err())
    }

    #[test]
    fn day_4_parse_cleaning_range_empty() {
        let actual = parse_cleaning_range("");

        assert!(actual.is_err())
    }

    #[test]
    fn day_4_parse_cleaning_assignments_missing_comma() {
        let input = "\
2-46-8
2-3,4-5
5-7,7-9
2-83-7
6-6,4-6
2-6,4-8";
        let actual = parse_cleaning_assignments(input);

        assert!(actual.is_err())
    }

    #[test]
    fn day_4_parse_input() -> Result<(), Box<dyn Error>> {
        let expected = vec![
            (CleaningAssignment::new(2, 4), CleaningAssignment::new(6, 8)),
            (CleaningAssignment::new(2, 3), CleaningAssignment::new(4, 5)),
            (CleaningAssignment::new(5, 7), CleaningAssignment::new(7, 9)),
            (CleaningAssignment::new(2, 8), CleaningAssignment::new(3, 7)),
            (CleaningAssignment::new(6, 6), CleaningAssignment::new(4, 6)),
            (CleaningAssignment::new(2, 6), CleaningAssignment::new(4, 8)),
        ];

        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let actual = parse_cleaning_assignments(input)?;

        assert_eq!(expected, actual);
        Ok(())
    }
}
