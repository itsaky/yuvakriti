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

use crate::cp::ConstantEntry;
use crate::cp::ConstantPool;
use crate::cp_info::NumberInfo;
use crate::cp_info::StringInfo;
use crate::cp_info::Utf8Info;
use crate::tests::util::parse;
use crate::writer::YKBFileWriter;

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
    let mut program = parse(
        "var a = 123;\
     var b = 123;\
     var c = \"something\";\
     var d = \"something\";\
     var e = \"something else\";\
    ",
    );
    let mut writer = YKBFileWriter::new();
    let file = writer.write(&mut program);
    let constant_pool = file.constant_pool();

    // 0 -> ConstantEntry::None
    // 1 -> NumberInfo => 123
    // 2 -> Utf8Info => "something"
    // 3 -> StringInfo => cp[2]
    // 4 -> Utf8Info => "something else"
    // 5 -> StringInfo => cp[4]
    // 6 -> Utf8Info => "Code" => Name of the "Code" attribute for the top-level statements
    assert!(7 <= constant_pool.len());
    assert_eq!(
        1,
        constant_pool
            .lookup(&ConstantEntry::Number(NumberInfo::from(&123f64)))
            .unwrap()
    );
    assert_eq!(
        2,
        constant_pool
            .lookup(&ConstantEntry::Utf8(Utf8Info::from("something")))
            .unwrap()
    );
    assert_eq!(
        3,
        constant_pool
            .lookup(&ConstantEntry::String(StringInfo { string_index: 2 }))
            .unwrap()
    );
    assert_eq!(
        4,
        constant_pool
            .lookup(&ConstantEntry::Utf8(Utf8Info::from("something else")))
            .unwrap()
    );
    assert_eq!(
        5,
        constant_pool
            .lookup(&ConstantEntry::String(StringInfo { string_index: 4 }))
            .unwrap()
    );
}
