use std::fs::read_to_string;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pos {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vent {
    from: Pos,
    to: Pos,
}

pub fn is_ortho(vent: &Vent) -> bool {
    vent.from.x == vent.to.x || vent.from.y == vent.to.y
}

fn parse_vents(input: &Vec<&str>) -> Option<Vec<Vent>> {
    None
}

pub fn draw_lines(board: &mut Vec<Vec<u8>>, vents: Vec<Vent>) {}

pub fn intersections(board: &Vec<Vec<u8>>) -> u32 {
    0
}

fn max_pos(vents: &Vec<Vent>) -> (u32, u32) {
    vents.iter().fold((0, 0), |(cur_x, cur_y), vent| (0, 0))
}

pub fn parse(file: &str) -> Option<(Vec<Vent>, u32, u32)> {
    if let Ok(input) = read_to_string(file) {
        if let Some(vents) = parse_vents(&input.split("\n").collect::<Vec<&str>>()) {
            let (max_x, max_y) = max_pos(&vents);
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
}
