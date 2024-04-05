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

use std::hash::Hash;

use proc_macros::CpInfo;

pub trait CpInfo: Eq + Hash {}

#[derive(CpInfo, Eq, PartialEq, Hash, Debug)]
pub struct Utf8Info {
    pub bytes: Vec<u8>,
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

#[derive(CpInfo, Eq, PartialEq, Hash, Debug)]
pub struct NumberInfo {
    pub high_bytes: u32,
    pub low_bytes: u32,
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

impl NumberInfo {
    pub fn to_f64(&self) -> f64 {
        return f64::from_bits(((self.high_bytes as u64) << 32) | self.low_bytes as u64);
    }
}

#[derive(CpInfo, Eq, PartialEq, Hash, Debug)]
pub struct StringInfo {
    pub string_index: u16,
}

pub struct CpInfoTag;
impl CpInfoTag {
    pub const UTF8: u8 = 0x00;
    pub const NUMBER: u8 = 0x01;
    pub const STRING: u8 = 0x02;
}
