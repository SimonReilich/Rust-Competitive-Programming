pub struct Graph<W> {
    adj_matrix: Vec<Vec<Option<W>>>,
    n: usize,
}

impl <W: Copy> super::GraphWeighted<W> for Graph<W> {
    fn new () -> Graph<W> {
        return Graph { adj_matrix: Vec::new(), n: 0 }
    }

    fn new_n (n: usize) -> Graph<W> {
        let mut adj_matrix = Vec::new();
        for i in 0 .. n {
            adj_matrix.push(Vec::new());
            for _ in 0 .. n {
                adj_matrix[i].push(None);
            }
        }
        return Graph { adj_matrix: adj_matrix, n: n }
    }

    fn add_vertex (self: &mut Graph<W>) -> usize {
        self.adj_matrix.push(Vec::new());
        for i in 0 .. self.n {
            self.adj_matrix[i].push(None);
            self.adj_matrix[self.n].push(None);
        }
        self.adj_matrix[self.n].push(None);

        self.n = self.n + 1;
        return self.n - 1;
    }

    fn add_edge (self: &mut Graph<W>, u: usize, weight: W, v: usize) {
        if self.n <= u.max(v) {
            loop {
                if self.add_vertex() < u.max(v) { break; }
            }
        }
        self.adj_matrix[u][v] = Some(weight);
    }

    fn has_edge (self: &Graph<W>, u: usize, v: usize) -> Vec<W> {
        if self.n <= u.max(v) {
            return Vec::new();
        } else if let Some(w) = &self.adj_matrix[u][v] {
            return vec![*w];
        } else {
            return Vec::new();
        }
    }

    fn iter_vertices (self: &Graph<W>) -> impl Iterator<Item=usize> {
        0 .. self.n
    }

    fn iter_edges (self: &Graph<W>) -> impl Iterator<Item=(usize, W, usize)> {
        return self.adj_matrix.clone()
            .into_iter()
            .enumerate()
            .flat_map(|(u, list)| list
                .into_iter()
                .enumerate()
                .filter_map(move |(v, o)| if let Some(w) = o { return Some((u, w, v)); } else { return None; }
            )
        );
    }
}

impl <W: Clone> super::super::Directed for Graph<W> {
    fn iter_successors (self: &Graph<W>, u: usize) -> impl Iterator<Item=usize> {
        return self.adj_matrix[u].clone()
            .into_iter()
            .enumerate()
            .filter_map(|(v, b)| if b.is_some() { return Some(v); } else { return None });
    }

    fn iter_predecessors (self: &Graph<W>, v: usize) -> impl Iterator<Item=usize> {
        return self.adj_matrix
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(move |(u, list)| if list
                .into_iter()
                .enumerate()
                .filter(|(v_prime, b)| b.is_some() && *v_prime == v)
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