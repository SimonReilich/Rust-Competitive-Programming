pub trait GraphWeighted<W: Clone + Copy> {
    fn new () -> impl GraphWeighted<W>;

    fn new_n (n: usize) -> impl GraphWeighted<W>;

    fn add_vertex (self: &mut Self) -> usize;

    fn add_edge (self: &mut Self, u: usize, weight: W, v: usize);

    fn has_edge (self: &Self, u: usize, v: usize) -> Vec<W>;

    fn iter_vertices (self: &Self) -> impl Iterator<Item=usize>;

    fn iter_edges (self: &Self) -> impl Iterator<Item=(usize, W, usize)>;
}

pub mod directed_list;
pub mod directed_matrix;
pub mod undirected_list;
pub mod undirected_matrix;