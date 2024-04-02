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
pub(crate) const LEX_UNKNOWN_TOKEN: &str = "unknown token";
pub(crate) const LEX_UNEXPECTED_EOF: &str = "unexpected EOF";
pub(crate) const LEX_STRING_MULTILINE_ERROR: &str = "multiline strings are not supported";
pub(crate) const LEX_STRING_EXPECTED_ESC_SEQ: &str = "expected an escape sequence";
pub(crate) const LEX_STRING_UNRECOGNIZED_ESC_SEQ: &str = "unrecognized escape sequence";
pub(crate) const LEX_STRING_ILLEGAL_UNICODE_ESC: &str = "illegal unicode escape";

// ------------------------ parser --------------------------
pub(crate) const PARS_DECL_OR_STMT_EXPECTED: &str = "expected a declaration or statement";
pub(crate) const PARS_UNEXPECTED_EOF: &str = "unexpected EOF";
pub(crate) const PARS_EXPECTED_VAR_NAME: &str = "expected a variable name";
pub(crate) const PARS_EXPECTED_UNARY_OP: &str = "expected a unary operator";
pub(crate) const PARS_EXPECTED_EXPR: &str = "expected an expression";
pub(crate) const PARS_EXPECTED_FUN_NAME: &str = "expected a function name";
pub(crate) const PARS_EXPECTED_PARAM_NAME: &str = "expected a parameter name";
pub(crate) const PARS_EXPECTED_STMT: &str = "expected a statement";
pub(crate) const PARS_EXPECTED_BODY: &str = "expected body";
pub(crate) const PARS_INVALID_ASSIGN_TARGET: &str = "invalid assignment target";

pub(crate) fn err_exp_kywrd(keyword: &str) -> String {
    return format!("expected '{}' keyword", keyword)
}

pub(crate) fn err_exp_sym(sym: &str) -> String {
    return format!("expected a '{}'", sym)
}
