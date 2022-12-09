use crate::puzzle::Puzzle;

pub struct DaySix;

impl Puzzle for DaySix {
    fn test(&self) -> (String, String) {
        ("7".to_string(), "19".to_string())
    }

    fn one(&self, input: String) -> String {
        let input = input.chars().collect::<Vec<char>>();
        let windows = input.windows(4);

        for (idx, i) in windows.enumerate() {
            let mut works = true;

            for char in i {
                if i.iter().filter(|&x| x == char).count() > 1 {
                    works = false;
                }
            }

            if works {
                return (idx + 4).to_string();
            }
        }

        panic!("no solution found")
    }

    fn two(&self, input: String) -> String {
        let input = input.chars().collect::<Vec<char>>();
        let windows = input.windows(14);

        for (idx, i) in windows.enumerate() {
            let mut works = true;

            for char in i {
                if i.iter().filter(|&x| x == char).count() > 1 {
                    works = false;
                }
            }

            if works {
                return (idx + 14).to_string();
            }
        }

        panic!("no solution found")
    }
}
