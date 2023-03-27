#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(pub usize);

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "v{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label {
    pub name: String,
    pub params: Vec<Value>,
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.params.is_empty() {
            write!(f, "{}", self.name)
        } else {
            write!(
                f,
                "{}({})",
                self.name,
                self.params
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

/// Three address code instruction
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThreeOp {
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
    ThreeOp(ThreeOp, Value, Value, Value),
    /// Loads an integer constant
    Int(Value, i32),
    Move(Value, Value),
    Label(Label),
    /// Jumps to the given label if the value is truthy
    Bnz(Value, Label),
    /// Jumps unconditionaly to the given label
    Jump(Label),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Instruction::Int(dest, value) => write!(f, "    {dest} = {value}"),
            Instruction::Label(label) => write!(f, "{label}:"),
            Instruction::Move(dest, src) => write!(f, "    {dest} = {src}"),
            Instruction::ThreeOp(op, dest, lhs, rhs) => {
                write!(f, "    {dest} = {op:?} {lhs}, {rhs}")
            }
            Instruction::Bnz(value, label) => write!(f, "    bnz {value}, {label}"),
            Instruction::Jump(label) => write!(f, "    jump {label}"),
            _ => panic!("Not implemented: {self:#?}"),
        }
    }
}
