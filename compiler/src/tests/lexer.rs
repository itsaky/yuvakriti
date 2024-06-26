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

use std::io::Cursor;
use std::io::Read;

use crate::diagnostics::CollectingDiagnosticHandler;
use crate::lexer::YKLexer;
use crate::messages;
use crate::tokens::TokenType;

fn check_token_types<R: Read>(lexer: &mut YKLexer<R>, expected_tokens: &Vec<TokenType>) {
    let tokens: Vec<TokenType> = lexer
        .all()
        .into_iter()
        .map(|token| token.token_type)
        .collect();

    assert_eq!(expected_tokens, &tokens);
}

fn match_token_types(src: &str, expected_tokens: &Vec<TokenType>) {
    let mut diag_handler = CollectingDiagnosticHandler::new();
    let mut lexer = YKLexer::new(Cursor::new(src), &mut diag_handler);
    check_token_types(&mut lexer, expected_tokens);
    assert_eq!(true, diag_handler.diagnostics.is_empty());
}

fn check_diagnostic_messages(
    diag_handler: &CollectingDiagnosticHandler,
    expected_messages: &Vec<&str>,
) {
    let messages: Vec<String> = diag_handler
        .diagnostics
        .iter()
        .map(|diag| diag.message.clone())
        .collect();

    // should contain 3 unknown tokens (of 3 bytes) because of the unicode characters
    assert_eq!(expected_messages, &messages);
}

#[test]
fn test_simple_operator_lexing() {
    match_token_types(
        "()[]{},.+-:;*!<> =",
        &vec![
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
            TokenType::Colon,
            TokenType::Semicolon,
            TokenType::Asterisk,
            TokenType::Bang,
            TokenType::Lt,
            TokenType::Gt,
            TokenType::Eq,
        ],
    );
}

#[test]
fn test_multiline_operator_lexing() {
    match_token_types(
        "()\n[]\n{}\n,\n.\n+\n-\n;\n*\n!\n<\n>\n=\n",
        &vec![
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
        ],
    );
}

#[test]
fn test_whitespaces_in_input_must_be_ignored() {
    match_token_types(
        "( )[ ]{ }\t,\r.\n+-;*",
        &vec![
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
        ],
    );
}

#[test]
fn test_multi_character_operator_lexing() {
    match_token_types(
        "!===<=>=",
        &vec![
            TokenType::BangEq,
            TokenType::EqEq,
            TokenType::LtEq,
            TokenType::GtEq,
        ],
    );
}

#[test]
fn test_comments_are_ignored_by_default() {
    match_token_types(
        "!=\n// something not equal to \n==",
        &vec![TokenType::BangEq, TokenType::EqEq],
    );
}

#[test]
fn test_comments_are_tokenized_if_enabled() {
    let mut diag_handler = CollectingDiagnosticHandler::new();
    let mut lexer = YKLexer::new(
        Cursor::new("!=\n// something not equal to \n=="),
        &mut diag_handler,
    );

    // enable comment tokenization
    lexer.ignore_comments = false;

    check_token_types(
        &mut lexer,
        &vec![TokenType::BangEq, TokenType::Comment, TokenType::EqEq],
    );

    assert_eq!(true, diag_handler.diagnostics.is_empty());
}

#[test]
fn test_lexer_reports_unrecognized_tokens() {
    let mut diag_handler = CollectingDiagnosticHandler::new();
    let mut lexer = YKLexer::new(Cursor::new("( )[ ]{ }\t,\r.\n�+-;*"), &mut diag_handler);

    check_token_types(
        &mut lexer,
        &vec![
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
        ],
    );

    check_diagnostic_messages(
        &diag_handler,
        &vec![
            messages::LEX_UNKNOWN_TOKEN,
            messages::LEX_UNKNOWN_TOKEN,
            messages::LEX_UNKNOWN_TOKEN,
        ],
    );
}

#[test]
fn test_simple_identifier_lexing() {
    match_token_types("identifier", &vec![TokenType::Identifier]);
}

#[test]
fn test_simple_keyword_lexing() {
    match_token_types(
        "and or if else while null return true fun for false var this super print",
        &vec![
            TokenType::And,
            TokenType::Or,
            TokenType::If,
            TokenType::Else,
            TokenType::While,
            TokenType::Null,
            TokenType::Return,
            TokenType::True,
            TokenType::Fun,
            TokenType::For,
            TokenType::False,
            TokenType::Var,
            TokenType::This,
            TokenType::Super,
            TokenType::Print,
        ],
    );
}

#[test]
fn test_mixed_identifier_and_keyword_lexing() {
    match_token_types(
        "and or andor if else ifelse while null return true fun identifier for false falseee",
        &vec![
            TokenType::And,
            TokenType::Or,
            TokenType::Identifier,
            TokenType::If,
            TokenType::Else,
            TokenType::Identifier,
            TokenType::While,
            TokenType::Null,
            TokenType::Return,
            TokenType::True,
            TokenType::Fun,
            TokenType::Identifier,
            TokenType::For,
            TokenType::False,
            TokenType::Identifier,
        ],
    );
}

#[test]
fn test_numbers_in_identifiers() {
    match_token_types("and123", &vec![TokenType::Identifier]);
}

#[test]
fn test_identifiers_starting_with_number() {
    match_token_types("123and", &vec![TokenType::Number, TokenType::And]);
}

#[test]
fn test_underscores_in_identifiers() {
    match_token_types(
        "and_ a_nd _and",
        &vec![
            TokenType::Identifier,
            TokenType::Identifier,
            TokenType::Identifier,
        ],
    );
}

#[test]
fn test_integer_number() {
    match_token_types("123", &vec![TokenType::Number]);
}

#[test]
fn test_decimal_number() {
    match_token_types("123.123", &vec![TokenType::Number]);
}

#[test]
fn test_invalid_number() {
    match_token_types(
        "123.123.123",
        &vec![TokenType::Number, TokenType::Dot, TokenType::Number],
    );
}

#[test]
fn test_simple_string_literal() {
    let mut diag_handler = CollectingDiagnosticHandler::new();
    let mut lexer = YKLexer::new(Cursor::new("\"something\""), &mut diag_handler);

    let tokens = lexer.all();

    assert_eq!(1, tokens.len());

    let token = tokens.get(0).unwrap();
    assert_eq!(TokenType::String, token.token_type);
    assert_eq!(0, token.range.start.line);
    assert_eq!(0, token.range.start.column);
    assert_eq!(0, token.range.end.line);
    assert_eq!(11, token.range.end.column);
    assert_eq!(0, token.range.start.index);
    assert_eq!(11, token.range.end.index);

    assert_eq!(true, diag_handler.diagnostics.is_empty());
}

#[test]
fn test_simple_consecutive_string_literals() {
    let mut diag_handler = CollectingDiagnosticHandler::new();
    let mut lexer = YKLexer::new(
        Cursor::new("\"something\" \"something else\""),
        &mut diag_handler,
    );

    let tokens = lexer.all();

    assert_eq!(2, tokens.len());

    let first = tokens.get(0).unwrap();
    assert_eq!(TokenType::String, first.token_type);
    assert_eq!(0, first.range.start.line);
    assert_eq!(0, first.range.start.column);
    assert_eq!(0, first.range.start.index);

    assert_eq!(0, first.range.end.line);
    assert_eq!(11, first.range.end.column); // end position is exclusive (column and index)
    assert_eq!(11, first.range.end.index);

    let second = tokens.get(1).unwrap();
    assert_eq!(TokenType::String, second.token_type);
    assert_eq!(0, second.range.start.line);
    assert_eq!(12, second.range.start.column);
    assert_eq!(12, second.range.start.index);

    assert_eq!(0, second.range.end.line);
    assert_eq!(28, second.range.end.column); // end position is exclusive (column and index)
    assert_eq!(28, second.range.end.index);

    assert_eq!(true, diag_handler.diagnostics.is_empty());
}

#[test]
fn test_multiline_string_literals_should_fail() {
    let mut diag_handler = CollectingDiagnosticHandler::new();
    let mut lexer = YKLexer::new(Cursor::new("\"some\nthing\""), &mut diag_handler);

    let tokens = lexer.all();

    // - lexer encounters '"' and enters the string
    // - 'some' is scanned as a valid part of the string
    // - lexer encounters '\n', so it discards the string and reports the multiline string error
    // - scanning continues and 'thing' is recognized as an identifier
    // - the closing '"' is encountered, but the lexer has reached EOF
    //   so the 'unexpected EOF' error is reported
    // - at last, the vector contains a single token with type 'Identifier'
    assert_eq!(false, tokens.is_empty());
    assert_eq!(TokenType::Identifier, tokens.get(0).unwrap().token_type);

    check_diagnostic_messages(
        &diag_handler,
        &vec![
            messages::LEX_STRING_MULTILINE_ERROR,
            messages::LEX_UNEXPECTED_EOF,
        ],
    );
}

#[test]
fn test_escape_sequences_in_string_literal() {
    let mut diag_handler = CollectingDiagnosticHandler::new();
    let mut lexer = YKLexer::new(
        Cursor::new("\"\\u2022 \\b \\s \\t \\n \\f \\r \\\" \\' \\\\ \""),
        &mut diag_handler,
    );

    let tokens = lexer.all();

    assert_eq!(1, tokens.len());

    let token = tokens.get(0).unwrap();
    assert_eq!(TokenType::String, token.token_type);
    assert_eq!(0, token.range.start.line);
    assert_eq!(0, token.range.start.column);
    assert_eq!(0, token.range.start.index);

    assert_eq!(0, token.range.end.line);
    assert_eq!(36, token.range.end.column);
    assert_eq!(36, token.range.end.index);

    assert_eq!(true, diag_handler.diagnostics.is_empty());
}

#[test]
fn test_unicode_escapes() {
    let mut diag_handler = CollectingDiagnosticHandler::new();
    let mut lexer = YKLexer::new(Cursor::new("\"\\u2022\""), &mut diag_handler);

    let tokens = lexer.all();

    assert_eq!(1, tokens.len());

    let token = tokens.get(0).unwrap();
    assert_eq!(TokenType::String, token.token_type);
    assert_eq!(0, token.range.start.line);
    assert_eq!(0, token.range.start.column);
    assert_eq!(0, token.range.start.index);

    assert_eq!(0, token.range.end.line);
    assert_eq!(8, token.range.end.column);
    assert_eq!(8, token.range.end.index);

    assert_eq!(true, diag_handler.diagnostics.is_empty());
}

#[test]
fn test_compound_assignment_operators() {
    match_token_types(
        "+= -= *= /=",
        &vec![
            TokenType::PlusEq,
            TokenType::MinusEq,
            TokenType::AsteriskEq,
            TokenType::SlashEq,
        ],
    );
}
