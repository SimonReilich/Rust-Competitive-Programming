pub struct Graph {
    adj_list: Vec<Vec<usize>>,
    n: usize,
}

impl Graph {
    pub fn new () -> Graph {
        return Graph { adj_list: Vec::new(), n: 0 }
    }

    pub fn new_n (n: usize) -> Graph {
        let mut adj_list = Vec::new();
        for _ in 0 .. n {
            adj_list.push(Vec::new());
        }
        return Graph { adj_list: adj_list, n: n }
    }
}

impl super::super::Graph for Graph {
    fn add_vertex (self: &mut Self) -> usize {
        self.adj_list.push(Vec::new());
        self.n = self.n + 1;
        return self.n - 1;
    }

    fn has_edge (self: &Self, u: usize, v: usize) -> bool {
        if self.n <= u.max(v) {
            return false;
        } else {
            return self.adj_list[u].binary_search(&v).is_ok();
        }
    }

    fn vertex_count (self: &Self) -> usize {
        return self.n;
    }
    
    fn iter_successors (self: &Self, u: usize) -> impl Iterator<Item=usize> {
        return self.adj_list[u].clone().into_iter();
    }

    fn iter_predecessors (self: &Self, v: usize) -> impl Iterator<Item=usize> {
        return self.adj_list
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(move |(u, list)| {
                if list.binary_search(&v).is_ok() { 
                    return Some(u); 
                } else { 
                    return None; 
                }
            }
        );
    }
}

impl super::Unweighted for Graph {
    fn add_edge (self: &mut Self, u: usize, v: usize) {
        if self.n <= u.max(v) {
            loop {
                if super::super::Graph::add_vertex(self) == u.max(v) { break; }
            }
        }
        let pos = self.adj_list[u].binary_search(&v).unwrap_or_else(|e| e);
        self.adj_list[u].insert(pos, v);
    }

    fn iter_edges (self: &Self) -> impl Iterator<Item=(usize, usize)> {
        return self.adj_list.clone()
            .into_iter()
            .enumerate()
            .flat_map(|(u, list)| list
                .into_iter()
                .map(move |v| (u, v)
            )
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