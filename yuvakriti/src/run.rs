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

use log::error;

use compiler::bytecode::bytes::ByteInput;
use compiler::bytecode::YKBFileReader;
use vm::args::RunArgs;
use vm::YKVM;

pub fn do_run(args: &mut RunArgs) -> Result<(), ()> {
    if !args.path.exists() {
        error!("File does not exist: {}", args.path.display());
        return Err(());
    }

    let mut reader = YKBFileReader::new(ByteInput::new(File::open(&args.path).unwrap()));
    let mut file = reader.read_file().unwrap();
    let mut vm = YKVM::new();

    vm.run(&mut file)
        .map_err(|err| error!("{}", err))
        .map(|_| ())
}
