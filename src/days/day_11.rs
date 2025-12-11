use crate::io::read_file_lines;
use crate::problem::Problem;
use std::collections::HashMap;

pub struct DayEleven {}

fn walk_network(
    network: &HashMap<String, Vec<String>>,
    current: &String,
    states: &mut HashMap<String, usize>,
) -> usize {
    if let Some(c) = states.get(current) {
        return *c;
    }

    if current == "out" {
        return 1;
    }

    let mut counter = 0;
    for neighbor in network.get(current).unwrap() {
        counter += walk_network(network, neighbor, states);
    }
    states.insert(current.clone(), counter);
    counter
}

fn walk_network_2(
    network: &HashMap<String, Vec<String>>,
    current: &String,
    states: &mut HashMap<(String, bool, bool), usize>,
    mut dac: bool,
    mut fft: bool,
) -> usize {
    if let Some(c) = states.get(&(current.to_string(), dac, fft)) {
        return *c;
    }

    if current == "out" {
        return (dac && fft) as usize;
    }
    if current == "dac" {
        dac = true;
    }
    if current == "fft" {
        fft = true;
    }

    let mut counter = 0;
    for neighbor in network.get(current).unwrap() {
        counter += walk_network_2(network, neighbor, states, dac, fft);
    }
    states.insert((current.clone(), dac, fft), counter);
    counter
}

impl Problem for DayEleven {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut network = HashMap::new();

        for line in contents.iter() {
            let splits = line.split(":").collect::<Vec<&str>>();
            let key = splits[0].to_string();
            let mut values = splits[1]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            for value in values.iter_mut() {
                *value = value.trim().to_string();
            }
            network.insert(key.clone(), values.clone());
        }

        let start = "you".to_string();

        let mut states = HashMap::new();
        let counter = walk_network(&network, &start, &mut states);

        format!("{}", counter)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut network = HashMap::new();

        for line in contents.iter() {
            let splits = line.split(":").collect::<Vec<&str>>();
            let key = splits[0].to_string();
            let mut values = splits[1]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            for value in values.iter_mut() {
                *value = value.trim().to_string();
            }
            network.insert(key.clone(), values.clone());
        }

        let start = "svr".to_string();

        let mut states = HashMap::new();
        let counter = walk_network_2(&network, &start, &mut states, false, false);

        format!("{}", counter)
    }
}

#[cfg(test)]
mod tests {}
