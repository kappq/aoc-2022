use std::collections::HashMap;

const DISK_SPACE: u32 = 70000000;
const UNUSED_SPACE: u32 = 30000000;

fn process_command(command: &str, stack: &mut Vec<String>) {
    if command.starts_with("ls") {
        return;
    }

    let (_, dir) = command.split_once(' ').unwrap();
    match dir {
        "/" => {
            stack.push("".to_string());
        }
        ".." => {
            stack.pop();
        }
        _ => {
            stack.push(dir.to_string());
        }
    }
}

fn process_output(line: &str, stack: &mut Vec<String>, directories: &mut HashMap<String, u32>) {
    if line.starts_with("dir") {
        return;
    }

    let filesize = line
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let cwd = stack.join("/");
    directories
        .entry(cwd)
        .and_modify(|dirsize| *dirsize += filesize)
        .or_insert(filesize);

    for i in 0..(stack.len() - 1) {
        let cwd = stack
            .clone()
            .into_iter()
            .take(i + 1)
            .collect::<Vec<String>>()
            .join("/");
        directories
            .entry(cwd)
            .and_modify(|dirsize| *dirsize += filesize)
            .or_insert(filesize);
    }
}

fn main() {
    let contents = include_str!("input.txt");

    let mut directories: HashMap<String, u32> = HashMap::new();
    let mut stack: Vec<String> = Vec::new();

    for line in contents.lines() {
        if line.starts_with('$') {
            process_command(&line[2..], &mut stack);
            continue;
        }

        process_output(line, &mut stack, &mut directories);
    }

    let first_star = directories
        .values()
        .filter(|size| **size <= 100000)
        .sum::<u32>();

    let needed_space = UNUSED_SPACE - (DISK_SPACE - directories.get("").unwrap());
    let second_star = directories
        .values()
        .filter(|size| **size >= needed_space)
        .min()
        .unwrap();

    println!("first star: {}", first_star);
    println!("second star: {}", second_star);
}
