use aoc2021::parser::E;
use core::u64::MAX;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::sequence::tuple;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::hash::Hash;
use std::process;

type Insertion = HashMap<(char, char), char>;

fn update<K: Eq + Hash>(map: &mut HashMap<K, u64>, p: K, k: u64) {
    match map.get_mut(&p) {
        Some(n) => *n += k,
        None => {
            map.insert(p, k);
            ()
        }
    }
}

fn solve(instructions: &(String, Insertion), steps: u8) -> HashMap<(char, char), u64> {
    let input = &instructions.0;
    let pairs = &instructions.1;

    let mut res: HashMap<(char, char), u64> = HashMap::new();

    input
        .chars()
        .zip(input[1..].chars())
        .for_each(|p| update(&mut res, p, 1));

    for _ in 0..steps {
        let mut new_res = HashMap::new();
        res.iter().for_each(|(p, v)| {
            if *v > 0 {
                if let Some(c) = pairs.get(p) {
                    update(&mut new_res, (p.0, *c), *v);
                    update(&mut new_res, (*c, p.1), *v);
                } else {
                    new_res.insert(*p, *v);
                }
            }
        });

        res = new_res.clone();
    }

    res
}

fn compute_number(result: &HashMap<(char, char), u64>) -> u64 {
    let mut chars_count: HashMap<char, u64> = HashMap::new();
    for (p, v) in result.iter() {
        update(&mut chars_count, p.0, *v);
        update(&mut chars_count, p.1, *v);
    }

    let (mut max, mut min) = (0, MAX);
    chars_count.iter().map(|(_, v)| v / 2).for_each(|v| {
        if v > max {
            max = v;
        }
        if v < min {
            min = v;
        }
    });

    max - min + 1
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
