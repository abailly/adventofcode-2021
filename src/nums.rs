use num::Bounded;
use num::FromPrimitive;
use num::Num;
use std::cmp::max;
use std::cmp::min;

/// Provide the minimum and maximum values in some list of values
pub fn bounds<N: Num + Ord + Copy>(vents: &Vec<N>) -> (N, N) {
    let (x, y) = vents
        .iter()
        .fold((N::zero(), N::zero()), |(cur_x, cur_y), vent| {
            (min(cur_x, *vent), max(cur_y, *vent))
        });
    (x, y + N::one())
}

/// Compute the sum of numbers from 1 to 'n'
pub fn sum_of_n<N: Num + FromPrimitive + Copy>(n: N) -> N {
    n * (n + N::one()) / FromPrimitive::from_u32(2).unwrap()
}

/// Retrieve the neighbours of some cell on a square board
pub fn neighbours<T>(nums: &Vec<Vec<T>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let (i, j) = pos;
    if i > 0 {
        res.push((i - 1, j));
    }
    if i < nums[0].len() - 1 {
        res.push((i + 1, j));
    }
    if j > 0 {
        res.push((i, (j - 1)));
    }
    if j < nums.len() - 1 {
        res.push((i, (j + 1)));
    }
    res
}

/// Retrieve all neighbours (including diagonals) of some cell on a square board
pub fn all_neighbours<T>(nums: &Vec<Vec<T>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let (i, j) = pos;

    if i > 0 {
        res.push((i - 1, j));
        if j > 0 {
            res.push((i - 1, (j - 1)));
        }
        if j < nums.len() - 1 {
            res.push((i - 1, (j + 1)));
        }
    }
    if i < nums[0].len() - 1 {
        res.push((i + 1, j));
        if j > 0 {
            res.push((i + 1, (j - 1)));
        }
        if j < nums.len() - 1 {
            res.push((i + 1, (j + 1)));
        }
    }
    if j > 0 {
        res.push((i, (j - 1)));
    }
    if j < nums.len() - 1 {
        res.push((i, (j + 1)));
    }
    res
}

/// Given a matrix of numbers, return the same matrix but with the neighbouring values.
pub fn transform<N: Num + Ord + Bounded + Copy>(input: Vec<Vec<N>>) -> Vec<Vec<(N, N, N, N, N)>> {
    let mut output = vec![];
    for (j, row) in input.iter().enumerate() {
        let mut new_row = vec![];
        for (i, v) in row.iter().enumerate() {
            let mut t = (
                *v,
                N::max_value(),
                N::max_value(),
                N::max_value(),
                N::max_value(),
            );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_all_neighbours() {
        let sample: Vec<Vec<u8>> = vec![
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

        let mut res = all_neighbours(&sample, (0, 0));
        res.sort();

        assert_eq!(res, vec![(0, 1), (1, 0), (1, 1)]);

        res = all_neighbours(&sample, (0, 3));
        res.sort();

        assert_eq!(res, vec![(0, 2), (0, 4), (1, 2), (1, 3), (1, 4)]);

        res = all_neighbours(&sample, (2, 3));
        res.sort();

        assert_eq!(
            res,
            vec![
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 2),
                (2, 4),
                (3, 2),
                (3, 3),
                (3, 4)
            ]
        );
    }
}
