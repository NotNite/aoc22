use crate::puzzle::Puzzle;

pub struct DayOne;

fn calc_groups(input: String) -> Vec<Vec<usize>> {
    input
        .split("\n\n")
        .map(|x| {
            x.trim()
                .split('\n')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

impl Puzzle for DayOne {
    fn test(&self) -> (String, String) {
        ("24000".to_string(), "45000".to_string())
    }

    fn one(&self, input: String) -> String {
        let groups = calc_groups(input);

        let mut max = 0;
        for group in groups {
            let sum = group.iter().sum();
            if sum > max {
                max = sum;
            }
        }

        max.to_string()
    }

    fn two(&self, input: String) -> String {
        let groups = calc_groups(input);

        let mut sum = groups
            .iter()
            .map(|x| x.iter().sum())
            .collect::<Vec<usize>>();
        sum.sort_by(|a, b| b.cmp(a));

        let a = sum[0];
        let b = sum[1];
        let c = sum[2];

        (a + b + c).to_string()
    }
}
