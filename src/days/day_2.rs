use crate::io::read_file_lines;
use crate::problem::Problem;
use itertools::Itertools;

pub struct DayTwo {}

fn get_invalid_ids(start: i64, end: i64) -> Vec<i64> {
    let mut ids = vec![];
    for i in start..=end {
        let i_string = i.to_string();
        let i_chars: Vec<char> = i_string.chars().collect();
        for j in 0..i_chars.len() {
            let slice = &i_chars[0..=j].iter().collect::<String>();
            if slice.repeat(2) == i_string {
                ids.push(i);
                break;
            }
        }
    }
    ids
}

fn get_invalid_ids_at_least_twice(start: i64, end: i64) -> Vec<i64> {
    let mut ids = vec![];
    for i in start..=end {
        let i_string = i.to_string();
        let i_chars: Vec<char> = i_string.chars().collect();
        let n = i_string.len();
        for j in 0..i_chars.len() {
            let slice = &i_chars[0..=j].iter().collect::<String>();
            let n_i = slice.len();
            if n / n_i >= 2 && slice.repeat(n / n_i) == i_string {
                ids.push(i);
                break;
            }
        }
    }
    ids
}

impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let line = contents.first().unwrap();
        let ranges = line.split(",").collect::<Vec<&str>>();

        let mut sum = 0;

        for range in ranges {
            let bounds = range.split("-").collect::<Vec<&str>>();
            let start = bounds[0].parse::<i64>().unwrap();
            let end = bounds[1].parse::<i64>().unwrap();
            let ids = get_invalid_ids(start, end);
            sum += ids.iter().sum::<i64>();
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let line = contents.first().unwrap();
        let ranges = line.split(",").collect::<Vec<&str>>();

        let mut sum = 0;

        for range in ranges {
            let bounds = range.split("-").collect::<Vec<&str>>();
            let start = bounds[0].parse::<i64>().unwrap();
            let end = bounds[1].parse::<i64>().unwrap();
            let ids = get_invalid_ids_at_least_twice(start, end);
            sum += ids.iter().sum::<i64>();
        }

        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {}
