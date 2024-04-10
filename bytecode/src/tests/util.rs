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

use std::cell::RefCell;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::rc::Rc;

use compiler::ast::Program;
use compiler::diagnostics;
use compiler::lexer::YKLexer;
use compiler::parser::YKParser;

use crate::bytes::ByteOutput;
use crate::YKBFileWriter;

pub(crate) fn compile_to_bytecode<'a>(source: &str, bytecode_path: &Path) -> YKBFileWriter {
    std::fs::create_dir_all(bytecode_path.parent().unwrap()).unwrap();
    if bytecode_path.exists() {
        std::fs::remove_file(bytecode_path).unwrap();
    }

    let mut program = parse(source);
    let mut ykbwriter = YKBFileWriter::new();
    ykbwriter.write(&mut program);

    let ykbfile = ykbwriter.file_mut();

    let display = bytecode_path.display();
    let file = match File::create(&bytecode_path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match ykbfile.write_to(&mut ByteOutput::new(&file)) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(size) => {
            println!("successfully wrote to {}", display);
            size
        }
    };

    file.sync_all().unwrap();

    assert!(bytecode_path.exists());

    ykbwriter
}

//noinspection DuplicatedCode
pub(crate) fn parse(source: &str) -> Program {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let lexer = YKLexer::new(Cursor::new(source), diag_handler.clone());
    let mut parser = YKParser::new(lexer, diag_handler.clone());
    parser.parse()
}
