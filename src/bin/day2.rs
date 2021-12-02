use aoc2021::files::read_lines;
use aoc2021::parser::{parse_move, Move};

fn main() {
    if let Ok(lines) = read_lines("./day2/input.txt") {
        let moves = lines.map(|line| parse_move(&line.unwrap()).unwrap());

        let apply_move = |(x, y, a), (mv, dy)| match mv {
            Move::Up => (x, y, a - dy),
            Move::Down => (x, y, a + dy),
            Move::Forward => (x + dy, y + (a * dy), a),
        };

        let final_pos = moves.fold((0, 0, 0), apply_move);
        println!("{}", final_pos.0 * final_pos.1);
    }
}
