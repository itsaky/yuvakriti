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

use crate::ast::{ASTPrinter, ASTVisitor};
use crate::ast::Program;
use crate::ast::Visitable;
use crate::comp::YKCompiler;
use crate::diagnostics::CollectingDiagnosticHandler;
use crate::features::CompilerFeatures;
use crate::lexer::YKLexer;
use crate::parser::YKParser;

#[macro_export]
macro_rules! boxed_vec {
    ($($x:expr),+ $(,)?) => {
        vec![$(Box::from($x)),+]
    };
}

pub(crate) fn parse_1(
    source: &str,
    diagnostics: &mut CollectingDiagnosticHandler,
) -> Program {
    let lexer = YKLexer::new(Cursor::new(source), diagnostics);
    let mut parser = YKParser::new(lexer);
    assert!(!parser.has_errors());
    parser.parse()
}

pub(crate) fn parse(source: &str) -> Program {
    parse_attr(source, false, &CompilerFeatures::default())
}

pub(crate) fn parse_attr(source: &str, attr: bool, features: &CompilerFeatures) -> Program {
    let mut compiler = YKCompiler::new();
    let (mut program, has_errors) = compiler
        .parse(Cursor::new(source))
        .expect("Failed to parse source");
    assert!(!has_errors);

    if attr {
        assert!(!compiler.attr(&mut program, features));
    }

    program
}

#[allow(unused)]
pub(crate) fn node_string(program: &mut Program, pretty: bool) -> String {
    let mut out = String::new();
    let mut printer = ASTPrinter::new(&mut out, pretty);
    program.accept(&mut printer, &mut 0);
    out
}

#[allow(unused)]
pub(crate) fn parse_to_string(source: &str, pretty: bool) -> String {
    let mut program = parse(source);
    node_string(&mut program, pretty)
}

pub fn match_node(node: &mut impl Visitable, visitor: &mut dyn ASTVisitor<(), bool>) {
    assert!(node.accept(visitor, &mut ()).unwrap());
}

pub fn match_ast(source: &str, matcher: &mut dyn ASTVisitor<(), bool>) {
    match_node(&mut parse(source), matcher);
}
