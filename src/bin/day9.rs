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

fn is_low_point(x: &u8, a: &u8, b: &u8, c: &u8, d: &u8) -> bool {
    x < a && x < b && x < c && x < d
}

fn solve(nums: &Vec<(u8, u8, u8, u8, u8)>) -> u64 {
    let mut sum = 0_u64;
    for (x, a, b, c, d) in nums {
        if is_low_point(x, a, b, c, d) {
            sum += 1_u64 + *x as u64;
        }
    }
    sum.into()
}

fn transform(input: Vec<Vec<u8>>) -> Vec<(u8, u8, u8, u8, u8)> {
    let mut output = vec![];
    for (j, row) in input.iter().enumerate() {
        for (i, v) in row.iter().enumerate() {
            let mut t = (*v, 10, 10, 10, 10);
            if i > 0 {
                t.1 = input[j][i - 1];
            }
            if i < row.len() - 1 {
                t.2 = input[j][i + 1];
            }
            if j > 0 {
                t.3 = input[j - 1][i];
            }
            if j < input.len() - 1 {
                t.4 = input[j + 1][i];
            }
            output.push(t);
        }
    }
    output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        if let Some(puzzle) = parse_digits(&input.split("\n").filter(|s| !s.is_empty()).collect()) {
            let solution = solve(&transform(puzzle));
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
    fn can_find_low_points() {
        let sample = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        let res = solve(&transform(sample));

        assert_eq!(res, 15);
    }
}
