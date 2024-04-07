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

use crate::bytes::ByteOutput;
use crate::tests::util::parse;
use crate::{ByteInput, YKBFileReader, YKBFileWriter};

#[test]
fn test_program_writer() {
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
    let readykb = match YKBFileReader::new(ByteInput::new(f)).read_file() {
        Ok(file) => file,
        Err(why) => {
            panic!("couldn't read from file {}: {}", display, why);
        }
    };

    assert_eq!(&readykb.version(), &ykbfile.version());
    assert_eq!(&readykb.constant_pool(), &ykbfile.constant_pool());
}
