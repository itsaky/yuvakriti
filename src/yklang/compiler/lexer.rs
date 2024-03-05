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
use crate::yklang::compiler::messages;
use crate::yklang::compiler::tokens::{Token, TokenType};

pub struct YKLexer<'a, R: Read> {
    diagnostics: Rc<RefCell<dyn DiagnosticHandler + 'a>>,
    input: Peekable<Bytes<BufReader<R>>>,
    current_char: Option<char>,
    current_word: Vec<char>,
    token_start: Position,
    position: Position,
    pub ignore_comments: bool
}

impl <R: Read> YKLexer<'_, R> {
    fn report(&mut self, diagnostic_kind: DiagnosticKind, message: &str) {
        self.diagnostics.borrow_mut().handle(self.create_diagnostic(diagnostic_kind, message));
    }

    fn create_diagnostic(
        &self,
        diagnostic_kind: DiagnosticKind,
        message: &str,
    ) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: self.token_start,
                end: self.position
            },

            message: String::from(message),

            kind: diagnostic_kind
        }
    }
}

impl<R: Read> YKLexer<'_, R> {

    /// Initial capacity of the vector in the lexer which is used to store the
    /// characters of the current word
    const WORD_VECTOR_INITIAL_CAPACITY: usize = 64;

    /// Creates a [YKLexer] which tokenizes the given source.
    pub fn new<'a>(
        source: R,
        diagnostics_handler: Rc<RefCell<dyn DiagnosticHandler + 'a>>,
    ) -> YKLexer<'a, R> {
        let iterator = BufReader::new(source).bytes().peekable();
        let mut lexer = YKLexer {
            diagnostics: diagnostics_handler,
            input: iterator,
            current_char: None,
            current_word: Vec::with_capacity(Self::WORD_VECTOR_INITIAL_CAPACITY),
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

    /// Tokenizes the input source and returns all the recognized tokens.
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

    /// Advance to the next token in the input source. This returns [Some] if a valid token
    /// is recognized, otherwise return [None].
    pub fn next(&mut self) -> Option<Token> {

        // Skip all whitespaces
        self.skip_whitespaces();

        // Reset the word vector
        self.reset_word();

        self.token_start = self.position.clone();
        let result = match self.advance() {
            None => None,
            Some(char) => {

                if self.is_identifier_start(char) {
                    return self.identifier();
                }

                if self.is_digit(char) {
                    return self.number();
                }

                return match char {
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
                            // comments start with a '//' token and span the entire line
                            // we seek to the end of line and return a comment token
                            if next == '/' {
                                while self.peek().unwrap_or('\0') != '\n' && !self.is_eof() {
                                    // we ignore comments
                                    self.advance();

                                    // TODO : Decide if the word vector should discard all the
                                    //  characters in a comment. This may help save some memory
                                    //  if the comment is longer than WORD_VECTOR_INITIAL_CAPACITY
                                    //
                                    //  Currently, we use the following condition to decide
                                    if self.ignore_comments {
                                       self.reset_word();
                                    }
                                }

                                return Some(self.token(TokenType::Comment))
                            }
                        }

                        return Some(self.token(TokenType::Slash));
                    }

                    _ => {
                        self.report(DiagnosticKind::Error, messages::LEX_UNKNOWN_TOKEN);
                        return None
                    }
                }
            }
        };

        return result;
    }

    /// Scans an identifier
    fn identifier(&mut self) -> Option<Token> {
        while self.is_identifier_part(self.peek().unwrap_or('\0')) && !self.is_eof() {
            self.advance();
        }

        return Some(self.token(self.identifier_type()))
    }

    /// Returns the type of identifier at the current lexer position
    fn identifier_type(&self) -> TokenType {
        let match_result = match self.current_word.get(0) {
            Some(c1) => match c1 {
                'a' => self.match_word_rest(1, "nd", TokenType::And),
                'o' => self.match_word_rest(1, "r", TokenType::Or),
                'i' => self.match_word_rest(1, "f", TokenType::If),
                'e' => self.match_word_rest(1, "lse", TokenType::Else),
                'w' => self.match_word_rest(1, "hile", TokenType::While),
                'n' => self.match_word_rest(1, "il", TokenType::Nil),
                'r' => self.match_word_rest(1, "eturn", TokenType::Return),
                't' => self.match_word_rest(1, "rue", TokenType::True),
                'f' => {
                    match self.current_word.get(1) {
                        None => None,
                        Some(c2) => match c2 {
                            'u' => self.match_word_rest(2, "n", TokenType::Fun),
                            'o' => self.match_word_rest(2, "r", TokenType::For),
                            'a' => self.match_word_rest(2, "lse", TokenType::False),
                            _ => None
                        }
                    }
                }
                _ => None,
            }
            None => None
        };

        return match_result.unwrap_or(TokenType::Identifier);
    }

    /// This method checks if the characters in `self.current_word`, starting at index `start`, are
    /// equal to the characters in `rest`. If the characters are same, returns `Some(result_type)`,
    /// returns `None` otherwise. This method will also return `None` if the `start` index is invalid
    /// or if `self.current_word.len()` is less than `rest.len()` (because the rest characters would
    /// never match in such cases).
    ///
    /// The behavior is similar to how a 'trie' works.
    fn match_word_rest(&self, start: usize, rest: &str, result_type: TokenType) -> Option<TokenType> {
        if start < 0 || start >= self.current_word.len() {
            return None
        }

        if self.current_word.len() > rest.len() + start {
            return None
        }

        let bytes = rest.as_bytes();
        for i in start..self.current_word.len() {
            if let Some(char) = self.current_word.get(i) {
                if char != &char::from(bytes[i - start]) {
                    return None
                }
            }
        }

        return Some(result_type)
    }

    fn number(&mut self) -> Option<Token>  {
        None
    }

    /// Returns the character at the current lexer position and advances to the next character
    fn advance(&mut self) -> Option<char> {
        let next_char = self.input.next();
        let result = self.peek();

        self.current_char = match next_char {
            None => None,
            Some(result) => u8_to_char(&result)
        };

        if let Some(char) = result {
            if self.is_identifier_part(char) {
                self.current_word.push(char);
            }
        }

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
        return self.current_char;
    }

    /// Single-character lookahead
    fn peek_next(&mut self) -> Option<char> {
        let result = self.input.peek();
        match result {
            None => None,
            Some(result) => u8_to_char(result)
        }
    }

    /// Resets the current word vector.
    fn reset_word(&mut self) {
        self.current_word.clear();
    }


    /// Skips through the input source until a non-whitespace character or EOF is encountered.
    fn skip_whitespaces(&mut self) {
        loop {
            let char = self.peek().unwrap_or('\0');
            if char == '\0' || !self.is_whitespace(char) || self.is_eof() {
                return;
            }

            self.advance();
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


    /// Checks whether the given character represents a valid start character of an identifier
    fn is_identifier_start(&self, char: char) -> bool {
        return self.is_alpha(char);
    }

    /// Checks whether the given character is a valid identifier 'part'. The 'part' of an
    /// identifier is everything after the first character in the identifier.
    fn is_identifier_part(&self, char: char) -> bool {
        return self.is_alpha(char) || self.is_digit(char);
    }

    /// Checks whether the given character is a valid alphabet in YuvaKriti lang
    fn is_alpha(&self, char: char) -> bool {
        return (char >= 'a' && char <= 'z') ||
            (char >= 'A' && char <= 'Z') ||
            char == '_';
    }

    /// Checks whether the given character is a valid digit in YuvaKriti lang
    fn is_digit(&self, char: char) -> bool {
        return char >= '0' && char <= '9';
    }

    /// Returns whether the given character is a whitespace
    fn is_whitespace(&self, c: char) -> bool {
        return c == ' '
        || c == '\t'
        || c == '\r'
        || c == '\n'
    }

    /// Returns whether the current character represents an end-of-file (EOF)
    fn is_eof(&self) -> bool {
        return self.peek().unwrap_or('\0') == '\0'
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