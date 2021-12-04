use aoc2021::files::read_lines;
use aoc2021::parser::{parse_bits, to_int, Bit};
use core::fmt::Error;
use std::cmp::Ordering;
use std::env;
use std::process;

struct Cell<'a> {
    number: i32,
    drawn: &'a mut bool,
}

struct Board<'a> {
    cells: [[&'a mut Cell<'a>; 5]; 5],
}

struct Bingo<'a> {
    /// The current list of 'random' numbers
    draw: Vec<i32>,
    /// The boards
    boards: Vec<Board<'a>>,
}

fn parse(file: &str) -> Option<Bingo> {
    None
}

fn sum_undrawn(board: &Board) -> i32 {
    0
}

fn play<'a>(bingo: &'a Bingo<'a>) -> (&'a Board<'a>, i32) {
    (&bingo.boards[0], 0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Some(bingo) = parse(&args[1]) {
        let (winning_board, last_drawn_number) = play(&bingo);
        let sum_of_undrawn = sum_undrawn(winning_board);
        println!("{}", sum_of_undrawn * last_drawn_number);
    } else {
        println!("fail to parse {}", args[1]);
    }
}
