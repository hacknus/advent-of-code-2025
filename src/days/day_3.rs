use crate::io::read_file_lines;
use crate::problem::Problem;
use itertools::Itertools;

pub struct DayThree {}

impl Problem for DayThree {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut sum = 0;
        for bank in contents {
            let batteries = bank
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            let batteries_sorted = batteries.iter().sorted().cloned().collect::<Vec<u32>>();
            let mut battery_1 = batteries_sorted[batteries_sorted.len() - 1];

            let mut pos_1 = batteries.iter().position(|&b| b == battery_1).unwrap();
            if pos_1 == batteries.len() - 1 {
                battery_1 = batteries_sorted[batteries_sorted.len() - 2];
                pos_1 = batteries.iter().position(|&b| b == battery_1).unwrap();
            }

            let batteries_slice_sorted = batteries
                .iter()
                .skip(pos_1 + 1)
                .sorted()
                .cloned()
                .collect::<Vec<u32>>();
            let battery_2 = batteries_slice_sorted[batteries_slice_sorted.len() - 1];

            let joltage = battery_2 + 10 * battery_1;

            sum += joltage;
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let num_batteries = 12;

        let mut sum = 0;
        for bank in contents {
            let batteries = bank
                .char_indices()
                .map(|(i, c)| (i, c.to_digit(10).unwrap()))
                .collect::<Vec<(usize, u32)>>();
            let mut batteries_sorted = batteries.clone();
            batteries_sorted.sort_by_key(|k| k.1);

            // take the highest one
            let mut battery_1 = batteries_sorted[batteries_sorted.len() - 1];

            // check that there is enough space left for the other batteries
            let mut i = 1;
            while battery_1.0 > batteries.len() - 1 - num_batteries + 1 {
                battery_1 = batteries_sorted[batteries_sorted.len() - 1 - i];
                i += 1;
            }

            // if there are multiple batteries with the same value, pick the first one
            let mut filtered_batteries = batteries_sorted
                .iter()
                .filter(|b| b.1 == battery_1.1)
                .cloned()
                .collect::<Vec<(usize, u32)>>();
            filtered_batteries.sort_by_key(|k| k.0);
            battery_1 = filtered_batteries[0];

            let mut last_pos = battery_1.0;
            let mut selected_batteries = vec![battery_1];

            for n_i in 1..num_batteries {
                let mut batteries_slice_sorted = batteries
                    .iter()
                    .skip(last_pos + 1)
                    .cloned()
                    .collect::<Vec<(usize, u32)>>();
                batteries_slice_sorted.sort_by_key(|k| k.1);

                // take the highest one
                let mut this_battery = batteries_slice_sorted[batteries_slice_sorted.len() - 1];
                // if there are multiple batteries with the same value, pick the first one
                let mut filtered_batteries = batteries_slice_sorted
                    .iter()
                    .filter(|b| b.1 == this_battery.1)
                    .cloned()
                    .collect::<Vec<(usize, u32)>>();
                filtered_batteries.sort_by_key(|k| k.0);
                this_battery = filtered_batteries[0];

                let mut i = 1;
                while this_battery.0 > batteries.len() - 1 - num_batteries + n_i + 1 {
                    this_battery = batteries_slice_sorted[batteries_slice_sorted.len() - 1 - i];
                    // if there are multiple batteries with the same value, pick the first one
                    let mut filtered_batteries = batteries_slice_sorted
                        .iter()
                        .filter(|b| b.1 == this_battery.1)
                        .cloned()
                        .collect::<Vec<(usize, u32)>>();
                    filtered_batteries.sort_by_key(|k| k.0);
                    this_battery = filtered_batteries[0];
                    i += 1;
                }
                last_pos = this_battery.0;
                selected_batteries.push(this_battery);
            }

            let mut joltage = 0;
            for (i, battery) in selected_batteries.iter().enumerate() {
                joltage += (battery.1 as u64) * 10u64.pow((num_batteries - 1 - i) as u32);
            }
            sum += joltage;
        }

        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {}
