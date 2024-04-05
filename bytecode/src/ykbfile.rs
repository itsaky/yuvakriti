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

use crate::cp::ConstantPool;
use crate::decls::YKBDecl;
use crate::insns::Insn;
use crate::ykbversion::YKBVersion;

/// Represents a YKB file.
pub struct YKBFile {
    /// The version of the YKB file.
    version: YKBVersion,

    /// The constant pool in the YKB file.
    constant_pool: ConstantPool,

    /// The declarations in the YKB file.
    declarations: Vec<Box<dyn YKBDecl>>,

    /// The instructions in the YKB file.
    instructions: Vec<Box<dyn Insn>>,
}

impl YKBFile {
    /// Creates a new YKBFile.
    pub fn new() -> YKBFile {
        return YKBFile {
            version: YKBVersion::NONE,
            constant_pool: ConstantPool::new(),
            declarations: Vec::with_capacity(0),
            instructions: Vec::with_capacity(0),
        };
    }

    /// Get the constant pool for this YKB file.
    pub fn constant_pool(&self) -> &ConstantPool {
        return &self.constant_pool;
    }

    /// Get the constant pool as a mutable reference for this YKB file.
    pub fn constant_pool_mut(&mut self) -> &mut ConstantPool {
        return &mut self.constant_pool;
    }

    pub fn declarations(&self) -> &Vec<Box<dyn YKBDecl>> {
        return &self.declarations;
    }

    pub fn declarations_mut(&mut self) -> &mut Vec<Box<dyn YKBDecl>> {
        return &mut self.declarations;
    }
}
