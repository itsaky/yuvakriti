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

use crate::cp_info::NumberInfo;
use crate::cp_info::StringInfo;
use crate::cp_info::Utf8Info;

/// The size of the constant pool entry indices.
pub type CpSize = u16;

/// The constant pool in a YKB file.
///
/// The first entry is always the reserved [ConstantEntry::None]
/// entry. As a result, the entries in the contant pool are 1-indexed.
#[allow(unused)]
pub struct ConstantPool {
    entries: Vec<ConstantEntry>,
}

impl ConstantPool {
    /// The maximum number of entries that can be stored in a constant pool.
    pub const MAX_ENTRIES: CpSize = 0xFFFF;
}

/// An entry in the constant pool.
#[derive(Hash, Debug)]
pub enum ConstantEntry {
    Utf8(Utf8Info),
    String(StringInfo),
    Number(NumberInfo),

    /// A special type of constant entry which is the first entry in the constant pool.
    None,
}

impl PartialEq for ConstantEntry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ConstantEntry::Utf8(ref a), ConstantEntry::Utf8(ref b)) => a.bytes == b.bytes,
            (ConstantEntry::String(ref a), ConstantEntry::String(ref b)) => {
                a.string_index == b.string_index
            }
            (ConstantEntry::Number(ref a), ConstantEntry::Number(ref b)) => {
                a.high_bytes == b.high_bytes && a.low_bytes == b.low_bytes
            }
            (ConstantEntry::None, ConstantEntry::None) => true,
            _ => false,
        }
    }
}

impl ConstantPool {
    /// Creates a new ConstantPool
    pub fn new() -> ConstantPool {
        return ConstantPool {
            entries: vec![ConstantEntry::None],
        };
    }

    pub fn len(&self) -> CpSize {
        return self.entries.len() as CpSize;
    }

    /// Pushes a constant to the constant pool and returns the index of the constant entry.
    pub fn push(&mut self, constant: ConstantEntry) -> CpSize {
        if self.len() >= ConstantPool::MAX_ENTRIES {
            panic!("Pool overflow");
        }

        if let Some(index) = self.lookup(&constant) {
            return index;
        }

        self.entries.push(constant);
        return (self.entries.len() as CpSize) - 1;
    }

    pub fn lookup(&self, constant: &ConstantEntry) -> Option<CpSize> {
        if let Some(index) = self.entries.iter().position(|x| x == constant) {
            return Some(index as CpSize);
        }

        return None;
    }

    /// Returns the constant entry at the given index.
    pub fn get(&self, index: CpSize) -> Option<&ConstantEntry> {
        return self.entries.get(index as usize);
    }
}
