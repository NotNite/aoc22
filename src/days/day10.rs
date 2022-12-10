use crate::puzzle::Puzzle;

pub struct DayTen;

fn cycle_check(cycle: i32, x: i32) -> Option<i32> {
    if cycle == 20 || (cycle - 20) % 40 == 0 {
        Some(x * cycle)
    } else {
        None
    }
}

fn pixel_check(cycle: i32, x: i32, output: &mut String) {
    let cycle_x = cycle % 40;

    if cycle_x == 0 {
        output.push('\n');
    }

    if cycle_x == x || cycle_x == x - 1 || cycle_x == x + 1 {
        output.push('#');
    } else {
        output.push('.');
    }
}

impl Puzzle for DayTen {
    fn test(&self) -> (String, String) {
        (
            "13140".to_string(),
            "dawg i am not putting this shit in a test case".to_string(),
        )
    }

    fn one(&self, input: String) -> String {
        let mut lines = input.lines().collect::<Vec<_>>();
        lines.reverse();

        let mut cycle = 1;
        let mut x = 1;

        let mut addx_in_progress = false;
        let mut addx_add_by = 0;

        let mut signal_strength_sum = 0;

        while !lines.is_empty() {
            let line = lines.pop().unwrap();
            if line.starts_with("addx") {
                let mut parts = line.split(' ');
                parts.next();
                let amnt = parts.next().unwrap().parse::<i32>().unwrap();

                addx_add_by += amnt;
                addx_in_progress = true;
            }

            cycle += 1;

            if let Some(strength) = cycle_check(cycle, x) {
                signal_strength_sum += strength;
            }

            if addx_in_progress {
                x += addx_add_by;
                addx_add_by = 0;
                addx_in_progress = false;

                cycle += 1;

                if let Some(strength) = cycle_check(cycle, x) {
                    signal_strength_sum += strength;
                }
            }
        }

        signal_strength_sum.to_string()
    }

    fn two(&self, input: String) -> String {
        let mut lines = input.lines().collect::<Vec<_>>();
        lines.reverse();

        let mut cycle = 1;
        let mut x = 1;

        let mut addx_in_progress = false;
        let mut addx_add_by = 0;

        let mut output = "\n".to_string();

        while !lines.is_empty() {
            let line = lines.pop().unwrap();
            if line.starts_with("addx") {
                let mut parts = line.split(' ');
                parts.next();
                let amnt = parts.next().unwrap().parse::<i32>().unwrap();

                addx_add_by += amnt;
                addx_in_progress = true;
            }

            pixel_check(cycle, x, &mut output);
            cycle += 1;

            if addx_in_progress {
                x += addx_add_by;
                addx_add_by = 0;
                addx_in_progress = false;

                pixel_check(cycle, x, &mut output);
                cycle += 1;
            }
        }

        output.to_string()
    }
}
