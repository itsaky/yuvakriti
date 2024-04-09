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

use std::fmt::{Debug, Display};

use crate::bytes::AssertingByteConversions;
use crate::cp_info::NumberInfo;
use crate::cp_info::StringInfo;
use crate::cp_info::Utf8Info;

/// The size of the constant pool entry indices.
pub type CpSize = u16;

/// The constant pool in a YKB file.
///
/// The first entry is always the reserved [ConstantEntry::None]
/// entry. As a result, the entries in the contant pool are 1-indexed.
#[derive(Eq, PartialEq, Hash, Debug)]
#[allow(unused)]
pub struct ConstantPool {
    entries: Vec<ConstantEntry>,
}

impl ConstantPool {
    /// The maximum number of entries that can be stored in a constant pool.
    pub const MAX_ENTRIES: CpSize = 0xFFFF;
}

/// An entry in the constant pool.
#[derive(Hash, Debug, Eq)]
pub enum ConstantEntry {
    Utf8(Utf8Info),
    String(StringInfo),
    Number(NumberInfo),

    /// A special type of constant entry which is the first entry in the constant pool.
    None,
}

impl Display for ConstantEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ConstantEntry::Utf8(utf8) => utf8.to_string(),
            ConstantEntry::String(str) => str.to_string(),
            ConstantEntry::Number(num) => num.to_string(),
            ConstantEntry::None => String::from("None"),
        };

        write!(f, "{}", str)
    }
}

impl ConstantEntry {
    pub fn as_utf8(&self) -> Option<&Utf8Info> {
        match self {
            ConstantEntry::Utf8(ref info) => Some(info),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&StringInfo> {
        match self {
            ConstantEntry::String(ref info) => Some(info),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<&NumberInfo> {
        match self {
            ConstantEntry::Number(ref info) => Some(info),
            _ => None,
        }
    }
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

    /// Creates a new ConstantPool with the given initial capacity.
    pub fn with_capacity(capacity: CpSize) -> ConstantPool {
        let mut entries = Vec::with_capacity(capacity as usize);
        entries.push(ConstantEntry::None);
        return ConstantPool { entries };
    }

    pub fn len(&self) -> CpSize {
        return self.entries.len().as_cp_size();
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
        return (self.entries.len().as_cp_size()) - 1;
    }

    pub fn lookup(&self, constant: &ConstantEntry) -> Option<CpSize> {
        if let Some(index) = self.entries.iter().position(|x| x == constant) {
            return Some(index.as_cp_size());
        }

        return None;
    }

    /// Returns the constant entry at the given index.
    pub fn get(&self, index: CpSize) -> Option<&ConstantEntry> {
        return self.entries.get(index as usize);
    }

    /// Push a string constant to the constant pool.
    pub fn push_str(&mut self, string: &str) -> CpSize {
        let utf8idx = self.push(ConstantEntry::Utf8(Utf8Info::from(string)));
        return self.push(ConstantEntry::String(StringInfo::new(utf8idx)));
    }

    /// Push a string constant to the constant pool.
    pub fn push_string(&mut self, string: &String) -> CpSize {
        let utf8idx = self.push(ConstantEntry::Utf8(Utf8Info::from(string)));
        return self.push(ConstantEntry::String(StringInfo::new(utf8idx)));
    }
}
