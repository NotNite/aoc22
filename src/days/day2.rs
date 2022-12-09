use crate::puzzle::Puzzle;

pub struct DayTwo;

#[derive(Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Lose,
    Draw,
    Win,
}

fn calc_input(input: String) -> Vec<(Move, Move)> {
    input
        .trim()
        .split('\n')
        .map(|x| {
            let mut split = x.split(' ');

            let opponent = match split.next().unwrap() {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => unreachable!(),
            };

            let player = match split.next().unwrap() {
                "X" => Move::Rock,
                "Y" => Move::Paper,
                "Z" => Move::Scissors,
                _ => unreachable!(),
            };

            (opponent, player)
        })
        .collect()
}

fn convert(input: Vec<(Move, Move)>) -> Vec<(Move, GameResult)> {
    input
        .into_iter()
        .map(|(opponent, fucked)| {
            let result = match fucked {
                Move::Rock => GameResult::Lose,
                Move::Paper => GameResult::Draw,
                Move::Scissors => GameResult::Win,
            };

            (opponent, result)
        })
        .collect()
}

fn calc_move(_move: Move) -> usize {
    match _move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn calc_outcome(opponent: Move, player: Move) -> usize {
    match (opponent, player) {
        (Move::Rock, Move::Rock) => 3,
        (Move::Rock, Move::Paper) => 6,
        (Move::Rock, Move::Scissors) => 0,
        (Move::Paper, Move::Rock) => 0,
        (Move::Paper, Move::Paper) => 3,
        (Move::Paper, Move::Scissors) => 6,
        (Move::Scissors, Move::Rock) => 6,
        (Move::Scissors, Move::Paper) => 0,
        (Move::Scissors, Move::Scissors) => 3,
    }
}

fn calc_move_for_outcome(opponent: Move, wanted_state: GameResult) -> Move {
    match (opponent, wanted_state) {
        (Move::Rock, GameResult::Lose) => Move::Scissors,
        (Move::Rock, GameResult::Draw) => Move::Rock,
        (Move::Rock, GameResult::Win) => Move::Paper,
        (Move::Paper, GameResult::Lose) => Move::Rock,
        (Move::Paper, GameResult::Draw) => Move::Paper,
        (Move::Paper, GameResult::Win) => Move::Scissors,
        (Move::Scissors, GameResult::Lose) => Move::Paper,
        (Move::Scissors, GameResult::Draw) => Move::Scissors,
        (Move::Scissors, GameResult::Win) => Move::Rock,
    }
}

impl Puzzle for DayTwo {
    fn test(&self) -> (String, String) {
        ("15".to_string(), "12".to_string())
    }

    fn one(&self, input: String) -> String {
        let input = calc_input(input);

        let mut sum = 0;

        for moves in input {
            let (opponent, player) = moves;

            let move_worth = calc_move(player);
            let outcome = calc_outcome(opponent, player);

            sum += move_worth + outcome;
        }

        sum.to_string()
    }

    fn two(&self, input: String) -> String {
        let input = calc_input(input);
        let input = convert(input);

        let mut sum = 0;

        for moves in input {
            let (opponent, wanted_state) = moves;
            let move_for_outcome = calc_move_for_outcome(opponent, wanted_state);

            let move_worth = calc_move(move_for_outcome);
            let outcome = calc_outcome(opponent, move_for_outcome);

            sum += move_worth + outcome;
        }

        sum.to_string()
    }
}
