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

#[allow(unused_imports)]
use crate::ConstantEntry;
use crate::CpSize;

pub trait BytecodeAttr {
    fn name(&self) -> &'static str;
}

macro_rules! bytecode_attr {
    ($name:ident) => {
        impl BytecodeAttr for $name {
            fn name(&self) -> &'static str {
                stringify!($name)
            }
        }
    };
}

pub enum Attr {
    Code(Code),
    SourceFile(SourceFile),
}

/// The Code attribute contains the instructions for a function or the top-level inststructions
/// in a program.
pub struct Code {
    pub instructions: Vec<u8>,
}
bytecode_attr!(Code);

/// The SourceFile attribute contains the index of the [ConstantEntry::Utf8] constant pool entry
/// containing the source file name.
pub struct SourceFile {
    pub name_index: CpSize,
}
bytecode_attr!(SourceFile);