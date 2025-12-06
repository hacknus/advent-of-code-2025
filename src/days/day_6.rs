use crate::io::read_file_lines;
use crate::problem::Problem;

pub struct DaySix {}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn from(element: &str) -> Operator {
        match element {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("Unknown operator"),
        }
    }
}

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut problems = vec![
            vec![];
            contents
                .first()
                .unwrap()
                .split_whitespace()
                .into_iter()
                .collect::<Vec<&str>>()
                .len()
        ];
        let mut operators: Vec<Operator> = vec![];
        for line in &contents {
            let line = line.split_whitespace().into_iter().collect::<Vec<&str>>();
            if let Ok(num) = line.first().unwrap().parse::<u64>() {
                for (i, element) in line.iter().enumerate() {
                    problems[i].push(element.parse::<u64>().unwrap());
                }
            } else {
                for element in line.iter() {
                    operators.push(Operator::from(element))
                }
            }
        }
        let mut counter = 0;
        for (i, problem) in problems.iter().enumerate() {
            let operator = &operators[i];
            let result: u64 = match operator {
                Operator::Add => problem.iter().sum(),
                Operator::Multiply => problem.iter().product(),
            };
            counter += result;
        }

        format!("{}", counter)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut operators = vec![];
        let mut separators = 1;
        for c in contents.last().unwrap().chars().rev() {
            match c {
                '+' => {
                    operators.push((separators, Operator::Add));
                    separators = 0;
                }
                '*' => {
                    operators.push((separators, Operator::Multiply));
                    separators = 0;
                }
                _ => {
                    separators += 1;
                }
            }
        }
        operators.reverse();

        let mut problems = vec![vec![vec![]; contents.len() - 1]; operators.len()];

        for (line_i, line) in contents.iter().take(contents.len() - 1).enumerate() {
            let mut i = 0;
            for (operator_i, operator) in operators.iter().enumerate() {
                problems[operator_i][line_i] = line
                    .chars()
                    .skip(i)
                    .take(operator.0 as usize)
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>();
                i += operator.0 as usize + 1;
            }
        }

        let mut counter = 0;
        for (i, problem) in problems.iter().enumerate() {
            let operator = &operators[i];
            match operator {
                (_, Operator::Add) => {
                    let mut result = 0;
                    let n = problem.first().unwrap().len();
                    for i in 0..n {
                        let mut val = "".to_string();
                        for p in problem.iter() {
                            val += &p[i].to_string();
                        }
                        result += val.trim().parse::<u64>().unwrap();
                    }
                    counter += result;
                }
                (_, Operator::Multiply) => {
                    let mut result = 1;
                    let n = problem.first().unwrap().len();
                    for i in 0..n {
                        let mut val = "".to_string();
                        for p in problem.iter() {
                            val += &p[i].to_string();
                        }
                        result *= val.trim().parse::<u64>().unwrap();
                    }
                    counter += result;
                }
            }
        }

        format!("{}", counter)
    }
}

#[cfg(test)]
mod tests {}
