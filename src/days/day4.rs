use crate::puzzle::Puzzle;
use itertools::Itertools;

pub struct DayFour;

#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}

fn parse_input(input: String) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|x| {
            let ranges = x.split(',').map(|x| {
                let mut range = x.split('-');

                let min = range.next().unwrap().parse::<u32>().unwrap();
                let max = range.next().unwrap().parse::<u32>().unwrap();

                Range { min, max }
            });

            let mut ranges = ranges.collect_vec();
            (ranges.remove(0), ranges.remove(0))
        })
        .collect::<Vec<_>>()
}

impl Puzzle for DayFour {
    fn test(&self) -> (String, String) {
        ("2".to_string(), "4".to_string())
    }

    fn one(&self, input: String) -> String {
        let ranges_vec = parse_input(input);
        let mut count = 0;

        for ranges in ranges_vec {
            let (range1, range2) = ranges;
            let range_one_fully_overlaps = range1.min >= range2.min && range1.max <= range2.max;
            let range_two_fully_overlaps = range2.min >= range1.min && range2.max <= range1.max;

            if range_one_fully_overlaps || range_two_fully_overlaps {
                count += 1;
            }
        }

        count.to_string()
    }

    fn two(&self, input: String) -> String {
        let ranges_vec = parse_input(input);
        let mut count = 0;

        for ranges in ranges_vec {
            let (range1, range2) = ranges;
            let range_one_overlaps = range1.min <= range2.max && range1.max >= range2.min;
            let range_two_overlaps = range2.min <= range1.max && range2.max >= range1.min;

            if range_one_overlaps || range_two_overlaps {
                count += 1;
            }
        }

        count.to_string()
    }
}
