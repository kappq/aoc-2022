use itertools::Itertools;

fn crate_mover_9000(mut stacks: [Vec<char>; 9], procedure: &str) {
    for step in procedure.lines() {
        let (_, n_crates, _, from_stack, _, to_stack) = step
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap_or(0))
            .collect_tuple()
            .unwrap();

        for _ in 0..n_crates {
            let current_crate = stacks[from_stack - 1].pop().unwrap();
            stacks[to_stack - 1].push(current_crate);
        }
    }

    let top_crates = stacks.iter().map(|stack| stack.last().unwrap()).join("");
    println!("first star: {}", top_crates);
}

fn crate_mover_9001(mut stacks: [Vec<char>; 9], procedure: &str) {
    for step in procedure.lines() {
        let (_, n_crates, _, from_stack, _, to_stack) = step
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap_or(0))
            .collect_tuple()
            .unwrap();

        let mut current_crates: Vec<char> = Vec::new();
        for _ in 0..n_crates {
            current_crates.insert(0, stacks[from_stack - 1].pop().unwrap());
        }
        stacks[to_stack - 1].append(&mut current_crates);
    }

    let top_crates = stacks.iter().map(|stack| stack.last().unwrap()).join("");
    println!("second star: {}", top_crates);
}

fn main() {
    let contents = include_str!("input.txt");

    let (starting, procedure) = contents.split_once("\n\n").unwrap();

    let mut stacks: [Vec<char>; 9] = Default::default();
    for line in starting.lines().take(starting.len() - 1) {
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                stacks[i / 4].insert(0, c);
            }
        }
    }

    crate_mover_9000(stacks.clone(), procedure);
    crate_mover_9001(stacks.clone(), procedure);
}
