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

use crate::compiler::bytecode::ConstantPool;
use crate::compiler::bytecode::YKBVersion;

/// Represents a YKB file.
pub(crate) struct YKBFile {
    /// The version of the YKB file.
    version: YKBVersion,

    /// The constant pool in the YKB file.
    constant_pool: ConstantPool,
}

impl YKBFile {
    /// Creates a new YKBFile.
    pub(crate) fn new() -> YKBFile {
        return YKBFile {
            version: YKBVersion::NONE,
            constant_pool: ConstantPool::new(),
        };
    }

    /// Get the constant pool for this YKB file.
    pub(crate) fn constant_pool(&self) -> &ConstantPool {
        return &self.constant_pool;
    }

    /// Get the constant pool as a mutable reference for this YKB file.
    pub(crate) fn constant_pool_mut(&mut self) -> &mut ConstantPool {
        return &mut self.constant_pool;
    }
}
