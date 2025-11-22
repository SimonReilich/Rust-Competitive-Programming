pub struct Graph {
    adj_matrix: Vec<Vec<bool>>,
    n: usize,
}

impl super::Graph for Graph {
    fn new () -> Graph {
        return Graph { adj_matrix: Vec::new(), n: 0 }
    }

    fn new_n (n: usize) -> Graph {
        let mut adj_matrix = Vec::new();
        for i in 0 .. n {
            adj_matrix.push(Vec::new());
            for _ in 0 .. n {
                adj_matrix[i].push(false);
            }
        }
        return Graph { adj_matrix: adj_matrix, n: n }
    }

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

    fn add_edge (self: &mut Graph, u: usize, v: usize) {
        if self.n <= u.max(v) {
            loop {
                if self.add_vertex() < u.max(v) { break; }
            }
        }
        self.adj_matrix[u][v] = true;
    }

    fn has_edge (self: &Graph, u: usize, v: usize) -> bool {
        if self.n <= u.max(v) {
            return false;
        } else {
            return self.adj_matrix[u][v];
        }
    }

    fn iter_vertices (self: &Graph) -> impl Iterator<Item=usize> {
        0 .. self.n
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
    fn iter_successors (self: &Graph, u: usize) -> impl Iterator<Item=usize> {
        return self.adj_matrix[u].clone()
            .into_iter()
            .enumerate()
            .filter_map(|(v, b)| if b { return Some(v); } else { return None });
    }

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