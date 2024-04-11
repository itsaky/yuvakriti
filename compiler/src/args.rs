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

use std::path::PathBuf;

use clap::Args;

#[derive(Args, Debug)]
#[command(visible_alias = "c")]
pub struct CompileArgs {
    #[arg(short, long, help = "Disable language features", value_delimiter = ',', num_args = 1.., value_name = "FEATURE")]
    pub disable_features: Vec<String>,

    #[arg(short, long, help = "Output file", value_name = "FILE")]
    pub output: Option<PathBuf>,

    #[arg(help = "Input source file(s)")]
    pub files: Vec<PathBuf>,
}

#[derive(Args, Debug)]
#[command(visible_alias = "d")]
pub struct DisassembleArgs {
    #[arg(help = "Input bytecode file")]
    pub file: PathBuf,
}
