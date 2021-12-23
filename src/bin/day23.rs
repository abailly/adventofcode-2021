use crate::Amphipod::*;
use crate::MoveType::*;
use core::u32::MAX;
use num::abs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::convert::TryInto;
use std::env;
use std::process;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Copy)]
enum Amphipod {
    A,
    B,
    C,
    D,
    X,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum MoveType {
    H,
    In(Amphipod),
    F, // orbidden
}

type Pos = [Amphipod; 19];

fn cost(a: Amphipod) -> u32 {
    match a {
        A => 1,
        B => 10,
        C => 100,
        D => 1000,
        X => 0,
    }
}

// #############
// #...........#
// ###D#B#D#B###
//   #C#A#A#C#
//   #########
static legal_moves: [[(MoveType, u8); 19]; 19] = [
    [
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (In(A), 3),
        (In(A), 4),
        (In(B), 5),
        (In(B), 6),
        (In(C), 7),
        (In(C), 8),
        (In(D), 9),
        (In(D), 10),
    ],
    [
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (In(A), 2),
        (In(A), 3),
        (In(B), 4),
        (In(B), 5),
        (In(C), 6),
        (In(C), 7),
        (In(D), 8),
        (In(D), 9),
    ],
    [
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
    ],
    [
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (In(A), 2),
        (In(A), 3),
        (In(B), 2),
        (In(B), 3),
        (In(C), 4),
        (In(C), 5),
        (In(D), 6),
        (In(D), 7),
    ],
    [
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
    ],
    [
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (In(A), 4),
        (In(A), 5),
        (In(B), 2),
        (In(B), 3),
        (In(C), 2),
        (In(C), 3),
        (In(D), 4),
        (In(D), 5),
    ],
    [
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
    ],
    [
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (In(A), 6),
        (In(A), 7),
        (In(B), 4),
        (In(B), 5),
        (In(C), 2),
        (In(C), 3),
        (In(D), 2),
        (In(D), 3),
    ],
    [
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
    ],
    [
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (In(A), 8),
        (In(A), 9),
        (In(B), 6),
        (In(B), 7),
        (In(C), 4),
        (In(C), 5),
        (In(D), 2),
        (In(D), 3),
    ],
    [
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (F, 0),
        (In(A), 9),
        (In(A), 10),
        (In(B), 7),
        (In(B), 8),
        (In(C), 5),
        (In(C), 6),
        (In(D), 3),
        (In(D), 4),
    ],
    [
        (H, 3),
        (H, 2),
        (F, 0),
        (H, 2),
        (F, 0),
        (H, 4),
        (F, 0),
        (H, 6),
        (F, 0),
        (H, 8),
        (H, 9),
        (F, 0),
        (H, 1),
        (In(B), 4),
        (In(B), 5),
        (In(C), 6),
        (In(C), 7),
        (In(D), 8),
        (In(D), 9),
    ],
    [
        (H, 4),
        (H, 3),
        (F, 0),
        (H, 3),
        (F, 0),
        (H, 5),
        (F, 0),
        (H, 7),
        (F, 0),
        (H, 9),
        (H, 10),
        (H, 1),
        (F, 0),
        (In(B), 5),
        (In(B), 6),
        (In(C), 7),
        (In(C), 8),
        (In(D), 9),
        (In(D), 10),
    ],
    [
        (H, 5),
        (H, 4),
        (F, 0),
        (H, 2),
        (F, 0),
        (H, 2),
        (F, 0),
        (H, 4),
        (F, 0),
        (H, 6),
        (H, 7),
        (In(A), 4),
        (In(A), 5),
        (F, 0),
        (H, 1),
        (In(C), 4),
        (In(C), 5),
        (In(D), 6),
        (In(D), 7),
    ],
    [
        (H, 6),
        (H, 5),
        (F, 0),
        (H, 3),
        (F, 0),
        (H, 3),
        (F, 0),
        (H, 5),
        (F, 0),
        (H, 7),
        (H, 8),
        (In(A), 5),
        (In(A), 6),
        (H, 1),
        (F, 0),
        (In(C), 5),
        (In(C), 6),
        (In(D), 7),
        (In(D), 8),
    ],
    [
        (H, 7),
        (H, 6),
        (F, 0),
        (H, 4),
        (F, 0),
        (H, 2),
        (F, 0),
        (H, 2),
        (F, 0),
        (H, 4),
        (H, 5),
        (In(A), 6),
        (In(A), 7),
        (In(B), 4),
        (In(B), 5),
        (F, 0),
        (H, 1),
        (In(D), 4),
        (In(D), 5),
    ],
    [
        (H, 8),
        (H, 7),
        (F, 0),
        (H, 5),
        (F, 0),
        (H, 3),
        (F, 0),
        (H, 3),
        (F, 0),
        (H, 5),
        (H, 6),
        (In(A), 7),
        (In(A), 8),
        (In(B), 5),
        (In(B), 6),
        (H, 1),
        (F, 0),
        (In(D), 5),
        (In(D), 6),
    ],
    [
        (H, 9),
        (H, 8),
        (F, 0),
        (H, 7),
        (F, 0),
        (H, 5),
        (F, 0),
        (H, 2),
        (F, 0),
        (H, 2),
        (H, 3),
        (In(A), 8),
        (In(A), 9),
        (In(B), 6),
        (In(B), 7),
        (In(C), 4),
        (In(C), 5),
        (F, 0),
        (H, 1),
    ],
    [
        (H, 10),
        (H, 9),
        (F, 0),
        (H, 8),
        (F, 0),
        (H, 6),
        (F, 0),
        (H, 3),
        (F, 0),
        (H, 3),
        (H, 4),
        (In(A), 9),
        (In(A), 10),
        (In(B), 7),
        (In(B), 8),
        (In(C), 5),
        (In(D), 6),
        (H, 1),
        (F, 0),
    ],
];

static legal_step: [[i32; 3]; 19] = [
    [1, -1, -1],
    [0, 2, -1],
    [1, 3, 11],
    [2, 4, -1],
    [3, 5, 13],
    [4, 6, -1],
    [5, 7, 15],
    [6, 8, -1],
    [7, 9, 17],
    [8, 10, -1],
    [9, -1, -1],
    [2, 12, -1],
    [11, -1, -1],
    [4, 14, -1],
    [13, -1, -1],
    [6, 16, -1],
    [15, -1, -1],
    [8, 18, -1],
    [17, -1, -1],
];

// #############
// #...........#
// ###D#B#D#B###
//   #C#A#A#C#
//   #########
static distances: [[u32; 19]; 19] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 3, 4, 5, 6, 7, 8, 9, 10], //0
    [1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 2, 3, 4, 5, 6, 7, 8, 9],   // 1
    [2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],   //2
    [3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 2, 3, 2, 3, 4, 5, 6, 7],   // 3
    [4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 3, 4, 1, 2, 3, 4, 5, 6],   //4
    [5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 4, 5, 2, 3, 2, 3, 4, 5],   //5
    [6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 3, 4, 1, 2, 3, 4],   //6
    [7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 6, 7, 4, 5, 2, 3, 2, 3],   // 7
    [8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 7, 8, 5, 6, 3, 4, 1, 2],   // 8
    [9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 8, 9, 6, 7, 4, 5, 2, 3],   // 9
    [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 9, 10, 7, 8, 5, 6, 3, 4], // 10
    [3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 4, 5, 6, 7, 8, 9],   // 11
    [4, 3, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 0, 5, 6, 7, 8, 9, 10], // 12
    [5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 4, 5, 0, 1, 4, 5, 6, 7],   // 13
    [6, 5, 4, 3, 2, 3, 4, 5, 6, 7, 8, 5, 6, 1, 0, 5, 6, 7, 8],   // 14
    [7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 4, 5, 0, 1, 4, 5],   // 15
    [8, 7, 6, 5, 4, 3, 2, 3, 4, 5, 6, 7, 8, 5, 6, 1, 0, 5, 6],   // 16
    [9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 8, 9, 6, 7, 4, 5, 0, 1],   // 17
    [10, 9, 8, 7, 6, 5, 4, 3, 2, 3, 4, 9, 10, 7, 8, 5, 6, 1, 0], // 18
];

fn distance(from: usize, to: usize) -> u32 {
    match legal_moves[from][to] {
        (F, _) => MAX,
        (_, _) => distances[from][to],
    }
}

fn compute_path(from: usize, to: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut cur = from;
    while cur != to {
        for n in legal_step[cur] {
            if n == -1 {
                continue;
            }
            let next = n as usize;
            if distances[next][to] < distances[cur][to] && !res.contains(&next) {
                res.push(next);
                cur = next;
            }
        }
    }
    res
}

/// compute path between any pair of cells
fn compute_all_paths() -> Vec<Vec<Vec<usize>>> {
    let mut res = vec![];
    for i in 0..19 {
        let mut row = vec![];
        for j in 0..19 {
            let cell = match legal_moves[i][j] {
                (F, _) => vec![],
                _ => compute_path(i, j),
            };
            row.push(cell);
        }
        res.push(row);
    }
    res
}

/// Check if there's a path free of amphiboids between the 2 given positions
fn path_is_free(all_paths: &Vec<Vec<Vec<usize>>>, pos: &Pos, from: usize, to: usize) -> bool {
    let path = &all_paths[from][to];

    for p in path {
        if pos[*p] != X {
            return false;
        }
    }

    return true;
}

fn compute_moves(all_paths: &Vec<Vec<Vec<usize>>>, pos: &Pos) -> Vec<(u64, Pos, u32)> {
    use crate::MoveType::*;
    let mut moves = vec![];
    for i in 0..19 {
        let a = pos[i];
        if a != X {
            for j in 0..19 {
                let m = legal_moves[i][j];
                match m {
                    // can't move there
                    (F, _) => (),
                    // hallway move, check move is possible
                    (H, c) => {
                        if path_is_free(all_paths, pos, i, j) {
                            let mut nm = pos.clone();
                            let a = nm[i];
                            nm[j] = a;
                            nm[i] = X;
                            let code = encode(&nm);
                            moves.push((evaluate(&nm), nm, cost(a) * c as u32));
                        }
                    }
                    (In(t), c) => {
                        if path_is_free(all_paths, pos, i, j) && t == a {
                            let mut nm = pos.clone();
                            let a = nm[i];
                            nm[j] = a;
                            nm[i] = X;
                            let code = encode(&nm);
                            moves.push((evaluate(&nm), nm, cost(a) * c as u32));
                        }
                    }
                }
            }
        }
    }
    moves
}

/// encode current position in base 5
fn encode(pos: &Pos) -> u64 {
    let mut code = 0;
    for p in pos {
        let digit = match p {
            X => 0,
            A => 1,
            B => 2,
            C => 3,
            D => 4,
        };
        code = code * 5 + digit;
    }
    code
}

fn evaluate(pos: &Pos) -> u64 {
    encode(pos) - 101724
}

fn is_winning(pos: u64) -> bool {
    pos == 101724
}

/// heuristic function computing minimal distance from given position to
/// goal
fn h(pos: &Pos) -> u32 {
    (0..19).fold(0, |n, i| {
        n + match pos[i] {
            X => 0,
            A => (distances[i][11].min(distances[i][12]) * 1).into(),
            B => (distances[i][13].min(distances[i][14]) * 10).into(),
            C => (distances[i][15].min(distances[i][16]) * 100).into(),
            D => (distances[i][17].min(distances[i][18]) * 1000).into(),
        }
    })
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    val: u32,
    e: u32,
    pos: Pos,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .val
            .cmp(&self.val)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compute_min_steps(
    all_paths: &Vec<Vec<Vec<usize>>>,
    cur_pos: &Pos,
    prev_pos: &mut Vec<u64>,
    min_e: &mut u32,
    energy: u32,
) {
    let mut heap = BinaryHeap::new();
    let mut visited = vec![];
    heap.push(Node {
        val: h(cur_pos),
        e: 0,
        pos: *cur_pos,
    });
    while let Some(Node { val, e, pos }) = heap.pop() {
        let code = encode(&pos);
        if is_winning(code) {
            *min_e = e;
            return;
        }
        let next_moves = compute_moves(all_paths, &pos);
        for (_, nm, ne) in next_moves {
            let ncode = encode(&nm);
            if !visited.contains(&ncode) {
                let newe = e + ne;
                println!("enqueuing {} {} {} {:?}", ncode, newe, h(&nm), nm);
                heap.push(Node {
                    val: newe + h(&nm),
                    e: newe,
                    pos: nm,
                });
            }
        }
        visited.push(code);
    }
}

fn main() {
    let puzzle: [Amphipod; 19] = [X, X, X, X, X, X, X, X, X, X, X, B, A, C, D, B, C, D, A];
    //    let puzzle: [Amphipod; 19] = [X, X, X, X, X, X, X, X, X, X, X, D, B, D, B, C, A, A, C];
    let _winning: [Amphipod; 19] = [X, X, X, X, X, X, X, X, X, X, X, A, A, B, B, C, C, D, D];
    let paths = compute_all_paths();
    let mut path = vec![];
    let mut min_e = MAX;
    compute_min_steps(&paths, &puzzle, &mut path, &mut min_e, 0);
    println!("min energy: {}", min_e);
}
