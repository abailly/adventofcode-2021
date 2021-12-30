use core::u64::MAX;
use std::collections::HashSet;
use std::iter::FromIterator;

/// Integral 3D geometry module

/// The type of points and vectors
pub type Point = [i64; 3];

/// Square of L2 distance within 2 points
/// To keep things in integer realm, we don't compute square root
pub fn distance(x: Point, y: Point) -> u64 {
    ((x[1] - y[1]) * (x[1] - y[1]) + (x[0] - y[0]) * (x[0] - y[0]) + (x[2] - y[2]) * (x[2] - y[2]))
        as u64
}

/// L1 (Manhattan) distance between 2 points
pub fn distanceL1(x: Point, y: Point) -> u64 {
    ((x[1] - y[1]).abs() + (x[0] - y[0]).abs() + (x[2] - y[2]).abs()) as u64
}

pub fn minus(a: Point, b: Point) -> Point {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn plus(a: Point, b: Point) -> Point {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub type Matrix = [[i64; 3]; 3];

/// Standard 3D matrix multiplication
pub fn mult(a: &Matrix, b: &Matrix) -> Matrix {
    let mut rot = [[0; 3]; 3];
    for j in 0..3 {
        for i in 0..3 {
            for k in 0..3 {
                rot[j][i] += a[j][k] * b[k][i];
            }
        }
    }
    rot
}

/// From https://stackoverflow.com/questions/33190042/how-to-calculate-all-24-rotations-of-3d-array
/// Compute a list of all rotations of a cube, there are 24 of them
fn define_all_rotations() -> Vec<Matrix> {
    let fam_a = vec![
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
        [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    ];

    let fam_b = vec![
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
        [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
        [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    ];

    let fam_c = vec![
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
    ];

    let mut res = vec![];
    for a in &fam_a {
        for b in &fam_b {
            for c in &fam_c {
                let rot = mult(&mult(a, b), c);
                res.push(rot);
            }
        }
    }
    res
}

/// Apply a rotation to a point
pub trait Rotate {
    fn rotate(&self, pos: Point) -> Point;
}

impl Rotate for Matrix {
    fn rotate(&self, pos: Point) -> Point {
        let mut res = [0; 3];
        for j in 0..3 {
            for i in 0..3 {
                res[j] += pos[j] * self[j][i];
            }
        }
        res
    }
}

pub static ALL_ROTATIONS: [[[i64; 3]; 3]; 24] = [
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
];

/// Transform all points given by the given rotation
fn apply_rotation(a: &HashSet<Point>, rotation: Matrix) -> HashSet<Point> {
    a.iter().map(|p| rotation.rotate(*p)).collect()
}

/// Assuming the points are centered at the same location, compute the rotation
/// of b that maximises the number of equal points.
// pub fn compute_rotation(a: &Vec<Point>, b: &Vec<Point>) -> Option<Matrix> {
//     let mut minimum = MAX;
//     let mut best_rot = 0;
//     for r in 0..24 {
//         let mut dists = 0;
//         let rotated = apply_rotation(b, ALL_ROTATIONS[r]);
//         for j in 0..a.len() {
//             let dist = distance(a[j], rotated[j]);
//             dists += dist;
//         }
//         if dists <= minimum {
//             minimum = dists;
//             best_rot = r;
//         }
//     }
//     if minimum < (a.len() + b.len()) as u64 {
//         None
//     } else {
//         Some(ALL_ROTATIONS[best_rot])
//     }
// }

/// Compute relative vectors between all points
fn compute_vectors(pos: &Vec<Point>) -> Vec<Vec<Point>> {
    let mut res = vec![];
    for j in 0..pos.len() {
        let mut new_row = Vec::with_capacity(pos.len());
        new_row.resize(pos.len(), [0; 3]);
        res.push(new_row);
        for i in j + 1..pos.len() {
            res[j][i] = minus(pos[i], pos[j]);
        }
    }
    res
}

/// Given a pair of vectors matrices, find pairs of vectors with identical
/// coordinates up to a rotation
fn matching_vectors(d1: &HashSet<Point>, d2: &HashSet<Point>) -> Option<(Point, HashSet<Point>)> {
    for r in ALL_ROTATIONS {
        for p1 in d1 {
            let rotd2 = apply_rotation(d2, r);
            for p2 in &rotd2 {
                let offset = minus(*p2, *p1);
                let d3 = rotd2
                    .clone()
                    .into_iter()
                    .map(|b| minus(b, offset))
                    .collect();
                let res: HashSet<Point> = d1.intersection(&d3).map(|p| *p).collect();
                println!("found {} matchings", res.len());
                if res.len() >= 12 {
                    return Some((offset.clone(), d3.clone()));
                }
            }
        }
    }
    None
}

/// Given 2 list of points, match points according to their relative positions with
/// other points
pub fn match_points(from: &HashSet<Point>, to: &HashSet<Point>) -> (Point, HashSet<Point>) {
    let matchings = matching_vectors(&from, &to);

    println!("matchings {:?}", matchings);
    matchings.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn can_identify_points_from_2_scanners() {
        let scanner0 = [
            [404 as i64, -588, -901],
            [528, -643, 409],
            [-838, 591, 734],
            [390, -675, -793],
            [-537, -823, -458],
            [-485, -357, 347],
            [-345, -311, 381],
            [-661, -816, -575],
            [-876, 649, 763],
            [-618, -824, -621],
            [553, 345, -567],
            [474, 580, 667],
            [-447, -329, 318],
            [-584, 868, -557],
            [544, -627, -890],
            [564, 392, -477],
            [455, 729, 728],
            [-892, 524, 684],
            [-689, 845, -530],
            [423, -701, 434],
            [7, -33, -71],
            [630, 319, -379],
            [443, 580, 662],
            [-789, 900, -551],
            [459, -707, 401],
        ];

        let scanner1 = [
            [686 as i64, 422, 578],
            [605, 423, 415],
            [515, 917, -361],
            [-336, 658, 858],
            [95, 138, 22],
            [-476, 619, 847],
            [-340, -569, -846],
            [567, -361, 727],
            [-460, 603, -452],
            [669, -402, 600],
            [729, 430, 532],
            [-500, -761, 534],
            [-322, 571, 750],
            [-466, -666, -811],
            [-429, -592, 574],
            [-355, 545, -477],
            [703, -491, -529],
            [-328, -685, 520],
            [413, 935, -424],
            [-391, 539, -444],
            [586, -435, 557],
            [-364, -763, -893],
            [807, -499, -711],
            [755, -354, -619],
            [553, 889, -390],
        ];

        let mut s0 = HashSet::new();
        s0.extend(scanner0);
        let mut s1 = HashSet::new();
        s1.extend(scanner1);

        let res = match_points(&s0, &s1);

        let expected = [
            ([-618 as i64, -824, -621]),
            ([-537, -823, -458]),
            ([-447, -329, 318]),
            ([404, -588, -901]),
            ([544, -627, -890]),
            ([528, -643, 409]),
            ([-661, -816, -575]),
            ([390, -675, -793]),
            ([423, -701, 434]),
            ([-345, -311, 381]),
            ([459, -707, 401]),
            ([-485, -357, 347]),
        ];

        let mut exp = HashSet::new();
        exp.extend(expected);

        assert!(exp.is_subset(&res.1));
    }
}
