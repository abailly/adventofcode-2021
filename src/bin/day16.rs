use aoc2021::parser::Ebits;
use hex;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::multi::count;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::convert::TryInto;
use std::env;
use std::fs::read_to_string;
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

fn as_num(s: &str) -> u64 {
    let mut num = 0;
    for c in s.chars() {
        match c {
            '0' => num *= 2,
            '1' => num = 2 * num + 1,
            _ => (),
        }
    }
    num
}

fn convert(vals: (Vec<u64>, u64)) -> u64 {
    let mut res = 0_u64;
    for x in vals.0 {
        res *= 16;
        res += x;
    }
    res *= 16;
    res += vals.1;
    res
}

fn parse_value(input: &str) -> IResult<&str, Packet> {
    let val_fragment = tuple((tag("1"), take(4usize))).map(|(_, n)| as_num(n));
    let val_term = tuple((tag("0"), take(4usize))).map(|(_, n)| as_num(n));
    let val_bits = tuple((many0(val_fragment), val_term));
    let mut value = map(
        tuple((take(3usize), tag("100"), val_bits)),
        |(v, _, bts)| Packet {
            version: as_num(v) as u8,
            content: Content::Value(convert(bts)),
        },
    );
    value(input)
}

fn parse_operator_0(input: &str) -> IResult<&str, Packet> {
    match tuple((
        take(3usize).map(|n| as_num(n) as u8),
        take(3usize).map(|n| as_num(n) as u8),
        tag("0").map(|_| ()),
        take(15usize).map(|n| as_num(n)),
    ))(input)
    {
        Ok((bs, (version, t, _, len))) => {
            let mut consumed = 0usize;
            let mut pkts = vec![];
            let mut inp = bs;
            while consumed < len.try_into().unwrap() {
                match parse_packet(inp) {
                    Ok((more, p)) => {
                        pkts.push(p);
                        consumed = consumed + (inp.len() - more.len());
                        inp = more;
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

fn parse_operator_1(input: &str) -> IResult<&str, Packet> {
    match tuple((
        take(3usize).map(|n| as_num(n) as u8),
        take(3usize).map(|n| as_num(n) as u8),
        tag("1").map(|_| ()),
        take(11usize).map(|n| as_num(n)),
    ))(input)
    {
        Ok((inp, (version, t, _, len))) => map(count(parse_packet, len as usize), |pkts| Packet {
            version,
            content: Content::Operator(t, pkts),
        })(inp),
        Err(e) => Err(e),
    }
}

pub fn to_bits(s: &str) -> String {
    let mut res = String::new();
    for c in s.chars() {
        let code = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        };
        res.push_str(code);
    }
    res
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    println!("parsing {:?}", input);
    alt((parse_value, parse_operator_0, parse_operator_1))(input)
}

fn parse_packets(input: &str) -> Option<Packet> {
    let input_bits = to_bits(input);
    match parse_packet(&input_bits) {
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
