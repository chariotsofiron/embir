use edged::dominance::frontiers;
use edged::graph::matrix::Graph;
use edged::graph::traits::Directed;

use super::basic_block::Terminator;
use super::instruction::{Instruction, Value};
use super::module::Module;
use std::collections::{HashMap, HashSet};

fn get_dom_fronts(module: &Module) -> Vec<Vec<usize>> {
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
    frontiers(&graph, 0)
}

/// A basic block in the SSA form.
/// Algorithm 3.1 from SSA book
/// Accepts a program that hasn't been converted to SSA form.
pub fn insert_phi_nodes(module: &mut Module) {
    // for each value, a list of basic blocks that assign to it
    let mut definitions: HashMap<Value, Vec<usize>> = HashMap::new();
    for (i, block) in module.blocks.iter().enumerate() {
        for inst in block.instructions.iter() {
            match inst {
                Instruction::Compute(_, dest, _, _) => {
                    definitions.entry(*dest).or_insert_with(Vec::new).push(i);
                }
                _ => {}
            }
        }
    }
    let dom_fronts = get_dom_fronts(module);

    // set of basic blocks where phi is added
    let mut f = HashSet::<usize>::new();

    for (variable, defs) in definitions {
        // set of basic blocks that contain definitions of `variable`
        let mut w = defs.clone();
        // remove a basic block `block` from `w`
        while let Some(x) = w.pop() {
            for &y in &dom_fronts[x] {
                if !f.contains(&y) {
                    module.blocks[y].params.push(variable);
                    f.insert(y);
                    w.push(y);
                    if !defs.contains(&y) {
                        w.push(y);
                    }
                }
            }
        }
    }
}
