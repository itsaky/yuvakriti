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

mod chunk;

/// Magic number for the YuvaKriti Binary file format (`.ykb`).
/// The first 4 bytes of all YKB files have this value to help recognize
/// the file format.
pub(crate) const MAGIC_NUMBER: u32 = 0x59564B52; // ASCII codes for 'YVKR'x


/// The opcodes in YKB files.
pub(crate) struct OpCode;
impl OpCode {

    /// Opcode for a return statement.
    pub(crate) const OP_RETURN: u8 = 0x01;
}