/*
 * Copyright (c) 2024 Akash Yadav
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
use std::io::Read;
use std::path::PathBuf;

use log::debug;
use log::error;
use log::info;

use crate::args::CompileArgs;
use crate::ast::Program;
use crate::bytecode::EXT_YK;
use crate::bytecode::EXT_YKB;
use crate::bytecode::YKBFile;
use crate::bytecode::YKBFileWriter;
use crate::bytecode::YKBVersion;
pub use crate::comp::attr::Attr;
pub use crate::comp::constfold::ConstFold;
pub use crate::comp::resolve::Resolve;
use crate::diagnostics::CollectingDiagnosticHandler;
use crate::features::CompilerFeatures;
use crate::lexer::YKLexer;
use crate::parser::YKParser;

mod attr;
mod constfold;
mod resolve;

// Compiles source files into bytecode.
pub struct YKCompiler {
    diagnostics: CollectingDiagnosticHandler,
}

impl YKCompiler {
    /// Creates a new compiler instance.
    pub fn new() -> YKCompiler {
        YKCompiler {
            diagnostics: CollectingDiagnosticHandler::new(),
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

        let (mut program, mut has_errors) = self.parse(file)?;
        has_errors |= self.attr(&mut program, features);

        if has_errors {
            info!("[{:?}] Compilation failed", display);
            return Err(());
        }

        info!("[{:?}] Generating bytecode", display);
        let mut ykbfile = self.ir(&mut program, features);

        info!("[{:?}] Writing bytecode", display);

        let bytecode_path = path.with_extension(EXT_YKB);
        let outfile = File::create(&bytecode_path).unwrap();

        ykbfile.write_to(&outfile).unwrap();

        info!("[{:?}] Compilation successful", display);

        Ok(())
    }

    /// Parse source code and return the resulting AST.
    pub fn parse<R: Read>(&mut self, source: R) -> Result<(Program, bool), ()> {
        let lexer = YKLexer::new(source, &mut self.diagnostics);
        let mut parser = YKParser::new(lexer);
        let program = parser.parse();
        Ok((program, parser.has_errors()))
    }

    /// Run the attribution phase on the given program and return whether any errors were found.
    pub fn attr(&mut self, program: &mut Program, features: &CompilerFeatures) -> bool {
        let mut attr = Attr::new(features, &mut self.diagnostics);
        attr.analyze(program);
        attr.has_errors()
    }

    /// Generate the intermediate [YKBFile] representation for the given program.
    pub fn ir(&mut self, program: &mut Program, features: &CompilerFeatures) -> YKBFile {
        let mut ykbfile = YKBFile::new(YKBVersion::LATEST.clone());
        let mut ykbwriter = YKBFileWriter::new(&mut ykbfile, features);
        ykbwriter.write(program);
        ykbfile
    }
}
