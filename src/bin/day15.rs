use aoc2021::nums::all_neighbours;
use aoc2021::parser::parse_digits;
use std::env;
use std::fs::read_to_string;
use std::process;
use std::thread::sleep;
use std::time::Duration;

fn color_of(cell: &u8) -> u8 {
    16 + 20 * cell
}

fn print_octopuses(nums: &Vec<Vec<u8>>) {
    for row in nums {
        for cell in row {
            let color = color_of(cell);
            print!("\x1b[48;5;{}m \x1b[0m", color);
        }
        println!("");
    }
}

fn solve(nums: &Vec<Vec<u8>>) -> u64 {
    0
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
    fn run_finds_lowest_energy_path() {
        let mut sample: Vec<Vec<u8>> = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];

        let res = solve(&sample);

        assert_eq!(res, 40);
    }
}
