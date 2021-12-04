use aoc2021::files::read_lines;
use aoc2021::parser::{parse_bits, to_int, Bit};
use core::fmt::Error;
use std::cmp::Ordering;
use std::env;
use std::process;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Cell {
    number: i32,
    drawn: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Board {
    cells: [[Cell; 5]; 5],
}

#[derive(Debug, PartialEq, Clone)]
struct Bingo {
    /// The current list of 'random' numbers
    draw: Vec<i32>,
    /// The boards
    boards: Vec<Board>,
}

fn parse(file: &str) -> Option<Bingo> {
    None
}

fn sum_undrawn(board: &Board) -> i32 {
    0
}

fn play1(bingo: &mut Bingo) -> i32 {
    let drawn = bingo.draw[0];

    let apply_drawn = |board: &mut Board| {
        for row in board.cells.iter_mut() {
            for mut cell in row.iter_mut() {
                if cell.number == drawn {
                    println!("found drawn");
                    cell.drawn = true;
                    println!("cell {:?}", cell);
                }
            }
        }
    };

    for mut board in bingo.boards.iter_mut() {
        apply_drawn(&mut board);
        println!("board {:?}", board);
    }

    println!("bingo {:?}", bingo);
    bingo.draw.pop();
    drawn
}

fn play(bingo: Bingo) -> (Board, i32) {
    (bingo.boards[0], 0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Some(bingo) = parse(&args[1]) {
        let (winning_board, last_drawn_number) = play(bingo);
        let sum_of_undrawn = sum_undrawn(&winning_board);
        println!("{}", sum_of_undrawn * last_drawn_number);
    } else {
        println!("fail to parse {}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_updates_boards() {
        let ten = Cell {
            number: 10,
            drawn: false,
        };
        let twelve = Cell {
            number: 12,
            drawn: false,
        };
        let board = Board {
            cells: [
                [ten; 5],
                [ten; 5],
                [ten; 5],
                [ten; 5],
                [ten, ten, ten, twelve, ten],
            ],
        };
        let draw = vec![12; 1];
        let mut bingo = Bingo {
            draw: draw,
            boards: vec![board],
        };

        let drawn = play1(&mut bingo);

        println!("bingo {:?}", bingo);

        assert_eq!(bingo.draw.len(), 0);
        assert_eq!(drawn, 12);
        assert_eq!(bingo.boards[0].cells[4][3].drawn, true);
    }
}
