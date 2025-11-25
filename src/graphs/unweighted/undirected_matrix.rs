pub struct Graph {
    adj_matrix: Vec<Vec<bool>>,
    n: usize,
}

impl Graph {
    pub fn new () -> Graph {
        return Graph { adj_matrix: Vec::new(), n: 0 }
    }

    pub fn new_n (n: usize) -> Graph {
        let mut adj_matrix = Vec::new();
        for i in 0 .. n {
            adj_matrix.push(Vec::new());
            for _ in 0 .. (i + 1) {
                adj_matrix[i].push(false);
            }
        }
        return Graph { adj_matrix: adj_matrix, n: n }
    }
}

impl super::super::Graph for Graph {
    fn add_vertex (self: &mut Graph) -> usize {
        self.adj_matrix.push(Vec::new());
        self.n = self.n + 1;
        for _ in 0 .. self.n {
            self.adj_matrix[self.n - 1].push(false);
        }
        return self.n - 1;
    }

    fn has_edge (self: &Graph, u: usize, v: usize) -> bool {
        if u < v {
            return self.has_edge(v, u);
        } else {
            if self.n <= u.max(v) {
                return false;
            } else {
                return self.adj_matrix[u][v];
            }
        }
    }

    fn vertex_count (self: &Self) -> usize {
        return self.n;
    }

    fn iter_successors (self: &Graph, u: usize) -> impl Iterator<Item=usize> {
        return self.iter_predecessors(u).chain(self.iter_successors_1(u).filter(move |v| *v != u));
    }
}

impl super::Unweighted for Graph {
    fn add_edge (self: &mut Graph, u: usize, v: usize) {
        if u < v {
            self.add_edge(v, u);
        } else {
            if self.n <= u.max(v) {
                loop {
                    if super::super::Graph::add_vertex(self) == u.max(v) { break; }
                }
            }
            self.adj_matrix[u][v] = true;
        }
    }

    fn iter_edges (self: &Graph) -> impl Iterator<Item=(usize, usize)> {
        return self.adj_matrix
            .clone()
            .into_iter()
            .enumerate()
            .flat_map(|(u, list)| list
                .into_iter()
                .enumerate()
                .filter_map(move |(v, b)| if b { return Some((u, v)); } else { return None; }))
            .map(|(u, v)| if u < v { return (u, v); } else { return (v, u); });
    }
}

impl Graph {
    fn iter_successors_1 (self: &Graph, u: usize) -> impl Iterator<Item=usize> {
        return self.adj_matrix[u].clone()
            .into_iter()
            .enumerate()
            .filter(|(_, b)| *b)
            .map(|(v, _)| v);
    }

    fn iter_predecessors (self: &Graph, v: usize) -> impl Iterator<Item=usize> {
        return self.adj_matrix.clone()
            .into_iter()
            .enumerate()
            .filter_map(move |(u, list)| {
                if list.len() > v && list[v] { 
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
    use crate::graphs::unweighted::_tests as TestsUnweighted;

    #[test]
    fn test_new_graph_is_empty() {
        TestsUnweighted::test_new_graph_is_empty(super::Graph::new());
    }

    #[test]
    fn test_new_n_initializes_vertices() {
        for i in 1 .. 10 {
            TestsUnweighted::test_new_n_initializes_vertices(super::Graph::new_n(i), i);
        }
    }

    #[test]
    fn test_add_vertex() {
        Tests::test_add_vertex(super::Graph::new());
    }

    #[test]
    fn test_add_and_check_edge() {
        TestsUnweighted::undirected::test_add_and_check_edge(super::Graph::new());
    }

    #[test]
    fn test_neighbors() {
        TestsUnweighted::undirected::test_neighbors(super::Graph::new());
    }

    #[test]
    fn test_iter_vertices() {
        Tests::test_iter_vertices(super::Graph::new());
    }

    #[test]
    fn test_iter_edges_global() {
        TestsUnweighted::undirected::test_iter_edges_global(super::Graph::new())
    }

    #[test]
    fn test_self_loop() {
        TestsUnweighted::undirected::test_self_loop(super::Graph::new());
    }
}