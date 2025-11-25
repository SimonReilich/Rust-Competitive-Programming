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
            for _ in 0 .. n {
                adj_matrix[i].push(false);
            }
        }
        return Graph { adj_matrix: adj_matrix, n: n }
    }
}

impl super::super::Graph for Graph {
    fn add_vertex (self: &mut Graph) -> usize {
        self.adj_matrix.push(Vec::new());
        for i in 0 .. self.n {
            self.adj_matrix[i].push(false);
            self.adj_matrix[self.n].push(false);
        }
        self.adj_matrix[self.n].push(false);

        self.n = self.n + 1;
        return self.n - 1;
    }

    fn has_edge (self: &Graph, u: usize, v: usize) -> bool {
        if self.n <= u.max(v) {
            return false;
        } else {
            return self.adj_matrix[u][v];
        }
    }

    fn vertex_count (self: &Self) -> usize {
        return self.n;
    }

    fn iter_successors (self: &Graph, u: usize) -> impl Iterator<Item=usize> {
        return self.adj_matrix[u].clone()
            .into_iter()
            .enumerate()
            .filter_map(|(v, b)| if b { return Some(v); } else { return None });
    }
}

impl super::Unweighted for Graph {
    fn add_edge (self: &mut Graph, u: usize, v: usize) {
        if self.n <= u.max(v) {
            loop {
                if super::super::Graph::add_vertex(self) == u.max(v) { break; }
            }
        }
        self.adj_matrix[u][v] = true;
    }

    fn iter_edges (self: &Graph) -> impl Iterator<Item=(usize, usize)> {
        return self.adj_matrix.clone()
            .into_iter()
            .enumerate()
            .flat_map(|(u, list)| list
                .into_iter()
                .enumerate()
                .filter_map(move |(v, b)| if b { return Some((u, v)); } else { return None; }
            )
        );
    }
}

impl super::super::Directed for Graph {
    fn iter_predecessors (self: &Graph, v: usize) -> impl Iterator<Item=usize> {
        return self.adj_matrix
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(move |(u, list)| if list
                .into_iter()
                .enumerate()
                .filter(|(v_prime, b)| *b && *v_prime == v)
                .peekable()
                .peek()
                .is_some() 
            { 
                return Some(u); 
            } else { 
                return None; 
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
        TestsUnweighted::directed::test_add_and_check_edge(super::Graph::new());
    }

    #[test]
    fn test_successors() {
        TestsUnweighted::directed::test_successors(super::Graph::new());
    }

    #[test]
    fn test_predecessors() {
        TestsUnweighted::directed::test_predecessors(super::Graph::new());
    }

    #[test]
    fn test_iter_vertices() {
        Tests::test_iter_vertices(super::Graph::new());
    }

    #[test]
    fn test_iter_edges_global() {
        TestsUnweighted::directed::test_iter_edges_global(super::Graph::new())
    }

    #[test]
    fn test_self_loop() {
        TestsUnweighted::directed::test_self_loop(super::Graph::new());
    }
}