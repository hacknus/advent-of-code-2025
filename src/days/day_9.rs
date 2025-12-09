use crate::io::read_file_lines;
use crate::problem::Problem;
use std::collections::HashMap;

pub struct DayNine {}

fn area_is_green(
    a: (i64, (i64, i64), (i64, i64)),
    edges: &Vec<((i64, i64), (i64, i64))>,
    cache: &mut HashMap<(i64, i64), bool>,
) -> bool {
    let x_min = a.1.0.min(a.2.0);
    let x_max = a.1.0.max(a.2.0);
    let y_min = a.1.1.min(a.2.1);
    let y_max = a.1.1.max(a.2.1);

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            if let Some(b) = cache.get(&(x, y)) {
                if !*b {
                    return false;
                } else {
                    continue;
                }
            }

            // lets do ray-cast! if any point crosses two edges, we are in a non-green area
            let mut h_crosses = 0;
            let mut v_crosses = 0;
            for edge in edges.iter() {
                let (p1, p2) = edge;
                // check if the point will cross the edge in the horizontal direction
                if p1.0 == p2.0 {
                    if (p1.0 >= x) && (y >= p1.1.min(p2.1) && y <= p2.1.max(p1.1)) {
                        h_crosses += 1;
                    }
                }

                // if p1.1 == p2.1 {
                //     if p1.1 == y && p1.0 >= x && p2.0 >= x {
                //         h_crosses -= 1;
                //     }
                // }

                // check if the point will cross the edge in the vertical direction
                // if p1.1 == p2.1 {
                //     if (p1.1 >= y) && (x >= p1.0.min(p2.0) && x <= p2.0.max(p1.0)) {
                //         v_crosses += 1;
                //     }
                // }
                // 
                // if p1.0 == p2.0 {
                //     if p1.0 == x && p1.1 >= y && p2.1 >= y {
                //         v_crosses -= 1;
                //     }
                // }
            }
            if h_crosses % 2 == 0 {
                cache.insert((x, y), false);
                return false;
            } else {
                cache.insert((x, y), true);
            }
        }
    }

    true
}

impl Problem for DayNine {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut points = vec![];

        for line in contents {
            let split = line.split(',').collect::<Vec<&str>>();
            let x = split[0].parse::<i64>().unwrap();
            let y = split[1].parse::<i64>().unwrap();
            points.push((x, y));
        }

        let mut areas = vec![];
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let dx = points[i].0 - points[j].0 + 1;
                let dy = points[i].1 - points[j].1 + 1;
                areas.push(dx.abs() * dy.abs());
            }
        }
        areas.sort();

        format!("{}", areas.last().unwrap())
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut points = vec![];

        for line in contents {
            let split = line.split(',').collect::<Vec<&str>>();
            let x = split[0].parse::<i64>().unwrap();
            let y = split[1].parse::<i64>().unwrap();
            points.push((x, y));
        }

        let mut areas = vec![];
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let dx = points[i].0 - points[j].0 + 1;
                let dy = points[i].1 - points[j].1 + 1;
                areas.push((dx.abs() * dy.abs(), points[i], points[j]));
            }
        }
        areas.sort_by_key(|a| a.0);
        areas.reverse();

        let mut edges = vec![];

        let mut cache: HashMap<(i64, i64), bool> = HashMap::new();

        for i in 0..points.len() {
            let a = points[i];
            let b = points[(i + 1) % points.len()];
            edges.push((a, b));
            for x in a.0.min(b.0)..=b.0.max(a.0) {
                for y in a.1.min(b.1)..=b.1.max(a.1) {
                    cache.insert((x, y), true);
                }
            }
        }

        let mut area = 0;

        for (i, a) in areas.iter().enumerate() {
            if i % 1000 == 0 {
                println!("Checking area: {}/{} with size: {}", i, areas.len(), a.0);
            }

            if !area_is_green(*a, &edges, &mut cache) {
                continue;
            }

            area = a.0;
            break;
        }

        format!("{}", area)
    }

    // not 909602, too low
    // not 973336, too low
    // not 466922, too low
    // not 820536, too low
    // not 91189930
    // not 40244622
}

#[cfg(test)]
mod tests {}
