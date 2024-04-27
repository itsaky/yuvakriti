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

use crate::tests::util::eval_src;

#[test]
fn test_simple_arr_get() {
    let value = eval_src("var arr = [1,2,3]; arr[0];");
    assert!(value.is_truthy());

    let num = value.take_Number();
    assert!(num.is_some());

    let num = num.unwrap();
    assert_eq!(1f64, num);
}

#[test]
fn test_simple_arr_get_with_var() {
    let value = eval_src("var arr = [1,2,3]; var idx = 2; arr[idx];");
    assert!(value.is_truthy());

    let num = value.take_Number();
    assert!(num.is_some());

    let num = num.unwrap();
    assert_eq!(3f64, num);
}

#[test]
fn test_simple_arr_put() {
    let value = eval_src("var arr = [1,2,3]; arr[0] = 5; arr[0];");
    assert!(value.is_truthy());

    let num = value.take_Number();
    assert!(num.is_some());

    let num = num.unwrap();
    assert_eq!(5f64, num);
}

#[test]
fn test_simple_arr_put_with_var() {
    let value = eval_src("var arr = [1,2,3]; var idx = 0; var val = 5; arr[idx] = val; arr[idx];");
    assert!(value.is_truthy());

    let num = value.take_Number();
    assert!(num.is_some());

    let num = num.unwrap();
    assert_eq!(5f64, num);
}