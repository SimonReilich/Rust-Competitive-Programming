pub trait Graph {
    fn add_vertex (self: &mut Self) -> usize;

    fn has_edge (self: &Self, u: usize, v: usize) -> bool;

    fn vertex_count (self: &Self) -> usize;
}

pub trait Directed {
    fn iter_successors (self: &Self, u: usize) -> impl Iterator<Item=usize>;

    fn iter_predecessors (self: &Self, v: usize) -> impl Iterator<Item=usize>;
}

pub trait Undirected {
    fn iter_neighbors (self: &Self, u: usize) -> impl Iterator<Item=usize>;
}

pub mod unweighted;
pub mod weighted;

mod _tests;