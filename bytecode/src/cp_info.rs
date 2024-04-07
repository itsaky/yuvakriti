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

use std::fmt::{Display, Formatter};
use std::hash::Hash;

use proc_macros::CpInfo;

pub trait CpInfo: Eq + Hash + ToString {
    fn typ(&self) -> &'static str;
}

#[derive(CpInfo, Eq, PartialEq, Hash, Debug)]
pub struct Utf8Info {
    pub bytes: Vec<u8>,
}

impl Utf8Info {
    pub fn new(bytes: Vec<u8>) -> Utf8Info {
        return Utf8Info { bytes };
    }
}

impl From<&String> for Utf8Info {
    fn from(value: &String) -> Self {
        return Utf8Info {
            bytes: value.as_bytes().to_vec(),
        };
    }
}

impl From<&str> for Utf8Info {
    fn from(value: &str) -> Self {
        return Utf8Info {
            bytes: value.as_bytes().to_vec(),
        };
    }
}

impl Display for Utf8Info {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", String::from_utf8(self.bytes.clone()).unwrap());
    }
}

#[derive(CpInfo, Eq, PartialEq, Hash, Debug)]
pub struct NumberInfo {
    pub high_bytes: u32,
    pub low_bytes: u32,
}

impl NumberInfo {
    pub fn new(high_bytes: u32, low_bytes: u32) -> NumberInfo {
        return NumberInfo {
            high_bytes,
            low_bytes,
        };
    }
    pub fn to_f64(&self) -> f64 {
        return f64::from_bits(((self.high_bytes as u64) << 32) | self.low_bytes as u64);
    }
}

impl Display for NumberInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_f64());
    }
}

impl From<&f64> for NumberInfo {
    fn from(value: &f64) -> Self {
        let bits = value.to_bits();
        let high = (bits >> 32) as u32;
        let low = bits as u32;
        return NumberInfo {
            high_bytes: high,
            low_bytes: low,
        };
    }
}

#[derive(CpInfo, Eq, PartialEq, Hash, Debug)]
pub struct StringInfo {
    pub string_index: u16,
}

impl StringInfo {
    pub fn new(string_index: u16) -> StringInfo {
        return StringInfo { string_index };
    }
}

impl Display for StringInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "#{}", self.string_index);
    }
}

pub struct CpInfoTag;
impl CpInfoTag {
    pub const UTF8: u8 = 0x00;
    pub const NUMBER: u8 = 0x01;
    pub const STRING: u8 = 0x02;
}
