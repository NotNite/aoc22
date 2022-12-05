use log::*;
use puzzle::Puzzle;

pub mod days;
pub mod puzzle;

fn get_day(day: u8) -> Option<Box<dyn Puzzle>> {
    match day {
        1 => Some(Box::new(days::one::DayOne)),
        2 => Some(Box::new(days::two::DayTwo)),
        3 => Some(Box::new(days::three::DayThree)),
        4 => Some(Box::new(days::four::DayFour)),
        5 => Some(Box::new(days::five::DayFive)),
        _ => None,
    }
}

fn run_day(day: u8) {
    let puzzle = get_day(day).expect("day not implemented in get_day");
    let input =
        std::fs::read_to_string(format!("input/day{}.txt", day)).expect("failed to read input");

    info!("day {}, part 1: {}", day, puzzle.one(input.clone()));
    info!("day {}, part 2: {}", day, puzzle.two(input));
}

fn setup_logging() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
        .expect("failed to setup logging");
}

fn main() {
    setup_logging();

    let day_str = std::env::args().nth(1);

    if let Some(day_str) = day_str {
        info!("running day {}", day_str);

        let day = day_str.parse::<u8>().expect("failed to parse day");
        run_day(day);
    } else {
        info!("running all days");

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
