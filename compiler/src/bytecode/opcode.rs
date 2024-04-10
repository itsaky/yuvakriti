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
    IfNe = 0x08,
    IfLt = 0x09,
    IfLe = 0x0A,
    IfGt = 0x0B,
    IfGe = 0x0C,
    Ldc = 0x0D,
    BPush0 = 0x0E,
    BPush1 = 0x0F,

    // when introducing new opcodes,
    // increment this
    OpCount = 0x10,
}

impl OpCode {
    pub fn as_op_size(self) -> OpSize {
        return self as OpSize;
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", get_opcode_mnemonic(*self))
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
        0x08 => OpCode::IfNe,
        0x09 => OpCode::IfLt,
        0x0A => OpCode::IfLe,
        0x0B => OpCode::IfGt,
        0x0C => OpCode::IfGe,
        0x0D => OpCode::Ldc,
        0x0E => OpCode::BPush0,
        0x0F => OpCode::BPush1,
        _ => panic!("Unknown/unsupported opcode: {}", code),
    };
}

pub fn get_opcode_mnemonic(opcode: OpCode) -> &'static str {
    return match opcode {
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
        _ => panic!("Unknown/unsupported opcode: {:?}", opcode),
    };
}
