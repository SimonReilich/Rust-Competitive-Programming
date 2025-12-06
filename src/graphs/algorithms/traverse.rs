use std::collections::VecDeque;

use crate::graphs::Graph;

pub fn depth_first_search(graph: &impl Graph, start: usize) -> Vec<Option<usize>> {
    let mut o = Vec::new();
    for _ in 0..graph.vertex_count() {
        o.push(None);
    }
    dfs_explore(graph, &mut o, &mut vec![start], 0);
    return o;
}

fn dfs_explore(graph: &impl Graph, o: &mut Vec<Option<usize>>, s: &mut Vec<usize>, i: usize) {
    let mut j = i;
    while let Some(v) = s.pop() {
        if o[v].is_none() {
            o[v] = Some(j);
            j += 1;
            let mut succ: Vec<usize> = graph.iter_successors(v).collect();
            succ.sort();
            succ.reverse();
            for u in succ {
                s.push(u);
            }
        }
    }
}

pub fn breadth_first_search(graph: &impl Graph, start: usize) -> Vec<Option<usize>> {
    let mut o = Vec::new();
    for _ in 0..graph.vertex_count() {
        o.push(None);
    }
    let mut queue = VecDeque::new();
    queue.push_back(start);
    bfs_explore(graph, &mut o, &mut queue, 0);
    return o;
}

fn bfs_explore(graph: &impl Graph, o: &mut Vec<Option<usize>>, q: &mut VecDeque<usize>, i: usize) {
    let mut j = i;
    while let Some(v) = q.pop_back() {
        if o[v].is_none() {
            o[v] = Some(j);
            j += 1;
            let mut succ: Vec<usize> = graph.iter_successors(v).collect();
            succ.sort();
            for u in succ {
                q.push_front(u);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod dfs {
        use super::super::depth_first_search;
        use crate::graphs::unweighted::Unweighted;
        use crate::graphs::unweighted::directed_list::Graph as Directed;
        use crate::graphs::unweighted::undirected_list::Graph as Undirected;

        #[test]
        fn test_dfs_linear_path() {
            // Scenario: 0 -> 1 -> 2 -> 3
            let mut graph = Directed::new();
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);

            let result = depth_first_search(&graph, 0);
            assert_eq!(result, vec![Some(0), Some(1), Some(2), Some(3)]);
        }

        #[test]
        fn test_dfs_branching_tree() {
            // Scenario:
            //      0
            //     / \
            //    1   2
            //   /
            //  3
            // Visitation should be 0 -> 1 -> 3 -> 2 (due to sorting neighbors)
            let mut graph = Undirected::new();
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph.add_edge(1, 3);

            let result = depth_first_search(&graph, 0);
            assert_eq!(result, vec![Some(0), Some(1), Some(3), Some(2)]);
        }

        #[test]
        fn test_dfs_with_cycle() {
            // Scenario: 0 -> 1 -> 2 -> 0 (Back to start)
            // Should visit 0, 1, 2 and stop, avoiding infinite loop.
            let mut graph = Directed::new();
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);
            graph.add_edge(2, 0);

            let result = depth_first_search(&graph, 0);
            assert_eq!(result, vec![Some(0), Some(1), Some(2)]);
        }

        #[test]
        fn test_dfs_disconnected_graph() {
            // Scenario:
            // Component A: 0 -> 1
            // Component B: 2 -> 3
            // DFS starting at 0 should only see 0 and 1.
            let mut graph = Directed::new();
            graph.add_edge(0, 1);
            graph.add_edge(2, 3);

            let result = depth_first_search(&graph, 0);
            assert_eq!(result, vec![Some(0), Some(1), None, None]);
        }

        #[test]
        fn test_dfs_single_node() {
            let mut graph = Undirected::new();

            // Test single node graph (self loop or just exists)
            graph.add_edge(0, 0); // Node pointing to self
            let result_single = depth_first_search(&graph, 0);
            assert_eq!(result_single, vec![Some(0)]);
        }

        #[test]
        fn test_dfs_complex_traversal() {
            // Scenario:
            // 0 -> 1 -> 3
            // |    |
            // v    v
            // 2 -> 4
            //
            // Neighbors of 0: [1, 2] -> Visits 1 first
            // Neighbors of 1: [3, 4] -> Visits 3 first
            // Backtrack to 2 -> Visits 4
            // Backtrack to 0 -> Visits 2
            // 2 -> 4 (Already visited)
            // Order: 0, 1, 3, 4, 2

            let mut graph = Directed::new();
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph.add_edge(1, 3);
            graph.add_edge(1, 4);
            graph.add_edge(2, 4);

            let result = depth_first_search(&graph, 0);
            assert_eq!(result, vec![Some(0), Some(1), Some(4), Some(2), Some(3)]);
        }
    }

    mod bfs {
        use super::super::breadth_first_search;
        use crate::graphs::unweighted::Unweighted;
        use crate::graphs::unweighted::directed_list::Graph as Directed;
        use crate::graphs::unweighted::undirected_list::Graph as Undirected;

        #[test]
        fn test_bfs_linear_path() {
            // Scenario: 0 -> 1 -> 2 -> 3
            // BFS order matches DFS here because there are no branches.
            let mut graph = Directed::new();
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);

            let result = breadth_first_search(&graph, 0);
            assert_eq!(result, vec![Some(0), Some(1), Some(2), Some(3)]);
        }

        #[test]
        fn test_bfs_branching_tree() {
            // Scenario:
            //      0
            //     / \
            //    1   2
            //   /
            //  3
            //
            // BFS Order:
            // Level 0: 0
            // Level 1: 1, 2 (Neighbors of 0)
            // Level 2: 3 (Neighbor of 1)
            // Result: 0 -> 1 -> 2 -> 3
            let mut graph = Undirected::new();
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph.add_edge(1, 3);

            let result = breadth_first_search(&graph, 0);
            assert_eq!(result, vec![Some(0), Some(1), Some(2), Some(3)]);
        }

        #[test]
        fn test_bfs_with_cycle() {
            // Scenario: 0 -> 1 -> 2 -> 0 (Back to start)
            // Should visit 0, 1, 2 and stop.
            let mut graph = Directed::new();
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);
            graph.add_edge(2, 0);

            let result = breadth_first_search(&graph, 0);
            assert_eq!(result, vec![Some(0), Some(1), Some(2)]);
        }

        #[test]
        fn test_bfs_disconnected_graph() {
            // Scenario:
            // Component A: 0 -> 1
            // Component B: 2 -> 3
            // BFS starting at 0 should only see 0 and 1.
            let mut graph = Directed::new();
            graph.add_edge(0, 1);
            graph.add_edge(2, 3);

            let result = breadth_first_search(&graph, 0);
            assert_eq!(result, vec![Some(0), Some(1), None, None]);
        }

        #[test]
        fn test_bfs_single_node() {
            let mut graph = Undirected::new();

            // 2. Test single node graph (self loop or just exists)
            graph.add_edge(0, 0);
            let result_single = breadth_first_search(&graph, 0);
            assert_eq!(result_single, vec![Some(0)]);
        }

        #[test]
        fn test_bfs_complex_traversal() {
            // Scenario:
            // 0 -> 1 -> 3
            // |    |
            // v    v
            // 2 -> 4
            //
            // Neighbors of 0: [1, 2]
            // Neighbors of 1: [3, 4]
            // Neighbors of 2: [4]
            //
            // BFS Execution:
            // 1. Visit 0. Q: [1, 2] (Sorted neighbors)
            // 2. Pop 1. Visit 1. Add 3, 4. Q: [2, 3, 4] (Sorted neighbors)
            // 3. Pop 2. Visit 2. Neighbor 4 already visited/in-queue?
            //    (In this impl, we check visited on push. 4 is added when processing 1.
            //    So when processing 2, 4 is skipped if already visited.)
            // 4. Pop 3. Visit 3.
            // 5. Pop 4. Visit 4.
            //
            // Order: 0, 1, 2, 3, 4

            let mut graph = Directed::new();
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph.add_edge(1, 3);
            graph.add_edge(1, 4);
            graph.add_edge(2, 4);

            let result = breadth_first_search(&graph, 0);
            assert_eq!(result, vec![Some(0), Some(1), Some(2), Some(3), Some(4)]);
        }
    }
}
