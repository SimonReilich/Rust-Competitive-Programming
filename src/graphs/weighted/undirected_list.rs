pub struct Graph<W> {
    adj_list: Vec<Vec<(usize, W)>>,
    n: usize,
}

impl <W: Clone + Copy> Graph<W> {
    fn new () -> Graph<W> {
        return Graph { adj_list: Vec::new(), n: 0 }
    }

    fn new_n (n: usize) -> Graph<W> {
        let mut adj_list = Vec::new();
        for _ in 0 .. n {
            adj_list.push(Vec::new());
        }
        return Graph { adj_list: adj_list, n: n }
    }

}

impl <W: Clone + Copy> super::super::Graph for Graph<W> {
    fn add_vertex (self: &mut Graph<W>) -> usize {
        self.adj_list.push(Vec::new());
        self.n = self.n + 1;
        return self.n - 1;
    }

    fn has_edge (self: &Graph<W>, u: usize, v: usize) -> bool {
        if u > v {
            return self.has_edge(v, u);
        } else if self.n <= u.max(v) {
            return false;
        } else {
            return self.adj_list[u].clone().into_iter()
                .any(|(v_prime, _)| v_prime == v);
        }
    }

    fn vertex_count (self: &Self) -> usize {
        return self.n;
    }
}

impl <W: Clone + Copy> super::Weighted<W> for Graph<W> {
    fn add_edge (self: &mut Graph<W>, u: usize, weight: W, v: usize) {
        if u > v {
            self.add_edge(v, weight, u);
        } else {
            if self.n <= u.max(v) {
                loop {
                    if super::super::Graph::add_vertex(self) == u.max(v) { break; }
                }
            }
            self.adj_list[u].push((v, weight));
        }
    }

    fn get_weight (self: &Graph<W>, u: usize, v: usize) -> Vec<W> {
        if u > v {
            return self.get_weight(v, u);
        } else if self.n <= u.max(v) {
            return Vec::new();
        } else {
            return self.adj_list[u].clone().into_iter()
                .filter(|(v_prime, _)| *v_prime == v)
                .map(|(_, w)| w)
                .collect();
        }
    }

    fn iter_edges (self: &Graph<W>) -> impl Iterator<Item=(usize, W, usize)> {
        return self.adj_list.clone()
            .into_iter()
            .enumerate()
            .flat_map(|(u, list)| list
                .into_iter()
                .map(move |(v, w)| (u, w, v)
            )
        );
    }
}

impl <W: Clone + Copy> super::super::Undirected for Graph<W> {
    fn iter_neighbors (self: &Graph<W>, u: usize) -> impl Iterator<Item=usize> {
        return self.iter_predecessors(u).chain(self.iter_successors(u).filter(move |v| *v != u));
    }
}

impl <W: Clone + Copy> Graph<W> {
    fn iter_successors (self: &Graph<W>, u: usize) -> impl Iterator<Item=usize> {
        return self.adj_list[u].clone().into_iter().map(|(v, _)| v);
    }

    fn iter_predecessors (self: &Graph<W>, v: usize) -> impl Iterator<Item=usize> {
        return self.adj_list
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(move |(u, list)| {
                if list.into_iter().any(|(v_prime, _)| v_prime == v) { 
                    return Some(u); 
                } else { 
                    return None; 
                }
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::graphs::_tests as Tests;
    use crate::graphs::weighted::_tests as TestsWeighted;

    #[test]
    fn test_new_graph_is_empty() {
        TestsWeighted::test_new_graph_is_empty(super::Graph::new());
    }

    #[test]
    fn test_new_n_initializes_vertices() {
        for i in 1 .. 10 {
            TestsWeighted::test_new_n_initializes_vertices(super::Graph::new_n(i), i);
        }
    }

    #[test]
    fn test_add_vertex() {
        Tests::test_add_vertex(super::Graph::<u32>::new());
    }

    #[test]
    fn test_add_and_check_edge() {
        TestsWeighted::undirected::test_add_and_check_edge(super::Graph::new());
    }

    #[test]
    fn test_neighbors() {
        TestsWeighted::undirected::test_neighbors(super::Graph::new());
    }

    #[test]
    fn test_iter_vertices() {
        Tests::test_iter_vertices(super::Graph::<u32>::new());
    }

    #[test]
    fn test_iter_edges_global() {
        TestsWeighted::undirected::test_iter_edges_global(super::Graph::new())
    }

    #[test]
    fn test_self_loop() {
        TestsWeighted::undirected::test_self_loop(super::Graph::new());
    }
}