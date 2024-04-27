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

use crate::ast::Program;
use crate::ast::Visitable;
use crate::comp::ConstFold;
use crate::comp::Resolve;
use crate::diagnostics::DiagnosticHandler;
use crate::features::CompilerFeatures;

/// The attribution phase of the compiler.
pub struct Attr<'inst> {
    resolve: Resolve<'inst>,
    constfold: ConstFold,
    features: &'inst CompilerFeatures,
    has_errors: bool,
}

impl<'inst> Attr<'inst> {
    /// Create a new instance.
    pub fn new<'a>(
        features: &'a CompilerFeatures,
        diagnostics: &'a mut (dyn DiagnosticHandler + 'a),
    ) -> Attr<'a> {
        let resolve = Resolve::new(diagnostics);
        return Attr {
            resolve,
            features,
            constfold: ConstFold::new(),
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

        // Fold constant expressions, if enabled
        if self.features.const_folding {
            program.accept(&mut self.constfold, &mut ());
        }
    }

    /// Perform the name resolution.
    fn resolve(&mut self, program: &mut Program) {
        self.resolve.analyze(program);
        self.has_errors |= self.resolve.has_errors();
    }
}
