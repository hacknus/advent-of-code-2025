use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DayOne {}

impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut position = 50;

        let mut counter = 0;

        for line in contents {
            let chars: Vec<char> = line.chars().collect();
            let direction = chars[0].to_string();
            let distance: i32 = chars[1..].iter().collect::<String>().parse().unwrap();
            match direction.as_str() {
                "L" => {
                    position -= distance;
                }
                "R" => {
                    position += distance;
                }
                _ => {}
            }

            position = position % 100;
            while position < 0 {
                position += 100;
            }

            if position == 0 {
                counter += 1;
            }
        }
        format!("{}", counter)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut position = 50;

        let mut counter = 0;

        for line in contents {
            let chars: Vec<char> = line.chars().collect();
            let direction = chars[0].to_string();
            let distance: i32 = chars[1..].iter().collect::<String>().parse().unwrap();
            match direction.as_str() {
                "L" => {
                    for _ in 0..distance {
                        position -= 1;
                        if position < 0 {
                            position += 100;
                        }
                        if position == 0 {
                            counter += 1;
                        }
                    }
                }
                "R" => {
                    for _ in 0..distance {
                        position += 1;
                        if position > 99 {
                            position -= 100;
                        }
                        if position == 0 {
                            counter += 1;
                        }
                    }
                }
                _ => {}
            }
        }

        format!("{}", counter)
    }
}

#[cfg(test)]
mod tests {}
