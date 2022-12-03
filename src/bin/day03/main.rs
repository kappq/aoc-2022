use std::fs;

use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("src/bin/day03/input.txt").expect("file not found");

    let first_star = contents
        .lines()
        .map(|rucksack| {
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            let common = left.chars().find(|c| right.contains(*c)).unwrap();
            if char::is_lowercase(common) {
                common as u32 - 96
            } else {
                common as u32 - 38
            }
        })
        .sum::<u32>();

    let second_star = contents
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let (first, second, third) = (
                group.next().unwrap(),
                group.next().unwrap(),
                group.next().unwrap(),
            );
            let common = first
                .chars()
                .find(|c| second.contains(*c) && third.contains(*c))
                .unwrap();
            if char::is_lowercase(common) {
                common as u32 - 96
            } else {
                common as u32 - 38
            }
        })
        .sum::<u32>();

    println!("first star: {}", first_star);
    println!("second star: {}", second_star);
}
