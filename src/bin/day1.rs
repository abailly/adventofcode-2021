// from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = aoc2021::files::read_lines("./day1/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let depths = lines
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        println!("{}", aoc2021::derivative::count_increasing(&depths));
    }
}
