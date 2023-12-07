// Please don't look at this solution, I'm so sorry for writing this
use std::{error::Error, fs};

use super::super::{AdventDay, Parse};

pub struct NewDay5Puzzle {}

pub struct Day5Puzzle {
    seeds: Vec<u64>,
    conversion_rules: Vec<Vec<AlmanacConversion>>,
}

impl Parse for NewDay5Puzzle {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>> {
        let puzzle_input = fs::read_to_string(input_path)?;

        let (seeds, conversion_rules) = split_input_into_sections(&puzzle_input);

        Ok(Box::new(Day5Puzzle {
            seeds,
            conversion_rules,
        }))
    }
}

fn parse_seeds_string(seeds_string: &str) -> Vec<u64> {
    let mut split_seeds_string = seeds_string.split(": ");
    split_seeds_string.next();
    split_seeds_string
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_almanac_conversion_string(almanac_conversion_string: &str) -> Vec<AlmanacConversion> {
    let mut split_almanac_conversion_string = almanac_conversion_string.split("\n");
    split_almanac_conversion_string.next();

    split_almanac_conversion_string
        .map(|x| {
            let mut conversion_fields = x.split(" ");
            let destination_range_start = conversion_fields.next().unwrap().parse::<u64>().unwrap();
            let source_range_start = conversion_fields.next().unwrap().parse::<u64>().unwrap();
            let range = conversion_fields.next().unwrap().parse::<u64>().unwrap();
            AlmanacConversion::new(source_range_start, destination_range_start, range)
        })
        .collect::<Vec<_>>()
}

fn split_input_into_sections(input: &str) -> (Vec<u64>, Vec<Vec<AlmanacConversion>>) {
    let mut input_sections = input.split("\n\n");

    let seeds_str = input_sections.next().unwrap();
    let seeds = parse_seeds_string(seeds_str);

    let conversion_rules = input_sections
        .map(parse_almanac_conversion_string)
        .collect::<Vec<_>>();

    (seeds, conversion_rules)
}

impl AdventDay for Day5Puzzle {
    fn solve_first_puzzle(&self) -> String {
        find_minimum_location_from_seeds(&self.conversion_rules, &self.seeds).to_string()
    }

    fn solve_second_puzzle(&self) -> String {
        // The commented out part of this function solved the puzzle but took 4 hours
        find_minimum_location_from_seed_ranges(&self.conversion_rules, &self.seeds).to_string()
        //     let mut expanded_seeds = vec![];
        //     for (index, seed_range_start) in self.seeds.iter().enumerate() {
        //         if index % 2 != 0 {
        //             continue;
        //         }
        //         let mut expanded_seed_range =
        //             Vec::from_iter(*seed_range_start..*seed_range_start + self.seeds[index + 1]);
        //         expanded_seeds.append(&mut expanded_seed_range)
        //     }
        //     find_minimum_location_from_seeds(&self.conversion_rules, &expanded_seeds).to_string()
    }
}

#[derive(Debug, PartialEq)]
struct AlmanacConversion {
    source_range_start: u64,
    destination_range_start: u64,
    range: u64,
}

impl AlmanacConversion {
    fn new(source_range_start: u64, destination_range_start: u64, range: u64) -> AlmanacConversion {
        AlmanacConversion {
            source_range_start,
            destination_range_start,
            range,
        }
    }

    fn is_number_in_source_range(&self, source_category_number: u64) -> bool {
        let conversion_rule_source_range =
            self.source_range_start..self.source_range_start + self.range;
        return conversion_rule_source_range.contains(&source_category_number);
    }

    fn convert_source_number_to_dest_number(&self, source_category_number: u64) -> u64 {
        let source_difference = source_category_number - self.source_range_start;
        self.destination_range_start + source_difference
    }

    fn is_source_category_range_contained_in_source_range(
        &self,
        source_category_range: (u64, u64),
    ) -> bool {
        source_category_range.0 >= self.source_range_start
            && source_category_range.0 + source_category_range.1
                <= self.source_range_start + self.range
    }

    fn is_source_category_range_outside_of_source_range(
        &self,
        source_category_range: (u64, u64),
    ) -> bool {
        (source_category_range.0 < self.source_range_start
            && source_category_range.0 + source_category_range.1 - 1 < self.source_range_start)
            || source_category_range.0 > self.source_range_start + self.range - 1
    }

    fn is_source_range_contained_in_source_category_range(
        &self,
        source_category_range: (u64, u64),
    ) -> bool {
        self.source_range_start > source_category_range.0
            && self.source_range_start + self.range
                < source_category_range.0 + source_category_range.1
    }

    fn convert_partially_overlapping_source_category_range_to_dest_range(
        &self,
        source_category_range: (u64, u64),
    ) -> ((u64, u64), (u64, u64)) {
        let source_category_range_end = source_category_range.0 + source_category_range.1 - 1;
        let source_range_end = self.source_range_start + self.range - 1;
        if source_category_range.0 >= self.source_range_start {
            let start_diff = source_category_range.0 - self.source_range_start;
            let converted_start = self.destination_range_start + start_diff;
            let new_range = source_range_end - source_category_range.0 + 1;
            let overshoot = source_category_range_end - source_range_end + 1;
            return ((converted_start, new_range), (source_range_end, overshoot));
        }
        let new_range = source_category_range_end - self.source_range_start + 1;
        let undershoot = self.source_range_start - source_category_range.0;
        return (
            (self.destination_range_start, new_range),
            (source_category_range.0, undershoot),
        );
    }

    fn convert_source_category_range_containing_source_range(&self) -> (u64, u64) {
        let converted_range = (self.destination_range_start, self.range);
        converted_range
    }
}

fn convert_between_almanac_categories(
    conversion_rules: &Vec<AlmanacConversion>,
    source_category_number: u64,
) -> u64 {
    for conversion_rule in conversion_rules {
        if conversion_rule.is_number_in_source_range(source_category_number) {
            return conversion_rule.convert_source_number_to_dest_number(source_category_number);
        }
    }
    source_category_number
}

fn find_minimum_location_from_seeds(
    almanac_conversions: &Vec<Vec<AlmanacConversion>>,
    seeds: &Vec<u64>,
) -> u64 {
    seeds
        .iter()
        .map(|x| {
            let mut converted_value = *x;
            for almanac_conversion in almanac_conversions {
                converted_value =
                    convert_between_almanac_categories(&almanac_conversion, converted_value)
            }
            converted_value
        })
        .min()
        .unwrap()
}

fn extract_seed_ranges_from_seeds(seeds: &Vec<u64>) -> Vec<(u64, u64)> {
    let mut seed_ranges = vec![];
    for (index, seed_range_start) in seeds.iter().enumerate() {
        if index % 2 != 0 {
            continue;
        }
        seed_ranges.push((*seed_range_start, seeds[index + 1]))
    }
    seed_ranges
}

fn convert_range_between_almanac_categories(
    conversion_rules: &Vec<AlmanacConversion>,
    source_category_ranges: Vec<(u64, u64)>,
) -> Vec<(u64, u64)> {
    // Please don't look at this function, I don't even know why it works
    let mut converted_ranges = vec![];
    for mut source_category_range in source_category_ranges {
        let mut conversion_complete = false;
        for conversion_rule in conversion_rules {
            if conversion_rule
                .is_source_category_range_contained_in_source_range(source_category_range)
            {
                let converted_start = conversion_rule.destination_range_start
                    + (source_category_range.0 - conversion_rule.source_range_start);
                converted_ranges.push((converted_start, source_category_range.1));
                conversion_complete = true;
                break;
            }
            if conversion_rule
                .is_source_category_range_outside_of_source_range(source_category_range)
            {
                continue;
            }
            if conversion_rule
                .is_source_range_contained_in_source_category_range(source_category_range)
            {
                let converted_range =
                    conversion_rule.convert_source_category_range_containing_source_range();
                converted_ranges.push(converted_range);
                continue;
            }
            let (converted_range, still_need_to_convert_range) = conversion_rule
                .convert_partially_overlapping_source_category_range_to_dest_range(
                    source_category_range,
                );
            converted_ranges.push(converted_range);
            source_category_range = still_need_to_convert_range;
        }
        if !conversion_complete {
            converted_ranges.push(source_category_range);
        }
    }
    converted_ranges
}

fn find_minimum_location_from_seed_ranges(
    almanac_conversions: &Vec<Vec<AlmanacConversion>>,
    seeds: &Vec<u64>,
) -> u64 {
    let initial_seed_ranges = extract_seed_ranges_from_seeds(seeds);
    initial_seed_ranges
        .iter()
        .map(|x| {
            let mut converted_ranges = vec![*x];
            for almanac_conversion in almanac_conversions {
                converted_ranges =
                    convert_range_between_almanac_categories(almanac_conversion, converted_ranges);
            }
            converted_ranges
        })
        .flatten()
        .map(|x| x.0)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_destination_category_number_1() {
        let expected = 81;

        let conversion_rule = AlmanacConversion::new(50, 52, 48);
        let actual = conversion_rule.convert_source_number_to_dest_number(79);

        assert_eq!(expected, actual);
    }

    #[test]
    fn find_destination_category_number_2() {
        let expected = 57;

        let conversion_rule = AlmanacConversion::new(50, 52, 48);
        let actual = conversion_rule.convert_source_number_to_dest_number(55);

        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_between_almanac_categories_1() {
        let expected = 81;

        let input = vec![
            AlmanacConversion::new(98, 50, 2),
            AlmanacConversion::new(50, 52, 48),
        ];
        let actual = convert_between_almanac_categories(&input, 79);

        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_between_almanac_categories_2() {
        let expected = 74;

        let input = vec![
            AlmanacConversion::new(18, 88, 7),
            AlmanacConversion::new(25, 18, 70),
        ];
        let actual = convert_between_almanac_categories(&input, 81);

        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_between_almanac_categories_3() {
        let expected = 45;

        let input = vec![
            AlmanacConversion::new(98, 50, 2),
            AlmanacConversion::new(50, 52, 48),
        ];
        let actual = convert_between_almanac_categories(&input, 45);

        assert_eq!(expected, actual);
    }

    #[test]
    fn split_input_into_sections_1() {
        let expected = (
            vec![79, 14, 55, 13],
            vec![
                vec![
                    AlmanacConversion {
                        source_range_start: 98,
                        destination_range_start: 50,
                        range: 2,
                    },
                    AlmanacConversion {
                        source_range_start: 50,
                        destination_range_start: 52,
                        range: 48,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 15,
                        destination_range_start: 0,
                        range: 37,
                    },
                    AlmanacConversion {
                        source_range_start: 52,
                        destination_range_start: 37,
                        range: 2,
                    },
                    AlmanacConversion {
                        source_range_start: 0,
                        destination_range_start: 39,
                        range: 15,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 53,
                        destination_range_start: 49,
                        range: 8,
                    },
                    AlmanacConversion {
                        source_range_start: 11,
                        destination_range_start: 0,
                        range: 42,
                    },
                    AlmanacConversion {
                        source_range_start: 0,
                        destination_range_start: 42,
                        range: 7,
                    },
                    AlmanacConversion {
                        source_range_start: 7,
                        destination_range_start: 57,
                        range: 4,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 18,
                        destination_range_start: 88,
                        range: 7,
                    },
                    AlmanacConversion {
                        source_range_start: 25,
                        destination_range_start: 18,
                        range: 70,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 77,
                        destination_range_start: 45,
                        range: 23,
                    },
                    AlmanacConversion {
                        source_range_start: 45,
                        destination_range_start: 81,
                        range: 19,
                    },
                    AlmanacConversion {
                        source_range_start: 64,
                        destination_range_start: 68,
                        range: 13,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 69,
                        destination_range_start: 0,
                        range: 1,
                    },
                    AlmanacConversion {
                        source_range_start: 0,
                        destination_range_start: 1,
                        range: 69,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 56,
                        destination_range_start: 60,
                        range: 37,
                    },
                    AlmanacConversion {
                        source_range_start: 93,
                        destination_range_start: 56,
                        range: 4,
                    },
                ],
            ],
        );

        let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let actual = split_input_into_sections(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn find_minimum_location_from_seeds_1() {
        let expected = 35;

        let input = (
            vec![79, 14, 55, 13],
            vec![
                vec![
                    AlmanacConversion {
                        source_range_start: 98,
                        destination_range_start: 50,
                        range: 2,
                    },
                    AlmanacConversion {
                        source_range_start: 50,
                        destination_range_start: 52,
                        range: 48,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 15,
                        destination_range_start: 0,
                        range: 37,
                    },
                    AlmanacConversion {
                        source_range_start: 52,
                        destination_range_start: 37,
                        range: 2,
                    },
                    AlmanacConversion {
                        source_range_start: 0,
                        destination_range_start: 39,
                        range: 15,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 53,
                        destination_range_start: 49,
                        range: 8,
                    },
                    AlmanacConversion {
                        source_range_start: 11,
                        destination_range_start: 0,
                        range: 42,
                    },
                    AlmanacConversion {
                        source_range_start: 0,
                        destination_range_start: 42,
                        range: 7,
                    },
                    AlmanacConversion {
                        source_range_start: 7,
                        destination_range_start: 57,
                        range: 4,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 18,
                        destination_range_start: 88,
                        range: 7,
                    },
                    AlmanacConversion {
                        source_range_start: 25,
                        destination_range_start: 18,
                        range: 70,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 77,
                        destination_range_start: 45,
                        range: 23,
                    },
                    AlmanacConversion {
                        source_range_start: 45,
                        destination_range_start: 81,
                        range: 19,
                    },
                    AlmanacConversion {
                        source_range_start: 64,
                        destination_range_start: 68,
                        range: 13,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 69,
                        destination_range_start: 0,
                        range: 1,
                    },
                    AlmanacConversion {
                        source_range_start: 0,
                        destination_range_start: 1,
                        range: 69,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 56,
                        destination_range_start: 60,
                        range: 37,
                    },
                    AlmanacConversion {
                        source_range_start: 93,
                        destination_range_start: 56,
                        range: 4,
                    },
                ],
            ],
        );
        let actual = find_minimum_location_from_seeds(&input.1, &input.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn find_minimum_location_from_seed_ranges_1() {
        let expected = 46;

        let input = (
            vec![79, 14, 55, 13],
            vec![
                vec![
                    AlmanacConversion {
                        source_range_start: 98,
                        destination_range_start: 50,
                        range: 2,
                    },
                    AlmanacConversion {
                        source_range_start: 50,
                        destination_range_start: 52,
                        range: 48,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 15,
                        destination_range_start: 0,
                        range: 37,
                    },
                    AlmanacConversion {
                        source_range_start: 52,
                        destination_range_start: 37,
                        range: 2,
                    },
                    AlmanacConversion {
                        source_range_start: 0,
                        destination_range_start: 39,
                        range: 15,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 53,
                        destination_range_start: 49,
                        range: 8,
                    },
                    AlmanacConversion {
                        source_range_start: 11,
                        destination_range_start: 0,
                        range: 42,
                    },
                    AlmanacConversion {
                        source_range_start: 0,
                        destination_range_start: 42,
                        range: 7,
                    },
                    AlmanacConversion {
                        source_range_start: 7,
                        destination_range_start: 57,
                        range: 4,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 18,
                        destination_range_start: 88,
                        range: 7,
                    },
                    AlmanacConversion {
                        source_range_start: 25,
                        destination_range_start: 18,
                        range: 70,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 77,
                        destination_range_start: 45,
                        range: 23,
                    },
                    AlmanacConversion {
                        source_range_start: 45,
                        destination_range_start: 81,
                        range: 19,
                    },
                    AlmanacConversion {
                        source_range_start: 64,
                        destination_range_start: 68,
                        range: 13,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 69,
                        destination_range_start: 0,
                        range: 1,
                    },
                    AlmanacConversion {
                        source_range_start: 0,
                        destination_range_start: 1,
                        range: 69,
                    },
                ],
                vec![
                    AlmanacConversion {
                        source_range_start: 56,
                        destination_range_start: 60,
                        range: 37,
                    },
                    AlmanacConversion {
                        source_range_start: 93,
                        destination_range_start: 56,
                        range: 4,
                    },
                ],
            ],
        );
        let actual = find_minimum_location_from_seed_ranges(&input.1, &input.0);

        assert_eq!(expected, actual);
    }
}
