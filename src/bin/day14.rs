use aoc2021::parser::E;
use core::u64::MAX;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::sequence::tuple;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::env;
use std::fs::read_to_string;
use std::iter::FromIterator;
use std::process;

type Insertion = HashMap<(char, char), char>;

fn solve(instructions: &(String, Insertion), steps: u8) -> String {
    let mut res = instructions.0.clone();
    for _ in 0..steps {
        let pairs: Vec<(char, char)> = res.chars().zip(res[1..].chars()).collect();

        let mut new_res = String::new();
        for (x, y) in pairs {
            new_res.pop();
            match &instructions.1.get(&(x, y)) {
                Some(c) => {
                    new_res.push(x);
                    new_res.push(**c);
                    new_res.push(y);
                }
                None => (),
            }
        }
        res = new_res;
    }

    res.to_string()
}

fn compute_number(result: &String) -> u64 {
    let mut chars_count: HashMap<char, u64> = HashMap::new();
    for c in result.chars() {
        match chars_count.get_mut(&c) {
            Some(n) => *n += 1,
            None => {
                chars_count.insert(c, 1);
                ()
            }
        }
    }

    let (mut max, mut min) = (0, MAX);
    chars_count.iter().for_each(|(_, v)| {
        if *v > max {
            max = *v;
        }
        if *v < min {
            min = *v;
        }
    });

    max - min
}

fn parse_instructions(lines: &Vec<&str>) -> (String, Insertion) {
    let template = lines[0].clone();
    let mut inserts = HashMap::new();
    for line in lines.iter().skip(2) {
        let res: Result<_, E> = tuple((anychar, anychar, tag(" -> "), anychar))(*line);

        match res {
            Ok((_, (x, y, _, t))) => {
                inserts.insert((x, y), t);
                ()
            }
            Err(_) => (),
        }
    }

    (template.to_string(), inserts)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        let instructions = parse_instructions(&input.split("\n").collect());
        let solution = solve(&instructions, 40);
        println!("{}", compute_number(&solution));
    } else {
        println!("fail to parse {}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_sample() {
        let input = vec![
            "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
            "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B",
            "CC -> N", "CN -> C",
        ];

        let insts = parse_instructions(&input);
        let res = compute_number(&solve(&insts, 10));

        assert_eq!(res, 1588);
    }
}
