use crate::graphs::unweighted::Unweighted;

// Helper to sort results from iterators for consistent comparison
fn sorted<I: Iterator<Item = T>, T: Ord>(iter: I) -> Vec<T> {
    let mut v: Vec<T> = iter.collect();
    v.sort();
    v
}

pub fn test_new_graph_is_empty(graph: impl Unweighted) {
    assert_eq!(graph.vertex_count(), 0);
    assert_eq!(graph.iter_edges().count(), 0);
}

pub fn test_new_n_initializes_vertices(graph: impl Unweighted, n: usize) {
    assert_eq!(graph.vertex_count(), n);

    // Should have no edges yet
    assert_eq!(graph.iter_edges().count(), 0);
}

pub mod directed {
    use crate::graphs::{Directed, unweighted::Unweighted};

    pub fn test_add_and_check_edge(mut graph: impl Unweighted) {
        // Add edge 0 -> 1
        graph.add_edge(0, 1);

        assert!(graph.has_edge(0, 1), "Edge 0->1 should exist");
        assert!(
            !graph.has_edge(1, 0),
            "Edge 1->0 should NOT exist (directed)"
        );
        assert!(!graph.has_edge(0, 2), "Edge 0->2 should NOT exist");
    }

    pub fn test_successors(mut graph: impl Unweighted + Directed) {
        // 0 points to 1 and 2
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);

        // 1 points to 3
        graph.add_edge(1, 3);

        let succ_0 = super::sorted(graph.iter_successors(0));
        assert_eq!(succ_0, vec![1, 2]);

        let succ_1 = super::sorted(graph.iter_successors(1));
        assert_eq!(succ_1, vec![3]);

        let succ_3 = super::sorted(graph.iter_successors(3));
        assert!(succ_3.is_empty());
    }

    pub fn test_predecessors(mut graph: impl Unweighted + Directed) {
        // 0 -> 2
        // 1 -> 2
        // 2 -> 3
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        let pred_2 = super::sorted(graph.iter_predecessors(2));
        assert_eq!(pred_2, vec![0, 1]);

        let pred_3 = super::sorted(graph.iter_predecessors(3));
        assert_eq!(pred_3, vec![2]);

        let pred_0 = super::sorted(graph.iter_predecessors(0));
        assert!(pred_0.is_empty());
    }

    pub fn test_self_loop(mut graph: impl Unweighted + Directed) {
        graph.add_edge(0, 0);

        assert!(graph.has_edge(0, 0));

        let succ = super::sorted(graph.iter_successors(0));
        assert_eq!(succ, vec![0]);

        let pred = super::sorted(graph.iter_predecessors(0));
        assert_eq!(pred, vec![0]);
    }

    pub fn test_iter_edges_global(mut graph: impl Unweighted) {
        // Create a cycle: 0 -> 1 -> 2 -> 0
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let edges = super::sorted(graph.iter_edges());

        let expected = vec![(0, 1), (1, 2), (2, 0)];
        assert_eq!(edges.len(), 3);
        // Note: Using sorted ensures order doesn't fail the test
        // provided the tuple comparison works (which it does in Rust).
        assert_eq!(edges, expected);
    }
}

pub mod undirected {
    use crate::graphs::{Undirected, unweighted::Unweighted};

    pub fn test_add_and_check_edge(mut graph: impl Unweighted) {
        // Add edge 0 -> 1
        graph.add_edge(0, 1);

        assert!(graph.has_edge(0, 1), "Edge 0->1 should exist");
        assert!(graph.has_edge(1, 0), "Edge 1->0 should exist (undirected)");
        assert!(!graph.has_edge(0, 2), "Edge 0->2 should NOT exist");
    }

    pub fn test_neighbors(mut graph: impl Unweighted + Undirected) {
        // 0 -- 2
        // 1 -- 2
        // 2 -- 3
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_vertex();

        let n_2 = super::sorted(graph.iter_neighbors(2));
        assert_eq!(n_2, vec![0, 1, 3]);

        let n_3 = super::sorted(graph.iter_neighbors(3));
        assert_eq!(n_3, vec![2]);

        let n_4 = super::sorted(graph.iter_neighbors(4));
        assert!(n_4.is_empty());
    }

    pub fn test_self_loop(mut graph: impl Unweighted + Undirected) {
        graph.add_edge(0, 0);

        assert!(graph.has_edge(0, 0));

        let n = super::sorted(graph.iter_neighbors(0));
        assert_eq!(n, vec![0]);
    }

    pub fn test_iter_edges_global(mut graph: impl Unweighted) {
        // Create a cycle: 0 -> 1 -> 2 -> 0
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let edges = super::sorted(graph.iter_edges());

        let expected = vec![(0, 1), (0, 2), (1, 2)];
        // expecting all pairs to be ordered
        assert_eq!(edges.len(), 3);
        // Note: Using sorted ensures order doesn't fail the test
        // provided the tuple comparison works (which it does in Rust).
        assert_eq!(edges, expected);
    }
}
