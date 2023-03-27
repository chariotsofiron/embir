// Define the Graph struct
struct Graph {
    nodes: Vec<Vec<usize>>,
}

impl Graph {
    fn get_predecessors(&self, node: usize) -> Vec<usize> {
        let mut preds = Vec::new();
        for (i, n) in self.nodes.iter().enumerate() {
            if n.contains(&node) {
                preds.push(i);
            }
        }
        preds
    }

    /// Computes the dominator tree for the given graph
    /// Returns a vector of immediate dominators for each node
    fn compute_dominator_tree(&self, start_node: usize) -> Vec<usize> {
        // <https://www.cs.rice.edu/~keith/EMBED/dom.pdf>
        // <https://github.com/static-analysis-engineering/CodeHawk-Binary/blob/master/chb/app/Cfg.py>
        // Initialize the dominators array
        let mut idoms: Vec<Option<usize>> = vec![None; self.nodes.len()];

        let rev_post_order = self.reverse_postorder_traversal(start_node);

        idoms[start_node] = Some(start_node);
        let mut changed = true;
        while changed {
            changed = false;
            // Iterate over the nodes in reverse postorder (except for the start node)
            for &b in &rev_post_order {
                if b == start_node {
                    continue;
                }
                let allpreds = self.get_predecessors(b);
                let mut new_idom = None;

                // Find the first (processed) predecessor of b and set it as the initial new_idom
                for &pred in &allpreds {
                    if idoms[pred].is_some() {
                        new_idom = Some(pred);
                        break;
                    }
                }
                let mut new_idom = new_idom.unwrap();

                // Find the nearest common dominator of b's other predecessors
                for &p in &allpreds {
                    if idoms[p].is_some() {
                        new_idom = Self::intersect(p, new_idom, &idoms, &rev_post_order);
                    }
                }
                // Update the dominator of b if necessary
                if idoms[b] != Some(new_idom) {
                    idoms[b] = Some(new_idom);
                    changed = true;
                }
            }
        }
        idoms.into_iter().map(|x| x.unwrap()).collect()
    }

    // Compute the nearest common dominator of two nodes using the dominators array
    fn intersect(mut b1: usize, mut b2: usize, doms: &Vec<Option<usize>>, rpo: &[usize]) -> usize {
        let mut ordering = vec![0; rpo.len()];
        let mut i: usize = 0;
        for &b in rpo {
            ordering[b] = i;
            i += 1;
        }

        while b1 != b2 {
            // The paper describes comparisons on postorder numbers; we're using
            // the reverse-postorder numbers, so we need to flip the comparison
            while ordering[b1] > ordering[b2] {
                b1 = doms[b1].unwrap();
            }
            while ordering[b2] > ordering[b1] {
                b2 = doms[b2].unwrap();
            }
        }
        b1
    }

    /// Returns a vector of nodes in postorder traversal order.
    fn reverse_postorder_traversal(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.nodes.len()];
        let mut stack = vec![start];
        let mut path = Vec::new();

        visited[start] = true;
        while let Some(&node) = stack.last() {
            let mut tail = true;
            for &neighbor in &self.nodes[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    stack.push(neighbor);
                    tail = false;
                    break;
                }
            }
            if tail {
                path.push(stack.pop().unwrap());
            }
        }
        path.reverse();
        path
    }

    fn dominance_frontier(&self, start: usize) -> Vec<Vec<usize>> {
        let idoms = self.compute_dominator_tree(start);
        let mut dom_frontiers: Vec<Vec<usize>> = vec![Vec::new(); self.nodes.len()];
        for node in 0..self.nodes.len() {
            let allpreds = self.get_predecessors(node);
            if allpreds.len() >= 2 {
                for &pred in &allpreds {
                    let mut runner = pred;
                    while runner != idoms[node] {
                        dom_frontiers[runner].push(node);
                        runner = idoms[runner];
                    }
                }
            }
        }
        dom_frontiers
    }
}

#[test]
fn test() {
    let graph = Graph {
        // nodes: vec![
        //     /*1*/ vec![1],
        //     /*2*/ vec![0, 2],
        //     /*3*/ vec![1],
        //     /*4*/ vec![1, 2],
        //     /*5*/ vec![0],
        //     /*6*/ vec![3, 4],
        // ],
        nodes: vec![
            /*0*/ vec![1, 5],
            /*1*/ vec![2, 3],
            /*2*/ vec![4],
            /*3*/ vec![4],
            /*4*/ vec![5],
            /*5*/ vec![],
        ],
    };

    let start_node = 0;

    let traversal = graph.reverse_postorder_traversal(start_node);
    println!("{:?}", traversal);

    let dominators = graph.compute_dominator_tree(start_node);
    println!("{:?}", dominators);

    let dom_frontiers = graph.dominance_frontier(start_node);
    println!("{:?}", dom_frontiers);
}
