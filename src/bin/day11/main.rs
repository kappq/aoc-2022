use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, one_of},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};

#[derive(Clone, Copy, Debug)]
enum Value {
    Old,
    Value(u64),
}

#[derive(Clone, Debug)]
enum Operation {
    Add(Value, Value),
    Mul(Value, Value),
}

#[derive(Clone, Debug)]
struct Test {
    divisible: u64,
    true_monkey: u64,
    false_monkey: u64,
}

#[derive(Clone, Debug)]
struct Monkey {
    starting_items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    inspections: u64,
}

fn parse_starting_items(input: &str) -> IResult<&str, VecDeque<u64>> {
    let (input, _) = multispace1(input)?;
    let (input, starting_items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), nom::character::complete::u64),
    )(input)?;
    let (input, _) = newline(input)?;
    Ok((input, VecDeque::from(starting_items)))
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    Ok(alt((
        tag("old").map(|_| Value::Old),
        nom::character::complete::u64.map(|v| Value::Value(v)),
    ))(input)?)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, value1) = parse_value(input)?;
    let (input, operator) = delimited(multispace1, one_of("+*"), multispace1)(input)?;
    let (input, value2) = parse_value(input)?;
    let operation = match operator {
        '+' => Operation::Add(value1, value2),
        '*' => Operation::Mul(value1, value2),
        _ => panic!("invalid operator: {}", operator),
    };
    Ok((input, operation))
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let (input, _) = multispace1(input)?;
    let (input, divisible) = delimited(
        tag("Test: divisible by "),
        nom::character::complete::u64,
        newline,
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, true_monkey) = delimited(
        tag("If true: throw to monkey "),
        nom::character::complete::u64,
        newline,
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, false_monkey) = delimited(
        tag("If false: throw to monkey "),
        nom::character::complete::u64,
        multispace0,
    )(input)?;
    let test = Test {
        divisible,
        true_monkey,
        false_monkey,
    };
    Ok((input, test))
}

fn parse_monkey(monkey: &str) -> IResult<&str, Monkey> {
    let (input, _) = delimited(tag("Monkey "), nom::character::complete::u64, tag(":"))(monkey)?;
    let (input, _) = newline(input)?;
    let (input, starting_items) = parse_starting_items(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, test) = parse_test(input)?;
    let monkey = Monkey {
        starting_items,
        operation,
        test,
        inspections: 0,
    };
    Ok((input, monkey))
}

fn part1(mut monkeys: Vec<Monkey>) -> u64 {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].starting_items.pop_front() {
                monkeys[i].inspections += 1;
                let mut worry_level = match monkeys[i].operation {
                    Operation::Add(lhs, rhs) => {
                        let lhs = match lhs {
                            Value::Old => item,
                            Value::Value(v) => v,
                        };
                        let rhs = match rhs {
                            Value::Old => item,
                            Value::Value(v) => v,
                        };
                        lhs + rhs
                    }
                    Operation::Mul(lhs, rhs) => {
                        let lhs = match lhs {
                            Value::Old => item,
                            Value::Value(v) => v,
                        };
                        let rhs = match rhs {
                            Value::Old => item,
                            Value::Value(v) => v,
                        };
                        lhs * rhs
                    }
                };
                worry_level /= 3;
                if worry_level % monkeys[i].test.divisible as u64 == 0 {
                    let index = monkeys[i].test.true_monkey as usize;
                    monkeys[index].starting_items.push_back(worry_level);
                } else {
                    let index = monkeys[i].test.false_monkey as usize;
                    monkeys[index].starting_items.push_back(worry_level);
                }
            }
        }
    }
    let most_active = monkeys
        .iter()
        .max_by_key(|monkey| monkey.inspections)
        .unwrap();
    let second_most_active = monkeys
        .iter()
        .max_by_key(|monkey| {
            if monkey.inspections == most_active.inspections {
                0
            } else {
                monkey.inspections
            }
        })
        .unwrap();
    most_active.inspections * second_most_active.inspections
}

fn part2(mut monkeys: Vec<Monkey>) -> u64 {
    let divisor_product = monkeys.iter().map(|m| m.test.divisible).product::<u64>();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].starting_items.pop_front() {
                monkeys[i].inspections += 1;
                let mut worry_level = match monkeys[i].operation {
                    Operation::Add(lhs, rhs) => {
                        let lhs = match lhs {
                            Value::Old => item,
                            Value::Value(v) => v,
                        };
                        let rhs = match rhs {
                            Value::Old => item,
                            Value::Value(v) => v,
                        };
                        lhs + rhs
                    }
                    Operation::Mul(lhs, rhs) => {
                        let lhs = match lhs {
                            Value::Old => item,
                            Value::Value(v) => v,
                        };
                        let rhs = match rhs {
                            Value::Old => item,
                            Value::Value(v) => v,
                        };
                        lhs * rhs
                    }
                };
                worry_level %= divisor_product;
                if worry_level % monkeys[i].test.divisible as u64 == 0 {
                    let index = monkeys[i].test.true_monkey as usize;
                    monkeys[index].starting_items.push_back(worry_level);
                } else {
                    let index = monkeys[i].test.false_monkey as usize;
                    monkeys[index].starting_items.push_back(worry_level);
                }
            }
        }
    }
    let most_active = monkeys
        .iter()
        .max_by_key(|monkey| monkey.inspections)
        .unwrap();
    let second_most_active = monkeys
        .iter()
        .max_by_key(|monkey| {
            if monkey.inspections == most_active.inspections {
                0
            } else {
                monkey.inspections
            }
        })
        .unwrap();
    most_active.inspections * second_most_active.inspections
}

fn main() {
    let input = include_str!("input.txt");

    let monkeys = input
        .split("\n\n")
        .map(|monkey| parse_monkey(monkey).unwrap().1)
        .collect::<Vec<Monkey>>();

    println!("part 1: {}", part1(monkeys.clone()));
    println!("part 2: {}", part2(monkeys.clone()));
}
