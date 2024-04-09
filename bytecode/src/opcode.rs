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

pub type OpSize = u8;

macro_rules! opcodes {
    ($count:literal, $($name:ident, $value:literal $(,)*)+) => {
        $(pub const $name: OpSize = $value;)+

        pub const OPCODE_COUNT: usize = $count;

        pub const OPCODE_MNEMONICS: [&str; OPCODE_COUNT] = [
            $(stringify!($name),)+
        ];
    };
}

opcodes!(
    13, NOP, 0x00, HALT, 0x01, ADD, 0x02, SUB, 0x03, MULT, 0x04, DIV, 0x05, PRINT, 0x06, IFEQ,
    0x07, IFNE, 0x08, IFLT, 0x09, IFLE, 0x0A, IFGT, 0x0B, IFGE, 0x0C,
);

pub fn get_mnemonic(opcode: OpSize) -> &'static str {
    return OPCODE_MNEMONICS[opcode as usize];
}
