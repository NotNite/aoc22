use itertools::Itertools;

use crate::puzzle::Puzzle;
use std::collections::HashMap;

pub struct DayFive;

// *internal screaming*
fn parse_input(input: String) -> HashMap<u8, Vec<String>> {
    let mut map: HashMap<u8, Vec<String>> = HashMap::new();

    let starting_state = input.split("\n\n").next().unwrap().to_string();
    let starting_state = starting_state;

    for line in starting_state.lines().rev().skip(1) {
        let mut groups = Vec::new();

        for (idx, char) in line.chars().enumerate() {
            if (idx + 1) % 4 != 0 {
                groups.push(char.to_string());
            }
        }

        groups = groups.chunks(3).map(|x| x.join("")).collect();

        for (group_idx, group) in groups.iter().enumerate() {
            let group_idx = group_idx + 1;
            if group.trim() == "" {
                continue;
            }

            let real_group_lol = group.chars().nth(1).unwrap();

            let mut vec: Vec<String> = map.entry(group_idx as u8).or_default().to_vec();
            vec.push(real_group_lol.to_string());
            map.insert(group_idx as u8, vec);
        }
    }

    map
}

impl Puzzle for DayFive {
    fn one(&self, input: String) -> String {
        let mut state = parse_input(input.clone());
        let instructions = input.split("\n\n").last().unwrap().trim().lines();

        for instruction in instructions {
            // could i use a regex here? sure. do i want to? no lmao
            let instruction = instruction.split(' ').collect::<Vec<_>>();
            let move_count = instruction.get(1).unwrap().parse::<u8>().unwrap();
            let move_from = instruction.get(3).unwrap().parse::<u8>().unwrap();
            let move_to = instruction.get(5).unwrap().parse::<u8>().unwrap();

            for _ in 0..move_count {
                let mut vec = state.get_mut(&move_from).unwrap().to_vec();
                let item = vec.pop().unwrap();
                state.insert(move_from, vec);

                let mut vec = state.get_mut(&move_to).unwrap().to_vec();
                vec.push(item);
                state.insert(move_to, vec);
            }
        }

        let mut result = String::new();

        for key in state.keys().sorted() {
            let vec = state.get(key).unwrap();
            let value = vec.last().unwrap();
            result.push_str(value);
        }

        result
    }

    fn two(&self, input: String) -> String {
        let mut state = parse_input(input.clone());
        let instructions = input.split("\n\n").last().unwrap().trim().lines();

        for instruction in instructions {
            let instruction = instruction.split(' ').collect::<Vec<_>>();
            let move_count = instruction.get(1).unwrap().parse::<u8>().unwrap();
            let move_from = instruction.get(3).unwrap().parse::<u8>().unwrap();
            let move_to = instruction.get(5).unwrap().parse::<u8>().unwrap();

            let mut temp = Vec::new();
            for _ in 0..move_count {
                let mut vec = state.get_mut(&move_from).unwrap().to_vec();
                let item = vec.pop().unwrap();
                state.insert(move_from, vec);

                temp.push(item);
            }

            temp.reverse();

            let mut move_to_vec = state.get_mut(&move_to).unwrap().to_vec();
            move_to_vec.append(&mut temp);
            state.insert(move_to, move_to_vec);
        }

        let mut result = String::new();

        for key in state.keys().sorted() {
            let vec = state.get(key).unwrap();
            let value = vec.last().unwrap();
            result.push_str(value);
        }

        result
    }
}
