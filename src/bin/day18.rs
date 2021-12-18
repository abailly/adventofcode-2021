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
use std::u64::MIN;

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

fn add_right_most(val: i8, sn: SN) -> SN {
    if val == -1 {
        sn
    } else {
        match sn {
            SN::Pair(d, l, r) => SN::Pair(d, l, Box::new(add_right_most(val, *r))),
            SN::Reg(v) => SN::Reg(v + val as u8),
        }
    }
}

fn add_left_most(val: i8, sn: SN) -> SN {
    if val == -1 {
        sn
    } else {
        match sn {
            SN::Pair(d, l, r) => SN::Pair(d, Box::new(add_left_most(val, *l)), r),
            SN::Reg(v) => SN::Reg(v + val as u8),
        }
    }
}

fn explode1(sn: SN) -> (i8, i8, SN) {
    match sn {
        SN::Pair(1, l, r) => match (*l, *r) {
            (SN::Reg(nl), SN::Reg(nr)) => (nl as i8, nr as i8, SN::Reg(0)),
            _ => unreachable!(),
        },
        SN::Pair(_, l, r) => match depth(&l).cmp(&depth(&r)) {
            Ordering::Less => {
                let (lv, rv, nr) = explode1(*r);
                (
                    -1,
                    rv,
                    SN::Pair(
                        max(depth(&l), depth(&nr)) + 1,
                        Box::new(add_right_most(lv, *l)),
                        Box::new(nr),
                    ),
                )
            }
            _ => {
                let (lv, rv, nl) = explode1(*l);
                (
                    lv,
                    -1,
                    SN::Pair(
                        max(depth(&nl), depth(&r)) + 1,
                        Box::new(nl),
                        Box::new(add_left_most(rv, *r)),
                    ),
                )
            }
        },
        _ => (-1, -1, sn),
    }
}

fn explode(sn: SN) -> Option<SN> {
    match sn {
        SN::Pair(depth, a, b) if depth == 5u8 => Some(explode1(SN::Pair(depth, a, b)).2),
        _ => None,
    }
}

fn split(sn: SN) -> Option<SN> {
    match sn {
        SN::Pair(_, a, b) => {
            let bn = b.clone();
            let an = a.clone();

            split(*a).map_or(
                split(*b).map(|sr| SN::Pair(max(depth(&an), depth(&sr)) + 1, an, Box::new(sr))),
                |newsn| {
                    Some(SN::Pair(
                        max(depth(&newsn), depth(&bn)) + 1,
                        Box::new(newsn),
                        bn,
                    ))
                },
            )
        }

        SN::Reg(val) if val > 9 => {
            let rem = val % 2;
            let res = SN::Pair(
                1,
                Box::new(SN::Reg(val / 2)),
                Box::new(SN::Reg(val / 2 + rem)),
            );
            Some(res)
        }
        _ => None,
    }
}

fn reduce(sn: SN) -> SN {
    match explode(sn.clone()) {
        Some(nsn) => reduce(nsn),
        None => match split(sn.clone()) {
            Some(nsn) => reduce(nsn),
            None => sn,
        },
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

fn magnitude(sn: SN) -> u64 {
    match sn {
        SN::Pair(depth, a, b) => 3 * magnitude(*a) + 2 * magnitude(*b),
        SN::Reg(v) => v as u64,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        let nums: Vec<&str> = input.split("\n").filter(|s| !s.is_empty()).collect();
        let sns: Vec<SN> = nums.iter().map(|s| parse_sn(s).unwrap().1).collect();
        let other = sns.clone();
        let mut max = MIN;
        for s1 in sns.iter() {
            for s2 in other.iter() {
                let a = s1.clone();
                let b = s2.clone();
                if *s1 != *s2 {
                    let mag = magnitude(reduce(add(a, b)));
                    if mag > max {
                        max = mag;
                    }
                }
            }
        }
        println!("max magnitude: {}", max);
    } else {
        println!("fail to parse {}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_explode() {
        let tests = vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];
        for (ins, outs) in tests {
            println!("testing {}", ins);
            let inp = parse_sn(ins).unwrap().1;
            let out = parse_sn(outs).unwrap().1;

            assert_eq!(explode(inp), Some(out));
        }
    }

    #[test]
    fn can_split() {
        let tests = vec![
            (
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            ),
            (
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            ),
        ];
        for (ins, outs) in tests {
            println!("testing {}", ins);
            let inp = parse_sn(ins).unwrap().1;
            let out = parse_sn(outs).unwrap().1;

            assert_eq!(split(inp), Some(out));
        }
    }

    #[test]
    fn can_reduce_added_value() {
        let input = vec!["[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"];

        let expected = parse_sn("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap().1;

        let sns: Vec<SN> = input.iter().map(|s| parse_sn(s).unwrap().1).collect();

        let res = reduce(add(sns[0].clone(), sns[1].clone()));

        assert_eq!(res, expected);
    }

    #[test]
    fn can_reduce_complex_value() {
        let input = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ];

        let sns: Vec<SN> = input.iter().map(|s| parse_sn(s).unwrap().1).collect();

        let expected = parse_sn("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
            .unwrap()
            .1;

        let res = sns[1..]
            .iter()
            .fold(sns[0].clone(), |a, b| reduce(add(a, b.clone())));

        assert_eq!(res, expected);
    }

    #[test]
    fn can_reduce_simple_value() {
        let input = vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"];

        let sns: Vec<SN> = input.iter().map(|s| parse_sn(s).unwrap().1).collect();

        let expected = parse_sn("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap().1;

        let res = sns[1..]
            .iter()
            .fold(sns[0].clone(), |a, b| reduce(add(a, b.clone())));

        assert_eq!(res, expected);
    }
    #[test]
    fn can_compute_magnitude_on_sample() {
        let input = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ];

        let sns: Vec<SN> = input.iter().map(|s| parse_sn(s).unwrap().1).collect();

        let expected = parse_sn("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
            .unwrap()
            .1;

        let res = sns[1..]
            .iter()
            .fold(sns[0].clone(), |a, b| reduce(add(a, b.clone())));

        assert_eq!(res, expected);
        assert_eq!(magnitude(res), 4140);
    }
}
