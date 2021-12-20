use disjoint_sets::UnionFind;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

pub trait Graph<V> {
    fn len(&self) -> usize;
    fn sorted_edges(&self) -> Vec<(V, V)>;
}

/// An implementation of basic 'Graph' for a simple list of
/// of pair of elements.
impl<V: Eq + Hash + Ord + Clone + Debug> Graph<V> for Vec<(V, V)> {
    fn len(&self) -> usize {
        let mut vs = HashSet::new();
        for (f, t) in self {
            vs.insert(f);
            vs.insert(t);
        }
        println!("{:?}", vs);
        vs.len()
    }

    fn sorted_edges(&self) -> Vec<(V, V)> {
        let mut res = self.clone();
        res.sort();
        res.to_vec()
    }
}

pub fn min_spanning_tree<V: disjoint_sets::ElementType + Display, G: Graph<V>>(
    graph: &G,
) -> Vec<(V, V)> {
    let mut result = vec![];
    println!("{}", graph.len());
    let mut uf = UnionFind::new(graph.len());

    for (src, dst) in graph.sorted_edges() {
        println!("{} {}", src, dst);
        if !uf.equiv(src, dst) {
            uf.union(src, dst);
            result.push((src, dst));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_compute_spanning_tree_for_connected_graph() {
        let g: Vec<(u8, u8)> = vec![
            (0, 1),
            (1, 2),
            (2, 4),
            (4, 6),
            (4, 1),
            (4, 5),
            (4, 3),
            (0, 3),
            (3, 5),
            (5, 6),
            (3, 1),
        ];

        let res = min_spanning_tree(&g);

        println!("{:?}", res);
    }
}
