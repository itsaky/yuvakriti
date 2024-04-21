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

use std::fmt::Display;
use std::fmt::Write;
use std::io::Read;

use crate::bytecode::attrs;
use crate::bytecode::attrs::Attr;
use crate::bytecode::attrs::Code;
use crate::bytecode::bytes::AssertingByteConversions;
use crate::bytecode::bytes::ByteInput;
use crate::bytecode::cp_info::CpInfo;
use crate::bytecode::opcode::get_opcode;
use crate::bytecode::opcode::OpCode;
use crate::bytecode::ConstantEntry;
use crate::bytecode::ConstantPool;
use crate::bytecode::CpSize;
use crate::bytecode::YKBFileReader;
use crate::bytecode::MAGIC_NUMBER;

pub struct YKBDisassembler<'a, R: Read> {
    r: YKBFileReader<R>,
    w: &'a mut dyn Write,
    indent: u8,
}

impl<R: Read> YKBDisassembler<'_, R> {
    pub fn new(buffer: ByteInput<R>, write: &mut dyn Write) -> YKBDisassembler<R> {
        return YKBDisassembler {
            r: YKBFileReader::new(buffer),
            w: write,
            indent: 0,
        };
    }
}

impl<'a, R: Read> YKBDisassembler<'a, R> {
    fn write(&mut self, s: &str) {
        self.w.write_str(s).unwrap();
    }

    fn write1(&mut self, s: &String) {
        self.w.write_str(s).unwrap();
    }

    fn indent(&mut self, level: u8) {
        for _ in 0..level {
            self.write("    ");
        }
    }

    fn linindent(&mut self) {
        self.linefeed();
        self.indent(self.indent);
    }

    fn linefeed(&mut self) {
        self.write("\n");
    }

    pub fn disassemble(&mut self) -> Result<(), String> {
        self.indent = 0;
        self.write("========= YKB =========");

        let magic = self.r.read_magic_number().unwrap();
        if MAGIC_NUMBER != magic {
            return Err(format!("Invalid magic number: {}", magic));
        }

        {
            self.linindent();
            let version = self.r.read_version().unwrap();
            self.write("major version: ");
            self.write1(&version.major_version().to_string());
            self.linindent();
            self.write("minor version: ");
            self.write1(&version.minor_version().to_string());
        }

        let mut constant_pool = ConstantPool::new();
        let constant_pool_count = self.r.read_constant_pool(&mut constant_pool).unwrap();

        let attrs = self.r.read_attrs(&constant_pool).unwrap();

        {
            self.linindent();
            self.write("Constant pool: ");
            self.indent += 1;
            self.write_constant_pool(&constant_pool, constant_pool_count);
            self.indent -= 1;
        }

        {
            self.linindent();
            self.write("Attributes: ");
            self.indent += 1;
            self.write_attrs(&attrs, &constant_pool);
            self.indent -= 1;
        }

        Ok(())
    }

    fn write_constant_pool(&mut self, constant_pool: &ConstantPool, count: CpSize) {
        for i in 1..count {
            self.linindent();
            let (info, typ): (&dyn Display, &str) = match constant_pool
                .get(i)
                .expect(format!("No constant at index {}", i).as_str())
            {
                ConstantEntry::Utf8(utf8) => (utf8, utf8.typ()),
                ConstantEntry::String(str) => (str, str.typ()),
                ConstantEntry::Number(num) => (num, num.typ()),
                ConstantEntry::None => {
                    unreachable!("None should not be in the constant pool, other than at index 0")
                }
            };

            self.write1(&format!("#{}: {:<20} {}", i, typ, info));
        }
    }

    fn write_attrs(&mut self, attrs: &Vec<Attr>, constant_pool: &ConstantPool) {
        for attr in attrs {
            self.write_attr(attr, constant_pool);
        }
    }

    fn write_attr(&mut self, attr: &Attr, constant_pool: &ConstantPool) {
        self.linindent();
        let attr_name = match attr {
            Attr::Code(_) => attrs::CODE,
            Attr::SourceFile(_) => attrs::SOURCE_FILE,
        };

        self.write(format!("{}: ", attr_name).as_str());

        match attr {
            Attr::Code(code) => {
                self.write1(&format!("max_stack={}", code.max_stack()));
                self.write1(&format!(" max_locals={}", code.max_locals()));

                self.indent += 1;
                self.write_code(code, constant_pool);
                self.indent -= 1;
            }
            Attr::SourceFile(file) => {
                let name = constant_pool
                    .get(file.name_index)
                    .map(|entry| entry.as_utf8().unwrap())
                    .expect(&format!(
                        "Expected a Utf8Info constant pool entry at index {}",
                        file.name_index
                    ));

                self.write1(&name.to_string());
            }
        }
    }

    fn write_code(&mut self, code: &Code, constant_pool: &ConstantPool) {
        let mut index: usize = 0;
        while index < code.instructions().len() {
            let instructions = code.instructions();
            let opcode = get_opcode(instructions[index]);

            self.linindent();
            self.write1(&format!("{:>5}: {} ", index, opcode));

            index += 1;

            match opcode {
                OpCode::Nop => {}
                OpCode::Halt => {}
                OpCode::Add => {}
                OpCode::Sub => {}
                OpCode::Mult => {}
                OpCode::Div => {}
                OpCode::Print => {}
                OpCode::BPush0 => {}
                OpCode::BPush1 => {}
                OpCode::Ldc => {
                    let const_index =
                        (instructions[index].as_u16()) << 8 | instructions[index + 1] as u16;
                    let constant = constant_pool.get(const_index).unwrap();
                    self.write(&format!("#{:<5} // {}", const_index, constant));
                    index += 2
                }
                OpCode::Load0 | OpCode::Load1 | OpCode::Load2 | OpCode::Load3 => {}
                OpCode::Store0 | OpCode::Store1 | OpCode::Store2 | OpCode::Store3 => {}

                OpCode::Load
                | OpCode::Store
                | OpCode::IfEq
                | OpCode::IfEqZ
                | OpCode::IfNe
                | OpCode::IfNeZ
                | OpCode::IfLt
                | OpCode::IfLtZ
                | OpCode::IfLe
                | OpCode::IfLeZ
                | OpCode::IfGt
                | OpCode::IfGtZ
                | OpCode::IfGe
                | OpCode::IfGeZ
                | OpCode::IfTruthy
                | OpCode::IfFalsy
                | OpCode::Jmp => {
                    self.write_16(instructions, index);
                    index += 2;
                }
            }
        }
    }

    fn write_16(&mut self, insns: &Vec<u8>, index: usize) {
        let idx = (insns[index].as_u16() << 8) | insns[index + 1].as_u16();
        self.write(&format!("{}", idx))
    }
}
