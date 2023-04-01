use crate::ssa::instruction::{Instruction, Value};

/// [`BasicBlockId`] represents a reference to a basic block in an IR function. See
/// [`ModuleBuilder::push_block`] for details on what a basic block is.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BasicBlockId(pub usize);

impl std::fmt::Display for BasicBlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}

/// [`Terminator`] terminates a given basic block. For information on basic blocks, see
/// [`ModuleBuilder::push_block`].
#[derive(Debug, Clone)]
pub enum Terminator {
    /// No terminator has been added to the block yet. Note that compiling blocks with this as its
    /// terminator results in undefined behaviour.
    NoTerminator,

    /// The block ends with a return with no value.
    ReturnVoid,

    /// The block ends with a return with a value.
    Return(Value),

    /// The block ends with a jump to another block.
    Jump(BasicBlockId),

    /// The block ends with a branch to two different blocks depending on the truthiness of the
    /// value. If the value is true, it jumps to the first block; otherwise, it jumps to the second
    /// block.
    Branch(Value, BasicBlockId, BasicBlockId),
}

impl std::fmt::Display for Terminator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terminator::NoTerminator => write!(f, "noterm"),
            Terminator::ReturnVoid => write!(f, "ret void"),
            Terminator::Return(v) => write!(f, "ret {v}"),
            Terminator::Jump(b) => write!(f, "jump {b}"),
            Terminator::Branch(c, t, e) => write!(f, "branch {c}, {t}, {e}"),
        }
    }
}

pub struct BasicBlock {
    pub params: Vec<Value>,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

impl BasicBlock {
    pub fn new(params: Vec<Value>, instructions: Vec<Instruction>, terminator: Terminator) -> Self {
        Self {
            params,
            instructions,
            terminator,
        }
    }
}

impl std::fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.params.is_empty() {
            writeln!(f, ":")?;
        } else {
            writeln!(
                f,
                "({}):",
                self.params
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        for instruction in self.instructions.iter() {
            writeln!(f, "    {}", instruction)?;
        }
        writeln!(f, "    {}", self.terminator)
    }
}
