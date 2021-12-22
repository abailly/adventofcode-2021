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
    lb: (i32, i32, i32),
    ub: (i32, i32, i32),
}

fn parse_range(s: &str) -> (i32, i32) {
    let cparts: Vec<&str> = s.split("=").collect::<Vec<&str>>()[1].split("..").collect();

    (
        cparts[0].parse::<i32>().unwrap(),
        cparts[1].parse::<i32>().unwrap(),
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
        ub: (xrange.1, yrange.1, zrange.1),
    }
}

/// Order cuboids by their
fn order<'a>(a: &'a Cuboid, b: &'a Cuboid) -> (&'a Cuboid, &'a Cuboid) {
    match a.lb.cmp(&b.lb) {
        Greater => (b, a),
        _ => (a, b),
    }
}

fn intersect(a: &Cuboid, b: &Cuboid) -> bool {
    a.lb.0 <= b.ub.0
        && a.lb.1 <= b.ub.1
        && a.lb.2 <= b.ub.1
        && a.ub.0 >= b.lb.0
        && a.ub.1 >= b.lb.1
        && a.ub.2 >= b.lb.2
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Step {
    InitStep,
    Step(Box<Step>, Cuboid, Cuboid, Option<Cuboid>),
}

fn intersection(a: &Cuboid, b: &Cuboid) -> Option<Cuboid> {
    if !intersect(a, b) {
        return None;
    }
    None
}

/// Computes the Cuboid cuboids resulting from the intersection of
/// the two given cuboids
fn apply_step(prev: Step, a: &Cuboid, b: &Cuboid) -> Step {
    if !intersect(a, b) {
        return Step::Step(Box::new(prev), *a, *b, None);
    }

    Step::InitStep
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
        println!("max magnitude: {:?}", cuboid_steps);
    } else {
        println!("fail to parse {}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_check_apply_step() {
        let cuboid1 = Cuboid {
            pos: Pos::On,
            lb: (2, -22, -23),
            ub: (47, 22, 27),
        };
        let cuboid2 = Cuboid {
            pos: Pos::On,
            lb: (-27, -28, -21),
            ub: (23, 26, 29),
        };

        let cuboid3 = Cuboid {
            pos: Pos::On,
            lb: (-27, -28, -25),
            ub: (50, 26, 29),
        };
        assert_eq!(intersect(&cuboid1, &cuboid2), true);
        assert_eq!(intersect(&cuboid2, &cuboid1), true);
        assert_eq!(intersect(&cuboid1, &cuboid3), true);
    }

    #[test]
    fn can_compute_apply_step_of_2_cuboids_as_a_cuboid() {
        let cuboid1 = Cuboid {
            pos: Pos::On,
            lb: (10, 10, 10),
            ub: (12, 12, 12),
        };

        let cuboid2 = Cuboid {
            pos: Pos::On,
            lb: (11, 11, 11),
            ub: (13, 13, 13),
        };

        let cuboid3 = Cuboid {
            pos: Pos::On,
            lb: (11, 11, 11),
            ub: (12, 12, 12),
        };

        assert_eq!(intersection(&cuboid1, &cuboid2), Some(cuboid3));
    }
}
