use crate::io::read_file_lines;
use crate::problem::Problem;
use itertools::Itertools;

pub struct DaySeven {}

fn scan_up(
    x: usize,
    y: usize,
    beam_splitters: &Vec<(usize, usize)>,
    x_max: usize,
    start: (usize, usize),
) -> bool {
    for y_i in (0..y).rev() {
        if beam_splitters.contains(&(x, y_i)) {
            return false;
        } else if x > 0 && beam_splitters.contains(&(x - 1, y_i)) {
            return true;
        } else if x + 1 < x_max && beam_splitters.contains(&(x + 1, y_i)) {
            return true;
        } else if (x, y_i) == start {
            return true;
        }
    }
    false
}

impl Problem for DaySeven {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut beam_splitters = vec![];
        let mut start = (0, 0);
        for (y, line) in contents.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '^' {
                    beam_splitters.push((x, y));
                }
                if char == 'S' {
                    start = (x, y);
                }
            }
        }

        let mut counter = 0;

        for beam_splitter in beam_splitters.iter() {
            let (x, y) = *beam_splitter;

            if scan_up(x, y, &beam_splitters, contents[0].len(), start) {
                counter += 1;
            }
        }

        format!("{}", counter)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut beam_splitters = vec![];
        let mut start = (0, 0);
        for (y, line) in contents.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '^' {
                    beam_splitters.push((x, y));
                }
                if char == 'S' {
                    start = (x, y);
                }
            }
        }

        let mut counter = 0;
        
        

        format!("{}", counter)
    }
}

#[cfg(test)]
mod tests {}
