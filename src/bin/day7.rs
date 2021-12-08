use aoc2021::nums::bounds;
use aoc2021::nums::sum_of_n;
use aoc2021::parser::parse_csv;
use aoc2021::parser::parse_file;
use core::i64::MAX;
use std::cmp::min;
use std::env;
use std::process;

fn solve(puzzle: &Vec<i64>) -> i64 {
    let (from, to) = bounds(puzzle);
    let mut min_fuel = MAX;
    for i in from..to {
        min_fuel = min(
            min_fuel,
            puzzle.iter().fold(0, |n, j| n + sum_of_n((i - j).abs())),
        );
    }
    min_fuel
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Some(puzzle) = parse_file(&args[1], parse_csv) {
        let solution = solve(&puzzle);
        println!("{}", solution);
    } else {
        println!("fail to parse {}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_sample() {
        let sample = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(solve(&sample), 168);
    }
}
