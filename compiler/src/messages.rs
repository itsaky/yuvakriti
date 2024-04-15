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

// ------------------------ lexer --------------------------
pub const LEX_UNKNOWN_TOKEN: &str = "unknown token";
pub const LEX_UNEXPECTED_EOF: &str = "unexpected EOF";
pub const LEX_STRING_MULTILINE_ERROR: &str = "multiline strings are not supported";
pub const LEX_STRING_EXPECTED_ESC_SEQ: &str = "expected an escape sequence";
pub const LEX_STRING_UNRECOGNIZED_ESC_SEQ: &str = "unrecognized escape sequence";
pub const LEX_STRING_ILLEGAL_UNICODE_ESC: &str = "illegal unicode escape";

// ------------------------ parser --------------------------
pub const PARS_DECL_OR_STMT_EXPECTED: &str = "expected a declaration or statement";
pub const PARS_UNEXPECTED_EOF: &str = "unexpected EOF";
pub const PARS_EXPECTED_VAR_NAME: &str = "expected a variable name";
pub const PARS_EXPECTED_UNARY_OP: &str = "expected a unary operator";
pub const PARS_EXPECTED_EXPR: &str = "expected an expression";
pub const PARS_EXPECTED_FUN_NAME: &str = "expected a function name";
pub const PARS_EXPECTED_PARAM_NAME: &str = "expected a parameter name";
pub const PARS_EXPECTED_STMT: &str = "expected a statement";
pub const PARS_EXPECTED_BODY: &str = "expected body";
pub const PARS_INVALID_ASSIGN_TARGET: &str = "invalid assignment target";

pub fn err_exp_kywrd(keyword: &str) -> String {
    return format!("expected '{}' keyword", keyword);
}

pub fn err_exp_sym(sym: &str) -> String {
    return format!("expected a '{}'", sym);
}

// ------------------------ analyzer --------------------------
pub fn err_dup_var(sym: &str) -> String {
    return format!("Variable '{}' is already declared", sym);
}
pub fn err_undecl_var(sym: &str) -> String {
    return format!("Variable '{}' is not declared", sym);
}
