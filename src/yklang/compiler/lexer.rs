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
use std::io;
use std::io::{BufReader, Bytes, Read};
use std::iter::Peekable;
use std::rc::Rc;

use log::error;

use crate::yklang::compiler::diagnostics::{Diagnostic, DiagnosticHandler, DiagnosticKind};
use crate::yklang::compiler::location::{Position, Range};
use crate::yklang::compiler::tokens::{Token, TokenType};

pub struct YKLexer<'a, R: Read> {
    diagnostics: Rc<RefCell<dyn DiagnosticHandler + 'a>>,
    input: Peekable<Bytes<BufReader<R>>>,
    current: Option<char>,
    token_start: Position,
    position: Position,
    pub ignore_comments: bool
}

impl <R: Read> YKLexer<'_, R> {
    fn report(&mut self, diagnostic_kind: DiagnosticKind, message: String) {
        self.diagnostics.borrow_mut().handle(self.create_diagnostic(diagnostic_kind, message));
    }

    fn create_diagnostic(
        &self,
        diagnostic_kind: DiagnosticKind,
        message: String,
    ) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: self.token_start,
                end: self.position
            },

            message,

            kind: diagnostic_kind
        }
    }
}

impl<R: Read> YKLexer<'_, R> {

    /// Creates a [YKLexer] which tokenizes the given source.
    pub fn new<'a>(
        source: R,
        diagnostics_handler: Rc<RefCell<dyn DiagnosticHandler + 'a>>,
    ) -> YKLexer<'a, R> {
        let iterator = BufReader::new(source).bytes().peekable();
        let mut lexer = YKLexer {
            diagnostics: diagnostics_handler,
            input: iterator,
            current: None,
            token_start: Position::NO_POS,
            position: Position::NO_POS,
            ignore_comments: true,
        };

        // advance to the first character in the input source
        lexer.advance();

        lexer
    }
}

impl <R: Read> YKLexer<'_, R> {

    pub fn all(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.is_eof() {
            if let Some(token) = self.next() {

                if self.ignore_comments
                    && token.token_type == TokenType::Comment {
                    // ignore comments
                    continue
                }

                tokens.push(token)
            }
        }
        return tokens
    }

    pub fn next(&mut self) -> Option<Token> {
        self.token_start = self.position.clone();
        let result = match self.advance() {
            None => None,
            Some(next) => match next {
                '(' => Some(self.token(TokenType::LParen)),
                ')' => Some(self.token(TokenType::RParen)),
                '[' => Some(self.token(TokenType::LBrack)),
                ']' => Some(self.token(TokenType::RBrack)),
                '{' => Some(self.token(TokenType::LBrace)),
                '}' => Some(self.token(TokenType::RBrace)),
                ',' => Some(self.token(TokenType::Comma)),
                '.' => Some(self.token(TokenType::Dot)),
                '+' => Some(self.token(TokenType::Plus)),
                '-' => Some(self.token(TokenType::Minus)),
                ';' => Some(self.token(TokenType::Semicolon)),
                '*' => Some(self.token(TokenType::Asterisk)),

                '!' => match self.cmatch('=') {
                    true => Some(self.token(TokenType::BangEq)),
                    false => Some(self.token(TokenType::Bang))
                },

                '=' => match self.cmatch('=') {
                    true => Some(self.token(TokenType::EqEq)),
                    false => Some(self.token(TokenType::Eq))
                },

                '>' => match self.cmatch('=') {
                    true => Some(self.token(TokenType::GtEq)),
                    false => Some(self.token(TokenType::Gt))
                },

                '<' => match self.cmatch('=') {
                    true => Some(self.token(TokenType::LtEq)),
                    false => Some(self.token(TokenType::Lt))
                },

                '/' => {
                    if let Some(next) = self.peek() {
                        if next == '/' {
                            while self.peek().unwrap_or('\0') != '\n' && !self.is_eof() {
                                // we ignore comments
                                self.advance();
                            }

                            return Some(self.token(TokenType::Comment))
                        }
                    }

                    return Some(self.token(TokenType::Slash));
                }

                _ => {
                    if self.is_whitespace(next) {
                        // ignore whitespaces
                        return None
                    }

                    self.report(DiagnosticKind::Error, String::from("Unknown token"));
                    return None
                }
            }
        };

        return result;
    }

    /// Returns the character at the current lexer position and advances to the next character
    fn advance(&mut self) -> Option<char> {
        let next_char = self.input.next();
        let result = self.peek();

        self.current = match next_char {
            None => None,
            Some(result) => u8_to_char(&result)
        };

        if self.position == Position::NO_POS {
            // we advanced to the first character
            // reset the position to the start of input
            self.position = Position {
                line: 0,
                column: 0,
                index: 0
            }
        } else {
            self.position.column += 1;
            self.position.index += 1;

            if result.unwrap_or('\0') == '\n' {
                // in case we encountered a line feed
                // increment the line number and set column to 0 (start of line)
                // index is unchanged, obviously
                self.position.line += 1;
                self.position.column = 0;
            }
        }

        return result;
    }

    fn peek(&self) -> Option<char> {
        return self.current;
    }

    /// Single-character lookahead
    fn peek_next(&mut self) -> Option<char> {
        let result = self.input.peek();
        match result {
            None => None,
            Some(result) => u8_to_char(result)
        }
    }

    /// Returns `true` if the current character is the expected character, `false` otherwise.
    fn cmatch(&mut self, expected: char) -> bool {
        if self.is_eof() {
            return false;
        }

        if self.peek().unwrap_or('\0') != expected {
            return false;
        }

        self.advance();

        return true;
    }

    /// Create a token without text
    fn token(
        &self,
        token_type: TokenType
    ) -> Token {
        return self.text_token(token_type, None)
    }

    /// Create a token with the given token text (content)
    fn text_token(
        &self,
        token_type: TokenType,
        content: Option<String>
    ) -> Token {
        return Token {
            token_type,
            content,
            range: Range {
                start: self.token_start,
                end: self.position.clone()
            }
        };
    }

    /// Returns whether the current character represents an end-of-file (EOF)
    fn is_eof(&self) -> bool {
        return self.peek().unwrap_or('\0') == '\0'
    }

    /// Returns whether the given character is a whitespace
    fn is_whitespace(&self, c: char) -> bool {
        return c == ' '
        || c == '\t'
        || c == '\r'
        || c == '\n'
    }
}

/// Converts the result from the read operation to a character
fn u8_to_char(result: &io::Result<u8>) -> Option<char> {
    match result {
        Err(err) => {
            error!("Failed to convert u8 to char: {:?}", err);
            None
        },
        Ok(character) => Some(char::from(*character))
    }
}