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

macro_rules! opcode {
    ($mnemonic:ident, $value:literal) => {
        pub const $mnemonic: OpCode = OpCode::new($value, stringify!($mnemonic));
    };
}

pub type OpSize = u8;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct OpCode {
    pub code: OpSize,
    pub mnemonic: &'static str,
}

impl OpCode {
    pub const fn new(code: OpSize, mnemonic: &'static str) -> OpCode {
        return OpCode { code, mnemonic };
    }
}

impl OpCode {
    opcode!(HALT, 0x00);
    opcode!(ADD, 0x01);
    opcode!(SUB, 0x02);
    opcode!(MULT, 0x03);
    opcode!(DIV, 0x04);
}

pub fn get_mnemonic(opcode: OpSize) -> &'static str {
    match opcode {
        NOP => "nop",
        _ => panic!("Unknown opcode: {}", opcode),
    }
}
