use crate::puzzle::Puzzle;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)] // https://serde.rs/enum-representations.html
enum NumberOrArray {
    Number(u8),
    Array(Vec<NumberOrArray>),
}

impl Eq for NumberOrArray {}
impl PartialEq for NumberOrArray {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Ord for NumberOrArray {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (NumberOrArray::Number(s), NumberOrArray::Number(o)) => s.cmp(o),
            (NumberOrArray::Array(s), NumberOrArray::Array(o)) => s.cmp(o),
            (NumberOrArray::Number(_), NumberOrArray::Array(_)) => {
                let s = NumberOrArray::Array(vec![(*self).clone()]);
                s.cmp(other)
            }
            (NumberOrArray::Array(_), NumberOrArray::Number(_)) => {
                let o = NumberOrArray::Array(vec![(*other).clone()]);
                self.cmp(&o)
            }
        }
    }
}

impl PartialOrd for NumberOrArray {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct DayThirteen;

impl Puzzle for DayThirteen {
    fn test(&self) -> (String, String) {
        ("13".to_string(), "140".to_string())
    }

    fn one(&self, input: String) -> String {
        let parts = input.split("\n\n").collect::<Vec<&str>>();

        let mut sum = 0;
        for (i, part) in parts.iter().enumerate() {
            let mut lines = part.lines();
            let one = lines.next().unwrap();
            let two = lines.next().unwrap();

            let one: Vec<NumberOrArray> = serde_json::from_str(one).unwrap();
            let two: Vec<NumberOrArray> = serde_json::from_str(two).unwrap();

            if one < two {
                sum += i + 1;
            }
        }

        sum.to_string()
    }

    fn two(&self, input: String) -> String {
        let mut parts = input
            .lines()
            .filter(|x| !x.trim().is_empty())
            .map(|x| serde_json::from_str::<NumberOrArray>(x).unwrap())
            .collect::<Vec<_>>();

        let divider_two = serde_json::from_str::<NumberOrArray>("[[2]]").unwrap();
        let divider_six = serde_json::from_str::<NumberOrArray>("[[6]]").unwrap();
        parts.push(divider_two.clone());
        parts.push(divider_six.clone());
        parts.sort();

        let d2p = parts.iter().position(|x| x == &divider_two).unwrap();
        let d6p = parts.iter().position(|x| x == &divider_six).unwrap();

        ((d2p + 1) * (d6p + 1)).to_string()
    }
}
