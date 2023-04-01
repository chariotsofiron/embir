use super::basic_block::BasicBlock;

/// A collection of basic blocks.
pub struct Module {
    /// The basic blocks in the module. Assumes that the first block is the entry block.
    pub blocks: Vec<BasicBlock>,
}

impl Default for Module {
    fn default() -> Self {
        Self { blocks: Vec::new() }
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, block) in self.blocks.iter().enumerate() {
            write!(f, "${}", i)?;
            writeln!(f, "{}", block)?;
        }
        Ok(())
    }
}
