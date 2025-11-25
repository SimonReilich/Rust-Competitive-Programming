pub trait Graph {
    fn add_vertex (self: &mut Self) -> usize;

    fn has_edge (self: &Self, u: usize, v: usize) -> bool;

    fn vertex_count (self: &Self) -> usize;

    fn iter_successors (self: &Self, u: usize) -> impl Iterator<Item=usize>;

    fn iter_predecessors (self: &Self, v: usize) -> impl Iterator<Item=usize>;
}

pub mod algorithms;
pub mod unweighted;
pub mod weighted;

mod _tests;