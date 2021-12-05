use aoc2021::vents::draw_lines;
use aoc2021::vents::intersections;
use aoc2021::vents::parse;
use std::convert::TryInto;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Some((vents, max_x, max_y)) = parse(&args[1]) {
        let mut board = vec![vec![0; max_x.try_into().unwrap()]; max_y.try_into().unwrap()];
        draw_lines(&mut board, vents);
        let num_intersections = intersections(&board);
        println!("{}", num_intersections);
    } else {
        println!("fail to parse {}", args[1]);
    }
}
