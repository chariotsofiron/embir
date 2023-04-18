//! Basic block module;

use crate::ssa::instruction::{Instruction, Value};

/// [`BasicBlockId`] represents a reference to a basic block in an IR function. See
/// [`ModuleBuilder::push_block`] for details on what a basic block is.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlockId(pub(crate) usize);

impl core::fmt::Display for BlockId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "${}", self.0)
    }
}

/// [`Terminator`] terminates a given basic block. For information on basic blocks, see
/// [`ModuleBuilder::push_block`].
#[derive(Debug, Clone)]
pub enum Terminator {
    /// No terminator has been added to the block yet. Note that compiling blocks with this as its
    /// terminator results in undefined behaviour.
    None,

    /// The block ends with a return with no value.
    ReturnVoid,

    /// The block ends with a return with a value.
    Return(Value),

    /// The block ends with a jump to another block.
    Jump(BlockId),

    /// The block ends with a branch to two different blocks depending on the truthiness of the
    /// value. If the value is true, it jumps to the first block; otherwise, it jumps to the second
    /// block.
    Branch(Value, BlockId, BlockId),
}

impl core::fmt::Display for Terminator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::None => write!(f, "noterm"),
            Self::ReturnVoid => write!(f, "ret void"),
            Self::Return(v) => write!(f, "ret {v}"),
            Self::Jump(b) => write!(f, "jump {b}"),
            Self::Branch(c, t, e) => write!(f, "branch {c}, {t}, {e}"),
        }
    }
}

/// A basic block is a sequence of instructions that ends with a terminator.
#[derive(Clone)]
pub struct BasicBlock {
    /// The parameters of the basic block.
    pub(crate) params: Vec<Value>,
    /// The instructions of the basic block.
    pub(crate) instructions: Vec<Instruction>,
    /// The terminator of the basic block.
    pub(crate) terminator: Terminator,
}

impl BasicBlock {
    /// Creates a new basic block with the given parameters, instructions, and terminator.
    #[must_use]
    pub fn new(params: Vec<Value>, instructions: Vec<Instruction>, terminator: Terminator) -> Self {
        Self {
            params,
            instructions,
            terminator,
        }
    }
}

impl core::fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.params.is_empty() {
            writeln!(f, ":")?;
        } else {
            writeln!(
                f,
                "({}):",
                self.params
                    .iter()
                    .map(alloc::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        for instruction in &self.instructions {
            writeln!(f, "    {instruction}")?;
        }
        writeln!(f, "    {}", self.terminator)
    }
}
