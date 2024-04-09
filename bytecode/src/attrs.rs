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

#[allow(unused_imports)]
use crate::ConstantEntry;
use crate::CpSize;
use crate::opcode::{OpCode, OpSize};

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
    pub pc: CodeSize,
    pub instructions: Vec<u8>,
}

impl Code {
    /// The number of bytes of instructions that can be written to the [Attr::Code] attribute.
    pub const MAX_INSN_SIZE: CodeSize = 0xFFFF;

    pub fn new() -> Self {
        return Self::with_capacity(0);
    }

    pub fn with_capacity(capacity: usize) -> Self {
        return Code {
            pc: 0,
            instructions: Vec::with_capacity(capacity),
        };
    }

    pub fn with_insns(insns: Vec<u8>) -> Self {
        return Code {
            pc: 0,
            instructions: insns,
        };
    }

    fn check_size(&self, extend: CodeSize) {
        if self.instructions.len() as CodeSize + extend > Self::MAX_INSN_SIZE {
            panic!("Instruction size too large!");
        }
    }

    fn ensure_size_incr(&mut self, extend: CodeSize) {
        self.check_size(extend);
        self.instructions.resize(
            max(
                self.instructions.len(),
                self.instructions.len() + extend as usize,
            ),
            0,
        );
    }

    pub fn instructions(&self) -> &Vec<u8> {
        return &self.instructions;
    }

    pub fn push_insns_0(&mut self, opcode: OpCode) {
        self.ensure_size_incr(OP_SIZE);
        self.instructions[self.pc as usize] = opcode as OpSize;
        self.pc += 1;
    }

    pub fn push_insns_1(&mut self, opcode: OpCode, operand: u8) {
        self.ensure_size_incr(OP_SIZE + 1);
        self.instructions[self.pc as usize] = opcode as OpSize;
        self.instructions[self.pc as usize + 1] = operand;
        self.pc += 2;
    }

    pub fn push_insns_1_16(&mut self, opcode: OpCode, operand: u16) {
        self.ensure_size_incr(OP_SIZE + 2);
        self.instructions[self.pc as usize] = opcode as OpSize;
        self.instructions[self.pc as usize + 1] = (operand >> 8) as u8;
        self.instructions[self.pc as usize + 2] = operand as u8;
        self.pc += 3;
    }

    pub fn push_insns_2(&mut self, opcode: OpCode, operand1: u8, operand2: u8) {
        self.ensure_size_incr(OP_SIZE + 2);
        self.instructions[self.pc as usize] = opcode as OpSize;
        self.instructions[self.pc as usize + 1] = operand1;
        self.instructions[self.pc as usize + 2] = operand2;
        self.pc += 3;
    }

    pub fn push_insns_3(&mut self, opcode: OpCode, operand1: u8, operand2: u8, operand3: u8) {
        self.ensure_size_incr(OP_SIZE + 3);
        self.instructions[self.pc as usize] = opcode as OpSize;
        self.instructions[self.pc as usize + 1] = operand1;
        self.instructions[self.pc as usize + 2] = operand2;
        self.instructions[self.pc as usize + 3] = operand3;
        self.pc += 4;
    }

    pub fn push_insns_n(&mut self, opcode: OpCode, operands: &[u8]) {
        let len = operands.len();
        self.ensure_size_incr(OP_SIZE + len as CodeSize);
        
        self.instructions[self.pc as usize] = opcode as OpSize;
        for i in 0..len {
            self.instructions[self.pc as usize + i + 1] = operands[i];
        }
        
        self.pc += len as CodeSize + 1;
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
