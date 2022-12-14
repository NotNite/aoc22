use crate::puzzle::Puzzle;

pub struct DayFourteen;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum State {
    Air,
    Stone,
    Sand,
}

fn parse_command(command: &str) -> (usize, usize) {
    let mut a = command.split(',');

    let one = a.next().unwrap();
    let two = a.next().unwrap();

    (one.parse().unwrap(), two.parse().unwrap())
}

fn build_puzzle(input: String) -> Vec<Vec<State>> {
    let mut command_groups = Vec::new();

    for line in input.lines() {
        let commands = line.split(" -> ").map(parse_command).collect::<Vec<_>>();
        command_groups.push(commands);
    }

    // calc size
    let mut max_y = 0;

    for group in &command_groups {
        for command in group {
            if command.1 > max_y {
                max_y = command.1;
            }
        }
    }

    let mut puzzle = Vec::new();

    for _ in 0..max_y + 2 {
        let mut a = Vec::new();

        // lol
        for _ in 0..1000 {
            a.push(State::Air);
        }

        puzzle.push(a);
    }

    for group in command_groups {
        let group = group.clone();
        let mut group = group.iter();

        let mut last_command = group.next().unwrap();
        for command in group {
            let x = command.0;
            let y = command.1;

            let last_x = last_command.0;
            let last_y = last_command.1;

            if x == last_x {
                let mut targets = [y, last_y];
                targets.sort();
                for y in targets[0]..targets[1] {
                    puzzle[y][x] = State::Stone;
                }
            } else {
                let mut targets = [x, last_x];
                targets.sort();
                for x in targets[0]..targets[1] + 1 {
                    puzzle[y][x] = State::Stone;
                }
            }

            last_command = command;
        }
    }

    puzzle
}

impl Puzzle for DayFourteen {
    fn test(&self) -> (String, String) {
        ("24".to_string(), "93".to_string())
    }

    fn one(&self, input: String) -> String {
        let mut puzzle = build_puzzle(input);
        let mut i = 0;

        loop {
            let mut new_sand = (500, 0);
            let mut hit_the_void = false;

            loop {
                if new_sand.1 >= puzzle.len() - 1 {
                    hit_the_void = true;
                    break;
                }

                if puzzle[new_sand.1 + 1][new_sand.0] == State::Air {
                    new_sand.1 += 1;
                } else if puzzle[new_sand.1 + 1][new_sand.0 - 1] == State::Air {
                    new_sand.1 += 1;
                    new_sand.0 -= 1;
                } else if puzzle[new_sand.1 + 1][new_sand.0 + 1] == State::Air {
                    new_sand.1 += 1;
                    new_sand.0 += 1;
                } else {
                    break;
                }
            }

            if hit_the_void {
                break;
            }

            puzzle[new_sand.1][new_sand.0] = State::Sand;
            i += 1;
        }

        i.to_string()
    }

    fn two(&self, input: String) -> String {
        let mut puzzle = build_puzzle(input);

        let mut temp = Vec::new();
        for _ in 0..1000 {
            temp.push(State::Stone);
        }
        puzzle.push(temp);

        let mut i = 0;

        loop {
            let mut new_sand = (500, 0);
            let mut stuck = false;

            loop {
                if puzzle[new_sand.1][new_sand.0] != State::Air {
                    stuck = true;
                    break;
                }
                if puzzle[new_sand.1 + 1][new_sand.0] == State::Air {
                    new_sand.1 += 1;
                } else if puzzle[new_sand.1 + 1][new_sand.0 - 1] == State::Air {
                    new_sand.1 += 1;
                    new_sand.0 -= 1;
                } else if puzzle[new_sand.1 + 1][new_sand.0 + 1] == State::Air {
                    new_sand.1 += 1;
                    new_sand.0 += 1;
                } else {
                    break;
                }
            }

            if stuck {
                break;
            }

            puzzle[new_sand.1][new_sand.0] = State::Sand;
            i += 1;
        }

        i.to_string()
    }
}
