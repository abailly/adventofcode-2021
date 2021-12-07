use aoc2021::parser::parse_csv;
use aoc2021::parser::parse_file;
use std::env;
use std::process;

fn solve(_puzzle: &Vec<i64>) -> i64 {
    0
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

        assert_eq!(solve(&sample), 37);
    }
}
