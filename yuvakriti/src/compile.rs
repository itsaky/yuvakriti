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
use std::path::PathBuf;
use std::rc::Rc;

use bytecode::bytes::ByteOutput;
use bytecode::YKBFileWriter;
use clap::Args;
use compiler::diagnostics::collecting_handler;
use compiler::lexer::YKLexer;
use compiler::parser::YKParser;
use log::debug;

#[derive(Args, Debug)]
#[command(visible_alias = "c")]
pub struct CompileArgs {
    #[arg(help = "Input source file(s)")]
    pub files: Vec<PathBuf>,
}

pub fn do_compile(args: &CompileArgs) -> Result<(), ()> {
    if args.files.is_empty() {
        println!("No files to compile...!");
        println!();
        return Err(());
    }

    perform_compilation(args)?;

    Ok(())
}

fn perform_compilation(args: &CompileArgs) -> Result<(), ()> {
    for path in &args.files {
        let path_display = path.display();
        debug!("Compiling: {}", path_display);
        let diagnostics_handler = Rc::new(RefCell::new(collecting_handler()));
        let file = File::open(path).expect(&format!("Failed to open file: {}", path_display));
        let lexer = YKLexer::new(file, diagnostics_handler.clone());
        let mut parser = YKParser::new(lexer, diagnostics_handler.clone());
        let mut program = parser.parse();

        let mut ykbwriter = YKBFileWriter::new();
        ykbwriter.write(&mut program);

        let ykbfile = ykbwriter.file_mut();
        let bytecode_path = path.with_extension("ykb");
        ykbfile
            .write_to(&mut ByteOutput::new(File::create(&bytecode_path).unwrap()))
            .unwrap();

        if !diagnostics_handler.borrow().diagnostics.is_empty() {
            // TODO(itsaky): write a diagnostics printer
            println!("Diagnostics found while compiling: {}", path_display);
        }

        if parser.has_errors() {
            println!("Errors found while compiling: {}", path_display);
            return Err(());
        }
    }
    Ok(())
}
