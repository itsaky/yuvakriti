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

use compiler::args::CompileArgs;
use compiler::bytecode::compiler::YKCompiler;
use compiler::features::CompilerFeatures;
use log::{info, trace};

pub fn do_compile(args: &mut CompileArgs) -> Result<(), ()> {
    if args.files.is_empty() {
        info!("No files to compile...!");
        return Err(());
    }

    perform_compilation(args)?;

    Ok(())
}

fn perform_compilation(args: &CompileArgs) -> Result<(), ()> {
    trace!("Compiler args: {:?}", args);

    let mut features = CompilerFeatures::default();
    for feature in &args.disable_features {
        features.set(&feature, false);
    }

    let mut compiler = YKCompiler::new();
    compiler.compile(args, &features)
}
