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

use crate::attrs::CodeSize;
use crate::opcode::OpSize;
use crate::CpSize;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;

pub struct ByteInput<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> ByteInput<R> {
    pub fn new(input: R) -> ByteInput<R> {
        return ByteInput {
            reader: BufReader::new(input),
        };
    }
}

impl<R: Read> ByteInput<R> {
    pub fn read_n_bytes(&mut self, count: usize) -> Result<Vec<u8>, Error> {
        let mut out = vec![0u8; count];
        let read = self.reader.read(&mut out)?;
        if read != count {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                format!("Unexpected EOF; bytesToRead: {}, read: {}", count, read),
            ));
        }

        Ok(out)
    }

    pub fn read_u8(&mut self) -> Result<u8, Error> {
        return Ok(self.read_n_bytes(1)?[0]);
    }

    pub fn read_u16(&mut self) -> Result<u16, Error> {
        let bytes_vec = &mut self.read_n_bytes(2)?;
        let bytes = &mut bytes_vec.as_slice();
        let (int_bytes, rest) = bytes.split_at(std::mem::size_of::<u16>());
        *bytes = rest;
        return Ok(u16::from_be_bytes(int_bytes.try_into().unwrap()));
    }

    pub fn read_u32(&mut self) -> Result<u32, Error> {
        let bytes_vec = &mut self.read_n_bytes(4)?;
        let bytes = &mut bytes_vec.as_slice();
        let (int_bytes, rest) = bytes.split_at(std::mem::size_of::<u32>());
        *bytes = rest;
        return Ok(u32::from_be_bytes(int_bytes.try_into().unwrap()));
    }

    pub fn read_u64(&mut self) -> Result<u64, Error> {
        let bytes_vec = &mut self.read_n_bytes(8)?;
        let bytes = &mut bytes_vec.as_slice();
        let (int_bytes, rest) = bytes.split_at(std::mem::size_of::<u64>());
        *bytes = rest;
        return Ok(u64::from_be_bytes(int_bytes.try_into().unwrap()));
    }
}

pub struct ByteOutput<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> ByteOutput<W> {
    pub fn new(output: W) -> ByteOutput<W> {
        return ByteOutput {
            writer: BufWriter::new(output),
        };
    }
}

impl<W: Write> ByteOutput<W> {
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        self.writer.write(bytes)
    }

    pub fn write_n_bytes(
        &mut self,
        bytes: &[u8],
        offset: usize,
        count: usize,
    ) -> Result<usize, Error> {
        self.writer.write(&bytes[offset..offset + count])
    }

    pub fn write_u8(&mut self, byte: u8) -> Result<usize, Error> {
        return self.write_bytes(&[byte]);
    }

    pub fn write_u16(&mut self, bytes: u16) -> Result<usize, Error> {
        self.write_bytes(&bytes.to_be_bytes())
    }

    pub fn write_u32(&mut self, bytes: u32) -> Result<usize, Error> {
        self.write_bytes(&bytes.to_be_bytes())
    }

    pub fn write_u64(&mut self, bytes: u64) -> Result<usize, Error> {
        self.write_bytes(&bytes.to_be_bytes())
    }
}

pub trait AssertingByteConversions {
    fn as_u8(&self) -> u8;
    fn as_u16(&self) -> u16;
    fn as_u32(&self) -> u32;
    fn as_u64(&self) -> u64;
    fn as_op_size(&self) -> OpSize {
        return self.as_u8();
    }
    fn as_cp_size(&self) -> CpSize {
        return self.as_u16();
    }
    fn as_code_size(&self) -> CodeSize {
        return self.as_u32();
    }
}

impl AssertingByteConversions for u8 {
    fn as_u8(&self) -> u8 {
        return *self;
    }

    fn as_u16(&self) -> u16 {
        return u16::try_from(*self).expect("Could not convert u8 to u16");
    }

    fn as_u32(&self) -> u32 {
        return u32::try_from(*self).expect("Could not convert u8 to u32");
    }

    fn as_u64(&self) -> u64 {
        return u64::try_from(*self).expect("Could not convert u8 to u64");
    }
}

impl AssertingByteConversions for u16 {
    fn as_u8(&self) -> u8 {
        return u8::try_from(*self).expect("Could not convert u16 to u8");
    }

    fn as_u16(&self) -> u16 {
        return *self;
    }

    fn as_u32(&self) -> u32 {
        return u32::try_from(*self).expect("Could not convert u16 to u32");
    }

    fn as_u64(&self) -> u64 {
        return u64::try_from(*self).expect("Could not convert u16 to u64");
    }
}

impl AssertingByteConversions for u32 {
    fn as_u8(&self) -> u8 {
        return u8::try_from(*self).expect("Could not convert u32 to u8");
    }

    fn as_u16(&self) -> u16 {
        return u16::try_from(*self).expect("Could not convert u32 to u16");
    }

    fn as_u32(&self) -> u32 {
        return *self;
    }

    fn as_u64(&self) -> u64 {
        return u64::try_from(*self).expect("Could not convert u32 to u64");
    }
}

impl AssertingByteConversions for u64 {
    fn as_u8(&self) -> u8 {
        return u8::try_from(*self).expect("Could not convert u64 to u8");
    }

    fn as_u16(&self) -> u16 {
        return u16::try_from(*self).expect("Could not convert u64 to u16");
    }

    fn as_u32(&self) -> u32 {
        return u32::try_from(*self).expect("Could not convert u64 to u32");
    }

    fn as_u64(&self) -> u64 {
        return *self;
    }
}

impl AssertingByteConversions for usize {
    fn as_u8(&self) -> u8 {
        return u8::try_from(*self).expect("Could not convert usize to u8");
    }

    fn as_u16(&self) -> u16 {
        return u16::try_from(*self).expect("Could not convert usize to u16");
    }

    fn as_u32(&self) -> u32 {
        return u32::try_from(*self).expect("Could not convert usize to u32");
    }

    fn as_u64(&self) -> u64 {
        return u64::try_from(*self).expect("Could not convert usize to u64");
    }
}
