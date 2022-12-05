use crate::puzzle::Puzzle;
use itertools::Itertools;

pub struct DayThree;

impl Puzzle for DayThree {
    fn one(&self, input: String) -> String {
        let input = input.trim().lines();

        let mut priorities = Vec::new();

        for line in input {
            let mut common = Vec::new();

            let half = line.len() / 2;
            let first = line[..half].to_string();
            let second = line[half..].to_string();

            for char in first.chars() {
                if second.contains(char) {
                    common.push(char);
                }
            }

            common = common.into_iter().unique().collect::<Vec<_>>();

            for char in common {
                let char_num = char as u32;
                let priority = if char.is_lowercase() {
                    char_num - 96
                } else {
                    (char_num - 64) + 26
                };

                priorities.push(priority);
            }
        }

        priorities.iter().sum::<u32>().to_string()
    }

    fn two(&self, input: String) -> String {
        let input = input.trim().lines();

        let mut priorities = Vec::new();

        for bap in &input.into_iter().chunks(3) {
            let bap = bap.collect::<Vec<_>>();

            let first = bap[0].to_string();
            let second = bap[1].to_string();
            let third = bap[2].to_string();

            let mut common = Vec::new();

            for char in first.chars() {
                if second.contains(char) && third.contains(char) {
                    common.push(char);
                }
            }

            common = common.into_iter().unique().collect::<Vec<_>>();

            for char in common {
                let char_num = char as u32;
                let priority = if char.is_lowercase() {
                    char_num - 96
                } else {
                    (char_num - 64) + 26
                };
                priorities.push(priority);
            }
        }

        priorities.iter().sum::<u32>().to_string()
    }
}
