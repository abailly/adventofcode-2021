use std::cmp::Ordering::Less;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day1/input.txt") {
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
        let windows_tail = &windows_length[1..];
        let num_increasing = windows_length
            .iter()
            .zip(windows_tail.iter())
            .map(|(a, b)| a.cmp(b))
            .filter(|ord| *ord == Less)
            .count();
        println!("{}", num_increasing);
    }
}

/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
