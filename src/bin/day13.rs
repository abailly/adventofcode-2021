use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::iter::FromIterator;
use std::process;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug, PartialEq, Clone)]
struct Instructions {
    dots: Vec<Vec<u8>>,
    folds: Vec<Fold>,
}

fn fold_paper(dots: &Vec<Vec<u8>>, f: Fold) -> Vec<Vec<u8>> {
    dots.clone()
}

fn count_dots(dots: &Vec<Vec<u8>>) -> u64 {
    dots.iter()
        .fold(0, |n, row| row.iter().fold(n, |k, c| *c as u64 + k))
}

fn solve(instructions: &Instructions) -> u64 {
    let res = fold_paper(&instructions.dots, instructions.folds[0]);
    count_dots(&res)
}

fn parse_instructions(lines: &Vec<&str>) -> Instructions {
    Instructions {
        dots: vec![],
        folds: vec![],
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        let instructions =
            parse_instructions(&input.split("\n").filter(|s| !s.is_empty()).collect());
        let solution = solve(&instructions);
        println!("{}", solution);
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
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ];

        let insts = parse_instructions(&input);
        let res = solve(&insts);

        assert_eq!(res, 17);
    }
}
