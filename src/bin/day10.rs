use std::env;
use std::fs::read_to_string;
use std::process;

fn points(r: Result<(), char>) -> u64 {
    if let Err(c) = r {
        return match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        };
    }
    0
}

fn parse_line(line: &str) -> Result<(), char> {
    Ok(())
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
            .partition(Result::is_ok);
        println!("{}", errs.iter().fold(0, |n, e| points(*e) + n));
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

        assert_eq!(res, Err('}'));
    }
}
