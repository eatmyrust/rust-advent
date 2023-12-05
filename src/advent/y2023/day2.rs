use std::{error::Error, fs};

use super::super::{AdventDay, Parse};

pub struct NewDay2Puzzle {}

pub struct Day2Puzzle {
    parsed_input: Vec<Game>,
}

impl Parse for NewDay2Puzzle {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>> {
        let puzzle_input = fs::read_to_string(input_path)?;

        let parsed_input = puzzle_input
            .split("\n")
            .map(parse_game_string)
            .collect::<Vec<_>>();

        Ok(Box::new(Day2Puzzle { parsed_input }))
    }
}

fn separate_game_identifier_from_revealed_cubes(game_string: &str) -> (&str, &str) {
    let mut separated_parts = game_string.split(": ");
    (
        separated_parts.next().unwrap(),
        separated_parts.next().unwrap(),
    )
}

fn extract_game_identifier(game_identifier_string: &str) -> u32 {
    game_identifier_string
        .split(" ")
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

fn parse_revealed_cubes_string(revealed_cubes_string: &str) -> RevealedCubes {
    let mut revealed_cubes = RevealedCubes::new(0, 0, 0);
    revealed_cubes_string.split(", ").for_each(|x| {
        let mut count_and_color = x.split(" ");
        let count = count_and_color.next().unwrap().parse::<u32>().unwrap();
        let color = count_and_color.next().unwrap();
        revealed_cubes.set_by_color(color, count);
    });
    revealed_cubes
}

fn parse_game_string(game_string: &str) -> Game {
    let (game_identifier_string, revealed_cubes_list_string) =
        separate_game_identifier_from_revealed_cubes(game_string);
    let game_identifier = extract_game_identifier(game_identifier_string);
    let revealed_cubes = revealed_cubes_list_string
        .split("; ")
        .map(parse_revealed_cubes_string)
        .collect::<Vec<RevealedCubes>>();
    Game::new(game_identifier, revealed_cubes)
}

#[derive(Debug, PartialEq)]
struct RevealedCubes {
    red: u32,
    blue: u32,
    green: u32,
}

impl RevealedCubes {
    fn new(red: u32, green: u32, blue: u32) -> RevealedCubes {
        RevealedCubes { red, blue, green }
    }

    fn set_by_color(&mut self, color: &str, count: u32) {
        if color == "red" {
            self.red = count;
        } else if color == "green" {
            self.green = count;
        } else if color == "blue" {
            self.blue = count;
        }
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    identifier: u32,
    revealed_cubes: Vec<RevealedCubes>,
}

impl Game {
    fn new(identifier: u32, revealed_cubes: Vec<RevealedCubes>) -> Game {
        Game {
            identifier,
            revealed_cubes,
        }
    }
}

impl AdventDay for Day2Puzzle {
    fn solve_first_puzzle(&self) -> String {
        find_sum_of_identifiers_of_valid_games(&self.parsed_input).to_string()
    }

    fn solve_second_puzzle(&self) -> String {
        self.parsed_input
            .iter()
            .map(|game| {
                let largest_number_of_each_colored_cube =
                    find_largest_number_of_each_colored_cube_in_revealed_cubes(
                        &game.revealed_cubes,
                    );
                calculate_power_of_cubes(&largest_number_of_each_colored_cube)
            })
            .reduce(|acc, e| acc + e)
            .unwrap()
            .to_string()
    }
}

fn check_if_cubes_invalidate_game(revealed_cubes: &RevealedCubes) -> bool {
    if revealed_cubes.red > 12 {
        return true;
    }
    if revealed_cubes.green > 13 {
        return true;
    }
    if revealed_cubes.blue > 14 {
        return true;
    }
    false
}

fn check_if_game_is_valid(cubes_revealed_in_game: &Vec<RevealedCubes>) -> bool {
    !cubes_revealed_in_game
        .iter()
        .any(check_if_cubes_invalidate_game)
}

fn find_sum_of_identifiers_of_valid_games(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .filter(|x| check_if_game_is_valid(&x.revealed_cubes))
        .map(|valid| valid.identifier)
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn find_largest_number_of_each_colored_cube_in_revealed_cubes(
    cubes_revealed_in_game: &Vec<RevealedCubes>,
) -> RevealedCubes {
    let mut largest_number_of_revealed_cubes = RevealedCubes::new(0, 0, 0);

    for revealed_cubes in cubes_revealed_in_game {
        if revealed_cubes.blue > largest_number_of_revealed_cubes.blue {
            largest_number_of_revealed_cubes.blue = revealed_cubes.blue;
        }
        if revealed_cubes.red > largest_number_of_revealed_cubes.red {
            largest_number_of_revealed_cubes.red = revealed_cubes.red;
        }
        if revealed_cubes.green > largest_number_of_revealed_cubes.green {
            largest_number_of_revealed_cubes.green = revealed_cubes.green;
        }
    }

    largest_number_of_revealed_cubes
}

fn calculate_power_of_cubes(cubes: &RevealedCubes) -> u32 {
    cubes.blue * cubes.green * cubes.red
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_if_cubes_invalidate_game_all_valid_1() {
        let expected = false;

        let actual = check_if_cubes_invalidate_game(&RevealedCubes::new(4, 0, 3));

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_if_cubes_invalidate_game_all_valid_2() {
        let expected = false;

        let actual = check_if_cubes_invalidate_game(&RevealedCubes::new(1, 2, 6));

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_if_cubes_invalidate_game_all_valid_3() {
        let expected = false;

        let actual = check_if_cubes_invalidate_game(&RevealedCubes::new(0, 2, 0));

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_if_cubes_invalidate_game_red_invalid() {
        let expected = true;

        let actual = check_if_cubes_invalidate_game(&RevealedCubes::new(20, 8, 6));

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_if_cubes_invalidate_game_blue_invalid() {
        let expected = true;

        let actual = check_if_cubes_invalidate_game(&RevealedCubes::new(12, 3, 15));

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_if_cubes_invalidate_game_green_invalid() {
        let expected = true;

        let actual = check_if_cubes_invalidate_game(&RevealedCubes::new(5, 17, 10));

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_if_game_is_valid_valid_1() {
        let expected = true;

        let input = vec![
            RevealedCubes::new(4, 0, 3),
            RevealedCubes::new(1, 2, 6),
            RevealedCubes::new(0, 2, 0),
        ];
        let actual = check_if_game_is_valid(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_if_game_is_valid_valid_2() {
        let expected = true;

        let input = vec![
            RevealedCubes::new(0, 2, 1),
            RevealedCubes::new(1, 3, 4),
            RevealedCubes::new(0, 1, 1),
        ];
        let actual = check_if_game_is_valid(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_if_game_is_valid_invalid_1() {
        let expected = false;

        let input = vec![
            RevealedCubes::new(20, 8, 6),
            RevealedCubes::new(4, 13, 5),
            RevealedCubes::new(1, 5, 0),
        ];
        let actual = check_if_game_is_valid(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_if_game_is_valid_invalid_2() {
        let expected = false;

        let input = vec![
            RevealedCubes::new(3, 1, 6),
            RevealedCubes::new(6, 3, 0),
            RevealedCubes::new(12, 3, 15),
        ];
        let actual = check_if_game_is_valid(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn collect_identifiers_of_valid_games_1() {
        let expected = 8;

        let input = vec![
            Game::new(
                1,
                vec![
                    RevealedCubes::new(4, 0, 3),
                    RevealedCubes::new(1, 2, 6),
                    RevealedCubes::new(0, 2, 0),
                ],
            ),
            Game::new(
                2,
                vec![
                    RevealedCubes::new(0, 2, 1),
                    RevealedCubes::new(1, 3, 4),
                    RevealedCubes::new(0, 1, 1),
                ],
            ),
            Game::new(
                3,
                vec![
                    RevealedCubes::new(20, 8, 6),
                    RevealedCubes::new(4, 13, 5),
                    RevealedCubes::new(1, 5, 0),
                ],
            ),
            Game::new(
                4,
                vec![
                    RevealedCubes::new(3, 1, 6),
                    RevealedCubes::new(6, 3, 0),
                    RevealedCubes::new(12, 3, 15),
                ],
            ),
            Game::new(
                5,
                vec![RevealedCubes::new(6, 3, 1), RevealedCubes::new(1, 2, 2)],
            ),
        ];
        let actual = find_sum_of_identifiers_of_valid_games(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn separate_game_identifier_from_revealed_cubes_1() {
        let expected = ("Game 1", "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        let actual = separate_game_identifier_from_revealed_cubes(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn extract_game_identifier_1() {
        let expected: u32 = 1;

        let actual = extract_game_identifier("Game 1");

        assert_eq!(expected, actual);
    }

    #[test]
    fn extract_game_identifier_2() {
        let expected: u32 = 25423;

        let actual = extract_game_identifier("Game 25423");

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_revealed_cubes_string_1() {
        let expected = RevealedCubes::new(4, 0, 3);

        let actual = parse_revealed_cubes_string("3 blue, 4 red");

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_revealed_cubes_string_2() {
        let expected = RevealedCubes::new(1, 2, 6);

        let actual = parse_revealed_cubes_string("1 red, 2 green, 6 blue");

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_revealed_cubes_string_3() {
        let expected = RevealedCubes::new(0, 2, 0);

        let actual = parse_revealed_cubes_string("2 green");

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_revealed_cubes_string_4() {
        let expected = RevealedCubes::new(14, 3, 15);

        let actual = parse_revealed_cubes_string("3 green, 15 blue, 14 red");

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_game_string_1() {
        let expected = Game::new(
            1,
            vec![
                RevealedCubes::new(4, 0, 3),
                RevealedCubes::new(1, 2, 6),
                RevealedCubes::new(0, 2, 0),
            ],
        );

        let actual = parse_game_string("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert_eq!(expected, actual);
    }

    #[test]
    fn find_largest_number_of_each_colored_cube_in_game_1() {
        let expected = RevealedCubes::new(4, 2, 6);

        let input = vec![
            RevealedCubes::new(4, 0, 3),
            RevealedCubes::new(1, 2, 6),
            RevealedCubes::new(0, 2, 0),
        ];
        let actual = find_largest_number_of_each_colored_cube_in_revealed_cubes(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn find_largest_number_of_each_colored_cube_in_game_2() {
        let expected = RevealedCubes::new(20, 13, 6);

        let input = vec![
            RevealedCubes::new(20, 8, 6),
            RevealedCubes::new(4, 13, 5),
            RevealedCubes::new(1, 5, 0),
        ];
        let actual = find_largest_number_of_each_colored_cube_in_revealed_cubes(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_power_of_cubes_1() {
        let expected = 48;

        let actual = calculate_power_of_cubes(&RevealedCubes::new(4, 2, 6));

        assert_eq!(expected, actual)
    }

    #[test]
    fn calculate_power_of_cubes_2() {
        let expected = 1560;

        let actual = calculate_power_of_cubes(&RevealedCubes::new(20, 13, 6));

        assert_eq!(expected, actual)
    }

    #[test]
    fn calculate_power_of_cubes_edge_1() {
        let expected = 1560;

        let actual = calculate_power_of_cubes(&RevealedCubes::new(20, 13, 6));

        assert_eq!(expected, actual)
    }
}
