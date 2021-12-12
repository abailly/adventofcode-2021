use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::iter::FromIterator;
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

fn has_at_most_one_lowercase(path: &Vec<&str>) -> bool {
    let lc: Vec<&&str> = path
        .iter()
        .filter(|s| s.chars().nth(0).unwrap().is_ascii_lowercase())
        .collect();

    let hc: HashSet<_> = HashSet::from_iter(lc.iter().cloned());
    lc.len() == hc.len()
}

fn can_extend_path_with(path: &Vec<&str>, c: &str) -> bool {
    (c.chars().nth(0).unwrap().is_ascii_uppercase()
        || !path.contains(&c)
        || has_at_most_one_lowercase(path))
        && c != "start"
}

fn solve(caves: &HashMap<&str, Vec<&str>>) -> usize {
    let mut res = vec![];
    let mut to_explore = vec![vec!["start"]];
    while !to_explore.is_empty() {
        if let Some(path) = to_explore.pop() {
            let head = path[path.len() - 1];
            if head == "end" {
                res.push(path);
            } else {
                if let Some(cs) = caves.get(head) {
                    for c in cs {
                        if can_extend_path_with(&path, c) {
                            let mut new_path = path.clone();
                            new_path.push(c);
                            to_explore.push(new_path);
                        }
                    }
                }
            }
        }
    }
    res.len()
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

    #[test]
    fn can_compute_number_of_paths_on_sample() {
        let lines = vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];

        let g = parse_edges(&lines).unwrap();

        assert_eq!(solve(&g), 10);
    }

    #[test]
    fn can_compute_number_of_paths_on_larger_sample() {
        let lines = vec![
            "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa",
            "kj-HN", "kj-dc",
        ];

        let g = parse_edges(&lines).unwrap();

        assert_eq!(solve(&g), 19);
    }

    #[test]
    fn can_compute_number_of_paths_on_an_even_larger_sample() {
        let lines = vec![
            "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
            "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
        ];

        let g = parse_edges(&lines).unwrap();

        assert_eq!(solve(&g), 226);
    }
}
