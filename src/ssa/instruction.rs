//! Instructions that exist in a basic block.

/// A reference to a value in an IR function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(pub(crate) usize);

impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "v{}", self.0)
    }
}

/// Operations that instructions can perform.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    /// Addition
    Add,
    /// Bitwise AND
    And,
    /// Equality comparison
    Eq,
    /// Less-than comparison
    Lt,
    /// Less-than-or-equal comparison
    Le,
}

/// An instruction that exists in a basic block.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    /// Computes a binary operation (v1 = v2 op v3)
    BinOp(BinaryOp, Value, Value, Value),
    /// Moves one value to another
    Move(Value, Value),
    /// Loads an integer constant
    Int(Value, i32),
}

impl core::fmt::Display for Instruction {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            Self::Int(dest, value) => write!(f, "{dest} = {value}"),
            Self::BinOp(op, dest, lhs, rhs) => {
                write!(f, "{dest} = {op:?} {lhs}, {rhs}")
            }
            Self::Move(dest, src) => write!(f, "{dest} = {src}"),
        }
    }
}
