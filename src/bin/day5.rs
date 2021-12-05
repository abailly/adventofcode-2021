use aoc2021::vents::{draw_lines, intersections, parse};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Some((vents, max_x, max_y)) = parse(&args[1]) {
        let mut board = vec![vec![0; max_x]; max_y];
        draw_lines(&mut board, vents);
        println!("{}", intersections(&board));
    } else {
        println!("fail to parse {}", args[1]);
    }
}
