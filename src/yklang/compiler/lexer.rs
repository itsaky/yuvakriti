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

use std::fs::File;
use std::io;
use std::io::{BufReader, Bytes, Read};
use std::iter::Peekable;

use crate::yklang::compiler::location::{Position, Range};
use crate::yklang::compiler::tokens::{Token, TokenType};

struct YKLexer {
    reader: Peekable<Bytes<BufReader<File>>>,
    current: Option<char>,
    position: Position
}

impl YKLexer {

    /// Creates a [YKLexer] which tokens the given file's contents.
    pub fn from_file(file: File) -> YKLexer {
        let iterator = BufReader::new(file).bytes().peekable();
        let mut lexer = YKLexer {
            reader: iterator,
            current: None,
            position: Position::NO_POS
        };

        // advance to the first character in the input source
        lexer.advance();

        return lexer;
    }

    pub fn all(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            match self.next() {
                Some(token) => tokens.push(token),
                None => break
            }
        }
        return tokens
    }

    pub fn next(&mut self) -> Option<Token> {
        let start = self.position.clone();
        let result = match self.advance() {
            None => None,
            Some(char) => match char {
                '(' => Some(self.token(TokenType::LParen, start)),
                ')' => Some(self.token(TokenType::RParen, start)),
                '[' => Some(self.token(TokenType::LBrack, start)),
                ']' => Some(self.token(TokenType::RBrack, start)),
                '{' => Some(self.token(TokenType::LBrace, start)),
                '}' => Some(self.token(TokenType::RBrace, start)),
                ',' => Some(self.token(TokenType::Comma, start)),
                '.' => Some(self.token(TokenType::Dot, start)),
                '+' => Some(self.token(TokenType::Plus, start)),
                '-' => Some(self.token(TokenType::Minus, start)),
                ';' => Some(self.token(TokenType::Semicolon, start)),
                '*' => Some(self.token(TokenType::Asterisk, start)),

                _ => {
                    if self.is_whitespace(char) {
                        // ignore whitespaces
                        return None
                    }

                    todo!("Handle unknown tokens")
                }
            }
        };

        return result;
    }

    /// Returns the character at the current lexer position and advances to the next character
    fn advance(&mut self) -> Option<char> {
        let next_char = self.reader.next();
        let result = self.current;

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
                // in case we just encountered a line feed
                // increment the line number and set column to 0 (start of line)
                // index is unchanged, obviously
                self.position.line += 1;
                self.position.column = 0;
            }
        }

        return result;
    }

    /// Single-character lookahead
    fn peek_next(&mut self) -> Option<char> {
        let result = self.reader.peek();
        match result {
            None => None,
            Some(result) => u8_to_char(result)
        }
    }

    /// Checks if the current character is the expected value. Returns `true` if it is.
    fn cmatch(&self, expected: char) -> bool {
        if self.is_eof() {
            return false;
        }

        return self.current.unwrap_or('\0') == expected;
    }

    fn token(
        &self,
        token_type: TokenType,
        token_start: Position
    ) -> Token {
        return self.text_token(token_type, None, token_start)
    }

    /// Create a token
    fn text_token(
        &self,
        token_type: TokenType,
        content: Option<String>,
        token_start: Position
    ) -> Token {
        return Token {
            token_type,
            content,
            range: Range {
                start: token_start,
                end: self.position.clone()
            }
        };
    }

    /// Returns whether the current character represents and end-of-file (EOF)
    fn is_eof(&self) -> bool {
        return self.current.unwrap_or('\0') == '\0'
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
        Err(err) => panic!("Error while reading from source: {}", err),
        Ok(character) => Some(char::from(*character))
    }
}