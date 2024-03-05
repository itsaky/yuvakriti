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

use crate::yklang::compiler::location::Range;

#[derive(Eq, Debug)]
pub struct Token {

    /// See [TokenType] for a list of valid tokens.
    pub token_type: TokenType,

    /// The substring from the source code represented by this token
    /// This is [None] for most cases except for Identifier, String, Number and other similar
    /// token types.
    pub content: Option<String>,

    /// The range of the token
    /// The end position of token is exclusive
    pub range: Range
}

impl PartialEq<Self> for Token {
    fn eq(&self, other: &Self) -> bool {
        return self.token_type == other.token_type
            && self.content == other.content
            && self.range == other.range
    }
}


/// Token types for YKLang
#[derive(Eq, Debug)]
pub enum TokenType {

    LParen,         // (
    RParen,         // )
    LBrack,         // [
    RBrack,         // ]
    LBrace,         // {
    RBrace,         // }

    Plus,           // +
    Minus,          // -
    Asterisk,       // *
    Slash,          // /
    Comma,          // ,
    Dot,            // .
    Semicolon,      // ;

    Bang,           // !
    Eq,             // =
    BangEq,         // !=
    EqEq,           // ==
    Gt,             // >
    GtEq,           // >=
    Lt,             // <
    LtEq,           // <=

    And,            // and
    Or,             // or

    If,             // if
    Else,           // else
    Fun,            // fun
    For,            // for
    While,          // while
    Nil,            // nil
    Return,         // Return

    Identifier,
    String,
    Number,

    True,           // true
    False,          // false

    Comment,        // Anything after a '//'
}

impl PartialEq<Self> for TokenType {
    fn eq(&self, other: &Self) -> bool {
        return std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
