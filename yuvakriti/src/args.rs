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

use clap::Parser;
use clap::Subcommand;

use bytecode::args::DisassembleArgs;
use compiler::args::CompileArgs;
use vm::args::RunArgs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct YkArgs {
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,

    #[arg(
        short,
        long,
        help = "Enable verbose output",
        default_value_t = false,
        global = true
    )]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Compile the input source file(s) and output the corresponding bytecode.
    Compile(CompileArgs),

    /// Run the compiled bytecode.
    Run(RunArgs),

    /// Disassemble the compiled bytecode.
    Disassemble(DisassembleArgs),
}

impl SubCommand {
    pub fn name(&self) -> &str {
        match self {
            SubCommand::Compile(_) => "compile",
            SubCommand::Run(_) => "run",
            SubCommand::Disassemble(_) => "disassemble",
        }
    }
}
