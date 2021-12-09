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
