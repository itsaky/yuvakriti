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

use crate::boxed_vec;
use crate::features::CompilerFeatures;
use crate::tests::matcher::Number;
use crate::tests::matcher::Program;
use crate::tests::matcher::{Bool, String};
use crate::tests::util::match_node;
use crate::tests::util::parse_attr;

#[test]
fn test_bool_op_expr_binary_fold() {
    let mut features = CompilerFeatures::default();
    features.const_folding = true;

    let cases = [
        ("true and 1", Number(1f64)),
        ("true and 0", Number(0f64)),
        ("false and 1", Bool(false)),
        ("false and 0", Bool(false)),
        ("true or 1", Bool(true)),
        ("true or 0", Bool(true)),
        ("false or 1", Number(1f64)),
        ("false or 0", Number(0f64)),
        ("true and \"str\"", String("\"str\"")),
        ("false and \"string\"", Bool(false)),
        ("true or \"str\"", Bool(true)),
        ("false or \"Str\"", String("\"Str\"")),
    ];

    for (src, expected) in cases {
        println!("[ConstantFolding] Check case: {}", src);
        match_node(
            &mut parse_attr(&format!("{};", src), true, &features),
            &mut Program(vec![], boxed_vec![expected]),
        );
    }
}
