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

use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;

use crate::cp_info::CpInfoTag;
use crate::cp_info::NumberInfo;
use crate::cp_info::StringInfo;
use crate::cp_info::Utf8Info;
use crate::file::MAGIC_NUMBER;
use crate::ConstantEntry;
use crate::ConstantPool;
use crate::CpSize;
use crate::YKBFile;
use crate::YKBVersion;
use crate::{attrs, bytes::ByteInput};

pub struct YKBFileReader<R: Read> {
    buf: ByteInput<R>,
}

impl<R: Read> YKBFileReader<R> {
    pub fn new(buffer: ByteInput<R>) -> Self {
        return YKBFileReader { buf: buffer };
    }
}

impl<R: Read> YKBFileReader<R> {
    fn map_err<T>(result: Result<T, Error>, err_msg: &str) -> Result<T, Error> {
        return result.map_err(|e| Error::new(e.kind(), format!("{}: {}", err_msg, e)));
    }

    pub fn read_file(&mut self) -> Result<YKBFile, Error> {
        let magic_number = self.read_magic_number()?;
        if magic_number != MAGIC_NUMBER {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid magic number"));
        }

        let version = self.read_version()?;
        let mut file = YKBFile::new(version);

        self.read_constant_pool(file.constant_pool_mut())?;

        return Ok(file);
    }

    pub fn read_magic_number(&mut self) -> Result<u32, Error> {
        return Self::map_err(self.buf.read_u32(), "Unable to read magic number");
    }

    pub fn read_version(&mut self) -> Result<YKBVersion, Error> {
        let major = Self::map_err(self.buf.read_u16(), "Unable to read major version number")?;
        let minor = Self::map_err(self.buf.read_u16(), "Unable to read minor version number")?;
        return Ok(YKBVersion::new(major, minor));
    }

    pub fn read_constant_pool(
        &mut self,
        constant_pool: &mut ConstantPool,
    ) -> Result<CpSize, Error> {
        let count: CpSize =
            Self::map_err(self.buf.read_u16(), "Unable to read constant pool count")?;
        for index in 1..count {
            let entry = Self::map_err(
                self.read_constant_entry(),
                format!("Unable to read constant entry at index {}", index).as_str(),
            )?;

            constant_pool.push(entry);
        }

        Ok(count)
    }

    pub fn read_constant_entry(&mut self) -> Result<ConstantEntry, Error> {
        let tag = self.buf.read_u8()?;
        match tag {
            CpInfoTag::UTF8 => self.read_utf8_contant_entry(),
            CpInfoTag::NUMBER => self.read_number_contant_entry(),
            CpInfoTag::STRING => self.read_string_constant_entry(),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Invalid constant pool tag",
                ))
            }
        }
    }

    pub fn read_utf8_contant_entry(&mut self) -> Result<ConstantEntry, Error> {
        let byte_count = Self::map_err(self.buf.read_u16(), "Unable to read byte count")?;
        let bytes = Self::map_err(
            self.buf.read_n_bytes(byte_count as usize),
            "Unable to read bytes",
        )?;
        Ok(ConstantEntry::Utf8(Utf8Info::new(bytes)))
    }

    pub fn read_number_contant_entry(&mut self) -> Result<ConstantEntry, Error> {
        let high_bytes = Self::map_err(self.buf.read_u32(), "Unable to read high bytes")?;
        let low_bytes = Self::map_err(self.buf.read_u32(), "Unable to read low bytes")?;
        Ok(ConstantEntry::Number(NumberInfo::new(
            high_bytes, low_bytes,
        )))
    }

    pub fn read_string_constant_entry(&mut self) -> Result<ConstantEntry, Error> {
        let string_index = Self::map_err(self.buf.read_u16(), "Unable to read string index")?;
        Ok(ConstantEntry::String(StringInfo::new(string_index)))
    }

    pub fn read_attrs(&mut self, constant_pool: &ConstantPool) -> Result<Vec<attrs::Attr>, Error> {
        let count = Self::map_err(self.buf.read_u16(), "Unable to read attribute count")?;
        let mut attrs = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let attr = Self::map_err(self.read_attr(constant_pool), "Unable to read attribute")?;
            attrs.push(attr);
        }

        return Ok(attrs);
    }

    pub fn read_attr(&mut self, constant_pool: &ConstantPool) -> Result<attrs::Attr, Error> {
        let name_index: CpSize =
            Self::map_err(self.buf.read_u16(), "Unable to read attribute name index")?;
        let info = constant_pool
            .get(name_index)
            .map(|entry| entry.as_utf8().unwrap())
            .expect(&format!(
                "Expected a Utf8Info entry at constant pool index {}",
                name_index
            ));

        let name = info.to_string();

        let attr = match name.as_str() {
            attrs::CODE => {
                let insn_count =
                    Self::map_err(self.buf.read_u32(), "Unable to read instruction count")?;
                let buf = self.buf.read_n_bytes(insn_count as usize)?;
                attrs::Attr::Code(attrs::Code::with_insns(buf))
            }
            attrs::SOURCE_FILE => {
                let name_index =
                    Self::map_err(self.buf.read_u16(), "Unable to read source file name index")?;
                attrs::Attr::SourceFile(attrs::SourceFile::new(name_index))
            }
            _ => {
                panic!("Unknown attribute: {}", name);
            }
        };

        Ok(attr)
    }
}
