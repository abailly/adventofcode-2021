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

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Forward,
}

pub fn parse_move(input: &str) -> Option<(i32, i32)> {
    let up = map(tag("up"), |_| Move::Up);
    let down = map(tag("down"), |_| Move::Down);
    let fwd = map(tag("forward"), |_| Move::Forward);
    let spaces = space1;
    let num = map_res(digit1, |s: &str| s.parse::<i32>());
    let mov = alt((up, down, fwd));
    let res: Result<(&str, (_, _, i32)), E> = tuple((mov, spaces, num))(input);
    match res {
        Ok((_, (Move::Up, _, n))) => Some((0, -n)),
        Ok((_, (Move::Down, _, n))) => Some((0, n)),
        Ok((_, (Move::Forward, _, n))) => Some((n, 0)),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse_move("up 4"), Some((0, -4)));
        assert_eq!(parse_move("down 2"), Some((0, 2)));
        assert_eq!(parse_move("forward 10"), Some((10, 0)));
        assert_eq!(parse_move("bla"), None);
    }
}
