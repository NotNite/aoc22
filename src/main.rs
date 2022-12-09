use puzzle::Puzzle;

pub mod days;
pub mod puzzle;

fn get_day(day: u8) -> Option<Box<dyn Puzzle>> {
    match day {
        1 => Some(Box::new(days::day1::DayOne)),
        2 => Some(Box::new(days::day2::DayTwo)),
        3 => Some(Box::new(days::day3::DayThree)),
        4 => Some(Box::new(days::day4::DayFour)),
        5 => Some(Box::new(days::day5::DayFive)),
        6 => Some(Box::new(days::day6::DaySix)),
        7 => Some(Box::new(days::day7::DaySeven)),
        8 => Some(Box::new(days::day8::DayEight)),
        9 => Some(Box::new(days::day9::DayNine)),
        _ => None,
    }
}

fn run_day(day: u8) {
    let puzzle = get_day(day).expect("day not implemented in get_day");

    let test_input = std::fs::read_to_string(format!("test/day{}.txt", day));

    if let Ok(test_input) = test_input {
        let (one_test, two_test) = puzzle.test();

        let one_test_result = puzzle.one(test_input.clone());
        let two_test_result = puzzle.two(test_input);

        assert!(
            one_test_result == one_test,
            "day {}, part 1 failed test (expected {}, got {})",
            day,
            one_test,
            one_test_result
        );
        assert!(
            two_test_result == two_test,
            "day {}, part 2 failed test (expected {}, got {})",
            day,
            two_test,
            two_test_result
        );
    } else {
        eprintln!("warning: day {} has no test input", day);
    }

    let input =
        std::fs::read_to_string(format!("input/day{}.txt", day)).expect("failed to read input");
    println!("day {}, part 1: {}", day, puzzle.one(input.clone()));
    println!("day {}, part 2: {}", day, puzzle.two(input));
}

fn main() {
    let day_str = std::env::args().nth(1);

    if let Some(day_str) = day_str {
        println!("running day {}", day_str);

        let day = day_str.parse::<u8>().expect("failed to parse day");
        run_day(day);
    } else {
        println!("running all days");

        let mut i = 1;
        loop {
            if get_day(i).is_some() {
                run_day(i);
            } else {
                break;
            }

            i += 1;
        }
    }
}
