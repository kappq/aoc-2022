use core::panic;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Node {
    height: char,
    position: (usize, usize),
}

impl Node {
    fn get_neighbours(&self, grid: &Vec<Vec<Node>>) -> Vec<Node> {
        let (x, y) = self.position;
        let mut neighbours: Vec<Node> = Vec::new();

        if let Some(neighbour) = grid[y].get(x + 1) {
            neighbours.push(*neighbour);
        }
        if x > 0 {
            if let Some(neighbour) = grid[y].get(x - 1) {
                neighbours.push(*neighbour);
            }
        }
        if let Some(neighbouring_row) = grid.get(y + 1) {
            neighbours.push(neighbouring_row[x]);
        }
        if y > 0 {
            if let Some(neighbouring_row) = grid.get(y - 1) {
                neighbours.push(neighbouring_row[x]);
            }
        }

        neighbours
            .into_iter()
            .filter(|neighbour| {
                if neighbour.height == 'E' && !(self.height == 'z') {
                    return false;
                }
                self.height == 'S' || neighbour.height as u32 <= self.height as u32 + 1
            })
            .collect()
    }
}

fn main() {
    let input = include_str!("input.txt");
    let grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, height)| Node {
                    height,
                    position: (j, i),
                })
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();
    println!("part 1: {}", dijkstra(&grid, vec!['S']));
    println!("part 2: {}", dijkstra(&grid, vec!['S', 'a']));
}

fn dijkstra(grid: &Vec<Vec<Node>>, starting_nodes: Vec<char>) -> usize {
    let mut dist: HashMap<Node, u32> = HashMap::new();
    let mut path: HashMap<Node, Node> = HashMap::new();
    let mut queue: HashSet<Node> = HashSet::new();

    for row in grid {
        for node in row {
            if starting_nodes.contains(&node.height) {
                dist.insert(*node, 0);
                queue.insert(*node);
                continue;
            }
            dist.insert(*node, u32::MAX);
            queue.insert(*node);
        }
    }

    while !queue.is_empty() {
        let current_node = queue
            .iter()
            .min_by_key(|node| dist.get(node).unwrap())
            .unwrap()
            .clone();
        if current_node.height == 'E' {
            let mut u = path.get(&current_node).unwrap();
            let mut counter = 0;
            while let Some(previous) = path.get(&u) {
                u = previous;
                counter += 1;
                if previous.height == 'S' {
                    break;
                }
            }
            return counter + 1; // Add the starting square
        }
        queue.remove(&current_node);

        for neighbour in current_node.get_neighbours(&grid) {
            if !queue.contains(&neighbour) {
                // We have already visited the node
                continue;
            }
            let current_dist = dist[&current_node] + 1;
            if current_dist < dist[&neighbour] {
                *dist.get_mut(&neighbour).unwrap() = current_dist;
                path.insert(neighbour, current_node);
            }
        }
    }

    panic!("path not found");
}
