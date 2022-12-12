use crate::puzzle::Puzzle;
use pathfinding::prelude::{bfs, Matrix};
pub struct DayTwelve;

impl Puzzle for DayTwelve {
    fn test(&self) -> (String, String) {
        ("31".to_string(), "29".to_string())
    }

    fn one(&self, input: String) -> String {
        let mut matrix = Matrix::from_rows(input.lines().map(|x| x.chars())).unwrap();

        let start_pos = matrix.indices().find(|x| matrix[*x] == 'S').unwrap();
        let end_pos = matrix.indices().find(|x| matrix[*x] == 'E').unwrap();

        matrix[start_pos] = 'a';
        matrix[end_pos] = 'z';

        let its_that_simple = bfs(
            &start_pos,
            |&pos| {
                let matrix3 = matrix.clone();
                matrix
                    .neighbours(pos, false)
                    .filter(move |x| matrix3[*x] as u8 <= matrix3[pos] as u8 + 1)
            },
            |&pos| pos == end_pos,
        )
        .unwrap();

        // extra step for some fucking reason???
        (its_that_simple.len() - 1).to_string()
    }

    fn two(&self, input: String) -> String {
        let mut matrix = Matrix::from_rows(input.lines().map(|x| x.chars())).unwrap();

        let start_pos = matrix.indices().find(|x| matrix[*x] == 'S').unwrap();
        let end_pos = matrix.indices().find(|x| matrix[*x] == 'E').unwrap();

        matrix[start_pos] = 'a';
        matrix[end_pos] = 'z';

        let its_that_simple = bfs(
            &end_pos,
            |&pos| {
                let matrix3 = matrix.clone();
                matrix
                    .neighbours(pos, false)
                    .filter(move |x| matrix3[pos] as u8 <= matrix3[*x] as u8 + 1)
            },
            |&pos| matrix[pos] == 'a',
        )
        .unwrap();

        (its_that_simple.len() - 1).to_string()
    }
}
