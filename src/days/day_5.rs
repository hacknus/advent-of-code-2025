use crate::io::read_file_lines;
use crate::problem::Problem;
use std::collections::HashSet;

pub struct DayFive {}

impl Problem for DayFive {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut ranges = vec![];
        let mut ingredients = vec![];
        let mut ranges_collected = false;
        for line in &contents {
            if line == "" {
                ranges_collected = true;
                continue;
            }
            if !ranges_collected {
                let mut split = line.split("-");
                let start = split.next().unwrap();
                let end = split.next().unwrap();
                ranges.push(start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap());
            } else {
                ingredients.push(line.parse::<u64>().unwrap());
            }
        }

        let mut counter = 0;
        for ingredient in &ingredients {
            let mut is_in_range = false;
            for range in &ranges {
                if range.contains(ingredient) {
                    is_in_range = true;
                }
            }
            if is_in_range {
                counter += 1;
            }
        }

        format!("{}", counter)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);
        let mut fresh_sets = vec![];
        for line in &contents {
            if line == "" {
                break;
            }
            let mut split = line.split("-");
            let start = split.next().unwrap().parse::<u64>().unwrap();
            let end = split.next().unwrap().parse::<u64>().unwrap();

            if fresh_sets.len() == 0 {
                fresh_sets.push((start, end));
            } else {
                let mut no_overlaps = true;
                for set in fresh_sets.iter_mut() {
                    if start >= set.0 && start <= set.1 {
                        // start is inside first set
                        // update end of first set
                        set.1 = set.1.max(end);
                        no_overlaps = false;
                    } else if end >= set.0 && end <= set.1 {
                        // end is inside first set
                        // update start of first set
                        set.0 = set.0.min(start);
                        no_overlaps = false;
                    }
                }
                if no_overlaps {
                    fresh_sets.push((start, end));
                }

                // delete overlaps
                let mut to_drop = HashSet::new();
                let other_sets = fresh_sets.clone();
                for other_set in other_sets.iter() {
                    for set in fresh_sets.iter_mut() {
                        if other_set == set || to_drop.contains(set) || to_drop.contains(other_set)
                        {
                            continue;
                        }
                        if other_set.0 >= set.0 && other_set.0 <= set.1 {
                            // start is inside first set
                            // update end of first set
                            set.1 = set.1.max(other_set.1);
                            to_drop.insert(other_set);
                        } else if other_set.1 >= set.0 && other_set.1 <= set.1 {
                            // end is inside first set
                            // update start of first set
                            set.0 = set.0.min(other_set.0);
                            to_drop.insert(other_set);
                        }
                    }
                }

                for drop in to_drop {
                    fresh_sets.retain(|x| x != drop);
                }
            }
        }

        let mut fresh_sets_hashed = HashSet::new();
        for fresh_set in fresh_sets {
            fresh_sets_hashed.insert(fresh_set);
        }

        let mut counter = 0;
        for set in fresh_sets_hashed {
            counter += set.1 - set.0 + 1;
        }

        format!("{}", counter)
    }
}

#[cfg(test)]
mod tests {}
