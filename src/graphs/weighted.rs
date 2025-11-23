use crate::graphs::Graph;

pub trait Weighted<W: Clone + Copy>: Graph {
    fn add_edge (self: &mut Self, u: usize, weight: W, v: usize);

    fn get_weight (self: &Self, u: usize, v: usize) -> Vec<W>;

    fn iter_edges (self: &Self) -> impl Iterator<Item=(usize, W, usize)>;
}

pub mod directed_list;
pub mod directed_matrix;
pub mod undirected_list;
pub mod undirected_matrix;

mod _tests;