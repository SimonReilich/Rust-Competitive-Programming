#[cfg(test)]
use crate::graphs::Graph;

#[cfg(test)]
pub fn test_add_vertex(mut graph: impl Graph) {
    let v0 = graph.add_vertex();
    let v1 = graph.add_vertex();

    assert_eq!(v0, 0);
    assert_eq!(v1, 1);
    assert_eq!(graph.vertex_count(), 2);
}

#[cfg(test)]
pub fn test_iter_vertices(mut graph: impl Graph) {
    graph.add_vertex(); // 0
    graph.add_vertex(); // 1
    graph.add_vertex(); // 2

    assert_eq!(graph.vertex_count(), 3);
}