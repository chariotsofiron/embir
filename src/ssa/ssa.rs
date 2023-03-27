use edged::dominance::frontiers;
use edged::graph::matrix::Graph;
use edged::graph::traits::Directed;

use super::basic_block::Terminator;
use super::module::Module;
use std::collections::HashSet;
use std::iter::FromIterator;

fn convert_ssa_to_graph(module: &Module) {
    let mut edges = Vec::new();
    for (block_id, block) in module.blocks.iter().enumerate() {
        match block.terminator {
            Terminator::Jump(target) => {
                edges.push((block_id, target.0));
            }
            Terminator::Branch(_, yes, no) => {
                edges.push((block_id, yes.0));
                edges.push((block_id, no.0));
            }
            _ => {}
        }
    }

    let graph: Graph<(), Directed> = edges.into_iter().collect();
    let fronts = frontiers(&graph, 0);
}


/// A basic block in the SSA form.
/// Algorithm 3.1 from SSA book
fn insert_params(dom_fronts: Vec<Vec<usize>>) {

    // set of basic blocks where phi is added
    let mut f = HashSet::<usize>::new();
    // set of basic blocks that contain definitions of `variable`
    let mut w = Vec::<usize>::new();

    for variable in variables {

        for block in blocks {
            if "block contains assignment to variable" {
                w.push(block);
                break // break since we only need to find one block
            }
        }

        while let Some(block) = w.pop() {
            for child in dom_fronts[block] {
                // blah
            }
        }
    }

}
