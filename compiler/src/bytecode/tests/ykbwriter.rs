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

use std::fs::File;
use std::path::Path;

use crate::bytecode::bytes::ByteInput;
use crate::bytecode::cp_info::NumberInfo;
use crate::bytecode::cp_info::Utf8Info;
use crate::bytecode::opcode::OpCode;
use crate::bytecode::opcode::OpSize;
use crate::bytecode::tests::util::compile_to_bytecode;
use crate::bytecode::attrs;
use crate::bytecode::ConstantEntry;
use crate::bytecode::YKBFileReader;

#[test]
fn test_program_writer() {
    let path = Path::new("target/reader.ykb");
    let display = path.display();
    let ykbfile = compile_to_bytecode("fun main() { var str = \"str\"; var num = 123; }", &path);

    let f = File::open(&path).unwrap();
    let readykb = match YKBFileReader::new(ByteInput::new(f)).read_file() {
        Ok(file) => file,
        Err(why) => {
            panic!("couldn't read from file {}: {}", display, why);
        }
    };

    assert_eq!(&readykb.version(), &ykbfile.version());
    assert_eq!(&readykb.constant_pool(), &ykbfile.constant_pool());
}

#[test]
fn test_arithemetic_constant_folding() {
    let path = Path::new("target/const_folding.ykb");
    let ykbfile = compile_to_bytecode("print 1 + 2; print 2 - 3; print 3 * 4; print 4 / 5;", &path);

    #[rustfmt::skip]
    assert_eq!(
        &vec![
            ConstantEntry::None, // constant at entry 0 is always None
            ConstantEntry::Number(NumberInfo::from(&3f64)), // 1+2 folded to 3
            ConstantEntry::Number(NumberInfo::from(&-1f64)), // 2-3 folded to -1
            ConstantEntry::Number(NumberInfo::from(&12f64)), // 3*4 folded to 12
            ConstantEntry::Number(NumberInfo::from(&0.8f64)), // 4/5 folded to 0.8
            ConstantEntry::Utf8(Utf8Info::from("Code")), // "Code" is the name of the "Code" attribute for the YKBFile's top-level statements
        ],
        ykbfile.constant_pool().entries()
    );

    let attrs = ykbfile.attributes();
    let attr = attrs
        .iter()
        .find(|attr| attr.name() == attrs::CODE)
        .unwrap();
    if let attrs::Attr::Code(code) = &attr {
        let instructions = code.instructions();
        
        #[rustfmt::skip]
        assert_eq!(
            &vec![
                OpCode::Ldc as OpSize, 0x00, 0x01, // 3
                OpCode::Print as OpSize,
                OpCode::Ldc as OpSize, 0x00, 0x02, // -1
                OpCode::Print as OpSize,
                OpCode::Ldc as OpSize, 0x00, 0x03, // 12
                OpCode::Print as OpSize,
                OpCode::Ldc as OpSize, 0x00, 0x04, // 0.8
                OpCode::Print as OpSize,
            ],
            instructions
        );
    }
}