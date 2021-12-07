use num::FromPrimitive;
use num::Num;
use std::cmp::max;
use std::cmp::min;

pub fn bounds<N: Num + Ord + Copy>(vents: &Vec<N>) -> (N, N) {
    let (x, y) = vents
        .iter()
        .fold((N::zero(), N::zero()), |(cur_x, cur_y), vent| {
            (min(cur_x, *vent), max(cur_y, *vent))
        });
    (x, y + N::one())
}

pub fn sum_of_n<N: Num + FromPrimitive + Copy>(n: N) -> N {
    n * (n + N::one()) / FromPrimitive::from_u32(2).unwrap()
}
