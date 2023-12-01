use std::{collections::VecDeque, error::Error, fs};

use super::super::{AdventDay, Parse};

pub struct NewDay5Puzzle {}

const OVERFLOW_ERROR_MESSAGE: &str = "Attempted to access crate at a negative stack index";
const STACK_INDEX_ERROR_MESSAGE: &str =
    "Tried to move a crate at a stack index that does not exist";
const MOVE_FROM_EMPTY_STACK_ERROR_MESSAGE: &str = "Tried to move too many crates from stack";
const STACK_EMPTY_AFTER_MOVES_ERROR_MESSAGE: &str =
    "Tried to find the crate at the top of the stack but the stack was empty";

pub struct Day5Puzzle {
    stacks: Vec<VecDeque<String>>,
    rearrangement_procedure: Vec<(u32, usize, usize)>,
}

impl Parse for NewDay5Puzzle {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>> {
        let puzzle_input = fs::read_to_string(input_path).unwrap();
        let (stacks_str, count_of_stacks_str, rearrangement_procedure_str) =
            split_input_into_sections(&puzzle_input);
        let count_of_stacks = parse_count_of_stacks(count_of_stacks_str);
        let stacks = parse_stacks_of_crates(stacks_str, count_of_stacks);
        let rearrangement_procedure = parse_rearrangement_procedure(rearrangement_procedure_str);
        Ok(Box::new(Day5Puzzle {
            stacks,
            rearrangement_procedure,
        }))
    }
}

fn split_input_into_sections(input: &str) -> (&str, &str, &str) {
    let mut stacks_and_count_separated_from_moves = input.split("\n\n");
    let (stacks, count) = stacks_and_count_separated_from_moves
        .next()
        .unwrap()
        .rsplit_once("\n")
        .unwrap();
    let moves = stacks_and_count_separated_from_moves.next().unwrap();
    (stacks, count, moves)
}

fn parse_stacks_of_crates(stacks: &str, number_of_stacks: usize) -> Vec<VecDeque<String>> {
    let mut stacks_of_crates = vec![VecDeque::new(); number_of_stacks];
    let mut current_stack: usize = 0;
    stacks.chars().enumerate().for_each(|x| {
        if x.0 > 0 && (x.0 - 1) % 4 == 0 {
            if x.1 != ' ' {
                stacks_of_crates[current_stack].push_back(x.1.to_string());
            }
            current_stack += 1
        } else if x.1 == '\n' {
            current_stack = 0;
        }
    });
    stacks_of_crates
}

fn parse_count_of_stacks(count_of_stacks: &str) -> usize {
    let final_stack_index = count_of_stacks.chars().nth_back(1).unwrap();
    final_stack_index.to_digit(10).unwrap() as usize
}

fn parse_rearrangement_procedure_line(rearrangement_procedure_line: &str) -> (u32, usize, usize) {
    let words_to_ignore = ["move", "from", "to"];
    let mut parsed_procedure_iter = rearrangement_procedure_line
        .split(" ")
        .filter(|x| !words_to_ignore.contains(x))
        .map(|x| x.parse::<u32>().unwrap());
    let number_to_move = parsed_procedure_iter.next().unwrap();
    let from_stack = parsed_procedure_iter.next().unwrap() as usize;
    let to_stack = parsed_procedure_iter.next().unwrap() as usize;
    (number_to_move, from_stack, to_stack)
}

fn parse_rearrangement_procedure(rearrangement_procedure: &str) -> Vec<(u32, usize, usize)> {
    rearrangement_procedure
        .lines()
        .map(|l| parse_rearrangement_procedure_line(l))
        .collect::<Vec<_>>()
}

impl AdventDay for Day5Puzzle {
    fn solve_first_puzzle(&self) -> String {
        let mut cloned_stacks = self.stacks.clone();
        for procedure in &self.rearrangement_procedure {
            cloned_stacks = move_crates_between_stacks(
                cloned_stacks,
                procedure.0,
                procedure.1,
                procedure.2,
                false,
            )
            .unwrap();
        }
        retrieve_crates_on_top_of_stacks(&cloned_stacks).unwrap()
    }

    fn solve_second_puzzle(&self) -> String {
        let mut cloned_stacks = self.stacks.clone();
        for procedure in &self.rearrangement_procedure {
            cloned_stacks = move_crates_between_stacks(
                cloned_stacks,
                procedure.0,
                procedure.1,
                procedure.2,
                true,
            )
            .unwrap();
        }
        retrieve_crates_on_top_of_stacks(&cloned_stacks).unwrap()
    }
}

fn move_crates_between_stacks(
    mut stacks: Vec<VecDeque<String>>,
    number_to_move: u32,
    from_stack: usize,
    to_stack: usize,
    move_all_at_once: bool,
) -> Result<Vec<VecDeque<String>>, &'static str> {
    let mut crates_to_move = VecDeque::new();
    for _ in 0..number_to_move {
        let crate_to_move = stacks
            .get_mut(from_stack.checked_sub(1).ok_or(OVERFLOW_ERROR_MESSAGE)?)
            .ok_or(STACK_INDEX_ERROR_MESSAGE)?
            .pop_front()
            .ok_or(MOVE_FROM_EMPTY_STACK_ERROR_MESSAGE)?;
        if !move_all_at_once {
            stacks
                .get_mut(to_stack.checked_sub(1).ok_or(OVERFLOW_ERROR_MESSAGE)?)
                .ok_or(STACK_INDEX_ERROR_MESSAGE)?
                .push_front(crate_to_move);
        } else {
            crates_to_move.push_back(crate_to_move)
        }
    }
    if move_all_at_once {
        crates_to_move.append(
            stacks
                .get_mut(to_stack.checked_sub(1).ok_or(OVERFLOW_ERROR_MESSAGE)?)
                .ok_or(STACK_INDEX_ERROR_MESSAGE)?,
        );
        stacks[to_stack - 1] = crates_to_move;
    }
    Ok(stacks)
}

fn retrieve_crates_on_top_of_stacks(
    stacks: &Vec<VecDeque<String>>,
) -> Result<String, &'static str> {
    let mut crates_at_top_stacks = String::from("");
    for stack in stacks {
        crates_at_top_stacks.push_str(stack.front().ok_or(STACK_EMPTY_AFTER_MOVES_ERROR_MESSAGE)?);
    }
    Ok(crates_at_top_stacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_5_move_1_from_2_to_1() -> Result<(), Box<dyn Error>> {
        let expected = vec![
            VecDeque::from([String::from("D"), String::from("N"), String::from("Z")]),
            VecDeque::from([String::from("C"), String::from("M")]),
            VecDeque::from([String::from("P")]),
        ];

        let input = vec![
            VecDeque::from([String::from("N"), String::from("Z")]),
            VecDeque::from([String::from("D"), String::from("C"), String::from("M")]),
            VecDeque::from([String::from("P")]),
        ];
        let actual = move_crates_between_stacks(input, 1, 2, 1, false)?;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn day_5_move_3_from_1_to_3() -> Result<(), Box<dyn Error>> {
        let expected = vec![
            VecDeque::from([]),
            VecDeque::from([String::from("C"), String::from("M")]),
            VecDeque::from([
                String::from("Z"),
                String::from("N"),
                String::from("D"),
                String::from("P"),
            ]),
        ];

        let input = vec![
            VecDeque::from([String::from("D"), String::from("N"), String::from("Z")]),
            VecDeque::from([String::from("C"), String::from("M")]),
            VecDeque::from([String::from("P")]),
        ];
        let actual = move_crates_between_stacks(input, 3, 1, 3, false)?;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn day_5_move_3_from_1_to_3_crate_mover_9001() -> Result<(), Box<dyn Error>> {
        let expected = vec![
            VecDeque::from([]),
            VecDeque::from([String::from("C"), String::from("M")]),
            VecDeque::from([
                String::from("D"),
                String::from("N"),
                String::from("Z"),
                String::from("P"),
            ]),
        ];

        let input = vec![
            VecDeque::from([String::from("D"), String::from("N"), String::from("Z")]),
            VecDeque::from([String::from("C"), String::from("M")]),
            VecDeque::from([String::from("P")]),
        ];
        let actual = move_crates_between_stacks(input, 3, 1, 3, true)?;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn day_5_move_2_from_2_to_1() -> Result<(), Box<dyn Error>> {
        let expected = vec![
            VecDeque::from([String::from("M"), String::from("C")]),
            VecDeque::from([]),
            VecDeque::from([
                String::from("Z"),
                String::from("N"),
                String::from("D"),
                String::from("P"),
            ]),
        ];

        let input = vec![
            VecDeque::from([]),
            VecDeque::from([String::from("C"), String::from("M")]),
            VecDeque::from([
                String::from("Z"),
                String::from("N"),
                String::from("D"),
                String::from("P"),
            ]),
        ];
        let actual = move_crates_between_stacks(input, 2, 2, 1, false)?;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn day_5_move_1_from_1_to_2() -> Result<(), Box<dyn Error>> {
        let expected = vec![
            VecDeque::from([String::from("C")]),
            VecDeque::from([String::from("M")]),
            VecDeque::from([
                String::from("Z"),
                String::from("N"),
                String::from("D"),
                String::from("P"),
            ]),
        ];

        let input = vec![
            VecDeque::from([String::from("M"), String::from("C")]),
            VecDeque::from([]),
            VecDeque::from([
                String::from("Z"),
                String::from("N"),
                String::from("D"),
                String::from("P"),
            ]),
        ];
        let actual = move_crates_between_stacks(input, 1, 1, 2, false)?;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn day_5_move_from_non_existent_stack_catch_overflow() {
        let expected = Some(OVERFLOW_ERROR_MESSAGE);

        let input = vec![
            VecDeque::from([String::from("N"), String::from("Z")]),
            VecDeque::from([String::from("D"), String::from("C"), String::from("M")]),
            VecDeque::from([String::from("P")]),
        ];
        let actual = move_crates_between_stacks(input, 1, 0, 2, false);

        assert_eq!(expected, actual.err());
    }

    #[test]
    fn day_5_move_to_non_existent_stack_catch_overflow() {
        let expected = Some(OVERFLOW_ERROR_MESSAGE);

        let input = vec![
            VecDeque::from([String::from("N"), String::from("Z")]),
            VecDeque::from([String::from("D"), String::from("C"), String::from("M")]),
            VecDeque::from([String::from("P")]),
        ];
        let actual = move_crates_between_stacks(input, 1, 1, 0, false);

        assert_eq!(expected, actual.err());
    }

    #[test]
    fn day_5_move_from_non_existent_stack() {
        let expected = Some(STACK_INDEX_ERROR_MESSAGE);

        let input = vec![
            VecDeque::from([String::from("N"), String::from("Z")]),
            VecDeque::from([String::from("D"), String::from("C"), String::from("M")]),
            VecDeque::from([String::from("P")]),
        ];
        let actual = move_crates_between_stacks(input, 1, 5, 2, false);

        assert_eq!(expected, actual.err());
    }

    #[test]
    fn day_5_move_to_non_existent_stack() {
        let expected = Some(STACK_INDEX_ERROR_MESSAGE);

        let input = vec![
            VecDeque::from([String::from("N"), String::from("Z")]),
            VecDeque::from([String::from("D"), String::from("C"), String::from("M")]),
            VecDeque::from([String::from("P")]),
        ];
        let actual = move_crates_between_stacks(input, 1, 1, 7, false);

        assert_eq!(expected, actual.err());
    }

    #[test]
    fn day_5_move_too_many_crates_from_stack() {
        let expected = Some(MOVE_FROM_EMPTY_STACK_ERROR_MESSAGE);

        let input = vec![
            VecDeque::from([String::from("N"), String::from("Z")]),
            VecDeque::from([String::from("D"), String::from("C"), String::from("M")]),
            VecDeque::from([String::from("P")]),
        ];
        let actual = move_crates_between_stacks(input, 4, 1, 3, false);

        assert_eq!(expected, actual.err());
    }

    #[test]
    fn day_5_get_crates_on_top_of_stacks() -> Result<(), Box<dyn Error>> {
        let expected = "CMZ";

        let input = vec![
            VecDeque::from([String::from("C")]),
            VecDeque::from([String::from("M")]),
            VecDeque::from([
                String::from("Z"),
                String::from("N"),
                String::from("D"),
                String::from("P"),
            ]),
        ];
        let actual = retrieve_crates_on_top_of_stacks(&input)?;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn day_5_parse_stacks_of_crates() {
        let expected = vec![
            VecDeque::from([String::from("D"), String::from("N"), String::from("Z")]),
            VecDeque::from([String::from("C"), String::from("M")]),
            VecDeque::from([String::from("P")]),
        ];

        let input = "\
[D]        
[N] [C]    
[Z] [M] [P]";
        let actual = parse_stacks_of_crates(input, 3);
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_5_parse_more_stacks_of_crates() {
        let expected = vec![
            VecDeque::from([
                String::from("R"),
                String::from("D"),
                String::from("N"),
                String::from("Z"),
            ]),
            VecDeque::from([String::from("V"), String::from("C"), String::from("M")]),
            VecDeque::from([
                String::from("B"),
                String::from("S"),
                String::from("Q"),
                String::from("P"),
            ]),
            VecDeque::from([String::from("G"), String::from("A")]),
            VecDeque::from([
                String::from("I"),
                String::from("Y"),
                String::from("O"),
                String::from("U"),
            ]),
        ];

        let input = "\
[R]     [B]     [I]
[D] [V] [S]     [Y]
[N] [C] [Q] [G] [O]
[Z] [M] [P] [A] [U]";
        let actual = parse_stacks_of_crates(input, 5);
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_5_parse_count_of_stacks_5() {
        let expected = 5;

        let input = " 1   2   3   4   5 ";
        let actual = parse_count_of_stacks(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_5_parse_count_of_stacks_3() {
        let expected = 3;

        let input = " 1   2   3 ";
        let actual = parse_count_of_stacks(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_5_parse_rearrangement_procedure_line() {
        let expected: (u32, usize, usize) = (1, 2, 1);

        let input = "move 1 from 2 to 1";
        let actual = parse_rearrangement_procedure_line(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_5_parse_rearrangement_procedure_line_complex_digits() {
        let expected: (u32, usize, usize) = (120102, 2239452, 64321);

        let input = "move 120102 from 2239452 to 64321";
        let actual = parse_rearrangement_procedure_line(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_5_parse_rearrangement_procedure() {
        let expected: Vec<(u32, usize, usize)> = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];

        let input = "\
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let actual = parse_rearrangement_procedure(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_5_parse_combined_input() {
        let expected = (
            "\
    [D]    
[N] [C]    
[Z] [M] [P]",
            " 1   2   3 ",
            "\
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );

        let input = "\
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let actual = split_input_into_sections(input);

        assert_eq!(expected, actual);
    }
}
