use std::collections::HashSet;

use super::{
    basic_block::{BasicBlock, BasicBlockId, Terminator},
    instruction::{Instruction, Value},
};

pub struct Cfg {
    pub blocks: Vec<BasicBlock>,
}

impl Cfg {
    fn get_all_variables(&self) -> Vec<Value> {
        let mut variables = Vec::new();
        for block in &self.blocks {
            for instruction in &block.instructions {
                match instruction {
                    Instruction::Int(dest, _) => {
                        // all variables will be initialized with an int instruction
                        variables.push(*dest);
                    }
                    _ => {}
                }
            }
        }
        variables
    }

    fn get_blocks_modifying_variable(&self, variable: Value) -> Vec<usize> {
        let mut blocks = Vec::new();
        for (i, block) in self.blocks.iter().enumerate() {
            for instruction in &block.instructions {
                match instruction {
                    Instruction::Int(dest, _) | Instruction::Compute(_, dest, _, _) => {
                        if *dest == variable {
                            blocks.push(i);
                        }
                    }
                    _ => {}
                }
            }
        }
        blocks
    }

    fn insert_phi_nodes(&mut self) {
        let variables = self.get_all_variables();
        for variable in variables {
            let mut has_already = vec![false; self.blocks.len()];
            let mut ever_worklist: Vec<usize> = Vec::new();
            let mut worklist: Vec<usize> = Vec::new();

            for block in self.get_blocks_modifying_variable(variable) {
                ever_worklist.push(block);
                worklist.push(block);
            }

            while let Some(block) = worklist.pop() {}
        }
    }

    fn to_adjacency_list(&self) -> Vec<Vec<BasicBlockId>> {
        let mut adjacency_list = vec![Vec::new(); self.blocks.len()];
        for (i, block) in self.blocks.iter().enumerate() {
            match block.terminator {
                Terminator::Jump(dest) => {
                    adjacency_list[i].push(dest);
                }
                Terminator::Branch(_, t, e) => {
                    adjacency_list[i].push(t);
                    adjacency_list[i].push(e);
                }
                _ => {}
            }
        }
        adjacency_list
    }

    /// Returns a vector of nodes in postorder traversal order.
    fn reverse_postorder_traversal(&self, start: BasicBlockId) -> Vec<BasicBlockId> {
        let graph = self.to_adjacency_list();
        let mut visited = HashSet::new();
        let mut stack = vec![start];
        let mut path = Vec::new();

        visited.insert(start);
        while let Some(&node) = stack.last() {
            let mut tail = true;
            for &neighbor in &graph[node] {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
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
}
