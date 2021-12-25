use crate::Cuke::*;
use std::env;
use std::fs::read_to_string;
use std::process;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Cuke {
    E,
    S,
    O,
}
fn to_cucumbers(s: &str) -> Vec<Cuke> {
    s.chars()
        .filter_map(|c| match c {
            '>' => Some(E),
            'v' => Some(S),
            '.' => Some(O),
            _ => None,
        })
        .collect()
}

fn step(cukes: &mut Vec<Vec<Cuke>>) -> u64 {
    let mut moves = 0;
    let mut moves_to_go = vec![];

    for (j, row) in cukes.iter().enumerate() {
        for (i, cuke) in row.iter().enumerate() {
            match cuke {
                E => {
                    let k = (i + 1) % row.len();
                    if cukes[j][k] == O {
                        moves_to_go.push((E, (j, i), (j, k)));
                    }
                }
                _ => (),
            }
        }
    }

    println!("to move {:?}", moves_to_go);

    moves += moves_to_go.len();
    for (t, (j, i), (k, l)) in &moves_to_go {
        cukes[*j][*i] = O;
        cukes[*k][*l] = *t;
    }

    moves_to_go.clear();

    for (j, row) in cukes.iter().enumerate() {
        for (i, cuke) in row.iter().enumerate() {
            match cuke {
                S => {
                    let k = (j + 1) % cukes.len();
                    if cukes[k][i] == O {
                        moves_to_go.push((S, (j, i), (k, i)));
                    }
                }
                _ => (),
            }
        }
    }

    moves += moves_to_go.len();
    for (t, (j, i), (k, l)) in moves_to_go {
        cukes[j][i] = O;
        cukes[k][l] = t;
    }

    moves as u64
}

fn move_until_still(cukes: &mut Vec<Vec<Cuke>>) -> u64 {
    let mut steps = 1;
    loop {
        let moves = step(cukes);
        if moves == 0 {
            break;
        }
        steps += 1;
    }
    steps
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }
    if let Ok(input) = read_to_string(&args[1]) {
        let nums: Vec<&str> = input.split("\n").filter(|s| !s.is_empty()).collect();
        let mut cucumbers = nums.iter().map(|s| to_cucumbers(s)).collect();
        let result = move_until_still(&mut cucumbers);
        println!("steps {}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_steps_to_stillness() {
        println!("{}", &msg);
        // expand input into a window with 3 more default cells on each side
        let output = expand(&msg, &sample_enhance);

        println!("{}", &output);
    }
}
