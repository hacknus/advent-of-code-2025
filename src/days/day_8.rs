use crate::io::read_file_lines;
use crate::problem::Problem;
use num::integer::Roots;
use num::Integer;
use std::collections::HashSet;
use std::fmt::Display;
use std::ops::Index;

pub struct DayEight {}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Ord, PartialOrd)]
struct Box {
    x: i64,
    y: i64,
    z: i64,
}

impl Display for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Box({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Box {
    fn distance(&self, other: &Box) -> i64 {
        ((self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z))
            .sqrt()
    }
}

impl Problem for DayEight {
    fn part_one(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let n_to_connect = 1000;

        let mut boxes = vec![];

        for line in contents {
            let pos: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
            let b = Box {
                x: pos[0],
                y: pos[1],
                z: pos[2],
            };
            boxes.push(b);
        }

        let mut distances = vec![];

        for i in 0..boxes.len() {
            for j in (i + 1)..boxes.len() {
                let dist = boxes[i].distance(&boxes[j]);
                distances.push((dist, (i, j)));
            }
        }

        distances.sort_by_key(|a| a.0);

        let mut circuits: Vec<HashSet<usize>> = vec![];

        for k in 0..n_to_connect {
            let (d, (i, j)) = distances[k];

            let mut solo = true;
            // check if i is in a circuit
            for circuit in circuits.iter_mut() {
                if circuit.contains(&i) {
                    circuit.insert(j);
                    solo = false;
                }
            }
            // check if j is in a circuit
            for circuit in circuits.iter_mut() {
                if circuit.contains(&j) {
                    circuit.insert(i);
                    solo = false;
                }
            }

            if solo {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(i);
                new_circuit.insert(j);
                circuits.push(new_circuit);
            }

            // merge circuits if both i and j are in different circuits
            let mut to_merge = vec![];
            for (index, circuit) in circuits.iter().enumerate() {
                if circuit.contains(&i) || circuit.contains(&j) {
                    to_merge.push(index);
                }
            }
            if to_merge.len() > 1 {
                let mut new_circuit = HashSet::new();
                for &index in to_merge.iter().rev() {
                    let circuit = circuits.remove(index);
                    for &box_index in circuit.iter() {
                        new_circuit.insert(box_index);
                    }
                }
                circuits.push(new_circuit);
            }
        }

        circuits.sort_by_key(|d| d.len());
        circuits.reverse();

        let mut prod = 1;
        for i in 0..3 {
            prod *= circuits[i].len();
        }

        format!("{}", prod)
    }

    fn part_two(&self, input: &str) -> String {
        let contents = read_file_lines(input);

        let mut boxes = vec![];

        for line in contents {
            let pos: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
            let b = Box {
                x: pos[0],
                y: pos[1],
                z: pos[2],
            };
            boxes.push(b.clone());
        }

        let mut distances = vec![];

        for i in 0..boxes.len() {
            for j in (i + 1)..boxes.len() {
                let dist = boxes[i].distance(&boxes[j]);
                distances.push((dist, (i, j)));
            }
        }

        // println!("distances:");
        // for d in distances.iter() {
        //     println!("{}, {:?}, {:?}", d.0, boxes[d.1.0], boxes[d.1.1]);
        // }

        distances.sort_by_key(|a| a.0);

        let mut circuits: Vec<HashSet<usize>> = boxes
            .iter()
            .map(|b| {
                let mut hs = HashSet::new();
                hs.insert(boxes.iter().position(|x| x == b).unwrap());
                hs
            })
            .collect();

        let mut x_prod = 0;
        let mut k = 0;

        while circuits.len() != 1 {
            let (d, (i, j)) = distances[k];

            let mut solo = true;
            // check if i is in a circuit
            for circuit in circuits.iter_mut() {
                if circuit.contains(&i) {
                    circuit.insert(j);
                    solo = false;
                }
            }
            // check if j is in a circuit
            for circuit in circuits.iter_mut() {
                if circuit.contains(&j) {
                    circuit.insert(i);
                    solo = false;
                }
            }

            if solo {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(i);
                new_circuit.insert(j);
                circuits.push(new_circuit);
            }

            // merge circuits if both i and j are in different circuits
            let mut to_merge = vec![];
            for (index, circuit) in circuits.iter().enumerate() {
                if circuit.contains(&i) || circuit.contains(&j) {
                    to_merge.push(index);
                }
            }
            if to_merge.len() > 1 {
                let mut new_circuit = HashSet::new();
                for &index in to_merge.iter().rev() {
                    let circuit = circuits.remove(index);
                    for &box_index in circuit.iter() {
                        new_circuit.insert(box_index);
                    }
                }
                circuits.push(new_circuit);
            }

            x_prod = boxes[i].x * boxes[j].x;

            k += 1;
        }

        format!("{}", x_prod)
    }
}

#[cfg(test)]
mod tests {}
