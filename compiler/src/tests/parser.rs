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

use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;

use log::info;

use crate::ast::ASTPrinter;
use crate::ast::ArithmeticASTPrinter;
use crate::ast::BinaryOp;
use crate::ast::NodeType;
use crate::ast::Spanned;
use crate::ast::Stmt;
use crate::ast::UnaryOp;
use crate::ast::Visitable;
use crate::boxed_vec;
use crate::comp::YKCompiler;
use crate::diagnostics;
use crate::diagnostics::DiagnosticKind;
use crate::features::CompilerFeatures;
use crate::lexer::YKLexer;
use crate::messages;
use crate::parser::YKParser;
use crate::tests::matcher::Any;
use crate::tests::matcher::Array;
use crate::tests::matcher::Binary;
use crate::tests::matcher::Bool;
use crate::tests::matcher::CompoundAssigment;
use crate::tests::matcher::Identifier;
use crate::tests::matcher::Node;
use crate::tests::matcher::Null;
use crate::tests::matcher::Number;
use crate::tests::matcher::Program;
use crate::tests::matcher::String;
use crate::tests::matcher::Unary;
use crate::tests::util::match_ast;
use crate::tests::util::match_node;
use crate::tests::util::parse;
use crate::tests::util::parse_1;

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
    let mut out = parse("for (var i = 0; i < 10; i = i + 1) {\n    print i;\n}");

    let mut str = String::new();
    let mut printer = ASTPrinter::new(&mut str, true);
    out.accept(&mut printer, &mut 0);
    println!("{}", str);

    let stmts = &out.stmts;
    assert_eq!(1, stmts.len());

    let mut matcher = Program(
        vec![],
        boxed_vec![Node(
            NodeType::ForStmt,
            boxed_vec![
                Any(),
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
        "true; false; null; this; 123; \"something\"; identifier; (\"grouping\");",
        &mut Program(
            vec![],
            boxed_vec![
                Bool(true),
                Bool(false),
                Null(),
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
    let mut program = parse("4 * 5 - (2 + 3) / 6 + 7;");
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
        let mut program = parse(source);
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
    let mut program = parser.parse();
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

#[test]
fn test_const_folded_ast() {
    let cases = [
        ("2 - 3 - 4", Number(-5f64)),
        ("2 - 3 + 4", Number(3f64)),
        ("2 + 3 - 4", Number(1f64)),
        ("2 + 3 + 4", Number(9f64)),
        ("2 * 3 * 4", Number(24f64)),
        ("2 * 3 / 4", Number(1.5f64)),
        ("2 / 3 * 4", Number(2.6666666666666665f64)),
        ("2 / 3 / 4", Number(0.16666666666666666f64)),
        ("2 + 3 * 4", Number(14f64)),
        ("2 * 3 + 4", Number(10f64)),
        ("2 + 3 / 4", Number(2.75f64)),
        ("2 / 3 + 4", Number(4.666666666666667f64)),
        ("2 - 3 * 4", Number(-10f64)),
        ("2 * 3 - 4", Number(2f64)),
        ("2 - 3 / 4", Number(1.25f64)),
        ("2 / 3 - 4", Number(-3.3333333333333335f64)),
        ("2 - 3 - 4", Number(-5f64)),
        ("2 - 3 + 4", Number(3f64)),
        ("-2 + 3 - 4", Number(-3f64)),
        ("-2 - 3 + 4", Number(-1f64)),
        ("2 + 3 - (-4)", Number(9f64)),
        ("-2 * 3 * 4", Number(-24f64)),
        ("2 * (-3) / 4", Number(-1.5f64)),
        ("-2 / 3 * 4", Number(-2.6666666666666665f64)),
        ("-2 / 3 / 4", Number(-0.16666666666666666f64)),
        ("2 + (-3) * 4", Number(-10f64)),
        ("(-2) * 3 + 4", Number(-2f64)),
        ("2 + (-3) / 4", Number(1.25f64)),
        ("-2 / 3 + 4", Number(3.3333333333333335f64)),
        ("2 * (3 + 4)", Number(14f64)),
        ("(2 * 3) + 4", Number(10f64)),
        ("2 + (3 / 4)", Number(2.75f64)),
        ("(2 / 3) + 4", Number(4.666666666666667f64)),
        ("2 - (3 * 4)", Number(-10f64)),
        ("(2 * 3) - 4", Number(2f64)),
        ("2 - (3 / 4)", Number(1.25f64)),
        ("(2 / 3) - 4", Number(-3.3333333333333335f64)),
        ("2 < 3", Bool(true)),
        ("2 <= 3", Bool(true)),
        ("2 > 3", Bool(false)),
        ("2 >= 3", Bool(false)),
        ("2 == 3", Bool(false)),
        ("2 != 3", Bool(true)),
        ("0 < 1", Bool(true)),
        ("0 <= 1", Bool(true)),
        ("0 > 1", Bool(false)),
        ("0 >= 1", Bool(false)),
        ("0 == 0", Bool(true)),
        ("0 != 0", Bool(false)),
        ("10.5 < 11", Bool(true)),
        ("10.5 <= 11", Bool(true)),
        ("10.5 > 11", Bool(false)),
        ("10.5 >= 11", Bool(false)),
        ("-5.2 < -5", Bool(true)),
        ("-5.2 <= -5", Bool(true)),
        ("-5.2 > -5", Bool(false)),
        ("-5.2 >= -5", Bool(false)),
        ("(2 < 3) and (2 <= 3)", Bool(true)),
        ("(2 > 3) or (2 >= 3)", Bool(false)),
        ("true and true", Bool(true)),
        ("true and false", Bool(false)),
        ("false and true", Bool(false)),
        ("false and false", Bool(false)),
        ("true or true", Bool(true)),
        ("true or false", Bool(true)),
        ("false or true", Bool(true)),
        ("false or false", Bool(false)),
        ("!true", Bool(false)),
        ("!false", Bool(true)),
        ("2 == 2", Bool(true)),
        ("3.14 < 3.15", Bool(true)),
        ("true != false", Bool(true)),
    ];

    for (src, exp) in cases {
        info!("Test constant folding on: {}", src);
        let mut compiler = YKCompiler::new();
        let (mut program, has_errors) = compiler
            .parse(Cursor::new(format!("print {};", src)))
            .expect("Failed to parse source");

        let mut features = CompilerFeatures::default();
        features.const_folding = true; // enable constant folding

        let mut out = String::new();
        let mut printer = ASTPrinter::new(&mut out, false);
        program.accept(&mut printer, &mut 0);

        assert!(!compiler.attr(&mut program, &features));

        assert!(!has_errors);

        match_node(
            &mut program,
            &mut Program(
                vec![],
                boxed_vec![Node(NodeType::PrintStmt, boxed_vec![exp])],
            ),
        )
    }
}

#[test]
fn test_break_stmt() {
    match_ast(
        "break;",
        &mut Program(vec![], boxed_vec![Node(NodeType::BreakStmt, vec![])]),
    );
}

#[test]
fn test_break_stmt_labeled() {
    match_ast(
        "break label;",
        &mut Program(
            vec![],
            boxed_vec![Node(NodeType::BreakStmt, boxed_vec![Identifier("label")])],
        ),
    );
}

#[test]
fn test_continue_stmt() {
    match_ast(
        "continue;",
        &mut Program(vec![], boxed_vec![Node(NodeType::ContinueStmt, vec![])]),
    );
}

#[test]
fn test_continue_stmt_labeled() {
    match_ast(
        "continue label;",
        &mut Program(
            vec![],
            boxed_vec![Node(
                NodeType::ContinueStmt,
                boxed_vec![Identifier("label")]
            )],
        ),
    );
}

#[test]
fn test_labeled_for_stmt() {
    match_ast(
        "\
    label: for (var i = 0; i < 10; i = i + 1) {\
        print i;
    }",
        &mut Program(
            vec![],
            boxed_vec![Node(
                NodeType::ForStmt,
                boxed_vec![
                    Identifier("label"),
                    Node(NodeType::VarStmt, boxed_vec![Identifier("i"), Number(0f64)]),
                    Binary(BinaryOp::Lt, boxed_vec![Identifier("i"), Number(10f64)]),
                    Node(
                        NodeType::AssignExpr,
                        boxed_vec![
                            Identifier("i"),
                            Binary(BinaryOp::Plus, boxed_vec![Identifier("i"), Number(1f64)])
                        ]
                    )
                ]
            )],
        ),
    )
}

#[test]
fn test_simple_while_stmt() {
    match_ast(
        "\
    while true {\
        print i;
    }",
        &mut Program(
            vec![],
            boxed_vec![Node(
                NodeType::WhileStmt,
                boxed_vec![
                    Any(),
                    Bool(true),
                    Node(
                        NodeType::BlockStmt,
                        boxed_vec![Node(NodeType::PrintStmt, boxed_vec![Identifier("i")])]
                    )
                ]
            )],
        ),
    )
}

#[test]
fn test_labeled_while_stmt() {
    match_ast(
        "\
    label: while true {\
        print i;
    }",
        &mut Program(
            vec![],
            boxed_vec![Node(
                NodeType::WhileStmt,
                boxed_vec![
                    Identifier("label"),
                    Bool(true),
                    Node(
                        NodeType::BlockStmt,
                        boxed_vec![Node(NodeType::PrintStmt, boxed_vec![Identifier("i")])]
                    )
                ]
            )],
        ),
    )
}

#[test]
fn test_compound_assignment_operator() {
    match_ast(
        "var i = 0; i += 1;",
        &mut Program(
            vec![],
            boxed_vec![
                Node(NodeType::VarStmt, boxed_vec![Identifier("i"), Number(0f64)]),
                CompoundAssigment(BinaryOp::Plus, boxed_vec![Identifier("i"), Number(1f64)]),
            ],
        ),
    );

    match_ast(
        "var i = 0; i -= 1;",
        &mut Program(
            vec![],
            boxed_vec![
                Node(NodeType::VarStmt, boxed_vec![Identifier("i"), Number(0f64)]),
                CompoundAssigment(BinaryOp::Minus, boxed_vec![Identifier("i"), Number(1f64)]),
            ],
        ),
    );

    match_ast(
        "var i = 0; i *= 1;",
        &mut Program(
            vec![],
            boxed_vec![
                Node(NodeType::VarStmt, boxed_vec![Identifier("i"), Number(0f64)]),
                CompoundAssigment(BinaryOp::Mult, boxed_vec![Identifier("i"), Number(1f64)]),
            ],
        ),
    );

    match_ast(
        "var i = 0; i /= 1;",
        &mut Program(
            vec![],
            boxed_vec![
                Node(NodeType::VarStmt, boxed_vec![Identifier("i"), Number(0f64)]),
                CompoundAssigment(BinaryOp::Div, boxed_vec![Identifier("i"), Number(1f64)]),
            ],
        ),
    );
}

#[test]
fn test_empty_statements() {
    match_ast(";", &mut Program(vec![], vec![]));
    match_ast(";;", &mut Program(vec![], vec![]));

    match_ast(
        "print \"Something\";;",
        &mut Program(
            vec![],
            boxed_vec![Node(
                NodeType::PrintStmt,
                boxed_vec![String("\"Something\"")]
            ),],
        ),
    );
}

#[test]
fn test_array_expr() {
    match_ast(
        "[1, 2, 3];",
        &mut Program(
            vec![],
            boxed_vec![Array(boxed_vec![Number(1f64), Number(2f64), Number(3f64)])],
        ),
    );

    // comma at the end
    match_ast(
        "[1, 2, 3,];",
        &mut Program(
            vec![],
            boxed_vec![Array(boxed_vec![Number(1f64), Number(2f64), Number(3f64)])],
        ),
    );

    // double comma at the beginning
    let diag_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    let mut program = parse_1("[1, 2, 3,,];", diag_handler.clone());
    let diags = &diag_handler.borrow().diagnostics;
    assert!(!diags.is_empty());
    assert_eq!(1, diags.len());

    let exp = diags.get(0).expect("Diagnostic expected");
    assert_eq!(DiagnosticKind::Error, exp.kind);
    assert_eq!(messages::PARS_EXPECTED_EXPR, exp.message);

    match_node(
        &mut program,
        &mut Program(
            vec![],
            boxed_vec![Array(boxed_vec![Number(1f64), Number(2f64), Number(3f64)])],
        ),
    )
}

#[test]
fn test_arr_var_decl() {
    match_ast(
        "var i = [1, 2, 3];",
        &mut Program(
            vec![],
            boxed_vec![Node(
                NodeType::VarStmt,
                boxed_vec![
                    Identifier("i"),
                    Array(boxed_vec![Number(1f64), Number(2f64), Number(3f64)])
                ]
            )],
        ),
    );

    match_ast(
        "var i = [1, 2, 3, 4, ];",
        &mut Program(
            vec![],
            boxed_vec![Node(
                NodeType::VarStmt,
                boxed_vec![
                    Identifier("i"),
                    Array(boxed_vec![
                        Number(1f64),
                        Number(2f64),
                        Number(3f64),
                        Number(4f64)
                    ])
                ]
            )],
        ),
    );
}
