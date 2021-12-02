use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::error::Error;
use nom::sequence::tuple;
use nom::Err;

// Wow. that's a type....
type E<'a> = Err<Error<&'a str>>;

pub fn parse_move(input: &str) -> Option<(i32, i32)> {
    let up = tag("up");
    let spaces = space1;
    let num = map_res(digit1, |s: &str| s.parse::<i32>());
    let res: Result<(&str, (_, _, i32)), E> = tuple((up, spaces, num))(input);
    match res {
        Ok((_, (_, _, n))) => Some((0, -n)),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse_move("up 4"), Some((0, -4)));
        assert_eq!(parse_move("bla"), None);
    }
}
