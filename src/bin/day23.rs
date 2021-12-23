use crate::Amphipod::*;
use core::u32::MAX;
use std::collections::HashMap;
use std::env;
use std::process;

enum Amphipod {
    A,
    B,
    C,
    D,
    X,
}

type Pos = [Amphipod; 19];

fn compute_moves(pos: &Pos, path: &mut Vec<u64>) -> Vec<(Pos, u32)> {
    vec![]
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

fn is_winning(pos: u64) -> bool {
    pos == 101724
}

fn compute_min_steps(cur_pos: &Pos, prev_pos: &mut Vec<u64>, energy: u32) -> u32 {
    let code = encode(cur_pos);
    if is_winning(code) {
        return energy;
    }

    let mut min_e = MAX;
    let next_moves = compute_moves(cur_pos, prev_pos);
    prev_pos.push(code);
    for (m, e) in next_moves {
        let nm = compute_min_steps(&m, prev_pos, energy + e);
        if nm <= min_e {
            min_e = nm;
        }
    }
    prev_pos.pop();
    min_e
}

fn main() {
    // #############
    // #...........#
    // ###D#B#D#B###
    //   #C#A#A#C#
    //   #########
    let puzzle: [Amphipod; 19] = [X, X, X, X, X, X, X, X, X, X, X, D, B, D, B, C, A, A, C];
    let winning: [Amphipod; 19] = [X, X, X, X, X, X, X, X, X, X, X, A, A, B, B, C, C, D, D];
    let mut path = vec![];
    let min_steps = compute_min_steps(&puzzle, &mut path, 0);
    println!("min energy: {:?}", min_steps);
}
