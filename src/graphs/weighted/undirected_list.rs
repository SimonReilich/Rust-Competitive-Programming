pub struct Graph<W> {
    adj_list: Vec<Vec<(usize, W)>>,
    n: usize,
}

impl <W: Clone + Copy> super::GraphWeighted<W> for Graph<W> {
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

    fn add_vertex (self: &mut Graph<W>) -> usize {
        self.adj_list.push(Vec::new());
        self.n = self.n + 1;
        return self.n - 1;
    }

    fn add_edge (self: &mut Graph<W>, u: usize, weight: W, v: usize) {
        if u > v {
            self.add_edge(v, weight, u);
        } else {
            if self.n <= u.max(v) {
                loop {
                    if self.add_vertex() < u.max(v) { break; }
                }
            }
            self.adj_list[u].push((v, weight));
        }
    }

    fn has_edge (self: &Graph<W>, u: usize, v: usize) -> Vec<W> {
        if u > v {
            return self.has_edge(v, u);
        } else if self.n <= u.max(v) {
            return Vec::new();
        } else {
            return self.adj_list[u].clone().into_iter()
                .filter(|(v_prime, _)| *v_prime == v)
                .map(|(_, w)| w)
                .collect();
        }
    }

    fn iter_vertices (self: &Graph<W>) -> impl Iterator<Item=usize> {
        0 .. self.n
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
        return self.iter_predecessors(u).chain(self.iter_successors(u));
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