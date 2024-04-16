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

use crate::comp::Resolve;
use crate::diagnostics::{collecting_handler, DiagnosticKind};
use crate::location::Position;
use crate::messages;
use crate::tests::util::parse_1;
use std::cell::RefCell;
use std::rc::Rc;

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
    assert_eq!(diagnostic.message, messages::err_dup_var("decl"));
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
    assert_eq!(diagnostic.message, messages::err_undecl_var("a"));
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
