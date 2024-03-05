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

use std::cell::RefCell;
use std::fmt::Pointer;
use std::io::Cursor;
use std::rc::Rc;

use crate::yklang::compiler::diagnostics;
use crate::yklang::compiler::lexer::YKLexer;
use crate::yklang::compiler::messages::CompilerMessages;
use crate::yklang::compiler::tokens::TokenType;

#[test]
fn test_simple_operator_lexing() {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let mut lexer = YKLexer::new(
        Cursor::new("()[]{},.+-;*!<>="),
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
        TokenType::Bang,
        TokenType::Lt,
        TokenType::Gt,
        TokenType::Eq,
    ];

    let tokens: Vec<TokenType> = lexer.all()
        .into_iter()
        .map(|token| token.token_type)
        .collect();


    assert_eq!(expected_tokens, tokens);
}

#[test]
fn test_multiline_operator_lexing() {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let mut lexer = YKLexer::new(
        Cursor::new("()\n[]\n{}\n,\n.\n+\n-\n;\n*\n!\n<\n>\n=\n"),
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
        TokenType::Bang,
        TokenType::Lt,
        TokenType::Gt,
        TokenType::Eq,
    ];

    let tokens: Vec<TokenType> = lexer.all()
        .into_iter()
        .map(|token| token.token_type)
        .collect();


    assert_eq!(expected_tokens, tokens);
}

#[test]
fn test_whitespaces_in_input_must_be_ignored() {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let mut lexer = YKLexer::new(
        Cursor::new("( )[ ]{ }\t,\r.\n+-;*"),
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

#[test]
fn test_multi_character_operator_lexing() {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let mut lexer = YKLexer::new(
        Cursor::new("!===<=>="),
        diag_handler.clone()
    );

    let expected_tokens = vec![
        TokenType::BangEq,
        TokenType::EqEq,
        TokenType::LtEq,
        TokenType::GtEq,
    ];

    let tokens: Vec<TokenType> = lexer.all()
        .into_iter()
        .map(|token| token.token_type)
        .collect();


    assert_eq!(expected_tokens, tokens);
}

#[test]
fn test_comments_are_ignored_by_default() {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let mut lexer = YKLexer::new(
        Cursor::new("!=\n// something not equal to \n=="),
        diag_handler.clone()
    );

    let expected_tokens = vec![
        TokenType::BangEq,
        TokenType::EqEq,
    ];

    let tokens: Vec<TokenType> = lexer.all()
        .into_iter()
        .map(|token| token.token_type)
        .collect();


    assert_eq!(expected_tokens, tokens);
}

#[test]
fn test_comments_are_tokenized_if_enabled() {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let mut lexer = YKLexer::new(
        Cursor::new("!=\n// something not equal to \n=="),
        diag_handler.clone()
    );

    // enable comment tokenization
    lexer.ignore_comments = false;

    let expected_tokens = vec![
        TokenType::BangEq,
        TokenType::Comment,
        TokenType::EqEq,
    ];

    let tokens: Vec<TokenType> = lexer.all()
        .into_iter()
        .map(|token| token.token_type)
        .collect();


    assert_eq!(expected_tokens, tokens);
}

#[test]
fn test_lexer_reports_unrecognized_tokens() {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let mut lexer = YKLexer::new(
        Cursor::new("( )[ ]{ }\t,\r.\n�+-;*"),
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

    let messages: Vec<String> = diag_handler
        .borrow()
        .diagnostics
        .iter()
        .map(|diag| diag.message.clone())
        .collect();

    // should contain 3 unknown tokens (of 3 bytes) because of the unicode characters
    assert_eq!(messages, vec![
        CompilerMessages::LEX_UNKNOWN_TOKEN,
        CompilerMessages::LEX_UNKNOWN_TOKEN,
        CompilerMessages::LEX_UNKNOWN_TOKEN
    ])
}