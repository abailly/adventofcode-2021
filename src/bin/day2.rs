fn main() {
    if let Ok(lines) = aoc2021::files::read_lines("./day2/input.txt") {
        let moves = lines.map(|line| aoc2021::parser::parse_move(&line.unwrap()).unwrap());

        let apply_move = |(x, y, a), (mv, dy)| match mv {
            aoc2021::parser::Move::Up => (x, y, a - dy),
            aoc2021::parser::Move::Down => (x, y, a + dy),
            aoc2021::parser::Move::Forward => (x + dy, y + (a * dy), a),
        };

        let final_pos = moves.fold((0, 0, 0), apply_move);
        println!("{}", final_pos.0 * final_pos.1);
    }
}
