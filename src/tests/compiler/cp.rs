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

use crate::compiler::bytecode::{ConstantEntry, YKBFileWriter};
use crate::compiler::bytecode::ConstantPool;
use crate::compiler::bytecode::cp_info::NumberInfo;
use crate::compiler::bytecode::cp_info::Utf8Info;
use crate::tests::compiler::util::parse;

#[test]
fn test_cp_push() {
    let mut pool = ConstantPool::new();

    pool.push(ConstantEntry::Number(NumberInfo::from(&123f64)));
    assert_eq!(2, pool.len());

    pool.push(ConstantEntry::Number(NumberInfo::from(&1234f64)));
    assert_eq!(3, pool.len());
}

#[test]
fn test_cp_duplicate_push() {
    let mut pool = ConstantPool::new();
    let mut index = pool.push(ConstantEntry::Number(NumberInfo::from(&123f64)));
    assert_eq!(2, pool.len());
    assert_eq!(1, index);

    index = pool.push(ConstantEntry::Number(NumberInfo::from(&123f64)));
    assert_eq!(2, pool.len());
    assert_eq!(1, index);

    index = pool.push(ConstantEntry::Utf8(Utf8Info::from("something")));
    assert_eq!(3, pool.len());
    assert_eq!(2, index);

    index = pool.push(ConstantEntry::Utf8(Utf8Info::from("something")));
    assert_eq!(3, pool.len());
    assert_eq!(2, index);

    index = pool.push(ConstantEntry::Utf8(Utf8Info::from("something else")));
    assert_eq!(4, pool.len());
    assert_eq!(3, index);
}

#[test]
fn test_cp_push_from_program() {
    let mut program = parse("var a = 123; var b = 123; var c = \"something\"; var d = \"something\"; var e = \"something else\";");
    let mut writer = YKBFileWriter::new();
    let file = writer.write(&mut program);
    let constant_pool = file.constant_pool();
    
    // 0 -> ConstantEntry::None
    // 1 -> NumberInfo
    // 2 -> Utf8Info
    // 3 -> StringInfo
    // 4 -> Utf8Info
    // 5 -> StringInfo
    assert_eq!(6, constant_pool.len());
}
