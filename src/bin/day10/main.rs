use std::str::FromStr;

enum Instruction {
    Addx(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }

        let (_, value) = s.split_once(' ').unwrap();
        Ok(Instruction::Addx(value.parse().unwrap()))
    }
}
fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect::<Vec<Instruction>>();

    println!("part 1: {}", part1(&input));
    println!("part 2:\n{}", part2(&input));
}

fn part1(input: &Vec<Instruction>) -> i32 {
    let mut strength = 0;

    let mut cycles = 1;
    let mut x = 1;

    for instruction in input {
        match instruction {
            Instruction::Addx(v) => {
                cycles += 1;
                if let 20 | 60 | 100 | 140 | 180 | 220 = cycles {
                    strength += cycles * x;
                }
                cycles += 1;
                x += v;
            }
            Instruction::Noop => {
                cycles += 1;
            }
        }

        if let 20 | 60 | 100 | 140 | 180 | 220 = cycles {
            strength += cycles * x;
        }
    }

    strength
}

struct Cpu {
    cycles: usize,
    x: i32,
    display: [bool; 240],
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            cycles: 1,
            x: 1,
            display: [false; 240],
        }
    }

    fn tick(&mut self) {
        let current_pixel = self.cycles - 1;
        self.display[current_pixel] =
            ((self.x - 1)..=(self.x + 1)).contains(&(current_pixel as i32 % 40));
        self.cycles += 1;
    }

    fn render(&self) -> String {
        let mut crt = String::new();
        for row in 0..6 {
            for col in 0..40 {
                crt.push(if self.display[row * 40 + col] {
                    '#'
                } else {
                    '.'
                });
            }
            crt.push('\n');
        }
        crt
    }
}

fn part2(input: &Vec<Instruction>) -> String {
    let mut cpu = Cpu::new();

    for instruction in input {
        match instruction {
            Instruction::Addx(v) => {
                cpu.tick();
                cpu.tick();
                cpu.x += v;
            }
            Instruction::Noop => {
                cpu.tick();
            }
        }
    }

    cpu.render()
}
