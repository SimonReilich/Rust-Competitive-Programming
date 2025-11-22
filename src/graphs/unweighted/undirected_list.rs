pub struct Graph {
    adj_list: Vec<Vec<usize>>,
    n: usize,
}

impl super::Graph for Graph {
    fn new () -> Graph {
        return Graph { adj_list: Vec::new(), n: 0 }
    }

    fn new_n (n: usize) -> Graph {
        let mut adj_list = Vec::new();
        for _ in 0 .. n {
            adj_list.push(Vec::new());
        }
        return Graph { adj_list: adj_list, n: n }
    }

    fn add_vertex (self: &mut Graph) -> usize {
        self.adj_list.push(Vec::new());
        self.n = self.n + 1;
        return self.n - 1;
    }

    fn add_edge (self: &mut Graph, u: usize, v: usize) {
        if u > v {
            self.add_edge(v, u);
        } else {
            if self.n <= u.max(v) {
                loop {
                    if self.add_vertex() < u.max(v) { break; }
                }
            }
            let pos = self.adj_list[u].binary_search(&v).unwrap_or_else(|e| e);
            self.adj_list[u].insert(pos, v);
        }
    }

    fn has_edge (self: &Graph, u: usize, v: usize) -> bool {
        if u > v {
            return self.has_edge(v, u);
        } else {
            if self.n <= u.max(v) {
                return false;
            } else {
                return self.adj_list[u].binary_search(&v).is_ok();
            }
        }
    }

    fn iter_vertices (self: &Graph) -> impl Iterator<Item=usize> {
        0 .. self.n
    }

    fn iter_edges (self: &Graph) -> impl Iterator<Item=(usize, usize)> {
        return self.adj_list
            .clone()
            .into_iter()
            .enumerate()
            .flat_map(|(u, list)| list
                .into_iter()
                .map(move |v| (u, v)
            )
        );
    }
}

impl super::super::Undirected for Graph {
    fn iter_neighbors (self: &Graph, u: usize) -> impl Iterator<Item=usize> {
        return self.iter_predecessors(u).chain(self.iter_successors(u));
    }
}

impl Graph {
    fn iter_successors (self: &Graph, u: usize) -> impl Iterator<Item=usize> {
        return self.adj_list[u].clone().into_iter();
    }

    fn iter_predecessors (self: &Graph, v: usize) -> impl Iterator<Item=usize> {
        return self.adj_list.clone()
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