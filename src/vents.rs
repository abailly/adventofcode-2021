use std::fs::read_to_string;

pub struct Pos {
    x: u32,
    y: u32,
}

pub struct Vent {
    from: Pos,
    to: Pos,
}

fn parse_vents(input: &Vec<&str>) -> Option<Vec<Vent>> {
    None
}

pub fn draw_lines(board: &mut Vec<Vec<u8>>, vents: Vec<Vent>) {}

pub fn intersections(board: &Vec<Vec<u8>>) -> u32 {
    0
}

pub fn parse(file: &str) -> Option<(Vec<Vent>, u32, u32)> {
    if let Ok(input) = read_to_string(file) {
        if let Some(vents) = parse_vents(&input.split("\n").collect::<Vec<&str>>()) {
            let (max_x, max_y) = vents.iter().fold((0, 0), |(cur_x, cur_y), vent| (0, 0));
            return Some((vents, max_x, max_y));
        }
    }
    None
}
