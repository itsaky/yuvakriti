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
use std::rc::Rc;

use crate::comp::Resolve;
use crate::diagnostics::collecting_handler;
use crate::diagnostics::DiagnosticKind;
use crate::location::Position;
use crate::messages::err_dup_var;
use crate::messages::err_undef_label;
use crate::messages::err_undef_var;
use crate::tests::util::parse_1;

fn match_single_diagnostic(src: &str, msg: String) {
    let diags = Rc::new(RefCell::new(collecting_handler()));
    let mut program = parse_1(src, diags.clone());
    let mut analyzer = Resolve::new(diags.clone());
    analyzer.analyze(&mut program);

    let diags = diags.borrow();
    let diagnostics = &diags.diagnostics;

    assert!(!diagnostics.is_empty());
    assert_eq!(1, diagnostics.len());
    assert_eq!(msg, diagnostics[0].message);
}

#[test]
fn test_duplicate_var_decl() {
    let diags = Rc::new(RefCell::new(collecting_handler()));
    let mut program = parse_1("var decl = 1; var decl = 2;", diags.clone());
    let mut analyzer = Resolve::new(diags.clone());
    analyzer.analyze(&mut program);

    let diags = diags.borrow();
    let diagnostics = &diags.diagnostics;

    assert!(!diagnostics.is_empty());
    assert_eq!(1, diagnostics.len());

    let diagnostic = &diagnostics[0];
    assert_eq!(diagnostic.kind, DiagnosticKind::Error);
    assert_eq!(diagnostic.range.start, Position::new(0, 18, 18));
    assert_eq!(diagnostic.range.end, Position::new(0, 22, 22));
    assert_eq!(diagnostic.message, err_dup_var("decl"));
}

#[test]
fn test_undeclared_var() {
    let diags = Rc::new(RefCell::new(collecting_handler()));
    let mut program = parse_1("var decl = 1 + a;", diags.clone());
    let mut analyzer = Resolve::new(diags.clone());
    analyzer.analyze(&mut program);

    let diags = diags.borrow();
    let diagnostics = &diags.diagnostics;

    assert!(!diagnostics.is_empty());
    assert_eq!(1, diagnostics.len());

    let diagnostic = &diagnostics[0];
    assert_eq!(diagnostic.kind, DiagnosticKind::Error);
    assert_eq!(diagnostic.range.start, Position::new(0, 15, 15));
    assert_eq!(diagnostic.range.end, Position::new(0, 16, 16));
    assert_eq!(diagnostic.message, err_undef_var("a"));
}

#[test]
fn test_var_decl_in_sep_scope() {
    let diags = Rc::new(RefCell::new(collecting_handler()));

    // variables are declared in different scopes
    // hence, they should not interfere
    let mut program = parse_1("{var decl = 1;} {var decl = 2;}", diags.clone());
    let mut analyzer = Resolve::new(diags.clone());
    analyzer.analyze(&mut program);

    let diags = diags.borrow();
    let diagnostics = &diags.diagnostics;

    assert!(diagnostics.is_empty());
}

#[test]
fn test_var_self_use_in_decl() {
    let diags = Rc::new(RefCell::new(collecting_handler()));

    // variables are declared in different scopes
    // hence, they should not interfere
    let mut program = parse_1("var a = a + 1;", diags.clone());
    let mut analyzer = Resolve::new(diags.clone());
    analyzer.analyze(&mut program);

    let diags = diags.borrow();
    let diagnostics = &diags.diagnostics;

    assert!(!diagnostics.is_empty());
    assert_eq!(err_undef_var("a"), diagnostics[0].message);
    assert_eq!(Position::new(0, 8, 8), diagnostics[0].range.start);
    assert_eq!(Position::new(0, 9, 9), diagnostics[0].range.end);
}

#[test]
fn test_undef_label_in_break_for_while() {
    match_single_diagnostic("while true { break something; }", err_undef_label("something"));
}

#[test]
fn test_undef_label_in_break_for_for() {
    match_single_diagnostic("for (var i =0; i<10; i=i+1) { break something; }", err_undef_label("something"));
}

#[test]
fn test_undef_label_in_continue_for_while() {
    match_single_diagnostic("while true { continue something; }", err_undef_label("something"));
}

#[test]
fn test_undef_label_in_continue_for_for() {
    match_single_diagnostic("for (var i =0; i<10; i=i+1) { continue something; }", err_undef_label("something"));
}

#[test]
fn test_undef_label_in_continue_for_nested_while() {
    match_single_diagnostic("outer: while true { while false {continue inner;} }", err_undef_label("inner"));
}

#[test]
fn test_undef_label_in_continue_for_nested_for() {
    match_single_diagnostic("for (var i =0; i<10; i=i+1) { for (var j =0; j<10; j=j+1) { continue inner; } }", err_undef_label("inner"));
}

#[test]
fn test_dup_var_in_nested_loop() {
    match_single_diagnostic("for (var i =0; i<10; i=i+1) { for (var i =0; i<10; i=i+1) {} }", err_dup_var("i"));
}

#[test]
fn test_dup_var_in_nested_scope() {
    match_single_diagnostic("var i = 0; { var i = 1; }", err_dup_var("i"));
}

