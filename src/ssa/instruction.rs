#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(pub usize);

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "v{}", self.0)
    }
}

/// Three address code instruction
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operation {
    /// Addition
    Add,
    Eq,
    /// Less-than comparison
    Lt,
    /// Less-than-or-equal comparison
    Le,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    Compute(Operation, Value, Value, Value),
    /// Loads an integer constant
    Int(Value, i32),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Instruction::Int(dest, value) => write!(f, "{dest} = {value}"),
            Instruction::Compute(op, dest, lhs, rhs) => {
                write!(f, "{dest} = {op:?} {lhs}, {rhs}")
            }
            _ => panic!("Not implemented: {self:#?}"),
        }
    }
}
