use crate::parser::num;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::error::Error;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use std::cmp::max;
use std::cmp::Ordering;
use std::fs::read_to_string;

type E<'a> = Err<Error<&'a str>>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vent {
    from: Pos,
    to: Pos,
}

fn dir(vent: &Vent) -> (i32, i32) {
    let dx = match vent.to.x.cmp(&vent.from.x) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    let dy = match vent.to.y.cmp(&vent.from.y) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };

    (dx, dy)
}

pub fn positions(vent: &Vent) -> Vec<Pos> {
    let mut res = vec![];
    let (dx, dy) = dir(&vent);
    let mut x = vent.from.x;
    let mut y = vent.from.y;

    while x != vent.to.x || y != vent.to.y {
        res.push(Pos { x, y });
        x = (x as i32 + dx) as usize;
        y = (y as i32 + dy) as usize;
    }

    res.push(vent.to);
    res
}

pub fn is_ortho(vent: &Vent) -> bool {
    vent.from.x == vent.to.x || vent.from.y == vent.to.y
}

fn pos(input: &str) -> IResult<&str, Pos> {
    map(tuple((num, char(','), num)), |(x, _, y)| Pos {
        x: x as usize,
        y: y as usize,
    })(input)
}

fn parse_vent(input: &str) -> Option<Vent> {
    let mut vent = tuple((pos, space1, tag("->"), space1, pos));
    let res: Result<_, E> = vent(&input.trim());

    match res {
        Ok((_, (from, _, _, _, to))) => Some(Vent { from, to }),
        Err(_) => None,
    }
}

fn parse_vents(input: &Vec<&str>) -> Option<Vec<Vent>> {
    input
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| parse_vent(s))
        .collect()
}

fn draw_line(board: &mut Vec<Vec<u8>>, vent: &Vent) {
    for pos in positions(vent) {
        board[pos.y][pos.x] += 1;
    }
}

pub fn draw_lines(board: &mut Vec<Vec<u8>>, vents: Vec<Vent>) {
    for vent in vents {
        draw_line(board, &vent);
    }
}

pub fn intersections(board: &Vec<Vec<u8>>) -> u32 {
    let mut count = 0;
    for row in board {
        for cell in row {
            if *cell >= 2 {
                count += 1;
            }
        }
    }
    count
}

fn bounds(vents: &Vec<Vent>) -> (usize, usize) {
    let (x, y) = vents.iter().fold((0, 0), |(cur_x, cur_y), vent| {
        (
            max(max(cur_x, vent.from.x), vent.to.x),
            max(max(cur_y, vent.from.y), vent.to.y),
        )
    });
    (x + 1, y + 1)
}

pub fn parse(file: &str) -> Option<(Vec<Vent>, usize, usize)> {
    if let Ok(input) = read_to_string(file) {
        if let Some(vents) = parse_vents(&input.split("\n").collect::<Vec<&str>>()) {
            let (max_x, max_y) = bounds(&vents);
            return Some((vents, max_x, max_y));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_VENTS: [Vent; 10] = [
        Vent {
            from: Pos { x: 0, y: 9 },
            to: Pos { x: 5, y: 9 },
        },
        Vent {
            from: Pos { x: 8, y: 0 },
            to: Pos { x: 0, y: 8 },
        },
        Vent {
            from: Pos { x: 9, y: 4 },
            to: Pos { x: 3, y: 4 },
        },
        Vent {
            from: Pos { x: 2, y: 2 },
            to: Pos { x: 2, y: 1 },
        },
        Vent {
            from: Pos { x: 7, y: 0 },
            to: Pos { x: 7, y: 4 },
        },
        Vent {
            from: Pos { x: 6, y: 4 },
            to: Pos { x: 2, y: 0 },
        },
        Vent {
            from: Pos { x: 0, y: 9 },
            to: Pos { x: 2, y: 9 },
        },
        Vent {
            from: Pos { x: 3, y: 4 },
            to: Pos { x: 1, y: 4 },
        },
        Vent {
            from: Pos { x: 0, y: 0 },
            to: Pos { x: 8, y: 8 },
        },
        Vent {
            from: Pos { x: 5, y: 5 },
            to: Pos { x: 8, y: 2 },
        },
    ];

    #[test]
    fn filter_orthogonal_vents() {
        let sample_vents = SAMPLE_VENTS.to_vec();
        assert_eq!(
            sample_vents
                .into_iter()
                .filter(is_ortho)
                .collect::<Vec<Vent>>()
                .len(),
            6
        );
    }

    #[test]
    fn retrieve_bounds_from_vents() {
        assert_eq!(bounds(&SAMPLE_VENTS.to_vec()), (10, 10));
    }

    #[test]
    fn can_draw_a_vertical_line() {
        let vents = vec![Vent {
            from: Pos { x: 1, y: 0 },
            to: Pos { x: 1, y: 3 },
        }];
        let mut board = vec![vec![0; 4]; 4];

        draw_lines(&mut board, vents);

        assert_eq!(
            board,
            [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]]
        );
    }

    #[test]
    fn can_draw_a_horizontal_line() {
        let vents = vec![Vent {
            from: Pos { x: 0, y: 3 },
            to: Pos { x: 3, y: 3 },
        }];
        let mut board = vec![vec![0; 4]; 4];

        draw_lines(&mut board, vents);

        assert_eq!(
            board,
            [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1]]
        );
    }

    #[test]
    fn can_draw_a_horizontal_line_inverted() {
        let vents = vec![Vent {
            from: Pos { x: 3, y: 3 },
            to: Pos { x: 0, y: 3 },
        }];
        let mut board = vec![vec![0; 4]; 4];

        draw_lines(&mut board, vents);

        assert_eq!(
            board,
            [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1]]
        );
    }

    #[test]
    fn can_compute_intersection() {
        let mut board = vec![vec![0; 10]; 10];

        draw_lines(
            &mut board,
            SAMPLE_VENTS.to_vec().into_iter().filter(is_ortho).collect(),
        );

        assert_eq!(intersections(&board), 5);
    }

    #[test]
    fn can_parse_vents_from_a_vec_of_strings() {
        let input = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];

        let vents = parse_vents(&input);

        assert_eq!(vents, Some(SAMPLE_VENTS.to_vec()));
    }
}
