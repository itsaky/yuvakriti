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

use crate::bytecode::cp_info::NumberInfo;
use crate::bytecode::cp_info::Utf8Info;

#[test]
fn test_cp_info_num_eq() {
    let f = NumberInfo::from(&123f64);
    let s = NumberInfo::from(&123f64);
    let t = NumberInfo::from(&-123.456);
    assert_eq!(f, s);
    assert_eq!(t, t);
    assert_ne!(f, t);
    assert_ne!(s, t);

    assert_eq!(f.to_f64(), 123f64);
    assert_eq!(s.to_f64(), 123f64);
    assert_eq!(t.to_f64(), -123.456);
}

#[test]
fn test_cp_info_utf_eq() {
    let f = Utf8Info::from("some");
    let s = Utf8Info::from("some");
    assert_eq!(f, s);
    assert_ne!(f, Utf8Info::from("something else"));
}
