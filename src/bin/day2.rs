fn main() {
    if let Ok(lines) = aoc2021::files::read_lines("./day2/input.txt") {
        let moves = lines.map(|line| aoc2021::parser::parse_move(&line.unwrap()).unwrap());
        let final_pos = moves.fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy));
        println!("{}", final_pos.0 * final_pos.1);
    }
}
