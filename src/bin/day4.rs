use aoc2021::bingo::{parse, play, sum_undrawn};
use core::fmt::Error;
use std::cmp::Ordering;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Some(mut bingo) = parse(&args[1]) {
        if let Some((winning_board, last_drawn_number)) = play(&mut bingo) {
            let sum_of_undrawn = sum_undrawn(&winning_board);
            println!("{}", sum_of_undrawn * last_drawn_number);
        }
    } else {
        println!("fail to parse {}", args[1]);
    }
}
