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

fn is_low_point(x: &u8, a: &u8, b: &u8, c: &u8, d: &u8) -> bool {
    x < a && x < b && x < c && x < d
}

fn find_basins(
    nums: &Vec<Vec<(u8, u8, u8, u8, u8)>>,
    low_points: &Vec<(usize, usize)>,
) -> Vec<u64> {
    let mut res = vec![];
    for (i, j) in low_points {
        let mut basin = vec![];
        let mut visited = vec![];
        let mut cur_neighbours = vec![(*i, *j)];
        while !cur_neighbours.is_empty() {
            if let Some(cur) = cur_neighbours.pop() {
                let cur_neigh = neighbours(nums, cur);
                for (x, y) in cur_neigh {
                    if nums[y][x].0 != 9
                        && !visited.contains(&(x, y))
                        && !basin.contains(&(x, y))
                        && !cur_neighbours.contains(&(x, y))
                    {
                        cur_neighbours.push((x, y));
                    }
                }
                if nums[cur.1][cur.0].0 != 9 {
                    basin.push(cur);
                }
                visited.push(cur);
            }
        }
        res.push(basin.len() as u64);
    }

    res
}

fn find_low_points(nums: &Vec<Vec<(u8, u8, u8, u8, u8)>>) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for (j, row) in nums.iter().enumerate() {
        for (i, (x, a, b, c, d)) in row.iter().enumerate() {
            if is_low_point(x, a, b, c, d) {
                res.push((i, j));
            }
        }
    }
    res
}

fn solve(nums: &Vec<Vec<(u8, u8, u8, u8, u8)>>) -> u64 {
    let mut res = find_basins(&nums, &find_low_points(&nums));
    res.sort_by(|a, b| b.partial_cmp(a).unwrap());
    res[0..3].iter().fold(1, |n, b| n * b)
}

fn transform(input: Vec<Vec<u8>>) -> Vec<Vec<(u8, u8, u8, u8, u8)>> {
    let mut output = vec![];
    for (j, row) in input.iter().enumerate() {
        let mut new_row = vec![];
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
            new_row.push(t);
        }
        output.push(new_row);
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
    fn can_solve_part_1() {
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

    #[test]
    fn can_find_low_points() {
        let sample = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        let res = find_low_points(&transform(sample));

        assert_eq!(res, vec![(1, 0), (9, 0), (2, 2), (6, 4)]);
    }

    #[test]
    fn can_find_basins() {
        let sample = transform(vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]);

        let res = find_basins(&sample, &find_low_points(&sample));

        assert_eq!(res, vec![3, 9, 14, 9]);
    }
}
