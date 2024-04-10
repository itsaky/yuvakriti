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

use bytecode::bytes::ByteInput;
use bytecode::YKBFileReader;
use clap::Args;
use log::error;
use std::fs::File;
use std::path::PathBuf;
use vm::YKVM;

#[derive(Args, Debug)]
#[command(visible_alias = "r")]
pub struct RunArgs {
    #[arg(help = "Input bytecode file(s)")]
    pub path: PathBuf,
}

pub fn do_run(args: &RunArgs) -> Result<(), ()> {
    if !args.path.exists() {
        println!("File does not exist: {}", args.path.display());
        return Err(());
    }

    let mut reader = YKBFileReader::new(ByteInput::new(File::open(&args.path).unwrap()));
    let mut file = reader.read_file().unwrap();
    let mut vm = YKVM::new();

    vm.run(&mut file).map_err(|err| error!("{}", err))
}
