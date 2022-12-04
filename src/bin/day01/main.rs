use itertools::Itertools;

fn main() {
    let contents = include_str!("input.txt");

    let calories = contents.split("\n\n").map(|inventory| {
        inventory
            .lines()
            .map(|calories| calories.parse::<u32>().unwrap())
            .sum::<u32>()
    });

    let first_star = calories.clone().max().unwrap();
    let second_star = calories.clone().sorted().rev().take(3).sum::<u32>();

    println!("first star: {}", first_star);
    println!("second star: {}", second_star);
}
