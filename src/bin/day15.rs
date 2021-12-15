use aoc2021::nums::all_neighbours;
use aoc2021::nums::neighbours;
use aoc2021::parser::parse_digits;
use aoc2021::vents::Pos;
use core::u64::MAX;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::fs::read_to_string;
use std::process;
use std::thread::sleep;
use std::time::Duration;

fn color_of(cell: &u8) -> u8 {
    16 + 20 * cell
}

fn print_path(nums: &Vec<Vec<u64>>) {
    for row in nums {
        for cell in row {
            // let color = color_of(&(*cell as u8));
            // print!("\x1b[48;5;{}m \x1b[0m", color);
            print!("{}", cell);
        }
        println!("");
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    position: Pos,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(nums: &Vec<Vec<u64>>) -> u64 {
    let mut distances = nums.clone();
    for row in distances.iter_mut() {
        for cell in row.iter_mut() {
            *cell = MAX;
        }
    }

    let mut heap = BinaryHeap::new();
    let start = Pos { x: 0, y: 0 };
    let goal = Pos {
        x: nums[0].len() - 1,
        y: nums.len() - 1,
    };

    // We're at `start`, with a zero cost
    distances[0][0] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return cost;
        }

        // Important as we may have already found a better way
        if cost > distances[position.y][position.x] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in neighbours(&nums, (position.x, position.y)) {
            let next = State {
                cost: cost + nums[edge.1][edge.0],
                position: Pos {
                    x: edge.0,
                    y: edge.1,
                },
            };

            // If so, add it to the frontier and continue
            if next.cost < distances[next.position.y][next.position.x] {
                heap.push(next);
                // Relaxation, we have now found a better way
                distances[next.position.y][next.position.x] = next.cost;
            }
        }
    }

    // Goal not reachable
    MAX
}

fn expand(nums: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let leny = nums.len();
    let lenx = nums[0].len();
    let mut new_nums: Vec<Vec<u64>> = Vec::with_capacity(leny * 5);
    new_nums.resize(leny * 5, vec![]);
    for j in 0..(leny * 5) {
        let mut new_row = Vec::with_capacity(lenx * 5);
        new_row.resize(lenx * 5, 0);
        for i in 0..(lenx * 5) {
            let mut val = nums[j % leny][i % lenx] + (i / lenx) as u64 + (j / leny) as u64;
            if val > 9 {
                val = val - 9;
            }
            new_row[i] = val;
        }
        new_nums[j] = new_row;
    }

    new_nums
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        if let Some(puzzle) = parse_digits(&input.split("\n").filter(|s| !s.is_empty()).collect()) {
            let real_cave = expand(&puzzle);
            let solution = solve(&real_cave);
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
    fn run_finds_lowest_energy_path() {
        let sample: Vec<Vec<u64>> = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];

        let res = solve(&sample);

        assert_eq!(res, 40);
    }

    #[test]
    fn run_finds_shortest_path_on_full_cave() {
        let sample: Vec<Vec<u64>> = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];

        let real_cave = expand(&sample);
        print_path(&real_cave);
        let res = solve(&real_cave);

        assert_eq!(res, 315);
    }
}
