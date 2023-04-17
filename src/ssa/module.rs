//! A collection of basic blocks.
use super::basic_block::BasicBlock;

/// A collection of basic blocks.
#[derive(Clone, Default)]
pub struct Module {
    /// The basic blocks in the module. Assumes that the first block is the entry block.
    pub(crate) blocks: Vec<BasicBlock>,
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
