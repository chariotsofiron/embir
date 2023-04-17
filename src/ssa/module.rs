//! A collection of basic blocks.
use edged::graph::{matrix::Graph, traits::Directed};

use super::basic_block::{BasicBlock, Terminator};

/// A collection of basic blocks.
#[derive(Clone, Default)]
pub struct Module {
    /// The basic blocks in the module. Assumes that the first block is the entry block.
    pub(crate) blocks: Vec<BasicBlock>,
}

impl From<&Module> for Graph<(), Directed> {
    fn from(module: &Module) -> Self {
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
        edges.into_iter().collect()
    }
}

impl core::fmt::Display for Module {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (i, block) in self.blocks.iter().enumerate() {
            write!(f, "${i}")?;
            writeln!(f, "{block}")?;
        }
        Ok(())
    }
}
