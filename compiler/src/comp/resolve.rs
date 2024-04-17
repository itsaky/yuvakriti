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

use crate::ast::ASTVisitor;
use crate::ast::BlockStmt;
use crate::ast::IdentifierExpr;
use crate::ast::Program;
use crate::ast::Spanned;
use crate::ast::VarStmt;
use crate::ast::Visitable;
use crate::diagnostics::Diagnostic;
use crate::diagnostics::DiagnosticHandler;
use crate::diagnostics::DiagnosticKind;
use crate::location::Range;
use crate::messages;
use crate::scope::Scope;
use crate::symtab::Symbol;
use crate::symtab::VarSym;

/// The name resolution helper.
pub struct Resolve<'inst> {
    scope: Option<Scope<'inst>>,
    diagnostics: Rc<RefCell<dyn DiagnosticHandler + 'inst>>,
    has_errors: bool,
}

impl Resolve<'_> {
    /// Create a new instance of the name resolution helper.
    pub fn new<'a>(diagnostics: Rc<RefCell<dyn DiagnosticHandler + 'a>>) -> Resolve<'a> {
        return Resolve {
            diagnostics,
            scope: None,
            has_errors: false,
        };
    }

    /// Reset the state of the name resolver.
    pub fn reset(&mut self) {
        self.scope = None;
        self.has_errors = false;
    }

    /// Returns whether there were any errors during name resolution.
    pub fn has_errors(&self) -> bool {
        return self.has_errors;
    }

    pub fn analyze(&mut self, program: &mut Program) {
        let mut scope = Scope::new();
        program.accept(self, &mut scope);
    }

    fn report_err(&mut self, range: &Range, msg: &str) {
        self.has_errors = true;
        self.diagnostics.borrow_mut().handle(Diagnostic {
            kind: DiagnosticKind::Error,
            range: range.clone(),
            message: msg.to_string(),
        });
    }
}

impl<'inst> ASTVisitor<Scope<'inst>, ()> for Resolve<'_> {
    fn visit_program(&mut self, program: &mut Program, p: &mut Scope) -> Option<()> {
        self.scope = Some(Scope::new());
        self.default_visit_program(program, p, true, true);
        self.scope = None;

        None
    }

    fn visit_var_stmt(&mut self, var_decl: &mut VarStmt, scope: &mut Scope) -> Option<()> {
        let var_name = &var_decl.name.name.clone();

        // visit the initializer first, so we could report usage of this variable in its initializer
        // var a = a + 1; // 'a' is used before it is declarec
        if let Some(expr) = var_decl.initializer.as_mut() {
            self.visit_expr(expr, scope);
        }

        match scope.push_sym(Symbol::Variable(VarSym::new(var_name.clone()))) {
            Err(_) => self.report_err(&var_decl.name.range(), &messages::err_dup_var(&var_name)),
            Ok(_) => {}
        };

        None
    }

    fn visit_block_stmt(&mut self, block_stmt: &mut BlockStmt, p: &mut Scope) -> Option<()> {
        let mut new = Scope::new();
        new.parent = Some(&p);
        self.default_visit_block_stmt(block_stmt, &mut new);

        None
    }

    fn visit_identifier_expr(
        &mut self,
        identifier: &mut IdentifierExpr,
        _p: &mut Scope<'inst>,
    ) -> Option<()> {
        let name = &identifier.name;
        if _p.find_sym(name).is_none() {
            self.report_err(identifier.range(), &messages::err_undecl_var(name));
        }

        None
    }
}
