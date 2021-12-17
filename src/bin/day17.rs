use crate::bits::complete::tag;
use crate::bits::complete::take;
use aoc2021::parser::Ebits;
use hex;
use nom::bits;
use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::cmp::Ordering;
use std::env;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::process;

fn beyond(ranges: ((i32, i32), (i32, i32)), pos: (i32, i32)) -> bool {
    pos.0 > ranges.0 .1 || pos.1 < ranges.1 .0
}

fn within(ranges: ((i32, i32), (i32, i32)), pos: (i32, i32)) -> bool {
    pos.0 >= ranges.0 .0 && pos.0 <= ranges.0 .1 && pos.1 >= ranges.1 .0 && pos.1 <= ranges.1 .1
}
fn hit(ranges: ((i32, i32), (i32, i32)), velocity: (i32, i32)) -> Option<i32> {
    let mut pos = (0, 0);
    let mut vel = velocity;
    let mut maxy = 0;
    while !beyond(ranges, pos) {
        if within(ranges, pos) {
            println!("in range pos {:?} vel {:?} maxy {}", pos, vel, maxy);
            return Some(maxy);
        }
        pos = (pos.0 + vel.0, pos.1 + vel.1);
        let velx = match vel.0.cmp(&0) {
            Ordering::Less => vel.0 + 1,
            Ordering::Equal => 0,
            Ordering::Greater => vel.0 - 1,
        };
        let vely = vel.1 - 1;
        vel = (velx, vely);
        if pos.1 > maxy {
            maxy = pos.1;
        }
    }
    None
}

fn solve(ranges: ((i32, i32), (i32, i32))) -> Vec<(i32, (i32, i32))> {
    let mut solutions = vec![];
    for x in 1..400 {
        for y in -75..1000 {
            if let Some(maxy) = hit(ranges, (x, y)) {
                solutions.push((maxy, (x, y)));
            }
        }
    }
    solutions
}

fn main() {
    let solution = solve(((253, 280), (-73, -46)));
    println!("{:?}", solution.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_literal_value() {
        let input = ((20, 30), (-10, 5));

        let res = solve(input);

        assert_eq!(res.len(), 112);
    }
}
