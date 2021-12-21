use crate::Win::*;
use std::env;
use std::fmt;
use std::fmt::Display;
use std::fs::read_to_string;
use std::process;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Game {
    pos: (u64, u64),
    score: (u64, u64),
    dice: u64,
    dice_rolls: u64,
    player: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Win {
    NoWin,
    Winner1(u64, u64),
    Winner2(u64, u64),
}

fn winner(game: &Game) -> Win {
    if game.score.0 >= 1000 {
        return Winner1(game.score.0, game.score.1);
    } else if game.score.1 >= 1000 {
        return Winner2(game.score.1, game.score.0);
    } else {
        return NoWin;
    }
}

fn play(game: &Game) -> Game {
    let mut new_game = game.clone();
    let mut movement = 0;
    for _ in 0..3 {
        new_game.dice = new_game.dice + 1;
        movement += new_game.dice;
    }
    new_game.dice_rolls += 3;

    if new_game.player == 0 {
        new_game.pos.0 = (new_game.pos.0 + movement) % 10;
        new_game.score.0 = new_game.score.0 + new_game.pos.0 + 1;
    }
    if new_game.player == 1 {
        new_game.pos.1 = (new_game.pos.1 + movement) % 10;
        new_game.score.1 = new_game.score.1 + new_game.pos.1 + 1;
    }

    new_game.player = (new_game.player + 1) % 2;
    println!("game:  {:?}", game);
    new_game
}

type Situation = Vec<Vec<(u64, (u8, u8), (u8, u8))>>;

fn print(v: &Situation) {
    let mut res = String::new();
    for j in 0..v.len() {
        for i in 0..v[0].len() {
            let (num, (p1, s1), (p2, s2)) = v[j][i];
            res.push_str(&format!(" {} ({} {}) ({} {})", num, p1, s1, p2, s2).to_owned());
        }
        res.push('\n');
    }
    println!("{}", res);
}

/// Matrix multiplication
/// Assumes matrices are square and of identical rank
fn mult(
    side: u8,
    a: &Vec<Vec<(u64, (u8, u8), (u8, u8))>>,
    b: &Vec<(u64, u8)>,
) -> Vec<Vec<(u64, (u8, u8), (u8, u8))>> {
    let mut newstate = vec![];
    for j in 0..a.len() {
        let mut row = vec![];
        for i in 0..a.len() {
            let (nb, p1, p2) = a[j][i];
            if p1.1 < 21 && p2.1 < 21 {
                let idx = if side == 0 { j } else { i };
                let nbn = nb * b[idx].0;
                let res = if side == 0 {
                    let newpos = (p1.0 + b[idx].1) % 10;
                    let np1 = (newpos, p1.1 + newpos + 1);
                    (nbn, np1, p2)
                } else {
                    let newpos = (p2.0 + b[idx].1) % 10;
                    let np2 = (newpos, p2.1 + newpos + 1);
                    (nbn, p1, np2)
                };
                row.push(res);
            } else {
                row.push((nb, p1, p2));
            }
        }
        newstate.push(row);
    }
    newstate
}

fn play_rec(
    side: u8,
    p1: u8,
    s1: u8,
    p2: u8,
    s2: u8,
    outcomes: &Vec<(u64, u8)>,
    count: u64,
    s1win: &mut u64,
    s2win: &mut u64,
) {
    for (num, roll) in outcomes {
        if side == 0 {
            let pos = (p1 + roll) % 10;
            let ns1 = s1 + (pos + 1);
            if ns1 >= 21 {
                *s1win += count * num;
            } else {
                play_rec(1, pos, ns1, p2, s2, outcomes, count * num, s1win, s2win);
            }
        } else {
            let pos = (p2 + roll) % 10;
            let ns2 = s2 + (pos + 1);
            if ns2 >= 21 {
                *s2win += count * num;
            } else {
                play_rec(0, p1, s1, pos, ns2, outcomes, count * num, s1win, s2win);
            }
        }
    }
}

fn initial_state(p1: u8, p2: u8) -> Vec<Vec<(u64, (u8, u8), (u8, u8))>> {
    let mut res = vec![];
    for j in 0..7 {
        let mut row = vec![];
        for i in 0..7 {
            row.push((1, (p1, 0), (p2, 0)));
        }
        res.push(row);
    }
    res
}

fn main() {
    let probas: Vec<(u64, u8)> = vec![(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];

    // each triple is the number of positions leading to that outcome, score of player 1 and score of player 2
    let mut curstate = initial_state(5, 6);
    let mut player = 0;
    loop {
        let newstate = mult(player, &curstate, &probas);
        player = (player + 1) % 2;
        if newstate == curstate {
            break;
        }
        curstate = newstate;
        print(&curstate);
    }

    let mut p1win = 0;
    let mut p2win = 0;
    play_rec(0, 5, 0, 6, 0, &probas, 1, &mut p1win, &mut p2win);

    println!("p1win {} p2win {}", p1win, p2win);
}
