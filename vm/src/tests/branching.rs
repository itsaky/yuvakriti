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

use crate::tests::util::eval_arithmetic_src;
use crate::tests::util::eval_src;
use crate::Value;

fn eval_bool4(expr: &str, cond: &dyn Fn(bool, bool, bool, bool) -> bool) {
    for a in 0..2 {
        for b in 0..2 {
            for c in 0..2 {
                for d in 0..2 {
                    let a = a != 0;
                    let b = b != 0;
                    let c = c != 0;
                    let d = d != 0;

                    println!("Test case : a={} b={} c={} d={}", a, b, c, d);
                    // a and b and c and d
                    assert_eq!(
                        Value::Bool(cond(a, b, c, d)),
                        eval_src(&format!(
                            "var a = {}; var b = {}; var c = {}; var d = {}; {};",
                            a, b, c, d, expr
                        ))
                    );
                }
            }
        }
    }
}

#[test]
fn test_simple_branching() {
    assert_eq!(
        &10f64,
        eval_src("var a = 10; var b = 20; if false { b + a; } else { b - a; }")
            .Number()
            .unwrap()
    );
}

#[test]
fn test_nested_and_branch() {
    eval_bool4("a and b and c and d", &|a, b, c, d| a && b && c && d);
}

#[test]
fn test_nested_or_branching() {
    eval_bool4("a or b or c or d", &|a, b, c, d| a || b || c || d);
}

#[test]
fn test_nested_and_or_branch() {
    eval_bool4("a and b or c and d", &|a, b, c, d| a && b || c && d);
}

#[test]
fn test_nested_or_and_branch() {
    eval_bool4("a or b and c or d", &|a, b, c, d| a || b && c || d);
}

#[test]
fn test_nested_or_and_branch1() {
    eval_bool4("a or b or c and d", &|a, b, c, d| a || b || c && d);
}

#[test]
fn test_nested_or_and_branch2() {
    eval_bool4("a and b or c or d", &|a, b, c, d| a && b || c || d);
}

#[test]
fn test_simple_control_flow() {
    assert_eq!(
        30f64,
        eval_arithmetic_src("var a = 10; var b = 20; if true { a + b; } else { b - a; }")
    );
}

#[test]
fn test_simple_control_flow2() {
    assert_eq!(
        10f64,
        eval_arithmetic_src("var a = 10; var b = 20; if false { a + b; } else { b - a; }")
    );
}

#[test]
fn test_simple_control_flow3() {
    assert_eq!(
        10f64,
        eval_arithmetic_src("var a = 10; var b = 20; if false and true { a + b; } else { b - a; }")
    );
}

#[test]
fn test_simple_control_flow4() {
    assert_eq!(
        10f64,
        eval_arithmetic_src("var a = 10; var b = 20; if true and false { a + b; } else { b - a; }")
    );
}

#[test]
fn test_simple_control_flow5() {
    assert_eq!(
        30f64,
        eval_arithmetic_src("var a = 10; var b = 20; if true or false { a + b; } else { b - a; }")
    );
}

#[test]
fn test_simple_control_flow6() {
    assert_eq!(
        30f64,
        eval_arithmetic_src("var a = 10; var b = 20; if false or true { a + b; } else { b - a; }")
    );
}

#[test]
fn test_cmp_eqeq() {
    assert_eq!(
        Value::Bool(false),
        eval_src("var a = 10; var b = 20; a == b;")
    )
}

#[test]
fn test_cmp_eqeqz() {
    assert_eq!(Value::Bool(false), eval_src("var a = 10; a == 0;"))
}

#[test]
fn test_cmp_eqeqz_r() {
    assert_eq!(Value::Bool(false), eval_src("var a = 10; 0 == a;"))
}

#[test]
fn test_cmp_neq() {
    assert_eq!(
        Value::Bool(true),
        eval_src("var a = 10; var b = 20; a != b;")
    )
}

#[test]
fn test_cmp_neqz() {
    assert_eq!(Value::Bool(true), eval_src("var a = 10; a != 0;"))
}

#[test]
fn test_cmp_neqz_r() {
    assert_eq!(Value::Bool(true), eval_src("var a = 10; 0 != a;"))
}

#[test]
fn test_cmp_lt() {
    assert_eq!(
        Value::Bool(true),
        eval_src("var a = 10; var b = 20; a < b;")
    )
}

#[test]
fn test_cmp_ltz() {
    assert_eq!(Value::Bool(false), eval_src("var a = 10; a < 0;"))
}

#[test]
fn test_cmp_ltz_r() {
    assert_eq!(Value::Bool(true), eval_src("var a = 10; 0 < a;"))
}

#[test]
fn test_cmp_le() {
    assert_eq!(
        Value::Bool(true),
        eval_src("var a = 10; var b = 20; a <= b;")
    )
}

#[test]
fn test_cmp_lez() {
    assert_eq!(Value::Bool(false), eval_src("var a = 10; a <= 0;"))
}

#[test]
fn test_cmp_lez_r() {
    assert_eq!(Value::Bool(true), eval_src("var a = 10; 0 <= a;"))
}

#[test]
fn test_cmp_gt() {
    assert_eq!(
        Value::Bool(false),
        eval_src("var a = 10; var b = 20; a > b;")
    )
}

#[test]
fn test_cmp_gtz() {
    assert_eq!(Value::Bool(true), eval_src("var a = 10; a > 0;"))
}

#[test]
fn test_cmp_gtz_r() {
    assert_eq!(Value::Bool(false), eval_src("var a = 10; 0 > a;"))
}

#[test]
fn test_cmp_ge() {
    assert_eq!(
        Value::Bool(false),
        eval_src("var a = 10; var b = 20; a >= b;")
    )
}

#[test]
fn test_cmp_gez() {
    assert_eq!(Value::Bool(true), eval_src("var a = 10; a >= 0;"))
}

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
    assert_eq!(Value::Number(1f64), eval_src("var a = 1; var b; while b { a = a + 1; } a;"))
}
