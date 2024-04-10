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
use crate::bytecode::disassembler::YKBDisassembler;
use crate::bytecode::tests::util::compile_to_bytecode;

#[test]
fn test_disassembler() {
    let path = Path::new("target/disassemble.ykb");
    compile_to_bytecode("fun main() { print 1 + 2; }", &path);

    let f = File::open(&path).unwrap();
    let mut out_string = String::new();
    let mut disassembler = YKBDisassembler::new(ByteInput::new(f), &mut out_string);
    disassembler.disassemble().unwrap();
    println!("{}", out_string);
}
