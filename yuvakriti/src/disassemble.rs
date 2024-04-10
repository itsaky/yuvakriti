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

use bytecode::args::DisassembleArgs;
use bytecode::bytes::ByteInput;
use bytecode::YKBDisassembler;

pub fn do_disassemble(args: &mut DisassembleArgs) -> Result<(), ()> {
    if !args.file.exists() {
        println!("File not found: {}", args.file.display());
        return Err(());
    }

    perform_disassembly(args)?;

    Ok(())
}

fn perform_disassembly(args: &DisassembleArgs) -> Result<(), ()> {
    let file = File::open(&args.file).unwrap();
    let input = ByteInput::new(file);
    let mut out = String::new();
    let mut disassembler = YKBDisassembler::new(input, &mut out);
    disassembler.disassemble().unwrap();

    println!("{}", out);

    Ok(())
}
