use crate::Pos::*;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::process;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Pos {
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Cuboid {
    pos: Pos,
    lb: (i64, i64, i64),
    ub: (i64, i64, i64),
}

fn parse_range(s: &str) -> (i64, i64) {
    let cparts: Vec<&str> = s.split("=").collect::<Vec<&str>>()[1].split("..").collect();

    (
        cparts[0].parse::<i64>().unwrap(),
        cparts[1].parse::<i64>().unwrap(),
    )
}

fn parse_cuboid_step(s: &str) -> Cuboid {
    let parts1: Vec<&str> = s.split(" ").collect();
    let pos = if parts1[0] == "on" { Pos::On } else { Pos::Off };
    let coordparts: Vec<&str> = parts1[1].split(",").collect();
    let xrange = parse_range(coordparts[0]);
    let yrange = parse_range(coordparts[1]);
    let zrange = parse_range(coordparts[2]);
    Cuboid {
        pos,
        lb: (xrange.0, yrange.0, zrange.0),
        ub: (xrange.1 + 1, yrange.1 + 1, zrange.1 + 1),
    }
}

fn make_treemap(
    bounds: &(Vec<i64>, Vec<i64>, Vec<i64>),
) -> (
    BTreeMap<i64, usize>,
    BTreeMap<i64, usize>,
    BTreeMap<i64, usize>,
) {
    let (bx, by, bz) = bounds;
    let mut btx = BTreeMap::new();
    for (i, v) in bx.iter().enumerate() {
        btx.insert(*v, i);
    }
    let mut bty = BTreeMap::new();
    for (i, v) in by.iter().enumerate() {
        bty.insert(*v, i);
    }
    let mut btz = BTreeMap::new();
    for (i, v) in bz.iter().enumerate() {
        btz.insert(*v, i);
    }

    (btx, bty, btz)
}

fn make_treemap(
    bounds: &(Vec<i64>, Vec<i64>, Vec<i64>),
) -> (
    BTreeMap<i64, usize>,
    BTreeMap<i64, usize>,
    BTreeMap<i64, usize>,
) {
    let (bx, by, bz) = bounds;
    let mut btx = BTreeMap::new();
    for (i, v) in bx.iter().enumerate() {
        btx.insert(*v, i);
    }
    let mut bty = BTreeMap::new();
    for (i, v) in by.iter().enumerate() {
        bty.insert(*v, i);
    }
    let mut btz = BTreeMap::new();
    for (i, v) in bz.iter().enumerate() {
        btz.insert(*v, i);
    }

    (btx, bty, btz)
}

fn on_cubes(bounds: &(Vec<i64>, Vec<i64>, Vec<i64>), steps: &Vec<Cuboid>) -> i64 {
    let (bvx, bvy, bvz) = bounds;
    let (bx, by, bz) = make_treemap(bounds);
    println!("bounds {:?}", bounds);
    let mut cubes = vec![vec![vec![Off; bz.len()]; by.len()]; bx.len()];
    let mut num_ons = 0;
    for cube in steps {
        println!("cube {:?}", cube);
        for (_, i) in bx.range(cube.lb.0..cube.ub.0) {
            for (_, j) in by.range(cube.lb.1..cube.ub.1) {
                for (_, k) in bz.range(cube.lb.2..cube.ub.2) {
                    cubes[*i][*j][*k] = cube.pos;
                }
            }
        }
    }

    for i in 0..bx.len() - 1 {
        for j in 0..by.len() - 1 {
            for k in 0..bz.len() - 1 {
                if cubes[i][j][k] == On {
                    let sx = bvx[i + 1] - bvx[i];
                    let sy = bvy[j + 1] - bvy[j];
                    let sz = bvz[k + 1] - bvz[k];
                    num_ons += sx * sy * sz;
                }
            }
        }
    }
    num_ons
}

fn make_bounds(cuboids: &Vec<Cuboid>) -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let mut vx = HashSet::new();
    let mut vy = HashSet::new();
    let mut vz = HashSet::new();
    for c in cuboids {
        vx.insert(c.lb.0 as i64);
        vx.insert(c.ub.0 as i64);
        vy.insert(c.lb.1 as i64);
        vy.insert(c.ub.1 as i64);
        vz.insert(c.lb.2 as i64);
        vz.insert(c.ub.2 as i64);
    }
    let mut vvx: Vec<i64> = vec![];
    vvx.extend(vx);
    let mut vvy: Vec<i64> = vec![];
    vvy.extend(vy);
    let mut vvz: Vec<i64> = vec![];
    vvz.extend(vz);
    vvx.sort();
    vvy.sort();
    vvz.sort();

    (vvx, vvy, vvz)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        let nums: Vec<&str> = input.split("\n").filter(|s| !s.is_empty()).collect();
        let cuboid_steps: Vec<Cuboid> = nums.iter().map(|s| parse_cuboid_step(s)).collect();
        let cube_bounds: (Vec<i64>, Vec<i64>, Vec<i64>) = make_bounds(&cuboid_steps);
        let num_on_cubes = on_cubes(&cube_bounds, &cuboid_steps);
        println!("num on cubes: {:?}", num_on_cubes);
    } else {
        println!("fail to parse {}", args[1]);
    }
}
