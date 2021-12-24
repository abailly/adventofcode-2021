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

type Pos = [Amphipod; 27];

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
//   #C#A#A#C#
//   #C#A#A#C#
//   #########
static legal_moves: [[(MoveType, u32); 27]; 27] = [
    [
        (F, 0),
        (F, 1),
        (F, 2),
        (F, 3),
        (F, 4),
        (F, 5),
        (F, 6),
        (F, 7),
        (F, 8),
        (F, 9),
        (F, 10),
        (In(A), 3),
        (In(A), 4),
        (In(A), 5),
        (In(A), 6),
        (In(B), 5),
        (In(B), 6),
        (In(B), 7),
        (In(B), 8),
        (In(C), 7),
        (In(C), 8),
        (In(C), 9),
        (In(C), 10),
        (In(D), 9),
        (In(D), 10),
        (In(D), 11),
        (In(D), 12),
    ],
    [
        (F, 1),
        (F, 0),
        (F, 1),
        (F, 2),
        (F, 3),
        (F, 4),
        (F, 5),
        (F, 6),
        (F, 7),
        (F, 8),
        (F, 9),
        (In(A), 2),
        (In(A), 3),
        (In(A), 4),
        (In(A), 5),
        (In(B), 4),
        (In(B), 5),
        (In(B), 6),
        (In(B), 7),
        (In(C), 6),
        (In(C), 7),
        (In(C), 8),
        (In(C), 9),
        (In(D), 8),
        (In(D), 9),
        (In(D), 10),
        (In(D), 11),
    ],
    [
        (F, 2),
        (F, 1),
        (F, 0),
        (F, 1),
        (F, 2),
        (F, 3),
        (F, 4),
        (F, 5),
        (F, 6),
        (F, 7),
        (F, 8),
        (In(A), 1),
        (In(A), 2),
        (In(A), 3),
        (In(A), 4),
        (In(B), 3),
        (In(B), 4),
        (In(B), 5),
        (In(B), 6),
        (In(C), 5),
        (In(C), 6),
        (In(C), 7),
        (In(C), 8),
        (In(D), 7),
        (In(D), 8),
        (In(D), 9),
        (In(D), 10),
    ],
    [
        (F, 3),
        (F, 2),
        (F, 1),
        (F, 0),
        (F, 1),
        (F, 2),
        (F, 3),
        (F, 4),
        (F, 5),
        (F, 6),
        (F, 7),
        (In(A), 2),
        (In(A), 3),
        (In(A), 4),
        (In(A), 5),
        (In(B), 2),
        (In(B), 3),
        (In(B), 4),
        (In(B), 5),
        (In(C), 4),
        (In(C), 5),
        (In(C), 6),
        (In(C), 7),
        (In(D), 6),
        (In(D), 7),
        (In(D), 8),
        (In(D), 9),
    ],
    [
        (F, 4),
        (F, 3),
        (F, 2),
        (F, 1),
        (F, 0),
        (F, 1),
        (F, 2),
        (F, 3),
        (F, 4),
        (F, 5),
        (F, 6),
        (In(A), 3),
        (In(A), 4),
        (In(A), 5),
        (In(A), 6),
        (In(B), 1),
        (In(B), 2),
        (In(B), 3),
        (In(B), 4),
        (In(C), 3),
        (In(C), 4),
        (In(C), 5),
        (In(C), 6),
        (In(D), 5),
        (In(D), 6),
        (In(D), 7),
        (In(D), 8),
    ],
    [
        (F, 6),
        (F, 4),
        (F, 3),
        (F, 2),
        (F, 1),
        (F, 0),
        (F, 1),
        (F, 2),
        (F, 3),
        (F, 4),
        (F, 5),
        (In(A), 4),
        (In(A), 5),
        (In(A), 6),
        (In(A), 7),
        (In(B), 2),
        (In(B), 3),
        (In(B), 4),
        (In(B), 5),
        (In(C), 2),
        (In(C), 3),
        (In(C), 4),
        (In(C), 5),
        (In(D), 4),
        (In(D), 5),
        (In(D), 6),
        (In(D), 7),
    ],
    [
        (F, 7),
        (F, 6),
        (F, 4),
        (F, 3),
        (F, 2),
        (F, 1),
        (F, 0),
        (F, 1),
        (F, 2),
        (F, 3),
        (F, 4),
        (In(A), 5),
        (In(A), 6),
        (In(A), 7),
        (In(A), 8),
        (In(B), 3),
        (In(B), 4),
        (In(B), 5),
        (In(B), 6),
        (In(C), 1),
        (In(C), 2),
        (In(C), 3),
        (In(C), 4),
        (In(D), 3),
        (In(D), 4),
        (In(D), 5),
        (In(D), 6),
    ],
    [
        (F, 8),
        (F, 7),
        (F, 6),
        (F, 4),
        (F, 3),
        (F, 2),
        (F, 1),
        (F, 0),
        (F, 1),
        (F, 2),
        (F, 3),
        (In(A), 6),
        (In(A), 7),
        (In(A), 8),
        (In(A), 9),
        (In(B), 4),
        (In(B), 5),
        (In(B), 6),
        (In(B), 7),
        (In(C), 2),
        (In(C), 3),
        (In(C), 4),
        (In(C), 5),
        (In(D), 2),
        (In(D), 3),
        (In(D), 4),
        (In(D), 5),
    ],
    [
        (F, 9),
        (F, 8),
        (F, 7),
        (F, 6),
        (F, 4),
        (F, 3),
        (F, 2),
        (F, 1),
        (F, 0),
        (F, 1),
        (F, 2),
        (In(A), 7),
        (In(A), 8),
        (In(A), 9),
        (In(A), 10),
        (In(B), 5),
        (In(B), 6),
        (In(B), 7),
        (In(B), 8),
        (In(C), 3),
        (In(C), 4),
        (In(C), 5),
        (In(C), 6),
        (In(D), 1),
        (In(D), 2),
        (In(D), 3),
        (In(D), 4),
    ],
    [
        (F, 10),
        (F, 9),
        (F, 8),
        (F, 7),
        (F, 6),
        (F, 4),
        (F, 3),
        (F, 2),
        (F, 1),
        (F, 0),
        (F, 1),
        (In(A), 8),
        (In(A), 9),
        (In(A), 10),
        (In(A), 11),
        (In(B), 6),
        (In(B), 7),
        (In(B), 8),
        (In(B), 9),
        (In(C), 4),
        (In(C), 5),
        (In(C), 6),
        (In(C), 7),
        (In(D), 2),
        (In(D), 3),
        (In(D), 4),
        (In(D), 5),
    ],
    [
        (F, 11),
        (F, 10),
        (F, 9),
        (F, 8),
        (F, 7),
        (F, 6),
        (F, 4),
        (F, 3),
        (F, 2),
        (F, 1),
        (F, 0),
        (In(A), 9),
        (In(A), 10),
        (In(A), 11),
        (In(A), 12),
        (In(B), 7),
        (In(B), 8),
        (In(B), 9),
        (In(B), 10),
        (In(C), 5),
        (In(C), 6),
        (In(C), 7),
        (In(C), 8),
        (In(D), 3),
        (In(D), 4),
        (In(D), 5),
        (In(D), 6),
    ],
    [
        (H, 3),
        (H, 2),
        (F, 1),
        (H, 2),
        (F, 3),
        (H, 4),
        (F, 5),
        (H, 6),
        (F, 7),
        (H, 8),
        (H, 9),
        (F, 0),
        (H, 1),
        (H, 2),
        (H, 3),
        (In(B), 4),
        (In(B), 5),
        (In(B), 6),
        (In(B), 7),
        (In(C), 6),
        (In(C), 7),
        (In(C), 8),
        (In(C), 9),
        (In(D), 8),
        (In(D), 9),
        (In(D), 10),
        (In(D), 11),
    ],
    [
        (H, 4),
        (H, 3),
        (F, 2),
        (H, 3),
        (F, 4),
        (H, 5),
        (F, 6),
        (H, 7),
        (F, 8),
        (H, 9),
        (H, 10),
        (H, 1),
        (F, 0),
        (H, 1),
        (H, 2),
        (In(B), 5),
        (In(B), 6),
        (In(B), 7),
        (In(B), 8),
        (In(C), 7),
        (In(C), 8),
        (In(C), 9),
        (In(C), 10),
        (In(D), 9),
        (In(D), 10),
        (In(D), 11),
        (In(D), 12),
    ],
    [
        (H, 5),
        (H, 4),
        (F, 3),
        (H, 4),
        (F, 5),
        (H, 6),
        (F, 7),
        (H, 8),
        (F, 9),
        (H, 10),
        (H, 11),
        (H, 2),
        (H, 1),
        (F, 0),
        (H, 1),
        (In(B), 6),
        (In(B), 7),
        (In(B), 8),
        (In(B), 9),
        (In(C), 8),
        (In(C), 9),
        (In(C), 10),
        (In(C), 11),
        (In(D), 10),
        (In(D), 11),
        (In(D), 12),
        (In(D), 13),
    ],
    [
        (H, 6),
        (H, 5),
        (F, 4),
        (H, 5),
        (F, 6),
        (H, 7),
        (F, 8),
        (H, 9),
        (F, 10),
        (H, 11),
        (H, 12),
        (H, 3),
        (H, 2),
        (H, 1),
        (F, 0),
        (In(B), 7),
        (In(B), 8),
        (In(B), 9),
        (In(B), 10),
        (In(C), 9),
        (In(C), 10),
        (In(C), 11),
        (In(C), 12),
        (In(D), 11),
        (In(D), 12),
        (In(D), 13),
        (In(D), 14),
    ],
    //B
    [
        (H, 5),
        (H, 4),
        (F, 3),
        (H, 2),
        (F, 1),
        (H, 2),
        (F, 3),
        (H, 4),
        (F, 5),
        (H, 6),
        (H, 7),
        (In(A), 4),
        (In(A), 5),
        (In(A), 6),
        (In(A), 7),
        (F, 0),
        (H, 1),
        (H, 2),
        (H, 3),
        (In(C), 4),
        (In(C), 5),
        (In(C), 6),
        (In(C), 7),
        (In(D), 6),
        (In(D), 7),
        (In(D), 8),
        (In(D), 9),
    ],
    [
        (H, 6),
        (H, 5),
        (F, 4),
        (H, 3),
        (F, 2),
        (H, 3),
        (F, 4),
        (H, 5),
        (F, 6),
        (H, 7),
        (H, 8),
        (In(A), 5),
        (In(A), 6),
        (In(A), 7),
        (In(A), 8),
        (H, 1),
        (F, 0),
        (H, 1),
        (H, 2),
        (In(C), 5),
        (In(C), 6),
        (In(C), 7),
        (In(C), 8),
        (In(D), 7),
        (In(D), 8),
        (In(D), 9),
        (In(D), 10),
    ],
    [
        (H, 7),
        (H, 6),
        (F, 5),
        (H, 4),
        (F, 3),
        (H, 4),
        (F, 5),
        (H, 6),
        (F, 7),
        (H, 8),
        (H, 9),
        (In(A), 6),
        (In(A), 7),
        (In(A), 8),
        (In(A), 9),
        (H, 2),
        (H, 1),
        (F, 0),
        (H, 1),
        (In(C), 6),
        (In(C), 7),
        (In(C), 8),
        (In(C), 9),
        (In(D), 8),
        (In(D), 9),
        (In(D), 10),
        (In(D), 11),
    ],
    [
        (H, 8),
        (H, 7),
        (F, 6),
        (H, 5),
        (F, 4),
        (H, 5),
        (F, 6),
        (H, 7),
        (F, 8),
        (H, 9),
        (H, 10),
        (In(A), 7),
        (In(A), 8),
        (In(A), 9),
        (In(A), 10),
        (H, 3),
        (H, 2),
        (H, 1),
        (F, 0),
        (In(C), 7),
        (In(C), 8),
        (In(C), 9),
        (In(C), 10),
        (In(D), 9),
        (In(D), 10),
        (In(D), 11),
        (In(D), 12),
    ],
    // C
    [
        (H, 7),
        (H, 6),
        (F, 5),
        (H, 4),
        (F, 3),
        (H, 2),
        (F, 1),
        (H, 2),
        (F, 3),
        (H, 4),
        (H, 5),
        (In(A), 6),
        (In(A), 7),
        (In(A), 8),
        (In(A), 9),
        (In(B), 4),
        (In(B), 5),
        (In(B), 6),
        (In(B), 7),
        (F, 0),
        (H, 1),
        (H, 2),
        (H, 3),
        (In(D), 4),
        (In(D), 5),
        (In(D), 6),
        (In(D), 7),
    ],
    [
        (H, 8),
        (H, 7),
        (F, 6),
        (H, 5),
        (F, 4),
        (H, 3),
        (F, 2),
        (H, 3),
        (F, 4),
        (H, 5),
        (H, 6),
        (In(A), 7),
        (In(A), 8),
        (In(A), 9),
        (In(A), 10),
        (In(B), 5),
        (In(B), 6),
        (In(B), 7),
        (In(B), 8),
        (H, 1),
        (F, 0),
        (H, 1),
        (H, 2),
        (In(D), 5),
        (In(D), 6),
        (In(D), 7),
        (In(D), 8),
    ],
    [
        (H, 9),
        (H, 8),
        (F, 7),
        (H, 6),
        (F, 5),
        (H, 4),
        (F, 3),
        (H, 4),
        (F, 5),
        (H, 6),
        (H, 7),
        (In(A), 8),
        (In(A), 9),
        (In(A), 10),
        (In(A), 11),
        (In(B), 6),
        (In(B), 7),
        (In(B), 8),
        (In(B), 9),
        (H, 2),
        (H, 1),
        (F, 0),
        (H, 1),
        (In(D), 6),
        (In(D), 7),
        (In(D), 8),
        (In(D), 9),
    ],
    [
        (H, 10),
        (H, 9),
        (F, 8),
        (H, 7),
        (F, 6),
        (H, 5),
        (F, 4),
        (H, 5),
        (F, 6),
        (H, 7),
        (H, 8),
        (In(A), 9),
        (In(A), 10),
        (In(A), 11),
        (In(A), 12),
        (In(B), 7),
        (In(B), 8),
        (In(B), 9),
        (In(B), 10),
        (H, 3),
        (H, 2),
        (H, 1),
        (F, 0),
        (In(D), 7),
        (In(D), 8),
        (In(D), 9),
        (In(D), 10),
    ],
    //D
    [
        (H, 9),
        (H, 8),
        (F, 7),
        (H, 6),
        (F, 5),
        (H, 4),
        (F, 3),
        (H, 2),
        (F, 1),
        (H, 2),
        (H, 3),
        (In(A), 8),
        (In(A), 9),
        (In(A), 10),
        (In(A), 11),
        (In(B), 6),
        (In(B), 7),
        (In(B), 8),
        (In(B), 9),
        (In(C), 4),
        (In(C), 5),
        (In(C), 6),
        (In(C), 7),
        (F, 0),
        (H, 1),
        (H, 2),
        (H, 3),
    ],
    [
        (H, 10),
        (H, 9),
        (F, 8),
        (H, 7),
        (F, 6),
        (H, 5),
        (F, 4),
        (H, 3),
        (F, 2),
        (H, 3),
        (H, 4),
        (In(A), 9),
        (In(A), 10),
        (In(A), 11),
        (In(A), 12),
        (In(B), 7),
        (In(B), 8),
        (In(B), 9),
        (In(B), 10),
        (In(C), 5),
        (In(C), 6),
        (In(C), 7),
        (In(C), 8),
        (H, 1),
        (F, 0),
        (H, 1),
        (H, 2),
    ],
    [
        (H, 11),
        (H, 10),
        (F, 9),
        (H, 8),
        (F, 7),
        (H, 6),
        (F, 5),
        (H, 4),
        (F, 3),
        (H, 4),
        (H, 5),
        (In(A), 10),
        (In(A), 11),
        (In(A), 12),
        (In(A), 13),
        (In(B), 8),
        (In(B), 9),
        (In(B), 10),
        (In(B), 11),
        (In(C), 6),
        (In(C), 7),
        (In(C), 8),
        (In(C), 9),
        (H, 2),
        (H, 1),
        (F, 0),
        (H, 1),
    ],
    [
        (H, 12),
        (H, 11),
        (F, 10),
        (H, 9),
        (F, 8),
        (H, 7),
        (F, 6),
        (H, 5),
        (F, 4),
        (H, 5),
        (H, 6),
        (In(A), 11),
        (In(A), 12),
        (In(A), 13),
        (In(A), 14),
        (In(B), 9),
        (In(B), 10),
        (In(B), 11),
        (In(B), 12),
        (In(C), 7),
        (In(C), 8),
        (In(C), 9),
        (In(C), 10),
        (H, 3),
        (H, 2),
        (H, 1),
        (F, 0),
    ],
];

static legal_step: [[i32; 3]; 27] = [
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
    [11, 13, -1],
    [12, 14, -1],
    [13, -1, -1],
    [4, 16, -1],
    [15, 17, -1],
    [16, 18, -1],
    [17, -1, -1],
    [6, 20, -1],
    [19, 21, -1],
    [20, 22, -1],
    [21, -1, -1],
    [8, 24, -1],
    [23, 25, -1],
    [24, 26, -1],
    [25, -1, -1],
];

fn distance(from: usize, to: usize) -> u32 {
    legal_moves[from][to].1
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
            if distance(next, to) < distance(cur, to) && !res.contains(&next) {
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
    for i in 0..27 {
        let mut row = vec![];
        for j in 0..27 {
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

// Amphipod can enter its cave iff there's no other amphipod type
// assumes move is legal, eg. there is a path from where a is to i
fn can_enter(a: Amphipod, pos: &Pos, i: usize) -> bool {
    match a {
        A => i == 12 || pos[12] == a,
        B => i == 14 || pos[14] == a,
        C => i == 16 || pos[16] == a,
        D => i == 18 || pos[18] == a,
        _ => true,
    }
}

fn compute_moves(all_paths: &Vec<Vec<Vec<usize>>>, pos: &Pos) -> Vec<(Pos, u32)> {
    use crate::MoveType::*;
    let mut moves = vec![];
    for i in 0..27 {
        let a = pos[i];
        if a != X {
            for j in 0..27 {
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
                            moves.push((nm, cost(a) * c as u32));
                        }
                    }
                    (In(t), c) => {
                        if path_is_free(all_paths, pos, i, j) && t == a && can_enter(a, pos, j) {
                            let mut nm = pos.clone();
                            let a = nm[i];
                            nm[j] = a;
                            nm[i] = X;
                            let code = encode(&nm);
                            moves.push((nm, cost(a) * c as u32));
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

static winning: [Amphipod; 27] = [
    X, X, X, X, X, X, X, X, X, X, X, A, A, A, A, B, B, B, B, C, C, C, C, D, D, D, D,
];

fn is_winning(pos: &Pos) -> bool {
    *pos == winning
}

/// heuristic function computing minimal distance from given position to
/// goal
fn h(pos: &Pos) -> u32 {
    (0..27).fold(0, |n, i| {
        n + match pos[i] {
            X => 0,
            A => (distance(i, 11).min(distance(i, 12)) * 1).into(),
            B => (distance(i, 13).min(distance(i, 14)) * 10).into(),
            C => (distance(i, 15).min(distance(i, 16)) * 100).into(),
            D => (distance(i, 17).min(distance(i, 18)) * 1000).into(),
        }
    })
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    val: u32,
    e: u32,
    pos: Pos,
    prev: Pos,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .val
            .cmp(&self.val)
            .then_with(|| encode(&self.pos).cmp(&encode(&other.pos)))
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
    path: &mut Vec<Pos>,
    min_e: &mut u32,
    energy: u32,
) {
    let mut heap = BinaryHeap::new();
    let mut visited = vec![];
    let mut open: HashMap<u64, u32> = HashMap::new();
    heap.push(Node {
        val: h(cur_pos),
        e: 0,
        pos: *cur_pos,
        prev: *cur_pos,
    });
    open.insert(encode(&cur_pos), 0);
    while let Some(Node { val, e, pos, prev }) = heap.pop() {
        let code = encode(&pos);
        if is_winning(&pos) {
            *min_e = e;
            return;
        }
        let next_moves = compute_moves(all_paths, &pos);
        println!(
            "checking {} {} {} {:?} {}",
            code,
            e,
            val,
            pos,
            next_moves.len()
        );
        for (nm, ne) in next_moves {
            let ncode = encode(&nm);
            if !visited.contains(&ncode) {
                let newe = e + ne;
                let node = Node {
                    val: newe + h(&nm),
                    e: newe,
                    pos: nm,
                    prev: pos,
                };
                match open.get(&ncode) {
                    None => {
                        heap.push(node);
                        open.insert(ncode, newe);
                    }
                    Some(olde) => {
                        if newe < *olde {
                            heap.push(node);
                            open.insert(ncode, newe);
                        }
                    }
                }
            }
        }
        visited.push(code);
    }
}

fn main() {
    // #############
    // #...........#
    // ###D#B#D#B###
    //   #D#C#B#A#
    //   #D#B#A#C#
    //   #C#A#A#C#
    //   #########

    //let puzzle: [Amphipod; 27] = [X, X, X, X, X, X, X, X, X, X, X, B, A, C, D, B, C, D, A];
    let puzzle: [Amphipod; 27] = [
        X, X, X, X, X, X, X, X, X, X, X, D, D, D, C, B, C, B, A, D, B, A, A, B, A, C, C,
    ];
    let paths = compute_all_paths();
    let mut path = vec![];
    let mut min_e = MAX;
    compute_min_steps(&paths, &puzzle, &mut path, &mut min_e, 0);
    println!("min energy: {}", min_e);
}
