use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use num::FromPrimitive;
use num::Num;
use std::fs::read_to_string;

// Wow. that's a type....
pub type E<'a> = Err<Error<&'a str>>;
pub type Ebits<'a> = Err<Error<&'a [u8]>>;

/// Possible moves
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Move {
    Up,
    Down,
    Forward,
}

/// Parse a single integer
pub fn num(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}

/// Parse a single move order
/// This function does not try to interpret the moves, it  only
/// parses them and produce typed structure representing the move.
pub fn parse_move(input: &str) -> Option<(Move, i32)> {
    let up = map(tag("up"), |_| Move::Up);
    let down = map(tag("down"), |_| Move::Down);
    let fwd = map(tag("forward"), |_| Move::Forward);
    let mov = alt((up, down, fwd));

    let res: Result<_, E> = tuple((mov, space1, num))(input);
    match res {
        Ok((_, (m, _, n))) => Some((m, n)),
        Err(_) => None,
    }
}

/// Possible moves
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Bit {
    Zero,
    One,
}

/// Parse a sequence of binary digits
pub fn parse_bits(input: &str) -> Option<Vec<Bit>> {
    let one = map(char('1'), |_| Bit::One);
    let zero = map(char('0'), |_| Bit::Zero);
    let dig = alt((one, zero));
    let mut digs = many1(dig);
    let res: Result<_, E> = digs(input);
    match res {
        Ok((_, res)) => Some(res),
        Err(_) => None,
    }
}

pub fn to_int(bits: &Vec<Bit>) -> i32 {
    bits.iter().fold(0, |acc, bit| match bit {
        Bit::One => acc * 2 + 1,
        Bit::Zero => acc * 2,
    })
}

/// Parse a list of comma-separated numbers from a string
/// Remove trailing new-line if any
pub fn parse_csv(input: &Vec<&str>) -> Option<Vec<i64>> {
    let mut nums = separated_list1(char(','), map(num, |n| n as i64));
    println!("{:?}", input);
    match nums(input[0]) {
        Ok((_, res)) => Some(res),
        Err(_) => None,
    }
}

/// Parse a matrix of single-digit numbers
pub fn parse_digits<N: Num + FromPrimitive>(lines: &Vec<&str>) -> Option<Vec<Vec<N>>> {
    let mut output: Vec<Vec<N>> = vec![];
    for line in lines {
        let mut row: Vec<N> = vec![];
        line.chars().for_each(|c| {
            row.push(
                FromPrimitive::from_u16(c as u16).map_or(N::zero(), |cn: N| {
                    FromPrimitive::from_u16('0' as u16).map_or(N::zero(), |o| cn - o)
                }),
            );
        });
        output.push(row);
    }
    Some(output)
}

/// Parse a file into a string and call given parser on this string
pub fn parse_file<R>(file: &str, parser: fn(&Vec<&str>) -> Option<R>) -> Option<R> {
    if let Ok(input) = read_to_string(file) {
        return parser(&input.split("\n").collect::<Vec<&str>>());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse_move("up 4"), Some((Move::Up, 4)));
        assert_eq!(parse_move("down 2"), Some((Move::Down, 2)));
        assert_eq!(parse_move("forward 10"), Some((Move::Forward, 10)));
        assert_eq!(parse_move("forward -10"), None);
        assert_eq!(parse_move("up    "), None);
        assert_eq!(parse_move("5"), None);
        assert_eq!(parse_move("bla"), None);
    }

    #[test]
    fn test_bits_parser() {
        assert_eq!(
            parse_bits("000111"),
            Some(vec![
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::One,
                Bit::One,
                Bit::One
            ])
        );
    }

    #[test]
    fn test_to_int() {
        assert_eq!(
            to_int(&vec![
                Bit::One,
                Bit::Zero,
                Bit::Zero,
                Bit::One,
                Bit::One,
                Bit::One
            ]),
            39
        );
    }
}
