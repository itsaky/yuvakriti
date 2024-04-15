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

use crate::ast::Spanned;
use crate::ast::UnaryOp;
use crate::ast::Visitable;
use crate::ast::{ASTVisitor, ArithmeticASTPrinter, BinaryOp};
use crate::ast::{NodeType, Stmt};
use crate::diagnostics;
use crate::lexer::YKLexer;
use crate::messages;
use crate::parser::YKParser;
use crate::tests::matcher::Binary;
use crate::tests::matcher::Bool;
use crate::tests::matcher::Identifier;
use crate::tests::matcher::Nil;
use crate::tests::matcher::Node;
use crate::tests::matcher::Number;
use crate::tests::matcher::Program;
use crate::tests::matcher::String;
use crate::tests::matcher::Unary;
use crate::tests::util::parse;

macro_rules! boxed_vec {
    ($($x:expr),+ $(,)?) => {
        vec![$(Box::new($x)),+]
    };
}

fn assert_ast(node: &mut impl Visitable, visitor: &mut dyn ASTVisitor<(), bool>) {
    assert!(node.accept(visitor, &mut ()).unwrap());
}

fn match_ast(source: &str, matcher: &mut dyn ASTVisitor<(), bool>) {
    assert_ast(&mut parse(source), matcher);
}

#[test]
fn test_simple_var_decl() {
    let source = "var something = 1234;";
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let lexer = YKLexer::new(Cursor::new(source), diag_handler.clone());

    let mut parser = YKParser::new(lexer, diag_handler.clone());
    let program = parser.parse();
    assert_eq!(true, diag_handler.borrow().diagnostics.is_empty());

    let stmts = program.stmts;
    assert_eq!(1, stmts.len());

    let stmt = stmts.get(0).expect("Statement expected");
    assert_eq!(0, stmt.range().start.line);
    assert_eq!(0, stmt.range().start.column);
    assert_eq!(0, stmt.range().start.index);
    assert_eq!(0, stmt.range().end.line);
    assert_eq!(20, stmt.range().end.column as usize);
    assert_eq!(20, stmt.range().end.index as usize);

    let var = if let Stmt::Var(var) = &stmt {
        var
    } else {
        panic!("Expected a variable statement")
    };

    let (num, _) = &var
        .initializer
        .as_ref()
        .and_then(|expr| expr.Literal())
        .and_then(|lit| lit.Number())
        .expect("Expected a number literal");

    assert_eq!("something", var.name.name);
    assert_eq!(0, var.name.range().start.line);
    assert_eq!(4, var.name.range().start.column);
    assert_eq!(4, var.name.range().start.index);
    assert_eq!(0, var.name.range().end.line);
    assert_eq!(13, var.name.range().end.column as usize);
    assert_eq!(13, var.name.range().end.index as usize);

    let init = var.initializer.as_ref().expect("Initializer expected");
    assert_eq!(0, init.range().start.line);
    assert_eq!(16, init.range().start.column);
    assert_eq!(16, init.range().start.index);
    assert_eq!(0, init.range().end.line);
    assert_eq!(20, init.range().end.column as usize);
    assert_eq!(20, init.range().end.index as usize);

    assert_eq!(1234f64, *num);
}

#[test]
fn test_simple_for_statement() {
    let out = parse("for (var i = 0; i < 10; i = i + 1) {\n    print i;\n}");

    let stmts = &out.stmts;
    assert_eq!(1, stmts.len());

    let mut matcher = Program(
        vec![],
        boxed_vec![Node(
            NodeType::ForStmt,
            boxed_vec![
                Node(
                    NodeType::VarStmt,
                    boxed_vec![Identifier("i"), Number(0f64),]
                ),
                Binary(BinaryOp::Lt, boxed_vec![Identifier("i"), Number(10f64),]),
                Node(
                    NodeType::AssignExpr,
                    boxed_vec![
                        Identifier("i"),
                        Binary(BinaryOp::Plus, boxed_vec![Identifier("i"), Number(1f64),])
                    ]
                )
            ]
        )],
    );

    assert!(out
        .accept(&mut matcher, &mut ())
        .expect("Failed to match program"));
}

#[test]
fn test_simple_unary_negation_expr() {
    match_ast(
        " !true; ",
        &mut Program(
            vec![],
            boxed_vec![Unary(UnaryOp::Not, Box::from(Bool(true)),)],
        ),
    );
}

#[test]
fn test_simple_unary_num_negation_expr() {
    match_ast(
        " -123; ",
        &mut Program(
            vec![],
            boxed_vec![Unary(UnaryOp::Negate, Box::from(Number(123f64)),)],
        ),
    );
}

#[test]
fn test_primary_exprs() {
    match_ast(
        "true; false; nil; this; 123; \"something\"; identifier; (\"grouping\");",
        &mut Program(
            vec![],
            boxed_vec![
                Bool(true),
                Bool(false),
                Nil(),
                Identifier("this"),
                Number(123f64),
                String("\"something\""),
                Identifier("identifier"),
                String("\"grouping\""),
            ],
        ),
    );
}

#[test]
fn test_terms() {
    match_ast(
        "2 + 3; 2 - 3;",
        &mut Program(
            vec![],
            boxed_vec![
                Binary(BinaryOp::Plus, boxed_vec![Number(2f64), Number(3f64)]),
                Binary(BinaryOp::Minus, boxed_vec![Number(2f64), Number(3f64)])
            ],
        ),
    );
}

#[test]
fn test_terms_assoc() {
    match_ast(
        "2 + 3 + 4; 2 - 3 + 4; 2 + 3 - 4; 2 - 3 - 4;",
        &mut Program(
            vec![],
            boxed_vec![
                Binary(
                    BinaryOp::Plus,
                    boxed_vec![
                        Binary(BinaryOp::Plus, boxed_vec![Number(2f64), Number(3f64)]),
                        Number(4f64)
                    ]
                ),
                Binary(
                    BinaryOp::Plus,
                    boxed_vec![
                        Binary(BinaryOp::Minus, boxed_vec![Number(2f64), Number(3f64)]),
                        Number(4f64)
                    ]
                ),
                Binary(
                    BinaryOp::Minus,
                    boxed_vec![
                        Binary(BinaryOp::Plus, boxed_vec![Number(2f64), Number(3f64)]),
                        Number(4f64)
                    ]
                ),
                Binary(
                    BinaryOp::Minus,
                    boxed_vec![
                        Binary(BinaryOp::Minus, boxed_vec![Number(2f64), Number(3f64)]),
                        Number(4f64)
                    ]
                )
            ],
        ),
    );
}

#[test]
fn test_factors() {
    match_ast(
        "2 * 3; 2 / 3;",
        &mut Program(
            vec![],
            boxed_vec![
                Binary(BinaryOp::Mult, boxed_vec![Number(2f64), Number(3f64)]),
                Binary(BinaryOp::Div, boxed_vec![Number(2f64), Number(3f64)])
            ],
        ),
    );
}

#[test]
fn test_factors_assoc() {
    match_ast(
        "2 * 3 * 4; 2 / 3 * 4; 2 * 3 / 4; 2 / 3 / 4;",
        &mut Program(
            vec![],
            boxed_vec![
                Binary(
                    BinaryOp::Mult,
                    boxed_vec![
                        Binary(BinaryOp::Mult, boxed_vec![Number(2f64), Number(3f64)]),
                        Number(4f64)
                    ]
                ),
                Binary(
                    BinaryOp::Mult,
                    boxed_vec![
                        Binary(BinaryOp::Div, boxed_vec![Number(2f64), Number(3f64)]),
                        Number(4f64)
                    ]
                ),
                Binary(
                    BinaryOp::Div,
                    boxed_vec![
                        Binary(BinaryOp::Mult, boxed_vec![Number(2f64), Number(3f64)]),
                        Number(4f64)
                    ]
                ),
                Binary(
                    BinaryOp::Div,
                    boxed_vec![
                        Binary(BinaryOp::Div, boxed_vec![Number(2f64), Number(3f64)]),
                        Number(4f64)
                    ]
                )
            ],
        ),
    );
}

#[test]
fn test_arith_prec() {
    let program = parse("4 * 5 - (2 + 3) / 6 + 7;");
    let mut out = String::new();
    let mut printer = ArithmeticASTPrinter::new(&mut out);
    program.accept(&mut printer, &mut ());
    assert_eq!("(((4 * 5) - ((2 + 3) / 6)) + 7)", out);
}

#[test]
fn test_arith_assoc() {
    let cases = [
        ("2 - 3 - 4;", "((2 - 3) - 4)"),
        ("2 - 3 + 4;", "((2 - 3) + 4)"),
        ("2 + 3 - 4;", "((2 + 3) - 4)"),
        ("2 + 3 + 4;", "((2 + 3) + 4)"),
        ("2 * 3 * 4;", "((2 * 3) * 4)"),
        ("2 * 3 / 4;", "((2 * 3) / 4)"),
        ("2 / 3 * 4;", "((2 / 3) * 4)"),
        ("2 / 3 / 4;", "((2 / 3) / 4)"),
        ("2 + 3 * 4;", "(2 + (3 * 4))"),
        ("2 * 3 + 4;", "((2 * 3) + 4)"),
        ("2 + 3 / 4;", "(2 + (3 / 4))"),
        ("2 / 3 + 4;", "((2 / 3) + 4)"),
        ("2 - 3 * 4;", "(2 - (3 * 4))"),
        ("2 * 3 - 4;", "((2 * 3) - 4)"),
        ("2 - 3 / 4;", "(2 - (3 / 4))"),
        ("2 / 3 - 4;", "((2 / 3) - 4)"),
    ];

    let mut ok = true;
    for (source, expected) in cases {
        print!("Checking:: source: {}, expected: {}", source, expected);
        let program = parse(source);
        let mut out = String::new();
        let mut printer = ArithmeticASTPrinter::new(&mut out);
        program.accept(&mut printer, &mut ());
        ok = ok && out == expected;
        if ok {
            println!("    ...OK")
        } else {
            println!("    ...FAIL")
        }
    }

    assert!(ok)
}

#[test]
fn test_parser_diagnostic_at_end() {
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let lexer = YKLexer::new(
        Cursor::new("2 + 3"), // missing semicolon
        diag_handler.clone(),
    );

    let mut parser = YKParser::new(lexer, diag_handler.clone());
    let program = parser.parse();
    let mut out = String::new();
    let mut printer = ArithmeticASTPrinter::new(&mut out);
    program.accept(&mut printer, &mut ());

    assert!(program.decls.is_empty());

    let diags = &diag_handler.borrow().diagnostics;
    assert!(!diags.is_empty());
    assert_eq!(2, diags.len());

    let semi_exp = diags.get(0).expect("Diagnostic expected");
    let stmt_exp = diags.get(1).expect("Diagnostic expected");

    assert_eq!(messages::err_exp_sym(";"), semi_exp.message);
    assert_eq!(messages::PARS_DECL_OR_STMT_EXPECTED, stmt_exp.message);
}

#[test]
fn test_simple_fun_decl() {
    match_ast(
        "fun main() {    print 1234; }",
        &mut Program(
            vec![],
            boxed_vec![Node(
                NodeType::FuncDecl,
                boxed_vec![
                    Identifier("main"),
                    Node(
                        NodeType::BlockStmt,
                        boxed_vec![Node(NodeType::PrintStmt, boxed_vec![Number(1234f64)])]
                    )
                ]
            )],
        ),
    );
}
