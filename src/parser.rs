use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::Err;

// Wow. that's a type....
type E<'a> = Err<Error<&'a str>>;

/// Possible moves
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Move {
    Up,
    Down,
    Forward,
}

/// Parse a single move order
/// This function does not try to interpret the moves, it  only
/// parses them and produce typed structure representing the move.
pub fn parse_move(input: &str) -> Option<(Move, i32)> {
    let up = map(tag("up"), |_| Move::Up);
    let down = map(tag("down"), |_| Move::Down);
    let fwd = map(tag("forward"), |_| Move::Forward);
    let mov = alt((up, down, fwd));
    let num = map_res(digit1, |s: &str| s.parse::<i32>());

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
