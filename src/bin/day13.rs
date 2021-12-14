use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::env;
use std::fs::read_to_string;
use std::iter::FromIterator;
use std::process;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug, PartialEq, Clone)]
struct Instructions {
    dots: Vec<Vec<u8>>,
    folds: Vec<Fold>,
}

fn fold_up(dots: &Vec<Vec<u8>>, up: usize) -> Vec<Vec<u8>> {
    let mut res: Vec<Vec<u8>> = vec![];
    let len = dots.len();
    for j in 0..len {
        match j.cmp(&up) {
            std::cmp::Ordering::Less => {
                let mut row: Vec<u8> = vec![];
                for i in 0..dots[0].len() {
                    row.push(dots[j][i]);
                }
                res.push(row);
            }
            std::cmp::Ordering::Greater => {
                for i in 0..dots[0].len() {
                    res[2 * up - j][i] += dots[j][i];
                }
            }
            std::cmp::Ordering::Equal => (),
        }
    }
    res
}

fn fold_left(dots: &Vec<Vec<u8>>, left: usize) -> Vec<Vec<u8>> {
    let mut res: Vec<Vec<u8>> = vec![];
    let len = dots.len();
    for j in 0..len {
        let mut row: Vec<u8> = vec![];
        let row_len = dots[0].len();
        for i in 0..row_len {
            match i.cmp(&left) {
                std::cmp::Ordering::Less => {
                    row.push(dots[j][i]);
                }
                std::cmp::Ordering::Greater => {
                    row[2 * left - i] += dots[j][i];
                }
                std::cmp::Ordering::Equal => (),
            }
        }
        res.push(row);
    }
    res
}

fn fold_paper(dots: &Vec<Vec<u8>>, f: Fold) -> Vec<Vec<u8>> {
    match f {
        Fold::Y(up) => fold_up(dots, up),
        Fold::X(left) => fold_left(dots, left),
    }
}

fn count_dots(dots: &Vec<Vec<u8>>) -> u64 {
    dots.iter().fold(0, |n, row| {
        row.iter().fold(n, |k, c| if *c > 0 { 1 + k } else { k })
    })
}
fn display(res: &Vec<Vec<u8>>) {
    for row in res {
        for c in row {
            if *c > 0 {
                print!("\x1b[48;5;5m \x1b[0m");
            } else {
                print!("\x1b[48;5;1m \x1b[0m");
            };
        }
        println!("");
    }
}

fn solve(instructions: &Instructions) -> u64 {
    let mut res = instructions.dots.clone();
    for f in &instructions.folds {
        res = fold_paper(&res, *f);
    }
    display(&res);
    count_dots(&res)
}

fn parse_folds(lines: &Vec<&str>) -> Vec<Fold> {
    let mut res = vec![];
    for line in lines {
        let mut s = (**line).to_string();
        s.replace_range(0..11, "");
        let es: Vec<&str> = s.split("=").collect();
        let f = match es[0] {
            "y" => Fold::Y(es[1].parse::<u32>().unwrap().try_into().unwrap()),
            "x" => Fold::X(es[1].parse::<u32>().unwrap().try_into().unwrap()),
            _ => panic!("not a fold"),
        };
        res.push(f);
    }
    res
}

fn parse_instructions(lines: &Vec<&str>) -> Instructions {
    let dot_pos: Vec<(u32, u32)> = lines
        .iter()
        .filter(|s| !s.starts_with("fold"))
        .map(|l| {
            let pos: Vec<&str> = l.split(",").collect::<Vec<&str>>();
            (
                pos[0].parse::<u32>().unwrap(),
                pos[1].parse::<u32>().unwrap(),
            )
        })
        .collect();
    let (max_x, max_y): (u32, u32) = dot_pos.iter().fold((0, 0), |(mx, my), (x, y)| {
        let nmx = if x > &mx { *x } else { mx };
        let nmy = if y > &my { *y } else { my };
        (nmx, nmy)
    });
    let (lenx, leny) = (max_x as usize + 1, max_y as usize + 1);
    let mut dots: Vec<Vec<u8>> = Vec::with_capacity(leny);
    dots.resize(leny, vec![]);
    for row in 0..leny {
        let mut new_row = Vec::with_capacity(lenx);
        new_row.resize(lenx, 0);
        dots[row as usize] = new_row;
        for c in 0..lenx {
            dots[row as usize][c as usize] = 0;
        }
    }

    dot_pos
        .iter()
        .for_each(|(x, y)| dots[*y as usize][*x as usize] = 1);

    let fold_lines = lines
        .iter()
        .filter(|s| s.starts_with("fold"))
        .map(|l| l.clone())
        .collect();
    let folds = parse_folds(&fold_lines);
    Instructions { dots, folds }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        let instructions =
            parse_instructions(&input.split("\n").filter(|s| !s.is_empty()).collect());
        let solution = solve(&instructions);
        println!("{}", solution);
    } else {
        println!("fail to parse {}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_sample() {
        let input = vec![
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "fold along y=7",
            "fold along x=5",
        ];

        let insts = parse_instructions(&input);
        let res = solve(&insts);

        assert_eq!(res, 17);
    }
}
