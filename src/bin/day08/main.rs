use itertools::Itertools;

fn part1(input: &str) -> i32 {
    let columns = (0..99)
        .map(|x| {
            input
                .lines()
                .map(move |line| line.chars().nth(x).unwrap())
                .join("")
        })
        .collect::<Vec<String>>();

    let mut visible_in_rows = [[false; 99]; 99];
    let mut visible_in_cols = [[false; 99]; 99];

    for (y, row) in input.lines().enumerate() {
        let current_row = visible_in_rows.get_mut(y).unwrap();
        // Check visibility from left to right
        let mut max = '.';
        for (x, tree) in row.chars().enumerate() {
            if tree > max {
                max = tree;
                current_row[x] = true;
                if tree == '9' {
                    break;
                }
            }
        }
        // Check visibility from right to left
        let mut max = '.';
        for (x, tree) in row
            .chars()
            .enumerate()
            .collect::<Vec<(usize, char)>>()
            .into_iter()
            .rev()
        {
            if current_row[x] {
                break;
            }
            if tree > max {
                max = tree;
                current_row[x] = true;
                if tree == '9' {
                    break;
                }
            }
        }
    }

    for (x, col) in columns.iter().enumerate() {
        let current_col = visible_in_cols.get_mut(x).unwrap();
        // Check visibility from top to bottom
        let mut max = '.';
        for (y, tree) in col.chars().enumerate() {
            if tree > max {
                max = tree;
                current_col[y] = true;
                if tree == '9' {
                    break;
                }
            }
        }
        // Check visibility from bottom to top
        let mut max = '.';
        for (y, tree) in col
            .chars()
            .enumerate()
            .collect::<Vec<(usize, char)>>()
            .into_iter()
            .rev()
        {
            if current_col[y] {
                break;
            }
            if tree > max {
                max = tree;
                current_col[y] = true;
                if tree == '9' {
                    break;
                }
            }
        }
    }

    let mut visible = 0;
    for row in 0..99 {
        for col in 0..99 {
            if visible_in_rows[row][col] || visible_in_cols[col][row] {
                visible += 1;
            }
        }
    }

    visible
}

fn part2(input: &str) -> i32 {
    let mut grid: [[char; 99]; 99] = [['0'; 99]; 99];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c;
        }
    }

    let mut max_score = 0;
    for y in 0..99 {
        for x in 0..99 {
            let tree = grid[y][x];

            let mut up_score = 0;
            {
                for y in (0..y).rev() {
                    up_score += 1;
                    if grid[y][x] >= tree {
                        break;
                    }
                }
            }

            let mut down_score = 0;
            {
                for y in (y + 1)..99 {
                    down_score += 1;
                    if grid[y][x] >= tree {
                        break;
                    }
                }
            }

            let mut left_score = 0;
            {
                for x in (0..x).rev() {
                    left_score += 1;
                    if grid[y][x] >= tree {
                        break;
                    }
                }
            }

            let mut right_score = 0;
            {
                for x in (x + 1)..99 {
                    right_score += 1;
                    if grid[y][x] >= tree {
                        break;
                    }
                }
            }

            let score = up_score * down_score * left_score * right_score;
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn main() {
    let contents = include_str!("input.txt");

    println!("first star: {}", part1(contents));
    println!("second star: {}", part2(contents));
}
