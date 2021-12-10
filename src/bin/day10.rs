use std::env;
use std::fs::read_to_string;
use std::process;

fn points(r: &Result<(), ParseErr>) -> u64 {
    match r {
        Err(ParseErr::InvalidChar(c)) => {
            return match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            }
        }
        Err(ParseErr::EOL(stack)) => {
            return stack.iter().rev().fold(0, |n, c| {
                let val = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                };
                5 * n + val
            })
        }
        Ok(_) => 0,
    }
}

#[derive(Debug, PartialEq, Clone)]
enum ParseErr {
    InvalidChar(char),
    EOL(Vec<char>),
}

fn is_not_eol(res: &Result<(), ParseErr>) -> bool {
    match res {
        Err(ParseErr::EOL(_)) => false,
        _ => true,
    }
}

fn is_eol(res: &Result<(), ParseErr>) -> bool {
    match res {
        Err(ParseErr::EOL(_)) => true,
        _ => false,
    }
}

fn parse_line(line: &str) -> Result<(), ParseErr> {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' => stack.push(c),
            '[' => stack.push(c),
            '{' => stack.push(c),
            '<' => stack.push(c),
            ')' => {
                if let Some(h) = stack.pop() {
                    match h {
                        '(' => (),
                        _ => return Err(ParseErr::InvalidChar(c)),
                    }
                } else {
                    return Err(ParseErr::EOL(stack));
                }
            }
            ']' => {
                if let Some(h) = stack.pop() {
                    match h {
                        '[' => (),
                        _ => return Err(ParseErr::InvalidChar(c)),
                    }
                } else {
                    return Err(ParseErr::EOL(stack));
                }
            }
            '}' => {
                if let Some(h) = stack.pop() {
                    match h {
                        '{' => (),
                        _ => return Err(ParseErr::InvalidChar(c)),
                    }
                } else {
                    return Err(ParseErr::EOL(stack));
                }
            }
            '>' => {
                if let Some(h) = stack.pop() {
                    match h {
                        '<' => (),
                        _ => return Err(ParseErr::InvalidChar(c)),
                    }
                } else {
                    return Err(ParseErr::EOL(stack));
                }
            }
            _ => return Err(ParseErr::InvalidChar(c)),
        };
    }
    if stack.is_empty() {
        Ok(())
    } else {
        Err(ParseErr::EOL(stack))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        let (_, errs): (Vec<_>, Vec<_>) = input
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| parse_line(s))
            .filter(is_eol)
            .partition(Result::is_ok);
        let mut res = errs.iter().map(points).collect::<Vec<u64>>();
        res.sort();
        println!("{:?}", res[(res.len() / 2)]);
    } else {
        println!("fail to parse {}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_corrupted_lines() {
        let sample = "{([(<{}[<>[]}>{[]{[(<()>";

        let res = parse_line(&sample);

        assert_eq!(res, Err(ParseErr::InvalidChar('}')));
    }
}
