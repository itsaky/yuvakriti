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

use log::debug;
use log::error;
use log::info;

use crate::args::CompileArgs;
use crate::diagnostics::collecting_handler;
use crate::diagnostics::CollectingDiagnosticHandler;
use crate::features::CompilerFeatures;
use crate::lexer::YKLexer;
use crate::parser::YKParser;

use crate::bytecode::file::EXT_YK;
use crate::bytecode::file::EXT_YKB;
use crate::bytecode::{YKBFile, YKBFileWriter, YKBVersion};

// Compiles source files into bytecode.
pub struct YKCompiler {
    diagnostics: Rc<RefCell<CollectingDiagnosticHandler>>,
}

impl YKCompiler {
    /// Creates a new compiler instance.
    pub fn new() -> YKCompiler {
        YKCompiler {
            diagnostics: Rc::new(RefCell::new(collecting_handler())),
        }
    }

    pub fn compile(&mut self, args: &CompileArgs, features: &CompilerFeatures) -> Result<(), ()> {
        for path in &args.files {
            if !path.exists() {
                error!("File not found: {}", path.display());
                return Err(());
            }

            if !path.extension().is_some_and(|ext| ext == EXT_YK) {
                error!("Invalid file type: {}", path.display());
                return Err(());
            }

            self.perform_compilation(path, args, features)?;
        }

        Ok(())
    }

    fn perform_compilation(
        &mut self,
        path: &PathBuf,
        _args: &CompileArgs,
        features: &CompilerFeatures,
    ) -> Result<(), ()> {
        let display = path.file_name().unwrap();
        debug!("[{:?}] Compiling", display);

        let file = File::open(path).unwrap();

        info!("[{:?}] Parsing file", display);

        let lexer = YKLexer::new(file, self.diagnostics.clone());
        let mut parser = YKParser::new(lexer, self.diagnostics.clone());
        let mut program = parser.parse();

        if parser.has_errors() {
            info!("[{:?}] Compilation failed", display);
            return Err(());
        }

        info!("[{:?}] Generating bytecode", display);
        let mut ykbfile = YKBFile::new(YKBVersion::LATEST.clone());
        let mut ykbwriter = YKBFileWriter::new(&mut ykbfile, features);
        ykbwriter.write(&mut program);

        info!("[{:?}] Writing bytecode", display);

        let bytecode_path = path.with_extension(EXT_YKB);
        let outfile = File::create(&bytecode_path).unwrap();

        ykbfile.write_to(&outfile).unwrap();

        info!("[{:?}] Compilation successful", display);

        Ok(())
    }
}
