/*
 * Copyright (c) 2024 Akash Yadav
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

use crate::tests::util::{eval_arithmetic_src, eval_src};
use crate::value::Value;

#[test]
fn test_simple_unary_number_negation() {
    assert_eq!(-10f64, eval_arithmetic_src("var a = 10; -a;"));
}

#[test]
fn test_simple_unary_number_double_negation() {
    assert_eq!(10f64, eval_arithmetic_src("var a = 10; -(-a);"));
}

#[test]
fn test_simple_unary_number_negation_on_boolean() {
    assert_eq!(0f64, eval_arithmetic_src("var a = true; -a;"));
    assert_eq!(0f64, eval_arithmetic_src("var a = false; -a;"));
}

#[test]
fn test_simple_unary_bool_negation() {
    assert_eq!(Value::Bool(false), eval_src("var a = true; !a;"));
}

#[test]
fn test_simple_unary_bool_double_negation() {
    assert_eq!(Value::Bool(true), eval_src("var a = true; !!a;"));
}

#[test]
fn test_simple_unary_bool_negation_on_number() {
    assert_eq!(Value::Bool(false), eval_src("var a = 10; !a;"));
    assert_eq!(Value::Bool(false), eval_src("var a = 0; !a;"));
}
