//! Algorithms for converting a module to SSA form.
use super::basic_block::Terminator;
use super::instruction::{Instruction, Value};
use super::module::Module;
use edged::dominance::frontiers;
use edged::graph::matrix::Graph;
use edged::graph::traits::Directed;
use std::collections::{HashMap, HashSet};

/// Computes the dominance frontiers of a module.
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
            Terminator::None | Terminator::ReturnVoid | Terminator::Return(_) => {}
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
        for inst in &block.instructions {
            match *inst {
                Instruction::BinOp(_, dest, _, _) | Instruction::Move(dest, _) => {
                    definitions.entry(dest).or_insert_with(Vec::new).push(i);
                }
                Instruction::Int(_, _) => {}
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

#[cfg(test)]
mod tests {
    use crate::ssa::{
        basic_block::{BasicBlock, BlockId, Terminator},
        builder::ModuleBuilder,
        instruction::{BinaryOp, Instruction, Value},
        module::Module,
    };

    use super::insert_phi_nodes;

    #[test]
    fn test() {
        // program from fig 3.1 of ssa book
        let mut builder = ModuleBuilder::default();
        // basic block definition
        let entry = builder.push_bb();
        let block_a = builder.push_bb();
        let block_b = builder.push_bb();
        let block_c = builder.push_bb();
        let block_d = builder.push_bb();
        let block_e = builder.push_bb();

        builder.switch_to_block(entry);
        builder.set_terminator(Terminator::Jump(block_a));

        // block B
        builder.switch_to_block(block_b);
        let x = builder.push_variable();
        let y = builder.push_variable();
        builder.load_int(x, 0);
        builder.load_int(y, 0);
        builder.set_terminator(Terminator::Jump(block_d));

        // block C
        builder.switch_to_block(block_c);
        let tmp = builder.push_variable();
        builder.build_move(x, tmp);
        builder.build_move(y, x);
        builder.build_move(tmp, y);
        builder.set_terminator(Terminator::Branch(x, block_d, block_e));

        // block D
        builder.switch_to_block(block_d);
        builder.build_binop(x, x, y, BinaryOp::Add);
        builder.set_terminator(Terminator::Branch(x, block_a, block_e));

        // block E
        builder.switch_to_block(block_e);
        builder.set_terminator(Terminator::Return(x));

        // block A
        builder.switch_to_block(block_a);
        builder.set_terminator(Terminator::Branch(tmp, block_b, block_c));

        let mut module = builder.build_module();
        insert_phi_nodes(&mut module);
        println!("{}", module);
    }
}
