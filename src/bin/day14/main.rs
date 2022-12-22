use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let mut cave: HashSet<(usize, usize)> = HashSet::new();
    input.lines().for_each(|line| {
        line.split(" -> ")
            .map(|pos| {
                let (x, y) = pos.split_once(',').unwrap();
                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
            .tuple_windows()
            .for_each(|((x1, y1), (x2, y2))| {
                if x1 == x2 {
                    let range = if y1 < y2 { y1..=y2 } else { y2..=y1 };
                    for y in range {
                        cave.insert((x1, y));
                    }
                }
                if y1 == y2 {
                    let range = if x1 < x2 { x1..=x2 } else { x2..=x1 };
                    for x in range {
                        cave.insert((x, y1));
                    }
                }
            });
    });
    println!("part 1: {}", part1(cave.clone()));
    println!("part 2: {}", part2(cave.clone()));
}

fn part1(mut cave: HashSet<(usize, usize)>) -> u32 {
    let mut counter = 0;
    'falling: loop {
        let (mut x, mut y) = (500, 0);
        loop {
            if cave.iter().find(|pos| x == pos.0).is_none() {
                break 'falling;
            }
            if !cave.contains(&(x, y + 1)) {
                y += 1;
                continue;
            }
            if !cave.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
                continue;
            }
            if !cave.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
                continue;
            }
            cave.insert((x, y));
            break;
        }
        counter += 1;
    }
    counter
}

fn part2(mut cave: HashSet<(usize, usize)>) -> u32 {
    let floor_y = cave.iter().max_by_key(|(_, y)| y).unwrap().1 + 2;
    let mut counter = 0;
    loop {
        if cave.contains(&(500, 0)) {
            break;
        }
        let (mut x, mut y) = (500, 0);
        loop {
            if y + 1 == floor_y {
                cave.insert((x, y));
                break;
            }
            if !cave.contains(&(x, y + 1)) {
                y += 1;
                continue;
            }
            if !cave.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
                continue;
            }
            if !cave.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
                continue;
            }
            cave.insert((x, y));
            break;
        }
        counter += 1;
    }
    counter
}
