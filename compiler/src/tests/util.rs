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
use std::io::Cursor;
use std::rc::Rc;

use crate::ast::ASTPrinter;
use crate::ast::AstNode;
use crate::ast::Program;
use crate::diagnostics;
use crate::lexer::YKLexer;
use crate::parser::YKParser;

pub(crate) fn parse(source: &str) -> Program {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let lexer = YKLexer::new(Cursor::new(source), diag_handler.clone());

    let mut parser = YKParser::new(lexer, diag_handler.clone());
    parser.parse()
}

pub(crate) fn node_string(program: &mut Program, pretty: bool) -> String {
    let mut out = String::new();
    let mut printer = ASTPrinter::new(&mut out, pretty);
    program.accept(&mut printer, &0);
    out
}

pub(crate) fn parse_to_string(source: &str, pretty: bool) -> String {
    let mut program = parse(source);
    node_string(&mut program, pretty)
}
