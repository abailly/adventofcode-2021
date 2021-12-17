use crate::bits::complete::tag;
use crate::bits::complete::take;
use aoc2021::parser::Ebits;
use hex;
use nom::bits;
use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
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

fn parse_operator_0(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let mut op_prefix = tuple((take(3usize), take(3usize), tag(0x0, 1usize), take(15usize)));
    match op_prefix(input) {
        Ok(((bytes, off), (version, t, _, len))) => {
            let remaining = bytes.len() * 8 - off;
            let mut consumed = 0;
            let mut pkts = vec![];
            let mut inp = (bytes, off);
            while consumed < len {
                match parse_packet(inp) {
                    Ok(((nbytes, noff), p)) => {
                        pkts.push(p);
                        consumed = consumed + (remaining - (nbytes.len() * 8 - noff));
                        inp = (nbytes, noff);
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

fn parse_operator_1(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let mut op_prefix = tuple((take(3usize), take(3usize), tag(0x01, 1usize), take(11usize)));
    match op_prefix(input) {
        Ok((inp, (version, t, _, len))) => {
            let mut remaining: usize = len;
            let mut pkts = vec![];
            let mut stream = inp;
            while remaining > 0 {
                match parse_packet(stream) {
                    Ok((ninp, p)) => {
                        pkts.push(p);
                        stream = ninp;
                        remaining -= 1;
                    }
                    Err(e) => return Err(e),
                }
            }
            println!("check len {}", pkts.len() == len);
            Ok((
                stream,
                Packet {
                    version,
                    content: Content::Operator(t, pkts),
                },
            ))
        }
        Err(e) => Err(e),
    }
}

pub fn decode_hex(s: &str) -> Vec<u8> {
    hex::decode(s).unwrap()
}

fn parse_packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    println!("parsing {:?}", input);
    alt((parse_value, parse_operator_1, parse_operator_0))(input)
}

fn parse_packets(input: &str) -> Option<Packet> {
    let bytes = &decode_hex(input)[0..];
    let res: Result<_, Ebits> = bits(parse_packet)(&bytes);
    match res {
        Ok((inp, p)) => {
            println!("Success parsing, remaining {:?}", inp);
            Some(p)
        }
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

    #[test]
    fn can_parse_operator_packet_with_len_1() {
        let input = "EE00D40C823060";

        let res = parse_packets(&input);

        assert_eq!(
            res,
            Some(Packet {
                version: 7,
                content: Content::Operator(
                    3,
                    vec![
                        Packet {
                            version: 2,
                            content: Content::Value(1)
                        },
                        Packet {
                            version: 4,
                            content: Content::Value(2)
                        },
                        Packet {
                            version: 1,
                            content: Content::Value(3)
                        }
                    ]
                )
            })
        );
    }

    #[test]
    fn can_compute_total_version() {
        let other = "620080001611562C8802118E34";
        let input = "A0016C880162017C3686B18A3D4780";

        let pkt = parse_packets(&input).unwrap();

        assert_eq!(versions(&parse_packets(&other).unwrap()), 12);
        assert_eq!(
            versions(&parse_packets("C0015000016115A2E0802F182340").unwrap()),
            23
        );
        assert_eq!(versions(&parse_packets("8A004A801A8002F478").unwrap()), 16);
        assert_eq!(versions(&pkt), 31);
        assert_eq!(
            pkt,
            Packet {
                version: 5,
                content: Content::Operator(
                    0,
                    vec![Packet {
                        version: 1,
                        content: Content::Operator(
                            0,
                            vec![Packet {
                                version: 3,
                                content: Content::Operator(
                                    0,
                                    vec![
                                        Packet {
                                            version: 7,
                                            content: Content::Value(6)
                                        },
                                        Packet {
                                            version: 6,
                                            content: Content::Value(6)
                                        },
                                        Packet {
                                            version: 5,
                                            content: Content::Value(12)
                                        },
                                        Packet {
                                            version: 2,
                                            content: Content::Value(15)
                                        },
                                        Packet {
                                            version: 2,
                                            content: Content::Value(15)
                                        }
                                    ]
                                )
                            }]
                        )
                    }]
                )
            }
        );
    }
}
