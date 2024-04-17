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

use std::fmt::Display;

pub type OpSize = u8;

pub trait OpCodeExt {
    fn stack_effect(&self) -> i8;
    fn get_mnemonic(&self) -> &'static str;
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Nop = 0x00,
    Halt = 0x01,
    Add = 0x02,
    Sub = 0x03,
    Mult = 0x04,
    Div = 0x05,
    Print = 0x06,
    IfEq = 0x07,
    IfEqZ = 0x08,
    IfNe = 0x09,
    IfNeZ = 0x0A,
    IfLt = 0x0B,
    IfLtZ = 0x0C,
    IfLe = 0x0D,
    IfLeZ = 0x0E,
    IfGt = 0x0F,
    IfGtZ = 0x10,
    IfGe = 0x11,
    IfGeZ = 0x12,
    Ldc = 0x13,
    BPush0 = 0x14,
    BPush1 = 0x15,
    Store = 0x16,
    Store0 = 0x17,
    Store1 = 0x18,
    Store2 = 0x19,
    Store3 = 0x1A,
    Load = 0x1B,
    Load0 = 0x1C,
    Load1 = 0x1D,
    Load2 = 0x1E,
    Load3 = 0x1F,

    // when introducing new opcodes,
    // increment this
    OpCount = 0x20,
}

impl OpCode {
    pub fn as_op_size(self) -> OpSize {
        return self as OpSize;
    }
}

impl OpCodeExt for OpCode {
    fn stack_effect(&self) -> i8 {
        match self {
            // these ops do not require operands and do not push anything
            OpCode::Nop | OpCode::Halt => 0,

            // these pop 2 operands, and push 1
            OpCode::Add | OpCode::Sub | OpCode::Mult | OpCode::Div => -1,

            // these pop 1 operand
            // --- comparison to zero ---
            OpCode::IfEqZ |
            OpCode::IfNeZ |
            OpCode::IfLtZ |
            OpCode::IfLeZ |
            OpCode::IfGtZ |
            OpCode::IfGeZ |
            // --- store insn with implicit var index ---
            OpCode::Store |
            OpCode::Store0 |
            OpCode::Store1 |
            OpCode::Store2 |
            OpCode::Store3 |
            // --- printing ---
            OpCode::Print => -1,

            // these push 1 operand
            OpCode::Ldc |
            OpCode::BPush0 |
            OpCode::BPush1 |
            OpCode::Load |
            OpCode::Load0 |
            OpCode::Load1 |
            OpCode::Load2 |
            OpCode::Load3 => 1,

            // unreachable
            _ => unreachable!("OpCode {} is not yet supported!", self),
        }
    }

    fn get_mnemonic(&self) -> &'static str {
        match self {
            OpCode::Nop => "nop",
            OpCode::Halt => "halt",
            OpCode::Add => "add",
            OpCode::Sub => "sub",
            OpCode::Mult => "mult",
            OpCode::Div => "div",
            OpCode::Print => "print",
            OpCode::IfEq => "ifeq",
            OpCode::IfNe => "ifne",
            OpCode::IfLt => "iflt",
            OpCode::IfLe => "ifle",
            OpCode::IfGt => "ifgt",
            OpCode::IfGe => "ifge",
            OpCode::Ldc => "ldc",
            OpCode::BPush0 => "bpush_0",
            OpCode::BPush1 => "bpush_1",
            OpCode::Store => "store",
            OpCode::Store0 => "store_0",
            OpCode::Store1 => "store_1",
            OpCode::Store2 => "store_2",
            OpCode::Store3 => "store_3",
            OpCode::Load => "load",
            OpCode::Load0 => "load_0",
            OpCode::Load1 => "load_1",
            OpCode::Load2 => "load_2",
            OpCode::Load3 => "load_3",
            _ => panic!("Unknown/unsupported opcode: {:?}", self),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_mnemonic())
    }
}

pub fn get_opcode(code: OpSize) -> OpCode {
    return match code {
        0x00 => OpCode::Nop,
        0x01 => OpCode::Halt,
        0x02 => OpCode::Add,
        0x03 => OpCode::Sub,
        0x04 => OpCode::Mult,
        0x05 => OpCode::Div,
        0x06 => OpCode::Print,
        0x07 => OpCode::IfEq,
        0x08 => OpCode::IfEqZ,
        0x09 => OpCode::IfNe,
        0x0A => OpCode::IfNeZ,
        0x0B => OpCode::IfLt,
        0x0C => OpCode::IfLtZ,
        0x0D => OpCode::IfLe,
        0x0E => OpCode::IfLeZ,
        0x0F => OpCode::IfGt,
        0x10 => OpCode::IfGtZ,
        0x11 => OpCode::IfGe,
        0x12 => OpCode::IfGeZ,
        0x13 => OpCode::Ldc,
        0x14 => OpCode::BPush0,
        0x15 => OpCode::BPush1,
        0x16 => OpCode::Store,
        0x17 => OpCode::Store0,
        0x18 => OpCode::Store1,
        0x19 => OpCode::Store2,
        0x1A => OpCode::Store3,
        0x1B => OpCode::Load,
        0x1C => OpCode::Load0,
        0x1D => OpCode::Load1,
        0x1E => OpCode::Load2,
        0x1F => OpCode::Load3,
        _ => panic!("Unknown/unsupported opcode: {}", code),
    };
}
