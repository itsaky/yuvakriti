/*
 * Copyright (c) 2024 Akash Yadav
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

use std::mem::size_of;

use crate::bytecode::opcode::OpSize;
#[allow(unused_imports)]
use crate::bytecode::ConstantEntry;
use crate::bytecode::CpSize;
use crate::castable_enum;

pub type CodeSize = u32;
pub const OP_SIZE: CodeSize = size_of::<OpSize>() as CodeSize;

pub const CODE: &str = "Code";
pub const SOURCE_FILE: &str = "SourceFile";

castable_enum!(pub enum Attr {
    Code: Code,
    SourceFile: SourceFile,
});

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
    max_locals: u16,
    instructions: Vec<u8>,
}

impl Code {
    /// Create a new [Code] attribute with the given `max_stack` size and the capacity for the
    /// instruction bytes. `max_stack` is the maximum depth of the operand stack of the [Code]
    /// attribute at any point during the execution of instructions of that [Code] attribute.
    /// `max_locals` is the maximum number of local variables of the [Code] attribute.
    pub fn new(max_stack: u16, max_locals: u16, insns_capacity: CodeSize) -> Self {
        return Self::with_insns(
            max_stack,
            max_locals,
            Vec::with_capacity(insns_capacity as usize),
        );
    }

    /// Create a new [Code] attribute with the given `max_stack` size and the instruction bytes.
    /// `max_stack` is the maximum depth of the operand stack of the [Code]
    /// attribute at any point during the execution of instructions of that [Code] attribute.
    /// `max_locals` is the maximum number of local variables of the [Code] attribute.
    pub fn with_insns(max_stack: u16, max_locals: u16, instructions: Vec<u8>) -> Code {
        return Code {
            max_stack,
            max_locals,
            instructions,
        };
    }

    /// Get the maximum number of local variables for this code attribute.
    pub fn max_locals(&self) -> u16 {
        return self.max_locals;
    }

    /// The maximum depth of the operand stack for this code attribute.
    pub fn max_stack(&self) -> u16 {
        return self.max_stack;
    }

    /// Get the instructions for this [Code] attribute.
    pub fn instructions(&self) -> &Vec<u8> {
        return &self.instructions;
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
