use edged::dominance::frontiers;
use edged::graph::matrix::Graph;
use edged::graph::traits::Directed;

use super::basic_block::Terminator;
use super::instruction::{Instruction, Value};
use super::module::Module;
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
                Instruction::BinOp(_, dest, _, _) | Instruction::Move(dest, _) => {
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

#[cfg(test)]
mod tests {
    use crate::ssa::{
        basic_block::{BasicBlock, BasicBlockId, Terminator},
        instruction::{BinaryOp, Instruction, Value},
        module::Module,
    };

    use super::insert_phi_nodes;

    #[test]
    fn test() {
        // program from fig 3.1 of ssa book
        let mut module = Module::default();
        // entry
        module.blocks.push(BasicBlock {
            params: vec![],
            instructions: vec![],
            terminator: Terminator::Jump(BasicBlockId(1)),
        });

        // block A
        module.blocks.push(BasicBlock {
            params: vec![],
            instructions: vec![],
            terminator: Terminator::Branch(Value(2), BasicBlockId(2), BasicBlockId(3)),
        });

        // block B
        module.blocks.push(BasicBlock {
            params: vec![],
            instructions: vec![
                Instruction::Int(Value(0), 0), // x = 0
                Instruction::Int(Value(1), 0), // y = 0
            ],
            terminator: Terminator::Jump(BasicBlockId(4)),
        });

        // block C
        module.blocks.push(BasicBlock {
            params: vec![],
            instructions: vec![
                Instruction::Move(Value(2), Value(0)), // tmp = x
                Instruction::Move(Value(0), Value(1)), // x = y
                Instruction::Move(Value(1), Value(2)), // y = tmp
            ],
            terminator: Terminator::Branch(Value(0), BasicBlockId(4), BasicBlockId(5)),
        });

        // block D
        module.blocks.push(BasicBlock {
            params: vec![],
            instructions: vec![
                Instruction::BinOp(BinaryOp::Add, Value(0), Value(0), Value(1)), // x = x + y
            ],
            terminator: Terminator::Branch(Value(0), BasicBlockId(1), BasicBlockId(5)),
        });

        // block E
        module.blocks.push(BasicBlock {
            params: vec![],
            instructions: vec![],
            terminator: Terminator::Return(Value(0)),
        });

        insert_phi_nodes(&mut module);
        println!("{}", module);
    }
}
