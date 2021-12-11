use aoc2021::nums::all_neighbours;
use aoc2021::nums::neighbours;
use std::env;
use std::fs::read_to_string;
use std::process;

fn parse_digits(lines: &Vec<&str>) -> Option<Vec<Vec<u8>>> {
    let mut output = vec![];
    for line in lines {
        let mut row = vec![];
        line.chars().for_each(|c| row.push((c as u8) - ('0' as u8)));
        output.push(row);
    }
    Some(output)
}

fn flashers(octopuses: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut flashers = vec![];
    octopuses.iter().enumerate().for_each(|(j, row)| {
        row.iter().enumerate().for_each(|(i, o)| {
            if *o > 9 {
                flashers.push((i, j));
            }
        })
    });

    flashers
}

fn flash(octopuses: &mut Vec<Vec<u8>>) {
    let mut flashing = flashers(&octopuses);
    let mut flashed = vec![];
    while !flashing.is_empty() {
        if let Some((i, j)) = flashing.pop() {
            let neighbours = all_neighbours(octopuses, (i, j));
            neighbours.iter().for_each(|(x, y)| {
                octopuses[*y][*x] += 1;
                if octopuses[*y][*x] > 9
                    && !flashing.contains(&(*x, *y))
                    && !flashed.contains(&(*x, *y))
                {
                    flashing.push((*x, *y));
                }
            });
            flashed.push((i, j));
        }
    }
}

fn reset(octopuses: &mut Vec<Vec<u8>>) {
    octopuses.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|o| {
            if *o > 9 {
                *o = 0;
            }
        })
    });
}

fn step(octopuses: &mut Vec<Vec<u8>>) {
    octopuses
        .iter_mut()
        .for_each(|row| row.iter_mut().for_each(|o| *o += 1));

    flash(octopuses);

    reset(octopuses);
}

fn count_zero(octopuses: &Vec<Vec<u8>>) -> u64 {
    octopuses.iter().fold(0, |n, row| {
        row.iter().fold(n, |k, o| if *o == 0 { k + 1 } else { k })
    })
}

fn solve(nums: &Vec<Vec<u8>>) -> u64 {
    let mut octopuses = nums.clone();
    for i in 1.. {
        step(&mut octopuses);
        if count_zero(&octopuses) == 100 {
            return i;
        }
    }
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        if let Some(puzzle) = parse_digits(&input.split("\n").filter(|s| !s.is_empty()).collect()) {
            let solution = solve(&puzzle);
            println!("{}", solution);
        }
    } else {
        println!("fail to parse {}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_one_step_of_energy() {
        let mut sample: Vec<Vec<u8>> = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        let expected = vec![
            vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
            vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
            vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
            vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
            vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
            vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
            vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
            vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
            vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
            vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
        ];

        step(&mut sample);

        assert_eq!(sample, expected);
    }

    #[test]
    fn run_two_steps_of_energy() {
        let mut sample: Vec<Vec<u8>> = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        let expected = vec![
            vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
            vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
            vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
            vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
            vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
            vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
            vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
            vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
            vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
            vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
        ];

        step(&mut sample);
        step(&mut sample);

        assert_eq!(sample, expected);
    }
}
