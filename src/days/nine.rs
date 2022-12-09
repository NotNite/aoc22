use crate::puzzle::Puzzle;

pub struct DayNine;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_head(head: &mut Pos, direction: Direction) {
    match direction {
        Direction::Up => head.y += 1,
        Direction::Down => head.y -= 1,
        Direction::Left => head.x -= 1,
        Direction::Right => head.x += 1,
    }
}

fn move_tail(head: Pos, tail: &mut Pos) {
    let touching_horizontal = head.y == tail.y
        && ((head.x == tail.x + 1) || (head.x == tail.x - 1) || (head.x == tail.x));
    let touching_vertical = head.x == tail.x
        && ((head.y == tail.y + 1) || (head.y == tail.y - 1) || (head.y == tail.y));
    let touching_side = touching_horizontal || touching_vertical;

    #[allow(clippy::nonminimal_bool)] // Fucke Off
    let touching_diagonal = (head.x == tail.x + 1 && head.y == tail.y + 1)
        || (head.x == tail.x - 1 && head.y == tail.y - 1)
        || (head.x == tail.x + 1 && head.y == tail.y - 1)
        || (head.x == tail.x - 1 && head.y == tail.y + 1);

    // if we're touching, no need to move
    if touching_side || touching_diagonal {
        //println!("touching {touching_side} {touching_diagonal}");
        return;
    }

    // if we only need to move straight
    if head.x == tail.x || head.y == tail.y {
        //println!("moving straight");
        if head.x > tail.x {
            tail.x += 1;
        } else if head.x < tail.x {
            tail.x -= 1;
        } else if head.y > tail.y {
            tail.y += 1;
        } else if head.y < tail.y {
            tail.y -= 1;
        }

        return;
    }

    // otherwise, we're moving diagonally
    // we need to find which corner to move diagonally 1 step in
    let top_left_quadrant = head.x < tail.x && head.y > tail.y;
    let top_right_quadrant = head.x > tail.x && head.y > tail.y;
    let bottom_left_quadrant = head.x < tail.x && head.y < tail.y;
    let bottom_right_quadrant = head.x > tail.x && head.y < tail.y;

    if top_left_quadrant {
        //println!("top left");
        tail.x -= 1;
        tail.y += 1;
    } else if top_right_quadrant {
        //println!("top right");
        tail.x += 1;
        tail.y += 1;
    } else if bottom_left_quadrant {
        //println!("bottom left");
        tail.x -= 1;
        tail.y -= 1;
    } else if bottom_right_quadrant {
        //println!("bottom right");
        tail.x += 1;
        tail.y -= 1;
    }
}

impl Puzzle for DayNine {
    fn one(&self, input: String) -> String {
        let instructions = input.lines();

        let mut head_pos = Pos { x: 0, y: 0 };
        let mut tail_pos = Pos { x: 0, y: 0 };

        let mut unique_pos = std::collections::HashSet::new();

        for instruction in instructions {
            let mut split = instruction.split(' ');
            let dir = split.next().unwrap();
            let num = split.next().unwrap().parse::<u8>().unwrap();

            let dir = match dir {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => unreachable!(),
            };

            for _ in 0..num {
                move_head(&mut head_pos, dir);
                move_tail(head_pos, &mut tail_pos);

                unique_pos.insert((tail_pos.x, tail_pos.y));
                //println!("head: {:?}, tail: {:?}", head_pos, tail_pos);
            }
        }

        unique_pos.len().to_string()
    }

    fn two(&self, input: String) -> String {
        let instructions = input.lines();

        let mut head_pos = Pos { x: 0, y: 0 };
        let mut tail_positions = Vec::new();

        let mut unique_pos = std::collections::HashSet::new();

        for _ in 0..9 {
            tail_positions.push(Pos { x: 0, y: 0 });
        }

        for instruction in instructions {
            let mut split = instruction.split(' ');
            let dir = split.next().unwrap();
            let num = split.next().unwrap().parse::<u8>().unwrap();

            let dir = match dir {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => unreachable!(),
            };

            for _ in 0..num {
                move_head(&mut head_pos, dir);
                let mut in_front = head_pos;

                for tail in tail_positions.iter_mut() {
                    move_tail(in_front, tail);
                    in_front = *tail;
                }

                unique_pos.insert((in_front.x, in_front.y));
                //println!("head: {:?}, tail: {:?}", head_pos, tail_pos);
            }
        }

        unique_pos.len().to_string()
    }
}
