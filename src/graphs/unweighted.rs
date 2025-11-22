pub trait Graph {
    fn new () -> impl Graph;

    fn new_n (n: usize) -> impl Graph;

    fn add_vertex (self: &mut Self) -> usize;

    fn add_edge (self: &mut Self, u: usize, v: usize);

    fn has_edge (self: &Self, u: usize, v: usize) -> bool;

    fn iter_vertices (self: &Self) -> impl Iterator<Item=usize>;

    fn iter_edges (self: &Self) -> impl Iterator<Item=(usize, usize)>;
}

pub mod directed_list;
pub mod directed_matrix;
pub mod undirected_list;
pub mod undirected_matrix;