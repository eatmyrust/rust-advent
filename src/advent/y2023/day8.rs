use std::{collections::HashMap, error::Error, fs};

use super::super::{AdventDay, Parse};

pub struct NewDay8Puzzle {}

pub struct Day8Puzzle {
    moves: Vec<LeftRight>,
    node_map: HashMap<String, (String, String)>,
}

impl Parse for NewDay8Puzzle {
    fn parse_input(&self, input_path: &str) -> Result<Box<dyn AdventDay>, Box<dyn Error>> {
        let puzzle_input = fs::read_to_string(input_path)?;

        let mut moves_and_node_map = puzzle_input.split("\n\n");

        let moves_str = moves_and_node_map.next().unwrap();
        let moves = parse_moves(moves_str);

        let node_map_str = moves_and_node_map.next().unwrap();
        let node_map = parse_nodes_map(node_map_str);

        Ok(Box::new(Day8Puzzle { moves, node_map }))
    }
}

fn parse_moves(left_or_right_string: &str) -> Vec<LeftRight> {
    left_or_right_string
        .chars()
        .map(LeftRight::from_char)
        .collect::<Vec<_>>()
}

fn parse_node(node_str: &str) -> (String, (String, String)) {
    let mut node_key_and_directions = node_str.split(" = ");
    let node_key = node_key_and_directions.next().unwrap();
    let node_directions_str = node_key_and_directions.next().unwrap();
    let mut node_directions = node_directions_str.split(", ").map(|x| {
        x.chars()
            .filter(|x| x != &'(' && x != &')')
            .collect::<String>()
    });
    (
        String::from(node_key),
        (
            node_directions.next().unwrap(),
            node_directions.next().unwrap(),
        ),
    )
}

fn parse_nodes_map(nodes_str: &str) -> HashMap<String, (String, String)> {
    let mut nodes_map = HashMap::new();
    nodes_str.split("\n").for_each(|x| {
        let parsed_node = parse_node(x);
        nodes_map.insert(parsed_node.0, parsed_node.1);
    });
    nodes_map
}

impl AdventDay for Day8Puzzle {
    fn solve_first_puzzle(&self) -> String {
        count_moves_to_end_of_map(&self.moves, &self.node_map).to_string()
    }

    fn solve_second_puzzle(&self) -> String {
        count_moves_to_end_of_map_as_ghost(&self.moves, &self.node_map).to_string()
    }
}

enum LeftRight {
    Left,
    Right,
}

impl LeftRight {
    fn from_char(c: char) -> LeftRight {
        if c == 'L' {
            return LeftRight::Left;
        }
        LeftRight::Right
    }
}

fn count_moves_to_end_of_map(
    moves: &Vec<LeftRight>,
    node_map: &HashMap<String, (String, String)>,
) -> u32 {
    let mut current_node = "AAA";
    let mut move_count = 0;
    let mut current_move_index = 0;

    while current_node != "ZZZ" {
        let possible_next_nodes = &node_map[current_node];
        if current_move_index == moves.len() {
            current_move_index = 0;
        }
        match moves[current_move_index] {
            LeftRight::Left => current_node = &possible_next_nodes.0,
            LeftRight::Right => current_node = &possible_next_nodes.1,
        }
        move_count += 1;
        current_move_index += 1;
    }
    move_count
}

fn count_moves_to_end_of_map_as_ghost(
    moves: &Vec<LeftRight>,
    node_map: &HashMap<String, (String, String)>,
) -> u64 {
    let nodes_to_traverse = node_map
        .keys()
        .filter(|x| x.chars().next_back().unwrap() == 'A')
        .collect::<Vec<_>>();
    let mut current_lcm = 1;

    for mut node in nodes_to_traverse {
        let mut move_count = 0;
        let mut current_move_index = 0;
        while node.chars().next_back().unwrap() != 'Z' {
            let possible_next_nodes = &node_map[node];
            if current_move_index == moves.len() {
                current_move_index = 0;
            }
            match moves[current_move_index] {
                LeftRight::Left => node = &possible_next_nodes.0,
                LeftRight::Right => node = &possible_next_nodes.1,
            }
            move_count += 1;
            current_move_index += 1;
        }
        current_lcm = lcm(current_lcm, move_count)
    }
    current_lcm
}

// Shamelessly stolen from https://www.geeksforgeeks.org/program-to-find-lcm-of-two-numbers/
fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_moves_to_end_of_map_1() {
        let expected = 6;

        let moves = parse_moves("LLR");
        let nodes_map_str = "\
AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let node_map = parse_nodes_map(nodes_map_str);
        let actual = count_moves_to_end_of_map(&moves, &node_map);

        assert_eq!(expected, actual);
    }

    #[test]
    fn count_moves_to_end_of_map_as_ghost_1() {
        let expected = 6;

        let moves = parse_moves("LR");
        let nodes_map_str = "\
11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let node_map = parse_nodes_map(nodes_map_str);
        let actual = count_moves_to_end_of_map_as_ghost(&moves, &node_map);

        assert_eq!(expected, actual);
    }
}
