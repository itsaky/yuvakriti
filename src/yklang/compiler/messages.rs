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

pub(crate) const LEX_UNKNOWN_TOKEN: &str = "unknown token";
pub(crate) const LEX_UNEXPECTED_EOF: &str = "unexpected EOF";
pub(crate) const LEX_STRING_MULTILINE_ERROR: &str = "multiline strings are not supported";
pub(crate) const LEX_STRING_EXPECTED_ESC_SEQ: &str = "expected an escape sequence";
pub(crate) const LEX_STRING_UNRECOGNIZED_ESC_SEQ: &str = "unrecognized escape sequence";
pub(crate) const LEX_STRING_ILLEGAL_UNICODE_ESC: &str = "illegal unicode escape";