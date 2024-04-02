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

use crate::compiler::ast::{AstNode, Decl, Expr, PrimaryExpr, Program, Stmt, UnaryOp};
use crate::compiler::ast::pretty::ASTPrinter;
use crate::compiler::diagnostics;
use crate::compiler::lexer::YKLexer;
use crate::compiler::parser::YKParser;

fn parse(source: &str) -> Program {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let lexer = YKLexer::new(
        Cursor::new(source),
        diag_handler.clone()
    );

    let mut parser = YKParser::new(lexer, diag_handler.clone());
    parser.parse()
}

fn node_string(program: &mut Program, pretty: bool) -> String {
    let mut out = String::new();
    let mut printer = ASTPrinter::new(&mut out, pretty);
    program.accept(&mut printer, &0);
    out
}

fn parse_to_string(source: &str, pretty: bool) -> String {
    let mut program = parse(source);
    node_string(&mut program, pretty)
}

#[test]
fn test_simple_var_decl() {
    let source = "var something = 1234;";
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let lexer = YKLexer::new(
        Cursor::new(source),
        diag_handler.clone()
    );

    let mut parser = YKParser::new(lexer, diag_handler.clone());
    let program = parser.parse();
    assert_eq!(true, diag_handler.borrow().diagnostics.is_empty());

    let decls = program.decls;
    assert_eq!(1, decls.len());

    let decl = decls.get(0).expect("Declaration expected");
    assert_eq!(0, decl.1.start.line);
    assert_eq!(0, decl.1.start.column);
    assert_eq!(0, decl.1.start.index);
    assert_eq!(0, decl.1.end.line);
    assert_eq!(20, decl.1.end.column as usize);
    assert_eq!(20, decl.1.end.index as usize);

    let stmt = if let Decl::Stmt(stmt) = &decl.0 { stmt } else { panic!("Expected a statement") };
    let var = if let Stmt::Var(var) = stmt { var } else { panic!("Expected a variable statement") };
    let primary = if let Expr::Primary(primary) = &var.initializer.as_ref().unwrap().0 { primary } else { panic!("Expected a primary expression") };
    let num = if let PrimaryExpr::Number(num) = &primary.0 { num } else { panic!("Expected a number primary expression") };

    assert_eq!("something", var.name.0);
    assert_eq!(0,  var.name.1.start.line);
    assert_eq!(4, var.name.1.start.column);
    assert_eq!(4, var.name.1.start.index);
    assert_eq!(0,  var.name.1.end.line);
    assert_eq!(13, var.name.1.end.column as usize);
    assert_eq!(13, var.name.1.end.index as usize);

    let init = var.initializer.as_ref().expect("Initializer expected");
    assert_eq!(0, init.1.start.line);
    assert_eq!(16, init.1.start.column);
    assert_eq!(16, init.1.start.index);
    assert_eq!(0, init.1.end.line);
    assert_eq!(20, init.1.end.column as usize);
    assert_eq!(20, init.1.end.index as usize);

    assert_eq!(1234f64, *num);
}

#[test]
fn test_simple_ast_printer() {
    let out = parse_to_string("for (var i = 0; i < 10; i = i + 1) {\n    print i;\n}", true);

    assert_eq!("(program
  (decl (stmt for ((stmt var i = (primary Number(0.0))); Lt (primary Identifier(\"i\"))(primary Number(10.0)); Eq (primary Identifier(\"i\"))(binary Plus (primary Identifier(\"i\"))(primary Number(1.0)))) {
      (stmt print  (primary Identifier(\"i\")))}))
  )", out);
}

#[test]
fn test_simple_unary_negation_expr() {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let lexer = YKLexer::new(
        Cursor::new("  !true  ;"),
        diag_handler.clone()
    );
    
    let mut parser = YKParser::new(lexer, diag_handler.clone());
    let program = parser.parse();
    let decls = program.decls;
    assert_eq!(1, decls.len());

    let decl = decls.get(0).expect("Declaration expected");
    let stmt = if let Decl::Stmt(stmt) = &decl.0 { stmt } else { panic!("Expected a statement") };
    let expr = if let Stmt::Expr(expr) = stmt { expr } else { panic!("Expected an expression statement") };
    let unary = if let Expr::Unary(unary) = &expr.expr.0 { unary } else { panic!("Expected an unary expression") };

    assert!(matches!(unary.op, UnaryOp::Not));
    assert!(matches!(unary.expr.0, Expr::Primary(_)));

    let tr = if let Expr::Primary(prim) = &unary.expr.0 { &prim.0 } else { panic!("Expected a primary expression") };
    assert!(matches!(tr, PrimaryExpr::True));
}

#[test]
fn test_simple_unary_num_negation_expr() {
    assert_eq!("(program(decl (stmt expr (unary Negate (primary Number(123.0))))))", parse_to_string("-123;", false));
}

#[test]
fn test_primary_exprs() {

    let out = parse_to_string("true; false; nil; this; 123; \"something\"; identifier; (\"grouping\");", true);
    
    // String expressions also include the quotes
    // the actual representaion of the above string is "\"something\""
    // so we need to escape them here
    assert_eq!(
"(program
  (decl (stmt expr (primary True)))
  (decl (stmt expr (primary False)))
  (decl (stmt expr (primary Nil)))
  (decl (stmt expr (primary This)))
  (decl (stmt expr (primary Number(123.0))))
  (decl (stmt expr (primary String(\"\\\"something\\\"\"))))
  (decl (stmt expr (primary Identifier(\"identifier\"))))
  (decl (stmt expr (primary String(\"\\\"grouping\\\"\"))))
  )", out);
}

#[test]
fn test_terms() {
    let out = parse_to_string("2 + 3; 2 - 3;", true);
    assert_eq!(
"(program
  (decl (stmt expr (binary Plus (primary Number(2.0))(primary Number(3.0)))))
  (decl (stmt expr (binary Minus (primary Number(2.0))(primary Number(3.0)))))
  )", out);
}

#[test]
fn test_terms_assoc() {
    let out = parse_to_string("2 + 3 + 4; 2 - 3 + 4; 2 + 3 - 4; 2 - 3 - 4;", true);
    assert_eq!(
"(program
  (decl (stmt expr (binary Plus (binary Plus (primary Number(2.0))(primary Number(3.0)))(primary Number(4.0)))))
  (decl (stmt expr (binary Plus (binary Minus (primary Number(2.0))(primary Number(3.0)))(primary Number(4.0)))))
  (decl (stmt expr (binary Minus (binary Plus (primary Number(2.0))(primary Number(3.0)))(primary Number(4.0)))))
  (decl (stmt expr (binary Minus (binary Minus (primary Number(2.0))(primary Number(3.0)))(primary Number(4.0)))))
  )", out);
}

#[test]
fn test_factors() {
    let out = parse_to_string("2 * 3; 2 / 3;", true);
    assert_eq!(
"(program
  (decl (stmt expr (binary Mult (primary Number(2.0))(primary Number(3.0)))))
  (decl (stmt expr (binary Div (primary Number(2.0))(primary Number(3.0)))))
  )", out);
}

#[test]
fn test_factors_assoc() {
    let out = parse_to_string("2 * 3 * 4; 2 / 3 * 4; 2 * 3 / 4; 2 / 3 / 4;", true);
    assert_eq!(
"(program
  (decl (stmt expr (binary Mult (binary Mult (primary Number(2.0))(primary Number(3.0)))(primary Number(4.0)))))
  (decl (stmt expr (binary Mult (binary Div (primary Number(2.0))(primary Number(3.0)))(primary Number(4.0)))))
  (decl (stmt expr (binary Div (binary Mult (primary Number(2.0))(primary Number(3.0)))(primary Number(4.0)))))
  (decl (stmt expr (binary Div (binary Div (primary Number(2.0))(primary Number(3.0)))(primary Number(4.0)))))
  )", out);
}

#[test]
fn test_simple_fun_decl() {
    let out = parse_to_string("fun main() {    print 1234; }", true);
    
    assert_eq!(
"(program
  (decl fun main() {
      (stmt print  (primary Number(1234.0)))})
  )", out);
}