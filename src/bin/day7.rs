use aoc2021::parser::parse_file;
use std::env;
use std::process;

type Puzzle = ();

fn parse_puzzle(input: &Vec<&str>) -> Option<Puzzle> {
    None
}

fn solve(puzzle: &Puzzle) -> i32 {
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Some(puzzle) = parse_file(&args[1], parse_puzzle) {
        let solution = solve(&puzzle);
        println!("{}", solution);
    } else {
        println!("fail to parse {}", args[1]);
    }
}
