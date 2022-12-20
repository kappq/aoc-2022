use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::newline, multi::separated_list0,
    IResult, Parser,
};

#[derive(Debug, Clone)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a == b,
            (Packet::List(list_a), Packet::List(list_b)) => list_a == list_b,
            (Packet::Integer(integer), Packet::List(list)) => {
                let integer = vec![Packet::Integer(*integer)];
                integer.eq(list)
            }
            (Packet::List(list), Packet::Integer(integer)) => {
                let integer = vec![Packet::Integer(*integer)];
                list.eq(&integer)
            }
        }
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
            (Packet::List(list_a), Packet::List(list_b)) => list_a.cmp(list_b),
            (Packet::Integer(integer), Packet::List(list)) => {
                let integer = vec![Packet::Integer(*integer)];
                integer.cmp(list)
            }
            (Packet::List(list), Packet::Integer(integer)) => {
                let integer = vec![Packet::Integer(*integer)];
                list.cmp(&integer)
            }
        }
    }
}

fn parse_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    let (input, first_packet) = parse_list(input)?;
    let (input, _) = newline(input)?;
    let (input, second_packet) = parse_list(input)?;
    Ok((input, (first_packet, second_packet)))
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    Ok(alt((
        nom::character::complete::u32.map(|v| Packet::Integer(v)),
        parse_list,
    ))(input)?)
}

fn parse_list(input: &str) -> IResult<&str, Packet> {
    let (input, _) = tag("[")(input)?;
    let (input, items) = separated_list0(tag(","), parse_packet)(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, Packet::List(items)))
}

fn main() {
    let input = include_str!("input.txt");
    let pairs = input
        .split("\n\n")
        .map(|pair| parse_pair(pair).unwrap().1)
        .collect::<Vec<(Packet, Packet)>>();

    println!("part 1: {}", part1(&pairs));
    println!("part 2: {}", part2(&pairs));
}

fn part1(pairs: &Vec<(Packet, Packet)>) -> usize {
    pairs
        .iter()
        .enumerate()
        .map(|(i, (left, right))| if left < right { i + 1 } else { 0 })
        .sum()
}

fn part2(pairs: &Vec<(Packet, Packet)>) -> usize {
    let divider1 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let divider2 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    let dividers = vec![divider1.clone(), divider2.clone()];
    let sorted = dividers
        .iter()
        .chain(pairs.iter().flat_map(|(left, right)| vec![left, right]))
        .sorted();
    let divider1_index = sorted
        .clone()
        .find_position(|packet| **packet == divider1)
        .unwrap()
        .0
        + 1;
    let divider2_index = sorted
        .clone()
        .find_position(|packet| **packet == divider2)
        .unwrap()
        .0
        + 1;
    divider1_index * divider2_index
}
