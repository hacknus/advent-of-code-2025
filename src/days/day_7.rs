use crate::io::read_file_lines;
use crate::problem::Problem;
use itertools::Itertools;

pub struct DaySeven {}

struct Node {
    position: (usize, usize),
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    is_leaf: bool,
}

fn build_tree(
    current_node: &mut Node,
    beam_splitters: &Vec<(usize, usize)>,
    x_max: usize,
    y_max: usize,
) {
    let (x, y) = current_node.position;
    for y_i in y..y_max {
        // check left
        if x > 0 && beam_splitters.contains(&(x - 1, y_i)) && current_node.left.is_none() {
            let mut left = Node {
                position: (x - 1, y_i),
                left: None,
                right: None,
                is_leaf: false,
            };
            build_tree(&mut left, beam_splitters, x_max, y_max);
            current_node.left = Some(Box::from(left));
        }
        // check right
        if x < x_max - 1 && beam_splitters.contains(&(x + 1, y_i)) && current_node.right.is_none() {
            let mut right = Node {
                position: (x + 1, y_i),
                left: None,
                right: None,
                is_leaf: false,
            };
            build_tree(&mut right, beam_splitters, x_max, y_max);
            current_node.right = Some(Box::from(right));
        }
        
        // check leaves
        if y_i == y_max - 1 && current_node.right.is_none() {
            let right = Node {
                position: (x + 1, y_i),
                left: None,
                right: None,
                is_leaf: true,
            };
            current_node.right = Some(Box::from(right));
        }
        if y_i == y_max - 1 && current_node.left.is_none() {
            let left = Node {
                position: (x - 1, y_i),
                left: None,
                right: None,
                is_leaf: true,
            };
            current_node.left = Some(Box::from(left));
        }
    }
}

fn count_leaves(current_node: &Node) -> u32 {
    if current_node.is_leaf {
        return 1;
    }
    let mut counter = 0;
    if let Some(left) = &current_node.left {
        counter += count_leaves(left);
    }
    if let Some(right) = &current_node.right {
        counter += count_leaves(right);
    }
    counter 
}

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

        let mut root = Node {
            position: start,
            left: None,
            right: None,
            is_leaf: false,
        };

        build_tree(
            &mut root,
            &beam_splitters,
            contents[0].len(),
            contents.len(),
        );
        
        println!("built tree!");
        
        let counter = count_leaves(&root);

        format!("{}", counter)
    }
}

#[cfg(test)]
mod tests {}
