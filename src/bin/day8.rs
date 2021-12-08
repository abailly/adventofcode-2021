use std::env;
use std::fs::read_to_string;
use std::process;

type DigitLine<'a> = (Vec<&'a str>, Vec<&'a str>);

fn is_simple(s: &str) -> bool {
    let ln = s.len();
    ln == 2 || ln == 3 || ln == 4 || ln == 7
}

fn solve<'a>(puzzle: &Vec<DigitLine<'a>>) -> i64 {
    let mut num_basic = 0_i64;
    for (_, ds) in puzzle {
        num_basic += ds
            .iter()
            .filter(|s| {
                println!("{}", s);
                is_simple(s)
            })
            .collect::<Vec<&&str>>()
            .len() as i64;
    }
    num_basic
}

fn parse_digits<'a>(lines: &Vec<&'a str>) -> Option<Vec<(Vec<&'a str>, Vec<&'a str>)>> {
    let mut res = vec![];
    for s in lines {
        let parts: Vec<&str> = s.split(" | ").collect();
        res.push((parts[0].split(' ').collect(), parts[1].split(' ').collect()));
    }
    Some(res)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        if let Some(puzzle) = parse_digits(&input.split("\n").filter(|s| !s.is_empty()).collect()) {
            let solution = solve(&puzzle);
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
    fn test_parse_input() {
        let sample = vec!["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"];

        if let Some(res) = parse_digits(&sample) {
            println!("{:?}", res);
            assert_eq!(
                res[0],
                (
                    vec![
                        "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb",
                        "fabcd", "edb"
                    ],
                    vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"]
                )
            );
        } else {
            panic!("cannot parse input");
        }
    }
}
