use super::{
    basic_block::{BasicBlock, BasicBlockId, Terminator},
    instruction::{BinaryOp, Instruction, Value},
    module::Module,
};

// [`ModuleBuilder`] represents the struct used for building a module
pub struct ModuleBuilder {
    module: Module,
    val_counter: usize,
    current_block: Option<BasicBlockId>,
}

impl ModuleBuilder {
    pub fn new() -> ModuleBuilder {
        ModuleBuilder {
            module: Module::default(),
            val_counter: 0,
            current_block: None,
        }
    }

    pub fn build_module(&self) -> Module {
        self.module.clone()
    }

    pub fn push_bb(&mut self) -> BasicBlockId {
        self.module.blocks.push(BasicBlock::new(
            Vec::new(),
            Vec::new(),
            Terminator::NoTerminator,
        ));
        BasicBlockId(self.module.blocks.len() - 1)
    }

    pub fn switch_to_block(&mut self, id: BasicBlockId) {
        self.current_block = Some(id);
    }

    pub fn set_terminator(&mut self, terminator: Terminator) {
        if let Some(id) = self.current_block {
            self.module.blocks[id.0].terminator = terminator;
        } else {
            panic!("Malformed IR: Tried to select a terminator with no basic block selected.");
        }
    }

    pub fn push_variable(&mut self) -> Value {
        let val = Value(self.val_counter);
        self.val_counter += 1;
        val
    }

    pub fn load_int(&mut self, var: Value, int: i32) {
        if let Some(id) = self.current_block {
            self.module.blocks[id.0]
                .instructions
                .push(Instruction::Int(var, int));
        } else {
            panic!("Malformed IR: Tried to load an int without a basic block selected.");
        }
    }

    pub fn build_move(&mut self, from: Value, to: Value) {
        if let Some(id) = self.current_block {
            self.module.blocks[id.0]
                .instructions
                .push(Instruction::Move(to, from));
        } else {
            panic!("Malformed IR: Tried to build a move without a basic block selected.");
        }
    }

    pub fn build_binop(&mut self, to: Value, lhs: Value, rhs: Value, operation: BinaryOp) {
        if let Some(id) = self.current_block {
            self.module.blocks[id.0]
                .instructions
                .push(Instruction::BinOp(operation, to, lhs, rhs));
        } else {
            panic!("Malformed IR: Tried to build a move without a basic block selected.");
        }
    }
}
