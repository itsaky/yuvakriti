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

use crate::ast::{ASTVisitor, Visitable};
use crate::ast::BlockStmt;
use crate::ast::Program;
use crate::ast::VarStmt;
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
    fn visit_program(&mut self, program: &Program, p: &mut Scope) -> Option<()> {
        self.scope = Some(Scope::new());
        self.default_visit_program(program, p, true, true);
        self.scope = None;

        None
    }

    fn visit_var_decl(&mut self, var_decl: &VarStmt, scope: &mut Scope) -> Option<()> {
        self.default_visit_var_decl(var_decl, scope);

        let var_name = &var_decl.name.0;
        match scope.push_sym(Symbol::Variable(VarSym::new(var_name.clone()))) {
            Err(_) => self.report_err(&var_decl.name.1, &messages::err_dup_var(&var_name)),
            Ok(_) => {}
        };
        
        None
    }

    fn visit_block_stmt(&mut self, block_stmt: &BlockStmt, p: &mut Scope) -> Option<()> {
        let mut new = Scope::new();
        new.parent = Some(&p);
        self.default_visit_block_stmt(block_stmt, &mut new);

        None
    }
}
