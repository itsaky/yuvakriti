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

use crate::bytes::ByteOutput;
use crate::disassembler::YKBDisassembler;
use crate::tests::util::parse;
use crate::{ByteInput, YKBFileWriter};
use std::fs::File;
use std::path::Path;

#[test]
fn test_disassembler() {
    let mut program = parse("fun main() { var str = \"str\"; var num = 123; }");
    let mut ykbwriter = YKBFileWriter::new();
    let ykbfile = ykbwriter.write(&mut program);

    let path = Path::new("test.ykb");
    let display = path.display();
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let size = match ykbfile.write_to(&mut ByteOutput::new(&file)) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(size) => {
            println!("successfully wrote to {}", display);
            size
        }
    };

    file.sync_all().unwrap();

    assert!(path.exists());
    assert_eq!(size, file.metadata().unwrap().len() as usize);

    let f = File::open(&path).unwrap();
    let mut out_string = String::new();
    let mut disassembler = YKBDisassembler::new(ByteInput::new(f), &mut out_string);
    disassembler.disassemble();
    println!("{}", out_string);
}