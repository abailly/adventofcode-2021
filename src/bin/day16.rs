use crate::bits::complete::tag;
use crate::bits::complete::take;
use aoc2021::parser::Ebits;
use nom::bits;
use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::env;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::process;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Content {
    Value(u64),
    Operator(u8, Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Packet {
    version: u8,
    content: Content,
}

fn convert(vals: (Vec<u8>, u8)) -> u64 {
    let mut res = 0_u64;
    for x in vals.0 {
        res *= 16;
        res += x as u64;
    }
    res *= 16;
    res += vals.1 as u64;
    res
}

fn parse_value(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let val_fragment = tuple((tag(0x01, 1usize), take(4usize))).map(|(_, n)| n);
    let val_term = tuple((tag(0x0, 1usize), take(4usize))).map(|(_, n)| n);
    let val_bits = tuple((many0(val_fragment), val_term));
    let mut value = map(
        tuple((take(3usize), tag(0x04, 3usize), val_bits)),
        |(v, _, bts)| Packet {
            version: v,
            content: Content::Value(convert(bts)),
        },
    );
    value(input)
}

fn parse_operator(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let mut op_prefix = tuple((take(3usize), take(3usize), tag(0x0, 1usize), take(15usize)));
    match op_prefix(input) {
        Ok(((bytes, off), (version, t, _, len))) => {
            let remaining = bytes.len() * 8 - off;
            let mut consumed = 0;
            println!(
                "bytes: {:?} offset: {} len: {} remaining: {} ",
                bytes, off, len, remaining
            );
            let mut pkts = vec![];
            let mut inp = (bytes, off);
            while consumed < len {
                match parse_packet(inp) {
                    Ok(((nbytes, noff), p)) => {
                        pkts.push(p);
                        println!("bytes: {:?} offset: {} ", nbytes, noff);
                        consumed = consumed + (remaining - (nbytes.len() * 8 - noff));
                        inp = (nbytes, noff);
                        println!(
                            "bytes: {:?} offset: {} len: {} consumed: {} ",
                            nbytes, noff, len, consumed
                        );
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok((
                inp,
                Packet {
                    version,
                    content: Content::Operator(t, pkts),
                },
            ))
        }
        Err(e) => Err(e),
    }
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn parse_packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    alt((parse_value, parse_operator))(input)
}

fn parse_packets(input: &str) -> Option<Packet> {
    let bytes = &decode_hex(input).unwrap()[0..];
    let res: Result<_, Ebits> = bits(parse_packet)(&bytes);
    match res {
        Ok((_, p)) => Some(p),
        Err(_) => None,
    }
}

fn versions(packet: &Packet) -> u64 {
    packet.version as u64
        + match &packet.content {
            Content::Value(_) => 0,
            Content::Operator(_, packets) => packets.iter().fold(0, |n, p| n + versions(p)),
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
    #[test]
    fn can_parse_operator_packet_with_len_0() {
        let input = "38006F45291200";

        let res = parse_packets(&input);

        assert_eq!(
            res,
            Some(Packet {
                version: 1,
                content: Content::Operator(
                    6,
                    vec![
                        Packet {
                            version: 6,
                            content: Content::Value(10)
                        },
                        Packet {
                            version: 2,
                            content: Content::Value(20)
                        }
                    ]
                )
            })
        );
    }
}
