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

use clap::Parser;
use clap::Subcommand;

use compiler::args::CompileArgs;
use compiler::args::DisassembleArgs;
use vm::args::RunArgs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct YkArgs {
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,

    #[arg(
        short,
        long,
        help = "Set output verbosity.
    0: silent,
    1: errors only,
    2: normal,
    3: information,
    4: debug,
    5: trace",
        default_value_t = 2,
        global = true
    )]
    pub verbosity: usize,

    #[arg(
        short,
        long,
        help = "Silence all output, no matter what the verbosity is set to. This is similar to '-v 0'.",
        default_value_t = false,
        global = true
    )]
    pub quiet: bool,
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
