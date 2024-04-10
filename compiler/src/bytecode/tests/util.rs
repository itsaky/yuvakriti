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

use crate::bytecode::{YKBFile, YKBFileWriter, YKBVersion};
use crate::features::CompilerFeatures;
use crate::tests::util::parse;

pub(crate) fn compile_to_bytecode<'a>(
    features: &CompilerFeatures,
    source: &str,
    bytecode_path: &Path,
) -> YKBFile {
    std::fs::create_dir_all(bytecode_path.parent().unwrap()).unwrap();
    if bytecode_path.exists() {
        std::fs::remove_file(bytecode_path).unwrap();
    }

    let mut program = parse(source);
    let mut ykbfile = YKBFile::new(YKBVersion::LATEST.clone());
    let mut ykbwriter = YKBFileWriter::new(&mut ykbfile, &features);
    ykbwriter.write(&mut program);

    let display = bytecode_path.display();
    let file = match File::create(&bytecode_path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match ykbfile.write_to(&file) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(size) => {
            println!("successfully wrote to {}", display);
            size
        }
    };

    file.sync_all().unwrap();

    assert!(bytecode_path.exists());

    ykbfile
}
