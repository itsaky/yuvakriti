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

use crate::yklang::compiler::ast::AstNode;
use crate::yklang::compiler::ast::pretty::ASTPrinter;
use crate::yklang::compiler::diagnostics;
use crate::yklang::compiler::lexer::YKLexer;
use crate::yklang::compiler::parser::YKParser;

#[test]
fn test_simple_var_decl() {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let lexer = YKLexer::new(
        Cursor::new("var something = 1234;"),
        diag_handler.clone()
    );
    
    let mut parser = YKParser::new(lexer, diag_handler.clone());
    let program = parser.parse();

    assert_eq!(true, diag_handler.borrow().diagnostics.is_empty());
}

#[test]
fn test_simple_ast_printer() {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let lexer = YKLexer::new(
        Cursor::new("for (var i = 0; i < 10; i = i + 1) {\n    print i;\n}"),
        diag_handler.clone()
    );

    let mut parser = YKParser::new(lexer, diag_handler.clone());
    let mut program = parser.parse();

    let mut out = String::new();
    let mut pretty_printer = ASTPrinter::new(&mut out);
    program.accept(&mut pretty_printer, &0);
    println!("{}", out);
}

#[test]
fn test_simple_fun_decl() {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let lexer = YKLexer::new(
        Cursor::new("fun main() {    print 1234; }"),
        diag_handler.clone()
    );

    let mut parser = YKParser::new(lexer, diag_handler.clone());
    let mut program = parser.parse();

    let mut out = String::new();
    let mut pretty_printer = ASTPrinter::new(&mut out);
    program.accept(&mut pretty_printer, &0);
    
    assert_eq!("(program
  (decl fun main() {
      (stmt print  (primary Number(1234.0))
      )
    }
  )
)", out);
}