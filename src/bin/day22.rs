use std::env;
use std::fs::read_to_string;
use std::process;

#[derive(Debug, Clone, PartialEq, Copy)]
enum Pos {
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Reboot {
    pos: Pos,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

fn parse_range(s: &str) -> (i32, i32) {
    let cparts: Vec<&str> = s.split("=").collect::<Vec<&str>>()[1].split("..").collect();

    (
        cparts[0].parse::<i32>().unwrap(),
        cparts[1].parse::<i32>().unwrap(),
    )
}

fn parse_reboot_step(s: &str) -> Reboot {
    let parts1: Vec<&str> = s.split(" ").collect();
    let pos = if parts1[0] == "on" { Pos::On } else { Pos::Off };
    let coordparts: Vec<&str> = parts1[1].split(",").collect();
    Reboot {
        pos,
        x: parse_range(coordparts[0]),
        y: parse_range(coordparts[1]),
        z: parse_range(coordparts[2]),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        let nums: Vec<&str> = input.split("\n").filter(|s| !s.is_empty()).collect();
        let reboot_steps: Vec<Reboot> = nums.iter().map(|s| parse_reboot_step(s)).collect();
        println!("max magnitude: {:?}", reboot_steps);
    } else {
        println!("fail to parse {}", args[1]);
    }
}
