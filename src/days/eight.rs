use crate::puzzle::Puzzle;

pub struct DayEight;

impl Puzzle for DayEight {
    // I misunderstood pt1 so it's a bit verbose
    // whoops
    fn one(&self, input: String) -> String {
        let mut rows = Vec::new();
        let mut columns = Vec::new();

        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            rows.push(row);
        }

        for i in 0..rows[0].len() {
            let mut column = Vec::new();
            for row in &rows {
                column.push(row[i]);
            }
            columns.push(column);
        }

        let mut visible_count = 0;

        for (x, row) in rows.iter().enumerate().take(rows.len() - 1).skip(1) {
            for (y, us) in row.iter().enumerate().take(columns.len() - 1).skip(1) {
                let us = us.to_digit(10).unwrap();

                let left = columns[0..y]
                    .iter()
                    .map(|z| z[x].to_digit(10).unwrap())
                    .collect::<Vec<u32>>();

                let right = columns[y + 1..]
                    .iter()
                    .map(|z| z[x].to_digit(10).unwrap())
                    .collect::<Vec<u32>>();

                let top = rows[0..x]
                    .iter()
                    .map(|z| z[y].to_digit(10).unwrap())
                    .collect::<Vec<u32>>();

                let bottom = rows[x + 1..]
                    .iter()
                    .map(|z| z[y].to_digit(10).unwrap())
                    .collect::<Vec<u32>>();

                let mut passes = false;
                for vec in ([left, right, top, bottom]).iter() {
                    let mut local_passes = true;
                    for num in vec {
                        if num >= &us {
                            local_passes = false;
                        }
                    }

                    if local_passes {
                        passes = true;
                    }
                }

                if passes {
                    visible_count += 1;
                }
            }
        }

        let edge_tree_count = (rows.len() - 2) * 2 + (columns.len() - 2) * 2 + 4;

        (visible_count + edge_tree_count).to_string()
    }

    fn two(&self, input: String) -> String {
        let mut rows = Vec::new();
        let mut columns = Vec::new();

        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            rows.push(row);
        }

        for i in 0..rows[0].len() {
            let mut column = Vec::new();
            for row in &rows {
                column.push(row[i]);
            }
            columns.push(column);
        }

        let mut max_visible = 0;

        for (x, row) in rows.iter().enumerate().take(rows.len() - 1).skip(1) {
            for (y, us) in row.iter().enumerate().take(columns.len() - 1).skip(1) {
                let us = us.to_digit(10).unwrap();

                let mut left = columns[0..y]
                    .iter()
                    .map(|z| z[x].to_digit(10).unwrap())
                    .collect::<Vec<u32>>();

                let right = columns[y + 1..]
                    .iter()
                    .map(|z| z[x].to_digit(10).unwrap())
                    .collect::<Vec<u32>>();

                let mut top = rows[0..x]
                    .iter()
                    .map(|z| z[y].to_digit(10).unwrap())
                    .collect::<Vec<u32>>();

                let bottom = rows[x + 1..]
                    .iter()
                    .map(|z| z[y].to_digit(10).unwrap())
                    .collect::<Vec<u32>>();

                left.reverse();
                top.reverse();

                let mut view_distance = None;
                for vec in ([left, right, top, bottom]).iter() {
                    let mut local_view_distance = 0;

                    for num in vec {
                        local_view_distance += 1;
                        if num >= &us {
                            break;
                        }
                    }

                    if view_distance.is_none() {
                        view_distance = Some(local_view_distance);
                    } else {
                        view_distance = Some(view_distance.unwrap() * local_view_distance);
                    }
                }

                if view_distance.unwrap() > max_visible {
                    max_visible = view_distance.unwrap();
                }
            }
        }

        max_visible.to_string()
    }
}
