use crate::bits::streaming::tag;
use crate::bits::streaming::take;
use aoc2021::parser::num;
use aoc2021::parser::Ebits;
use hex;
use nom::bits;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::cmp::max;
use std::cmp::Ordering;
use std::env;
use std::fs::read_to_string;
use std::process;

#[derive(Debug, PartialEq, Eq, Clone)]
enum SN {
    Reg(u8),
    Pair(u8, Box<SN>, Box<SN>),
}

fn depth(sn: &SN) -> u8 {
    match sn {
        SN::Reg(_) => 0,
        SN::Pair(lvl, _, _) => *lvl,
    }
}

fn explode1(sn: SN) -> SN {
    println!("explode1 {:?}", sn);
    match sn {
        SN::Pair(1, l, r) => SN::Reg(0),
        SN::Pair(_, l, r) => match depth(&l).cmp(&depth(&r)) {
            Ordering::Less => {
                let nr = explode1(*r);
                SN::Pair(max(depth(&l), depth(&nr)) + 1, l, Box::new(nr))
            }
            _ => {
                let nl = explode1(*l);
                SN::Pair(max(depth(&nl), depth(&r)) + 1, Box::new(nl), r)
            }
        },
        _ => sn,
    }
}

fn explode(sn: SN) -> Option<SN> {
    println!("exploding {:?}", sn);
    match sn {
        SN::Pair(depth, a, b) if depth > 4u8 => Some(explode1(SN::Pair(depth, a, b))),
        _ => None,
    }
}

fn reduce(sn: SN) -> SN {
    match explode(sn.clone()) {
        Some(nsn) => reduce(nsn),
        None => sn,
    }
}

fn parse_sn(input: &str) -> IResult<&str, SN> {
    let reg = num.map(|n| SN::Reg(n as u8));
    let pair = map(
        delimited(
            char('['),
            separated_pair(parse_sn, char(','), parse_sn),
            char(']'),
        ),
        |(a, b)| add(a, b),
    );
    alt((reg, pair))(input)
}

fn add(n1: SN, n2: SN) -> SN {
    SN::Pair(max(depth(&n1), depth(&n2)) + 1, Box::new(n1), Box::new(n2))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(_input) = read_to_string(&args[1]) {
        ()
    } else {
        println!("fail to parse {}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_explode() {
        let inp = parse_sn("[[[[[9,8],1],2],3],4]").unwrap().1;
        let out = parse_sn("[[[[0,9],2],3],4]").unwrap().1;

        assert_eq!(explode(inp), Some(out));
    }

    // #[test]
    // fn can_parse_literal_value() {
    //     let input = vec![
    //         "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
    //         "[[[5,[2,8]],4],[5,[[9,9],0]]]",
    //         "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
    //         "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
    //         "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
    //         "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
    //         "[[[[5,4],[7,7]],8],[[8,3],8]]",
    //         "[[9,3],[[9,9],[6,[4,9]]]]",
    //         "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
    //         "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    //     ];

    //     let sns: Vec<SN> = input.iter().map(|s| parse_sn(s).unwrap().1).collect();

    //     let expected = parse_sn("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
    //         .unwrap()
    //         .1;

    //     let res = sns[1..].iter().fold(sns[0].clone(), |a, b| {
    //         reduce(SN::Pair(
    //             max(depth(&a), depth(b)) + 1,
    //             Box::new(a),
    //             Box::new(b.clone()),
    //         ))
    //     });

    //     assert_eq!(res, expected);
    // }
}
