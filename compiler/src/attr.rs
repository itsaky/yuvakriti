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
use crate::ast::Program;
use crate::diagnostics::DiagnosticHandler;
use crate::resolve::Resolve;

/// The attribution phase of the compiler.
pub struct Attr<'inst> {
    #[allow(unused)]
    diagnostics: Rc<RefCell<dyn DiagnosticHandler + 'inst>>,
    resolve: Resolve<'inst>,
    has_errors: bool,
}

impl <'inst> Attr<'inst> {
    
    /// Create a new instance.
    pub fn new<'a>(diagnostics: Rc<RefCell<dyn DiagnosticHandler + 'a>>) -> Attr<'a> {
        let resolve = Resolve::new(diagnostics.clone());
        return Attr {
            diagnostics,
            resolve,
            has_errors: false,
        };
    }
    
    /// Returns whether the analysis resulted in any errors.
    pub fn has_errors(&self) -> bool {
        return self.has_errors;
    }
    
    /// Reset the state.
    pub fn reset(&mut self) {
        self.resolve.reset();
        self.has_errors = false;
    }
    
    /// Analyze the given program.
    pub fn analyze(&mut self, program: &mut Program) {
        self.reset();
        self.resolve(program);
    }
    
    /// Perform the name resolution.
    fn resolve(&mut self, program: &mut Program) {
        self.resolve.analyze(program);
        self.has_errors |= self.resolve.has_errors();
    }
}