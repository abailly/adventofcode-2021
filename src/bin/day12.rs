use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::process;

fn parse_edges<'a>(lines: &Vec<&'a str>) -> Option<HashMap<&'a str, Vec<&'a str>>> {
    let mut res: HashMap<&str, Vec<&str>> = HashMap::new();
    for s in lines {
        let parts: Vec<&str> = s.split("-").collect();
        let (k, v) = (parts[0], parts[1]);
        match res.get_mut(k) {
            Some(l) => l.push(v),
            None => {
                res.insert(k, vec![v]);
                ()
            }
        }
        match res.get_mut(v) {
            Some(l) => l.push(k),
            None => {
                res.insert(v, vec![k]);
                ()
            }
        }
    }

    Some(res)
}

fn solve(_caves: &HashMap<&str, Vec<&str>>) -> u64 {
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        if let Some(puzzle) = parse_edges(&input.split("\n").filter(|s| !s.is_empty()).collect()) {
            let solution = solve(&puzzle);
            println!("{}", solution);
        }
    } else {
        println!("fail to parse {}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_graph_from_input() {
        let lines = vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];

        let g = parse_edges(&lines).unwrap();

        let res = vec!["start", "c", "b", "end"];
        assert_eq!(g.get("A"), Some(&res));
    }
}
