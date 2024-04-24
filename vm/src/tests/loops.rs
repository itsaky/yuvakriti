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

use crate::tests::util::eval_src;
use crate::Value;

#[test]
fn test_cmp_gez_r() {
    assert_eq!(Value::Bool(false), eval_src("var a = 10; 0 >= a;"))
}

#[test]
fn test_while_loop() {
    assert_eq!(
        Value::Number(10f64),
        eval_src("var a = 0; while a < 10 { a = a + 1; } a;")
    )
}

#[test]
fn test_while_loop2() {
    assert_eq!(
        Value::Number(0f64),
        eval_src("var a = 10; while a > 0 { a = a - 1; } a;")
    )
}

#[test]
fn test_while_loop3() {
    assert_eq!(
        Value::Number(1f64),
        eval_src("var a = 1; var b; while b { a = a + 1; } a;")
    )
}

#[test]
fn test_for_loop() {
    assert_eq!(
        Value::Number(55f64),
        eval_src("var sum = 0; for (var i = 1; i <= 10; i = i + 1) { sum = sum + i; } sum;")
    )
}

#[test]
fn test_for_loop2() {
    assert_eq!(
        Value::Number(0f64),
        eval_src("var sum = 0; for (var i = 1; false; i = i + 1) { sum = sum + i; } sum;")
    )
}

#[test]
fn test_for_loop3() {
    assert_eq!(
        Value::Number(55f64),
        eval_src("var sum = 0; for (var i = 10; i > 0; i = i - 1) { sum = sum + i; } sum;")
    )
}

#[test]
fn test_continue_in_while_stmt() {
    assert_eq!(
        Value::Number(50f64),
        eval_src(
            "var a = 11;
            var s = 0;
            while a > 0 {
                a = a - 1;
                if a == 5 {
                    continue;
                }
                s = s + a;
            }
            s;"
        )
    )
}

#[test]
fn test_continue_in_nested_while_stmt() {
    assert_eq!(
        Value::Number(143f64),
        eval_src(
            "var a = 11;
            var s = 0;
            while a > 0 {
                a = a - 1;
                var j = 0;
                while j < 4 {
                    j = j + 1;
                    if j == 2 {
                        continue;
                    }
                    s = s + j;
                }
                s = s + a;
            }
            s;"
        )
    )
}

#[test]
fn test_break_in_nested_while_stmt() {
    assert_eq!(
        Value::Number(66f64),
        eval_src(
            "var a = 11;
            var s = 0;
            while a > 0 {
                a = a - 1;
                var j = 0;
                while j < 4 {
                    j = j + 1;
                    if j == 2 {
                        break;
                    }
                    s = s + j;
                }
                s = s + a;
            }
            s;"
        )
    )
}

#[test]
fn test_break_in_nested_for_stmt() {
    assert_eq!(
        Value::Number(65f64),
        eval_src(
            "
            var s = 0;
            for (var a = 10; a > 0; a = a-1) {
                for(var j = 1; j < 4; j = j + 1) {
                    if j == 2 {
                        break;
                    }
                    s = s + j;
                }
                s = s + a;
            }
            s;"
        )
    )
}

#[test]
fn test_break_in_while_stmt() {
    assert_eq!(
        Value::Number(40f64),
        eval_src(
            "var a = 11;
            var s = 0;
            while a > 0 {
                a = a - 1;
                if a == 5 {
                    break;
                }
                s = s + a;
            }
            s;"
        )
    )
}

#[test]
fn test_continue_in_for_stmt() {
    assert_eq!(
        Value::Number(40f64),
        eval_src(
            "
            var s = 0;
            for (var i =0; i < 10; i = i+1) {
              if i == 5 {
                continue;
              }
              s = s + i;
            }
            s;"
        )
    )
}

#[test]
fn test_break_in_for_stmt() {
    assert_eq!(
        Value::Number(10f64),
        eval_src(
            "
            var s = 0;
            for (var i =0; i < 10; i = i+1) {
              if i == 5 {
                break;
              }
              s = s + i;
            }
            s;"
        )
    )
}

#[test]
fn test_labeled_break_in_nested_for_stmt() {
    assert_eq!(
        Value::Number(1f64),
        eval_src(
            "
            var s = 0;
            outer: for (var a = 10; a > 0; a = a-1) {
                for(var j = 1; j < 4; j = j + 1) {
                    if j == 2 {
                        break outer;
                    }
                    s = s + j;
                }
                s = s + a;
            }
            s;"
        )
    )
}

#[test]
fn test_labeled_break_in_nested_while_stmt() {
    assert_eq!(
        Value::Number(1f64),
        eval_src(
            "var a = 11;
            var s = 0;
            outer: while a > 0 {
                a = a - 1;
                var j = 0;
                while j < 4 {
                    j = j + 1;
                    if j == 2 {
                        break outer;
                    }
                    s = s + j;
                }
                s = s + a;
            }
            s;"
        )
    )
}

#[test]
fn test_labeled_continue_in_nested_for_stmt() {
    assert_eq!(
        Value::Number(10f64),
        eval_src(
            "
            var s = 0;
            outer: for (var a = 10; a > 0; a = a-1) {
                for(var j = 1; j < 4; j = j + 1) {
                    if j == 2 {
                        continue outer;
                    }
                    s = s + j;
                }
                s = s + a;
            }
            s;"
        )
    )
}

#[test]
fn test_labeled_continue_in_nested_while_stmt() {
    assert_eq!(
        Value::Number(11f64),
        eval_src(
            "var a = 11;
            var s = 0;
            outer: while a > 0 {
                a = a - 1;
                var j = 0;
                while j < 4 {
                    j = j + 1;
                    if j == 2 {
                        continue outer;
                    }
                    s = s + j;
                }
                s = s + a;
            }
            s;"
        )
    )
}
