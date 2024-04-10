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
use std::path::Path;

use crate::tests::util::compile_to_bytecode;
use crate::{bytes::ByteInput, YKBFileReader};

#[test]
fn test_program_writer() {
    let path = Path::new("target/reader.ykb");
    let display = path.display();
    let ykbwriter = compile_to_bytecode("fun main() { var str = \"str\"; var num = 123; }", &path);
    let ykbfile = ykbwriter.file();

    let f = File::open(&path).unwrap();
    let readykb = match YKBFileReader::new(ByteInput::new(f)).read_file() {
        Ok(file) => file,
        Err(why) => {
            panic!("couldn't read from file {}: {}", display, why);
        }
    };

    assert_eq!(&readykb.version(), &ykbfile.version());
    assert_eq!(&readykb.constant_pool(), &ykbfile.constant_pool());
}
