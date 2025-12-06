use crate::graphs::Graph;

pub trait Unweighted: Graph {
    fn add_edge (self: &mut Self, u: usize, v: usize);

    fn iter_edges (self: &Self) -> impl Iterator<Item = (usize, usize)>;
}

pub mod directed_list;
pub mod directed_matrix;
pub mod undirected_list;
pub mod undirected_matrix;

#[cfg(test)]
mod _tests;