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

use std::fmt::Pointer;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

use crate::yklang::compiler::diagnostics;
use crate::yklang::compiler::lexer::YKLexer;
use crate::yklang::compiler::tokens::TokenType;

#[test]
fn test_simple_operator_lexing() {
    let diag_handler = Arc::new(Mutex::new(diagnostics::collecting_handler()));
    let mut lexer = YKLexer::new(
        Cursor::new("()[]{},.+-;*"),
        diag_handler.clone()
    );

    let expected_tokens = vec![
        TokenType::LParen,
        TokenType::RParen,
        TokenType::LBrack,
        TokenType::RBrack,
        TokenType::LBrace,
        TokenType::RBrace,
        TokenType::Comma,
        TokenType::Dot,
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Semicolon,
        TokenType::Asterisk,
    ];

    let tokens: Vec<TokenType> = lexer.all()
        .into_iter()
        .map(|token| token.token_type)
        .collect();


    assert_eq!(expected_tokens, tokens);
}