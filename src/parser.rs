use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::error::Error;
use nom::sequence::tuple;
use nom::Err;

// Wow. that's a type....
type E<'a> = Err<Error<&'a str>>;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Move {
    Up,
    Down,
    Forward,
}

pub fn parse_move(input: &str) -> Option<(Move, i32)> {
    let up = map(tag("up"), |_| Move::Up);
    let down = map(tag("down"), |_| Move::Down);
    let fwd = map(tag("forward"), |_| Move::Forward);
    let spaces = space1;
    let num = map_res(digit1, |s: &str| s.parse::<i32>());
    let mov = alt((up, down, fwd));

    let res: Result<(&str, (_, _, i32)), E> = tuple((mov, spaces, num))(input);
    match res {
        Ok((_, (m, _, n))) => Some((m, n)),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse_move("up 4"), Some((Move::Up, 4)));
        assert_eq!(parse_move("down 2"), Some((Move::Down, 2)));
        assert_eq!(parse_move("forward 10"), Some((Move::Forward, 10)));
        assert_eq!(parse_move("bla"), None);
    }
}
