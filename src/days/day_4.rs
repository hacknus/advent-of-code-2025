use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayFour {}

fn look_around(map: &Vec<Vec<i32>>, pos: (usize, usize)) -> bool {
    let directions = vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut counter = 0;
    for dir in directions {
        let new_x = pos.0 as isize + dir.0;
        let new_y = pos.1 as isize + dir.1;
        if new_x >= 0 && new_x < map[0].len() as isize && new_y >= 0 && new_y < map.len() as isize {
            if map[new_y as usize][new_x as usize] == 1 {
                counter += 1;
            }
        }
    }

    counter < 4
}

impl Problem for DayFour {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = vec![vec![0; contents.first().unwrap().len()]; contents.len()];
        for (y, row) in contents.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '@' {
                    map[y][x] = 1;
                } else {
                    map[y][x] = 0;
                }
            }
        }

        let mut count = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == 1 {
                    if look_around(&map, (x, y)) {
                        count += 1;
                    }
                }
            }
        }

        format!("{}", count)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut map = vec![vec![0; contents.first().unwrap().len()]; contents.len()];
        for (y, row) in contents.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '@' {
                    map[y][x] = 1;
                } else {
                    map[y][x] = 0;
                }
            }
        }

        let mut count = 0;
        let mut last_count = -1;
        loop {
            let mut new_map = map.clone();
            for y in 0..map.len() {
                for x in 0..map[0].len() {
                    if map[y][x] == 1 {
                        if look_around(&map, (x, y)) {
                            new_map[y][x] = 0;
                            count += 1;
                        }
                    }
                }
            }
            map = new_map;
            if last_count == count {
                break;
            }
            last_count = count;
        }

        format!("{}", count)
    }
}

#[cfg(test)]
mod tests {}
