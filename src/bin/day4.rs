use aoc2021::files::read_lines;
use aoc2021::parser::{parse_bits, to_int, Bit};
use core::fmt::Error;
use std::cmp::Ordering;
use std::env;
use std::fs::read_to_string;
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
    if let Ok(input) = read_to_string(file) {}
    None
}

fn sum_undrawn(board: &Board) -> i32 {
    board
        .cells
        .iter()
        .fold(0, |n, row| row.iter().fold(n, |m, cell| cell.number + m))
}

fn play1(bingo: &mut Bingo) -> i32 {
    let drawn = bingo.draw[0];

    let apply_drawn = |board: &mut Board| {
        for row in board.cells.iter_mut() {
            for mut cell in row.iter_mut() {
                if cell.number == drawn {
                    cell.drawn = true;
                }
            }
        }
    };

    for mut board in bingo.boards.iter_mut() {
        apply_drawn(&mut board);
    }

    bingo.draw.pop();
    drawn
}

fn is_winning(board: &Board) -> bool {
    for row in board.cells {
        if row.iter().fold(true, |acc, cell| cell.drawn && acc) {
            return true;
        }
    }

    for i in 0..4 {
        if board
            .cells
            .iter()
            .fold(true, |acc, row| row[i].drawn && acc)
        {
            return true;
        }
    }

    false
}

fn has_winning_board(bingo: &Bingo) -> Option<Board> {
    for board in &bingo.boards {
        if is_winning(board) {
            return Some(*board);
        }
    }
    None
}

fn play(bingo: &mut Bingo) -> (Board, i32) {
    let drawn = play1(bingo);
    if let Some(winning_board) = has_winning_board(&bingo) {
        (winning_board, drawn)
    } else {
        play(bingo)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Some(mut bingo) = parse(&args[1]) {
        let (winning_board, last_drawn_number) = play(&mut bingo);
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

        assert_eq!(bingo.draw.len(), 0);
        assert_eq!(drawn, 12);
        assert_eq!(bingo.boards[0].cells[4][3].drawn, true);
    }

    #[test]
    fn test_has_winning_board_when_full_row_has_true() {
        let loss = Cell {
            number: 10,
            drawn: false,
        };
        let win = Cell {
            number: 12,
            drawn: true,
        };
        let winning = Board {
            cells: [[loss; 5], [loss; 5], [loss; 5], [win; 5], [loss; 5]],
        };

        let not_winning = Board {
            cells: [[loss; 5], [loss; 5], [loss; 5], [loss; 5], [loss; 5]],
        };

        let bingo = Bingo {
            draw: vec![],
            boards: vec![winning, not_winning],
        };

        let some_winning = has_winning_board(&bingo);

        assert_eq!(some_winning, Some(winning));
    }

    #[test]
    fn test_has_winning_board_when_full_column_has_true() {
        let loss = Cell {
            number: 10,
            drawn: false,
        };
        let win = Cell {
            number: 12,
            drawn: true,
        };
        let winning = Board {
            cells: [
                [win, loss, loss, loss, loss],
                [win, loss, loss, loss, loss],
                [win, loss, loss, loss, loss],
                [win, loss, loss, loss, loss],
                [win, loss, loss, loss, loss],
            ],
        };

        let not_winning = Board {
            cells: [[loss; 5], [loss; 5], [loss; 5], [loss; 5], [loss; 5]],
        };

        let bingo = Bingo {
            draw: vec![],
            boards: vec![winning, not_winning],
        };

        let some_winning = has_winning_board(&bingo);

        assert_eq!(some_winning, Some(winning));
    }
}
