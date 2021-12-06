use aoc2021::parser::parse_csv;
use aoc2021::parser::parse_file;
use std::env;
use std::process;

fn evolve(fishes: &mut Vec<i32>, days: u32) {
    for _ in 0..days {
        let mut new_fishes = vec![];
        for i in 0..fishes.len() {
            let f = fishes[i];
            fishes[i] = if f == 0_i32 {
                new_fishes.push(8);
                6
            } else {
                f - 1_i32
            }
        }
        println!("{:?}", new_fishes);
        fishes.append(&mut new_fishes);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Some(lanternfishes) = parse_file(&args[1], parse_csv) {
        let mut fishes = lanternfishes;
        evolve(&mut fishes, 80);
        println!("{}", fishes.len());
    } else {
        println!("fail to parse {}", args[1]);
    }
}
