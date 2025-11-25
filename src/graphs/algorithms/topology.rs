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
    return None;
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