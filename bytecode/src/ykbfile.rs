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

use std::io::{Error, Write};

use crate::attrs::Attr;
use crate::bytes::ByteOutput;
use crate::ConstantEntry;
use crate::cp::ConstantPool;
use crate::cp_info::CpInfoTag;
use crate::decls::YKBDecl;
use crate::ykbversion::YKBVersion;

pub const MAGIC_NUMBER: u32 = 0x59754B72;

/// Represents a YKB file.
pub struct YKBFile {
    /// The version of the YKB file.
    version: YKBVersion,

    /// The constant pool in the YKB file.
    constant_pool: ConstantPool,

    /// The declarations in the YKB file.
    declarations: Vec<Box<dyn YKBDecl>>,

    /// The instructions in the YKB file.
    attributes: Vec<Attr>,
}

impl YKBFile {
    /// Creates a new YKBFile.
    pub fn new(version: YKBVersion) -> YKBFile {
        return YKBFile {
            version,
            constant_pool: ConstantPool::new(),
            declarations: Vec::with_capacity(0),
            attributes: Vec::with_capacity(0),
        };
    }

    pub fn version(&self) -> &YKBVersion {
        return &self.version;
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
    
    pub fn attributes(&self) -> &Vec<Attr> {
        return &self.attributes;
    }
    
    pub fn attributes_mut(&mut self) -> &mut Vec<Attr> {
        return &mut self.attributes;
    }
}

impl YKBFile {
    pub fn write_to<W: Write>(&self, writer: &mut ByteOutput<W>) -> Result<usize, Error> {
        let mut size = writer.write_u32(MAGIC_NUMBER)?;
        size += writer.write_u16(self.version.major_version())?;
        size += writer.write_u16(self.version.minor_version())?;
        size += self.write_constant_pool(writer)?;
        Ok(size)
    }

    fn write_constant_pool<W: Write>(&self, writer: &mut ByteOutput<W>) -> Result<usize, Error> {
        let constant_pool = self.constant_pool();
        let mut size = writer.write_u16(constant_pool.len())?;
        if constant_pool.len() <= 1 && constant_pool.get(0).unwrap() == &ConstantEntry::None {
            return Ok(size);
        }

        for index in 1..constant_pool.len() {
            let entry = constant_pool.get(index).unwrap();
            match entry {
                ConstantEntry::Utf8(utf8) => {
                    size += writer.write_u8(CpInfoTag::UTF8)?;
                    size += writer.write_u16(utf8.bytes.len() as u16)?;
                    size += writer.write_bytes(utf8.bytes.as_slice())?;
                }
                ConstantEntry::String(str) => {
                    size += writer.write_u8(CpInfoTag::STRING)?;
                    size += writer.write_u16(str.string_index)?;
                }
                ConstantEntry::Number(num) => {
                    size += writer.write_u8(CpInfoTag::NUMBER)?;
                    size += writer.write_u32(num.high_bytes)?;
                    size += writer.write_u32(num.low_bytes)?;
                }
                ConstantEntry::None => {
                    unreachable!("None should not be written to the constant pool")
                }
            }
        }

        Ok(size)
    }
}
