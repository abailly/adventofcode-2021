use aoc2021::parser::parse_csv;
use aoc2021::parser::parse_file;
use std::env;
use std::process;

fn evolve(fishes: &mut [u64; 9], days: u32) {
    for _ in 0..days {
        let mut new_fishes = [0_u64; 9];
        println!("{:?}", fishes);
        for i in 1..fishes.len() {
            new_fishes[i - 1] = fishes[i];
        }
        new_fishes[8] = fishes[0];
        new_fishes[6] += fishes[0];
        for i in 0..fishes.len() {
            fishes[i] = new_fishes[i];
        }
    }
}

fn fill(fishes: &mut [u64; 9], lanternfishes: Vec<i32>) {
    for i in 0..fishes.len() {
        fishes[i] = 0;
    }

    lanternfishes.iter().for_each(|f| fishes[*f as usize] += 1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    let mut fishes = [0_u64; 9];

    if let Some(lanternfishes) = parse_file(&args[1], parse_csv) {
        fill(&mut fishes, lanternfishes);
        evolve(&mut fishes, 256);
        println!("{}", fishes.iter().fold(0, |n, i| n + i));
    } else {
        println!("fail to parse {}", args[1]);
    }
}
