use std::{collections::HashSet, str::FromStr};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn touch(&mut self, other: Position) {
        let distance_x = self.x - other.x;
        let distance_y = self.y - other.y;

        if distance_x.abs() > 1 || distance_y.abs() > 1 {
            self.x -= if self.x == other.x {
                0
            } else {
                distance_x / distance_x.abs()
            };
            self.y -= if self.y == other.y {
                0
            } else {
                distance_y / distance_y.abs()
            };
        }
    }
}

fn main() {
    let input = include_str!("input.txt")
        //    let input = "R 5
        //U 8
        //L 8
        //D 3
        //R 17
        //D 10
        //L 25
        //U 20"
        .lines()
        .flat_map(|line| {
            let (direction, times) = line.split_once(' ').unwrap();
            let direction = Direction::from_str(direction).unwrap();
            let times = times.parse::<usize>().unwrap();
            vec![direction; times]
        })
        .collect::<Vec<Direction>>();

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &Vec<Direction>) -> usize {
    let mut positions: HashSet<Position> = HashSet::new();
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    for motion in input {
        match motion {
            Direction::Up => {
                head.y -= 1;
                tail.touch(head);
                positions.insert(tail);
            }
            Direction::Down => {
                head.y += 1;
                tail.touch(head);
                positions.insert(tail);
            }
            Direction::Left => {
                head.x -= 1;
                tail.touch(head);
                positions.insert(tail);
            }
            Direction::Right => {
                head.x += 1;
                tail.touch(head);
                positions.insert(tail);
            }
        }
    }

    positions.len()
}

fn part2(input: &Vec<Direction>) -> usize {
    let mut positions: HashSet<Position> = HashSet::new();
    let mut rope = [Position { x: 0, y: 0 }; 10];

    for motion in input {
        match motion {
            Direction::Up => {
                rope[0].y -= 1;
                for i in 1..10 {
                    rope[i].touch(rope[i - 1]);
                }
                positions.insert(rope[9]);
            }
            Direction::Down => {
                rope[0].y += 1;
                for i in 1..10 {
                    rope[i].touch(rope[i - 1]);
                }
                positions.insert(rope[9]);
            }
            Direction::Left => {
                rope[0].x -= 1;
                for i in 1..10 {
                    rope[i].touch(rope[i - 1]);
                }
                positions.insert(rope[9]);
            }
            Direction::Right => {
                rope[0].x += 1;
                for i in 1..10 {
                    rope[i].touch(rope[i - 1]);
                }
                positions.insert(rope[9]);
            }
        }
    }

    positions.len()
}
