/*
 * Copyright (c) 2024 The YuvaKriti Authors.
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

use crate::compiler::location::Range;
use std::fmt::{Display, Formatter};

#[derive(Eq, Debug)]
pub(crate) struct Token {
    /// See [TokenType] for a list of valid tokens.
    pub(crate) token_type: TokenType,

    /// The substring from the source code represented by this token
    pub(crate) text: String,

    /// The range of the token
    /// The column and index in end position of token is exclusive
    pub(crate) range: Range,
}

impl PartialEq<Self> for Token {
    fn eq(&self, other: &Self) -> bool {
        return self.token_type == other.token_type
            && self.text == other.text
            && self.range == other.range;
    }
}

/// Token types for YKLang
#[derive(Eq, Debug)]
pub(crate) enum TokenType {
    LParen, // (
    RParen, // )
    LBrack, // [
    RBrack, // ]
    LBrace, // {
    RBrace, // }

    Plus,      // +
    Minus,     // -
    Asterisk,  // *
    Slash,     // /
    Comma,     // ,
    Dot,       // .
    Colon,     // :
    Semicolon, // ;

    Bang,   // !
    Eq,     // =
    BangEq, // !=
    EqEq,   // ==
    Gt,     // >
    GtEq,   // >=
    Lt,     // <
    LtEq,   // <=

    And, // and
    Or,  // or

    If,     // if
    Else,   // else
    Fun,    // fun
    For,    // for
    While,  // while
    Nil,    // nil
    Return, // return
    Var,    // var
    Super,  // super
    This,   // this
    Print,  // print

    Identifier,
    String,
    Number,

    True,  // true
    False, // false

    Comment, // Anything after a '//' (inclusive)
}

impl PartialEq<Self> for TokenType {
    fn eq(&self, other: &Self) -> bool {
        return std::mem::discriminant(self) == std::mem::discriminant(other);
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
