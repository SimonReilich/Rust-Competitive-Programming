pub trait Directed {
    fn iter_successors (self: &Self, u: usize) -> impl Iterator<Item=usize>;

    fn iter_predecessors (self: &Self, v: usize) -> impl Iterator<Item=usize>;
}

pub trait Undirected {
    fn iter_neighbors (self: &Self, u: usize) -> impl Iterator<Item=usize>;
}

pub mod unweighted;
pub mod weighted;