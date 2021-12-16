use aoc2021::nums::all_neighbours;
use aoc2021::nums::neighbours;
use aoc2021::parser::parse_digits;
use aoc2021::parser::Ebits;
use aoc2021::parser::E;
use aoc2021::vents::Pos;
use core::u64::MAX;
use nom::bits;
use nom::branch::alt;
use nom::IResult;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::process;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Content {
    Value(u64),
    Operator(Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Packet {
    version: u8,
    content: Content,
}

fn parse_value(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    Ok((
        input,
        Packet {
            version: 3,
            content: Content::Value(1),
        },
    ))
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn parse_packets(input: &str) -> Option<Packet> {
    let bytes = &decode_hex(input).unwrap()[0..];
    let res: Result<_, Ebits> = bits(parse_value)(&bytes);
    match res {
        Ok((_, p)) => Some(p),
        Err(_) => None,
    }
}

fn versions(packet: &Packet) -> u64 {
    packet.version as u64
        + match &packet.content {
            Content::Value(_) => 0,
            Content::Operator(packets) => packets.iter().fold(0, |n, p| n + versions(p)),
        }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        if let Some(packet) = parse_packets(&input) {
            let solution = versions(&packet);
            println!("{}", solution);
        }
    } else {
        println!("fail to parse {}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_literal_value() {
        let input = "D2FE28";

        let res = parse_packets(&input);

        assert_eq!(
            res,
            Some(Packet {
                version: 6,
                content: Content::Value(2021)
            })
        );
    }
}
