/*
 * Copyright (c) 2024 The YuvaKriti Lang Authors.
 *
 * This program is free software: you can redistribute it and/or modify it under the
 *  terms of the GNU General Public License as published by the Free Software
 *  Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this
 * program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::cmp::max;
use std::mem::size_of;

use crate::bytecode::bytes::AssertingByteConversions;
use crate::bytecode::opcode::OpCode;
use crate::bytecode::opcode::OpCodeExt;
use crate::bytecode::opcode::OpSize;
#[allow(unused_imports)]
use crate::bytecode::ConstantEntry;
use crate::bytecode::CpSize;

pub type CodeSize = u32;
pub const OP_SIZE: CodeSize = size_of::<OpSize>() as CodeSize;

pub const CODE: &str = "Code";
pub const SOURCE_FILE: &str = "SourceFile";

#[derive(Debug, PartialEq, Clone)]
pub enum Attr {
    Code(Code),
    SourceFile(SourceFile),
}

impl Attr {
    pub fn name(&self) -> &'static str {
        match self {
            Attr::Code(_) => CODE,
            Attr::SourceFile(_) => SOURCE_FILE,
        }
    }
}

/// The Code attribute contains the instructions for a function or the top-level inststructions
/// in a program.
#[derive(Debug, PartialEq, Clone)]
pub struct Code {
    max_stack: u16,
    instructions: Vec<u8>,
}

impl Code {
    /// The number of bytes of instructions that can be written to the [Attr::Code] attribute.
    pub const MAX_INSN_SIZE: CodeSize = 0xFFFFFFFF;

    /// The maximum (overall) depth of the operand stack for [Code] attributes.
    pub const MAX_STACK_SIZE: u16 = 0xFFFF;

    /// Create a new [Code] attribute with the given `max_stack` size and the capacity for the
    /// instruction bytes. `max_stack` is the maximum depth of the operand stack of the [Code]
    /// attribute at any point during the execution of instructions of that [Code] attribute.
    pub fn new(max_stack: u16, insns_capacity: CodeSize) -> Self {
        return Self::with_insns(max_stack, Vec::with_capacity(insns_capacity as usize));
    }

    /// Create a new [Code] attribute with the given `max_stack` size and the instruction bytes.
    /// `max_stack` is the maximum depth of the operand stack of the [Code]
    /// attribute at any point during the execution of instructions of that [Code] attribute.
    pub fn with_insns(max_stack: u16, insns: Vec<u8>) -> Self {
        return Code {
            max_stack,
            instructions: insns,
        };
    }

    /// The maximum depth of the operand stack for this code attribute.
    pub fn max_stack(&self) -> u16 {
        return self.max_stack;
    }

    /// Update the maximum stack size for this code attribute, based on the given stack effect.
    pub fn update_max_stack(&mut self, stack_effect: i8) {
        self.max_stack = max(
            self.max_stack,
            self.max_stack
                .checked_add_signed(stack_effect as i16)
                .unwrap_or(self.max_stack),
        );
    }

    fn check_size(&self, additional: CodeSize) {
        if self.instructions.len().as_code_size() + additional > Self::MAX_INSN_SIZE {
            panic!("Instruction size too large!");
        }
    }

    fn ensure_size_incr(&mut self, additional: CodeSize) {
        self.check_size(additional);
        self.instructions.reserve(additional as usize);
    }

    pub fn instructions(&self) -> &Vec<u8> {
        return &self.instructions;
    }

    pub fn push_insns_0(&mut self, opcode: OpCode) {
        self.ensure_size_incr(OP_SIZE);
        self.instructions.push(opcode.as_op_size());
        self.update_max_stack(opcode.stack_effect());
    }

    pub fn push_insns_1(&mut self, opcode: OpCode, operand: u8) {
        self.ensure_size_incr(OP_SIZE + 1);
        self.instructions.extend([opcode.as_op_size(), operand]);
        self.update_max_stack(opcode.stack_effect());
    }

    pub fn push_insns_1_16(&mut self, opcode: OpCode, operand: u16) {
        self.ensure_size_incr(OP_SIZE + 2);
        self.instructions.push(opcode.as_op_size());
        self.instructions
            .extend([(operand >> 8) as u8, operand as u8]);
        self.update_max_stack(opcode.stack_effect());
    }

    pub fn push_insns_2(&mut self, opcode: OpCode, operand1: u8, operand2: u8) {
        self.ensure_size_incr(OP_SIZE + 2);
        self.instructions
            .extend([opcode.as_op_size(), operand1, operand2]);
        self.update_max_stack(opcode.stack_effect());
    }

    pub fn push_insns_3(&mut self, opcode: OpCode, operand1: u8, operand2: u8, operand3: u8) {
        self.ensure_size_incr(OP_SIZE + 3);
        self.instructions
            .extend([opcode.as_op_size(), operand1, operand2, operand3]);
        self.update_max_stack(opcode.stack_effect());
    }

    pub fn push_insns_n(&mut self, opcode: OpCode, operands: &[u8]) {
        let len = operands.len();
        self.ensure_size_incr(OP_SIZE + len.as_code_size());

        self.instructions.push(opcode.as_op_size());
        self.instructions.extend(operands);
        self.update_max_stack(opcode.stack_effect());
    }
}

/// The SourceFile attribute contains the index of the [ConstantEntry::Utf8] constant pool entry
/// containing the source file name.
#[derive(Debug, PartialEq, Clone)]
pub struct SourceFile {
    pub name_index: CpSize,
}

impl SourceFile {
    pub fn new(name_index: CpSize) -> Self {
        return SourceFile { name_index };
    }
}
