//! Module builder.

use super::{
    algorithms::insert_phi_nodes,
    basic_block::{BasicBlock, BlockId, Terminator},
    instruction::{BinaryOp, Instruction, Value},
    module::Module,
};

/// [`ModuleBuilder`] represents the struct used for building a module
#[derive(Clone, Default)]
pub struct ModuleBuilder {
    /// The module being built.
    module: Module,
    /// The next value to be used.
    val_counter: usize,
    /// The current block being built.
    current_block: Option<BlockId>,
}

impl ModuleBuilder {
    #[must_use]
    // Applies the SSA algorithms and returns the module.
    // NOTE: This function invalidates any further usage of the ModuleBuilder
    // struct, and is only meant to be used when youre done generating SSA.
    pub fn build_module(self) -> Module {
        /* SSA ALGOS GO HERE */
        let mut module = self.module;
        insert_phi_nodes(&mut module);
        module
    }

    /// Pushes a new basic block to the module and returns the reference to it.
    pub fn push_bb(&mut self) -> BlockId {
        self.module
            .blocks
            .push(BasicBlock::new(Vec::new(), Vec::new(), Terminator::None));
        BlockId(self.module.blocks.len() - 1)
    }

    /// Sets the current block to the given block.
    pub fn switch_to_block(&mut self, id: BlockId) {
        self.current_block = Some(id);
    }

    /// Sets the terminator of the current block.
    ///
    /// # Panics
    ///
    /// Panics if the current block is `None`.
    pub fn set_terminator(&mut self, terminator: Terminator) {
        let id = self
            .current_block
            .expect("Malformed IR: Tried to set a terminator with no basic block selected.");
        self.module.blocks[id.0].terminator = terminator;
    }

    /// Pushes a new variable to the module and returns the reference to it.
    pub fn push_variable(&mut self) -> Value {
        let val = Value(self.val_counter);
        self.val_counter += 1;
        val
    }

    /// Builds a load int instruction.
    ///
    /// # Panics
    ///
    /// Panics if the current block is `None`.
    pub fn load_int(&mut self, var: Value, int: i32) {
        let id = self
            .current_block
            .expect("Malformed IR: Tried to load an int without a basic block selected.");
        self.module.blocks[id.0]
            .instructions
            .push(Instruction::Int(var, int));
    }

    /// Builds a move instruction.
    ///
    /// # Panics
    ///
    /// Panics if the current block is `None`.
    pub fn build_move(&mut self, from: Value, to: Value) {
        let id = self
            .current_block
            .expect("Malformed IR: Tried to build a move without a basic block selected.");
        self.module.blocks[id.0]
            .instructions
            .push(Instruction::Move(to, from));
    }

    /// Builds a binary operation instruction.
    ///
    /// # Panics
    ///
    /// Panics if the current block is `None`.
    pub fn build_binop(&mut self, to: Value, lhs: Value, rhs: Value, operation: BinaryOp) {
        let id = self.current_block.expect(
            "Malformed IR: Tried to build a binary operation without a basic block selected.",
        );
        self.module.blocks[id.0]
            .instructions
            .push(Instruction::BinOp(operation, to, lhs, rhs));
    }
}
