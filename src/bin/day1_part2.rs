// from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = aoc2021::files::read_lines("./day1/input.txt") {
        let depths = lines
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let shift1 = &depths[1..];
        let shift2 = &shift1[1..];
        let windows = depths
            .iter()
            .zip(shift1.iter())
            .zip(shift2.iter())
            .collect::<Vec<_>>();
        let windows_length = windows
            .iter()
            .map(|((a, b), c)| *a + *b + *c)
            .collect::<Vec<_>>();
        let num_increasing = aoc2021::derivative::count_increasing(&windows_length);
        println!("{}", num_increasing);
    }
}
