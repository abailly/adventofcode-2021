use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::Err;
use std::convert::TryInto;
use std::fs::read_to_string;

type E<'a> = Err<Error<&'a str>>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cell {
    number: i32,
    drawn: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Board {
    cells: [[Cell; 5]; 5],
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bingo {
    /// The current list of 'random' numbers
    draw: Vec<i32>,
    /// The boards
    boards: Vec<Board>,
}

fn parse_chunk(chunk: &[&str]) -> Board {
    let num = map_res(digit1, |s: &str| s.parse::<i32>());
    let mut nums = separated_list1(space1, num);
    let mut rows = vec![];
    for input in chunk {
        let res: Result<_, E> = nums(&input.trim());
        match res {
            Ok((_, ns)) => rows.push(
                ns.iter()
                    .map(|n| Cell {
                        number: *n,
                        drawn: false,
                    })
                    .collect::<Vec<Cell>>()
                    .try_into()
                    .unwrap(),
            ),
            Err(e) => {
                println!("error {:?}", e);
                panic!("fail to parse input");
            }
        }
    }
    Board {
        cells: rows.try_into().unwrap_or_else(|v: Vec<_>| {
            panic!("Expected a Vec of length {} but it was {}", 5, v.len())
        }),
    }
}

fn parse_boards(input: &Vec<&str>) -> Vec<Board> {
    let lines: Vec<&str> = input
        .iter()
        .filter(|&s| !s.is_empty())
        .map(|&s| s)
        .collect();
    lines.chunks(5).map(|chunk| parse_chunk(chunk)).collect()
}

fn parse_bingo(input: &Vec<&str>) -> Option<Bingo> {
    let num = map_res(digit1, |s: &str| s.parse::<i32>());
    let mut nums = separated_list1(char(','), num);
    let res: Result<_, E> = nums(&input[0]);
    match res {
        Ok((_, ns)) => {
            let bds = parse_boards(&input[2..].into());
            return Some(Bingo {
                draw: ns,
                boards: bds,
            });
        }
        Err(_) => {
            return None;
        }
    };
}

pub fn parse(file: &str) -> Option<Bingo> {
    if let Ok(input) = read_to_string(file) {
        return parse_bingo(&input.split("\n").collect::<Vec<&str>>());
    }
    None
}

pub fn sum_undrawn(board: &Board) -> i32 {
    board.cells.iter().fold(0, |n, row| {
        row.iter()
            .fold(n, |m, cell| if !cell.drawn { cell.number + m } else { m })
    })
}

fn play1(bingo: &mut Bingo) -> i32 {
    println!("play1 {:?}", bingo);
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

    bingo.draw.remove(0);
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

pub fn play(bingo: &mut Bingo) -> (Board, i32) {
    let drawn = play1(bingo);
    if let Some(winning_board) = has_winning_board(&bingo) {
        (winning_board, drawn)
    } else {
        play(bingo)
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

    #[test]
    fn test_parse_boards() {
        assert_eq!(
            parse_bingo(&vec![
                "1,2,3,4",
                "",
                "1 2 3 4 5",
                "1 2 3 4 5",
                "1 2 3 4 5",
                "1 2 3 4 5",
                "1 2 3 4 5",
            ]),
            Some(Bingo {
                draw: vec![1, 2, 3, 4],
                boards: vec![Board {
                    cells: [
                        [
                            Cell {
                                number: 1,
                                drawn: false
                            },
                            Cell {
                                number: 2,
                                drawn: false
                            },
                            Cell {
                                number: 3,
                                drawn: false
                            },
                            Cell {
                                number: 4,
                                drawn: false
                            },
                            Cell {
                                number: 5,
                                drawn: false
                            },
                        ],
                        [
                            Cell {
                                number: 1,
                                drawn: false
                            },
                            Cell {
                                number: 2,
                                drawn: false
                            },
                            Cell {
                                number: 3,
                                drawn: false
                            },
                            Cell {
                                number: 4,
                                drawn: false
                            },
                            Cell {
                                number: 5,
                                drawn: false
                            },
                        ],
                        [
                            Cell {
                                number: 1,
                                drawn: false
                            },
                            Cell {
                                number: 2,
                                drawn: false
                            },
                            Cell {
                                number: 3,
                                drawn: false
                            },
                            Cell {
                                number: 4,
                                drawn: false
                            },
                            Cell {
                                number: 5,
                                drawn: false
                            },
                        ],
                        [
                            Cell {
                                number: 1,
                                drawn: false
                            },
                            Cell {
                                number: 2,
                                drawn: false
                            },
                            Cell {
                                number: 3,
                                drawn: false
                            },
                            Cell {
                                number: 4,
                                drawn: false
                            },
                            Cell {
                                number: 5,
                                drawn: false
                            },
                        ],
                        [
                            Cell {
                                number: 1,
                                drawn: false
                            },
                            Cell {
                                number: 2,
                                drawn: false
                            },
                            Cell {
                                number: 3,
                                drawn: false
                            },
                            Cell {
                                number: 4,
                                drawn: false
                            },
                            Cell {
                                number: 5,
                                drawn: false
                            },
                        ]
                    ]
                }]
            })
        );
    }

    #[test]
    fn test_can_parse_sample_bingo() {
        let input = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ];
        assert_eq!(
            parse_bingo(&input),
            Some(Bingo {
                draw: vec![
                    7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18,
                    20, 8, 19, 3, 26, 1
                ],
                boards: vec![
                    Board {
                        cells: [
                            [
                                Cell {
                                    number: 22,
                                    drawn: false
                                },
                                Cell {
                                    number: 13,
                                    drawn: false
                                },
                                Cell {
                                    number: 17,
                                    drawn: false
                                },
                                Cell {
                                    number: 11,
                                    drawn: false
                                },
                                Cell {
                                    number: 0,
                                    drawn: false
                                }
                            ],
                            [
                                Cell {
                                    number: 8,
                                    drawn: false
                                },
                                Cell {
                                    number: 2,
                                    drawn: false
                                },
                                Cell {
                                    number: 23,
                                    drawn: false
                                },
                                Cell {
                                    number: 4,
                                    drawn: false
                                },
                                Cell {
                                    number: 24,
                                    drawn: false
                                }
                            ],
                            [
                                Cell {
                                    number: 21,
                                    drawn: false
                                },
                                Cell {
                                    number: 9,
                                    drawn: false
                                },
                                Cell {
                                    number: 14,
                                    drawn: false
                                },
                                Cell {
                                    number: 16,
                                    drawn: false
                                },
                                Cell {
                                    number: 7,
                                    drawn: false
                                }
                            ],
                            [
                                Cell {
                                    number: 6,
                                    drawn: false
                                },
                                Cell {
                                    number: 10,
                                    drawn: false
                                },
                                Cell {
                                    number: 3,
                                    drawn: false
                                },
                                Cell {
                                    number: 18,
                                    drawn: false
                                },
                                Cell {
                                    number: 5,
                                    drawn: false
                                }
                            ],
                            [
                                Cell {
                                    number: 1,
                                    drawn: false
                                },
                                Cell {
                                    number: 12,
                                    drawn: false
                                },
                                Cell {
                                    number: 20,
                                    drawn: false
                                },
                                Cell {
                                    number: 15,
                                    drawn: false
                                },
                                Cell {
                                    number: 19,
                                    drawn: false
                                }
                            ]
                        ]
                    },
                    Board {
                        cells: [
                            [
                                Cell {
                                    number: 3,
                                    drawn: false
                                },
                                Cell {
                                    number: 15,
                                    drawn: false
                                },
                                Cell {
                                    number: 0,
                                    drawn: false
                                },
                                Cell {
                                    number: 2,
                                    drawn: false
                                },
                                Cell {
                                    number: 22,
                                    drawn: false
                                }
                            ],
                            [
                                Cell {
                                    number: 9,
                                    drawn: false
                                },
                                Cell {
                                    number: 18,
                                    drawn: false
                                },
                                Cell {
                                    number: 13,
                                    drawn: false
                                },
                                Cell {
                                    number: 17,
                                    drawn: false
                                },
                                Cell {
                                    number: 5,
                                    drawn: false
                                }
                            ],
                            [
                                Cell {
                                    number: 19,
                                    drawn: false
                                },
                                Cell {
                                    number: 8,
                                    drawn: false
                                },
                                Cell {
                                    number: 7,
                                    drawn: false
                                },
                                Cell {
                                    number: 25,
                                    drawn: false
                                },
                                Cell {
                                    number: 23,
                                    drawn: false
                                }
                            ],
                            [
                                Cell {
                                    number: 20,
                                    drawn: false
                                },
                                Cell {
                                    number: 11,
                                    drawn: false
                                },
                                Cell {
                                    number: 10,
                                    drawn: false
                                },
                                Cell {
                                    number: 24,
                                    drawn: false
                                },
                                Cell {
                                    number: 4,
                                    drawn: false
                                }
                            ],
                            [
                                Cell {
                                    number: 14,
                                    drawn: false
                                },
                                Cell {
                                    number: 21,
                                    drawn: false
                                },
                                Cell {
                                    number: 16,
                                    drawn: false
                                },
                                Cell {
                                    number: 12,
                                    drawn: false
                                },
                                Cell {
                                    number: 6,
                                    drawn: false
                                }
                            ]
                        ]
                    },
                    Board {
                        cells: [
                            [
                                Cell {
                                    number: 14,
                                    drawn: false
                                },
                                Cell {
                                    number: 21,
                                    drawn: false
                                },
                                Cell {
                                    number: 17,
                                    drawn: false
                                },
                                Cell {
                                    number: 24,
                                    drawn: false
                                },
                                Cell {
                                    number: 4,
                                    drawn: false
                                }
                            ],
                            [
                                Cell {
                                    number: 10,
                                    drawn: false
                                },
                                Cell {
                                    number: 16,
                                    drawn: false
                                },
                                Cell {
                                    number: 15,
                                    drawn: false
                                },
                                Cell {
                                    number: 9,
                                    drawn: false
                                },
                                Cell {
                                    number: 19,
                                    drawn: false
                                }
                            ],
                            [
                                Cell {
                                    number: 18,
                                    drawn: false
                                },
                                Cell {
                                    number: 8,
                                    drawn: false
                                },
                                Cell {
                                    number: 23,
                                    drawn: false
                                },
                                Cell {
                                    number: 26,
                                    drawn: false
                                },
                                Cell {
                                    number: 20,
                                    drawn: false
                                }
                            ],
                            [
                                Cell {
                                    number: 22,
                                    drawn: false
                                },
                                Cell {
                                    number: 11,
                                    drawn: false
                                },
                                Cell {
                                    number: 13,
                                    drawn: false
                                },
                                Cell {
                                    number: 6,
                                    drawn: false
                                },
                                Cell {
                                    number: 5,
                                    drawn: false
                                }
                            ],
                            [
                                Cell {
                                    number: 2,
                                    drawn: false
                                },
                                Cell {
                                    number: 0,
                                    drawn: false
                                },
                                Cell {
                                    number: 12,
                                    drawn: false
                                },
                                Cell {
                                    number: 3,
                                    drawn: false
                                },
                                Cell {
                                    number: 7,
                                    drawn: false
                                }
                            ]
                        ]
                    }
                ]
            })
        );
    }
}
