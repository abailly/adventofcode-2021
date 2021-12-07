use aoc2021::parser::parse_csv;
use aoc2021::parser::parse_file;
use std::cmp::max;
use std::cmp::min;
use std::env;
use std::process;

type Puzzle = ();

fn bounds(vents: &Vec<i32>) -> (i32, i32) {
    let (x, y) = vents.iter().fold((0, 0), |(cur_x, cur_y), vent| {
        (min(cur_x, *vent), max(cur_y, *vent))
    });
    (x, y + 1)
}

fn solve(puzzle: &Vec<i32>) -> i32 {
    let (from, to) = bounds(puzzle);
    let mut min_fuel = to * puzzle.len() as i32;
    for i in from..to {
        min_fuel = min(min_fuel, puzzle.iter().fold(0, |n, j| n + (i - j).abs()));
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

        assert_eq!(solve(&sample), 37);
    }
}
