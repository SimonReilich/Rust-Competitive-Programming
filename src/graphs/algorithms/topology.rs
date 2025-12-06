use std::collections::VecDeque;

use crate::graphs::Graph;

pub fn sort (graph: &impl Graph) -> Option<Vec<usize>> {
    let mut o: Vec<Option<usize>> = Vec::new();
    let mut pre: Vec<usize> = Vec::new();

    for v in 0 .. graph.vertex_count() {
        o.push(None);
        pre.push(graph.iter_predecessors(v).count());
    }

    let mut s = VecDeque::new();
    let mut i = 0;

    for v in 0 .. graph.vertex_count() {
        if pre[v] == 0 {
            i = ts_explore(v, graph, &mut o, &mut pre, &mut s, i);
        }
    }

    if i != graph.vertex_count() {
        return None;
    }

    let mut result = vec![0; graph.vertex_count()];
    for (v, order) in o.iter().enumerate() {
        if let Some(pos) = order {
            result[*pos] = v;
        } else {
            return None;
        }
    }
    Some(result)
}

fn ts_explore (v: usize, graph: &impl Graph, o: &mut Vec<Option<usize>>, pre: &mut Vec<usize>, s: &mut VecDeque<usize>, i: usize) -> usize {
    let mut j = i;
    if o[v].is_none() {
        s.push_back(v);
    }
    while let Some(u) = s.pop_front() {
        o[u] = Some(j);
        j += 1;
        for succ in graph.iter_successors(u) {
            pre[succ] -= 1;
            if pre[succ] == 0 {
                s.push_back(succ);
            }
        }
    }
    return j;
}

#[cfg(test)]
mod tests {
    use crate::graphs::Graph;
    use crate::graphs::unweighted::Unweighted;
    use crate::graphs::unweighted::directed_list::Graph as Directed;
    use super::sort;

    #[test]
    fn test_linear_graph() {
        // 0 -> 1 -> 2
        let mut graph = Directed::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let result = sort(&graph);
        assert_eq!(result, Some(vec![0, 1, 2]));
    }

    #[test]
    fn test_diamond_graph() {
        //   /-> 1 -\
        // 0          -> 3
        //   \-> 2 -/
        let mut graph = Directed::new();
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 3);

        let result = sort(&graph).expect("Should return a valid sort");
        
        // In a diamond, 0 must be first, 3 must be last. 
        // 1 and 2 can be in either order in the middle.
        assert_eq!(result[0], 0);
        assert_eq!(result[3], 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
    }

    #[test]
    fn test_disconnected_graph() {
        // 0 -> 1    2 -> 3
        let mut graph = Directed::new();
        graph.add_edge(0, 1);
        graph.add_edge(2, 3);

        let result = sort(&graph).unwrap();
        
        // Ensure constraints are respected
        // 0 comes before 1
        let pos_0 = result.iter().position(|&x| x == 0).unwrap();
        let pos_1 = result.iter().position(|&x| x == 1).unwrap();
        assert!(pos_0 < pos_1);

        // 2 comes before 3
        let pos_2 = result.iter().position(|&x| x == 2).unwrap();
        let pos_3 = result.iter().position(|&x| x == 3).unwrap();
        assert!(pos_2 < pos_3);
        
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_cycle_detection() {
        // 0 -> 1 -> 2 -> 0 (Cycle)
        let mut graph = Directed::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let result = sort(&graph);
        assert_eq!(result, None, "Cycle should return None");
    }

    #[test]
    fn test_empty_graph() {
        let graph = Directed::new();
        let result = sort(&graph);
        assert_eq!(result, Some(vec![]));
    }

    #[test]
    fn test_single_node() {
        let mut graph = Directed::new();
        graph.add_vertex();
        let result = sort(&graph);
        assert_eq!(result, Some(vec![0]));
    }
}