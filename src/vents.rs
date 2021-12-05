use std::cmp::max;
use std::fs::read_to_string;

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

pub fn positions(vent: &Vent) -> Vec<Pos> {
    let mut res = vec![];
    for i in vent.from.x..(vent.to.x + 1) {
        for j in vent.from.y..(vent.to.y + 1) {
            res.push(Pos { x: i, y: j })
        }
    }
    res.clone()
}

pub fn is_ortho(vent: &Vent) -> bool {
    vent.from.x == vent.to.x || vent.from.y == vent.to.y
}

fn parse_vents(input: &Vec<&str>) -> Option<Vec<Vent>> {
    None
}

fn draw_line(board: &mut Vec<Vec<u8>>, vent: &Vent) {
    for pos in positions(vent) {
        board[pos.y][pos.x] = 1;
    }
}

pub fn draw_lines(board: &mut Vec<Vec<u8>>, vents: Vec<Vent>) {
    for vent in vents {
        draw_line(board, &vent);
    }
}

pub fn intersections(board: &Vec<Vec<u8>>) -> u32 {
    0
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

    #[test]
    fn test_filter_orthogonal_vents() {
        let vents = vec![
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

        assert_eq!(
            vents
                .into_iter()
                .filter(is_ortho)
                .collect::<Vec<Vent>>()
                .len(),
            6
        );
    }

    #[test]
    fn test_retrieve_bounds_from_vents() {
        let vents = vec![
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

        assert_eq!(bounds(&vents), (10, 10));
    }

    #[test]
    fn test_can_draw_a_vertical_line() {
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
    fn test_can_draw_a_horizontal_line() {
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
}
