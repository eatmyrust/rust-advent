use std::fs;

use super::{AdventDay, Parse};

pub struct NewDay2Puzzle {}

pub struct Day2Puzzle {
    parsed_input: Vec<(String, String)>,
}

impl Parse for NewDay2Puzzle {
    fn parse_input(&mut self, input_path: &str) -> Box<dyn AdventDay> {
        let puzzle_input = fs::read_to_string(input_path).unwrap();

        let parsed_input = parse_rock_paper_scissors_games(&puzzle_input);

        Box::new(Day2Puzzle {
            parsed_input: parsed_input,
        })
    }
}

fn parse_rock_paper_scissors_games(unparsed_games: &str) -> Vec<(String, String)> {
    unparsed_games
        .split("\n")
        .map(|rock_paper_scissors_game| {
            let mut moves_played_iter = rock_paper_scissors_game.split(" ");
            let opponent_played = String::from(moves_played_iter.next().unwrap());
            let you_played = String::from(moves_played_iter.next().unwrap());
            (opponent_played, you_played)
        })
        .collect::<Vec<(String, String)>>()
}

impl AdventDay for Day2Puzzle {
    fn solve_first_puzzle(&self) -> String {
        calculate_total_tournament_score_part_one(self.parsed_input.iter()).to_string()
    }

    fn solve_second_puzzle(&self) -> String {
        calculate_total_tournament_score_part_two(self.parsed_input.iter()).to_string()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for RockPaperScissors {
    fn from(s: &str) -> RockPaperScissors {
        if s == "A" || s == "X" {
            return RockPaperScissors::Rock;
        } else if s == "B" || s == "Y" {
            return RockPaperScissors::Paper;
        }
        RockPaperScissors::Scissors
    }
}

impl RockPaperScissors {
    fn value(&self) -> u32 {
        match self {
            &RockPaperScissors::Rock => 1,
            &RockPaperScissors::Paper => 2,
            &RockPaperScissors::Scissors => 3,
        }
    }
}

impl From<&str> for WinLossDraw {
    fn from(s: &str) -> WinLossDraw {
        if s == "X" {
            return WinLossDraw::Loss;
        } else if s == "Y" {
            return WinLossDraw::Draw;
        }
        WinLossDraw::Win
    }
}

#[derive(Debug, PartialEq)]
enum WinLossDraw {
    Win,
    Loss,
    Draw,
}

impl WinLossDraw {
    fn value(&self) -> u32 {
        match self {
            &WinLossDraw::Loss => 0,
            &WinLossDraw::Draw => 3,
            &WinLossDraw::Win => 6,
        }
    }
}

fn rock_paper_scissors(
    opponent_played: &RockPaperScissors,
    you_played: &RockPaperScissors,
) -> WinLossDraw {
    if opponent_played == you_played {
        return WinLossDraw::Draw;
    }
    match opponent_played {
        RockPaperScissors::Rock => {
            if you_played == &RockPaperScissors::Paper {
                return WinLossDraw::Win;
            }
        }
        RockPaperScissors::Scissors => {
            if you_played == &RockPaperScissors::Rock {
                return WinLossDraw::Win;
            }
        }
        RockPaperScissors::Paper => {
            if you_played == &RockPaperScissors::Scissors {
                return WinLossDraw::Win;
            }
        }
    }
    WinLossDraw::Loss
}

fn find_move_to_play(
    win_lose_or_draw: &WinLossDraw,
    opponent_played: &RockPaperScissors,
) -> RockPaperScissors {
    if win_lose_or_draw == &WinLossDraw::Draw {
        return *opponent_played;
    } else if win_lose_or_draw == &rock_paper_scissors(opponent_played, &RockPaperScissors::Rock) {
        return RockPaperScissors::Rock;
    } else if win_lose_or_draw == &rock_paper_scissors(opponent_played, &RockPaperScissors::Paper) {
        return RockPaperScissors::Paper;
    }
    RockPaperScissors::Scissors
}

fn calculate_score(game_result: &WinLossDraw, you_played: &RockPaperScissors) -> u32 {
    game_result.value() + you_played.value()
}

fn calculate_total_tournament_score_part_one<'a>(
    games: impl Iterator<Item = &'a (String, String)>,
) -> u32 {
    games
        .map(|x| {
            let you_played = RockPaperScissors::from(x.1.as_str());
            let game_result =
                rock_paper_scissors(&RockPaperScissors::from(x.0.as_str()), &you_played);
            calculate_score(&game_result, &you_played)
        })
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn calculate_total_tournament_score_part_two<'a>(
    games: impl Iterator<Item = &'a (String, String)>,
) -> u32 {
    games
        .map(|x| {
            let game_result = WinLossDraw::from(x.1.as_str());
            let you_played =
                find_move_to_play(&game_result, &RockPaperScissors::from(x.0.as_str()));
            calculate_score(&game_result, &you_played)
        })
        .reduce(|acc, e| acc + e)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_to_rock() {
        let expected = RockPaperScissors::Rock;

        let actual = RockPaperScissors::from("A");

        assert_eq!(actual, expected)
    }

    #[test]
    fn b_to_paper() {
        let expected = RockPaperScissors::Paper;

        let actual = RockPaperScissors::from("B");

        assert_eq!(actual, expected)
    }

    #[test]
    fn c_to_scissors() {
        let expected = RockPaperScissors::Scissors;

        let actual = RockPaperScissors::from("C");

        assert_eq!(actual, expected)
    }

    #[test]
    fn x_to_rock() {
        let expected = RockPaperScissors::Rock;

        let actual = RockPaperScissors::from("X");

        assert_eq!(actual, expected)
    }

    #[test]
    fn y_to_paper() {
        let expected = RockPaperScissors::Paper;

        let actual = RockPaperScissors::from("Y");

        assert_eq!(actual, expected)
    }

    #[test]
    fn z_to_scissors() {
        let expected = RockPaperScissors::Scissors;

        let actual = RockPaperScissors::from("Z");

        assert_eq!(actual, expected)
    }

    #[test]
    fn x_to_loss() {
        let expected = WinLossDraw::Loss;

        let actual = WinLossDraw::from("X");

        assert_eq!(actual, expected)
    }

    #[test]
    fn y_to_draw() {
        let expected = WinLossDraw::Draw;

        let actual = WinLossDraw::from("Y");

        assert_eq!(actual, expected)
    }

    #[test]
    fn z_to_win() {
        let expected = WinLossDraw::Win;

        let actual = WinLossDraw::from("Z");

        assert_eq!(actual, expected)
    }

    #[test]
    fn rock_beats_scissors_win() {
        let expected = WinLossDraw::Win;

        let actual = rock_paper_scissors(&RockPaperScissors::Scissors, &RockPaperScissors::Rock);

        assert_eq!(actual, expected);
    }

    #[test]
    fn scissors_beats_paper_win() {
        let expected = WinLossDraw::Win;

        let actual = rock_paper_scissors(&RockPaperScissors::Paper, &RockPaperScissors::Scissors);

        assert_eq!(actual, expected);
    }

    #[test]
    fn paper_beats_rock_win() {
        let expected = WinLossDraw::Win;

        let actual = rock_paper_scissors(&RockPaperScissors::Rock, &RockPaperScissors::Paper);

        assert_eq!(actual, expected);
    }

    #[test]
    fn rock_beats_scissors_loss() {
        let expected = WinLossDraw::Loss;

        let actual = rock_paper_scissors(&RockPaperScissors::Rock, &RockPaperScissors::Scissors);

        assert_eq!(actual, expected);
    }

    #[test]
    fn scissors_beats_paper_loss() {
        let expected = WinLossDraw::Loss;

        let actual = rock_paper_scissors(&RockPaperScissors::Scissors, &RockPaperScissors::Paper);

        assert_eq!(actual, expected);
    }

    #[test]
    fn paper_beats_rock_loss() {
        let expected = WinLossDraw::Loss;

        let actual = rock_paper_scissors(&RockPaperScissors::Paper, &RockPaperScissors::Rock);

        assert_eq!(actual, expected);
    }

    #[test]
    fn rock_draw() {
        let expected = WinLossDraw::Draw;

        let actual = rock_paper_scissors(&RockPaperScissors::Rock, &RockPaperScissors::Rock);

        assert_eq!(actual, expected);
    }

    #[test]
    fn scissors_draw() {
        let expected = WinLossDraw::Draw;

        let actual =
            rock_paper_scissors(&RockPaperScissors::Scissors, &RockPaperScissors::Scissors);

        assert_eq!(actual, expected);
    }

    #[test]
    fn paper_draw() {
        let expected = WinLossDraw::Draw;

        let actual = rock_paper_scissors(&RockPaperScissors::Paper, &RockPaperScissors::Paper);

        assert_eq!(actual, expected);
    }

    #[test]
    fn draw_against_rock() {
        let expected = RockPaperScissors::Rock;

        let actual = find_move_to_play(&WinLossDraw::Draw, &RockPaperScissors::Rock);

        assert_eq!(actual, expected);
    }

    #[test]
    fn lose_against_scissors() {
        let expected = RockPaperScissors::Paper;

        let actual = find_move_to_play(&WinLossDraw::Loss, &RockPaperScissors::Scissors);

        assert_eq!(actual, expected);
    }

    #[test]
    fn win_against_paper() {
        let expected = RockPaperScissors::Scissors;

        let actual = find_move_to_play(&WinLossDraw::Win, &RockPaperScissors::Paper);

        assert_eq!(actual, expected);
    }

    #[test]
    fn rock_vs_paper_win_score() {
        let expected = 8;

        let actual = calculate_score(&WinLossDraw::Win, &RockPaperScissors::Paper);

        assert_eq!(actual, expected)
    }

    #[test]
    fn paper_vs_rock_loss_score() {
        let expected = 1;

        let actual = calculate_score(&WinLossDraw::Loss, &RockPaperScissors::Rock);

        assert_eq!(actual, expected)
    }

    #[test]
    fn scissors_vs_scissors_draw_score() {
        let expected = 6;

        let actual = calculate_score(&WinLossDraw::Draw, &RockPaperScissors::Scissors);

        assert_eq!(actual, expected)
    }

    #[test]
    fn total_score_of_multiple_games_part_one() {
        let expected = 15;

        let games = vec![
            (String::from("A"), String::from("Y")),
            (String::from("B"), String::from("X")),
            (String::from("C"), String::from("Z")),
        ];
        let actual = calculate_total_tournament_score_part_one(games.iter());

        assert_eq!(actual, expected)
    }

    #[test]
    fn total_score_of_multiple_games_part_two() {
        let expected = 12;

        let games = vec![
            (String::from("A"), String::from("Y")),
            (String::from("B"), String::from("X")),
            (String::from("C"), String::from("Z")),
        ];
        let actual = calculate_total_tournament_score_part_two(games.iter());

        assert_eq!(actual, expected)
    }

    #[test]
    fn parse_input() {
        let expected = vec![
            (String::from("A"), String::from("Y")),
            (String::from("B"), String::from("X")),
            (String::from("C"), String::from("Z")),
        ];

        let input = "\
A Y
B X
C Z";
        let actual = parse_rock_paper_scissors_games(&input);

        assert_eq!(actual, expected);
    }
}
