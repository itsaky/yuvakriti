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

use std::sync::OnceLock;

use crate::yklang::compiler::location::Range;

pub trait DiagnosticHandler {

    fn handle(&mut self, diagnostic: Diagnostic);
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct NoOpDiagnosticHandler {}

impl NoOpDiagnosticHandler {
    pub fn instance() -> &'static NoOpDiagnosticHandler {
        static INSTANCE: OnceLock<NoOpDiagnosticHandler> = OnceLock::new();
        return INSTANCE.get_or_init(|| NoOpDiagnosticHandler {})
    }
}

impl DiagnosticHandler for NoOpDiagnosticHandler {
    fn handle(&mut self, _diagnostic: Diagnostic) {}
}

pub fn no_op_handler() -> &'static NoOpDiagnosticHandler {
    return NoOpDiagnosticHandler::instance();
}


pub struct CollectingDiagnosticHandler {
    diagnostics: Vec<Diagnostic>
}

impl CollectingDiagnosticHandler {
    pub fn new() -> CollectingDiagnosticHandler {
        return CollectingDiagnosticHandler {
            diagnostics: Vec::new()
        }
    }
}

impl DiagnosticHandler for CollectingDiagnosticHandler {
    fn handle(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
}

pub fn collecting_handler() -> CollectingDiagnosticHandler {
    return CollectingDiagnosticHandler::new();
}

#[derive(Eq)]
pub struct Diagnostic {
    pub range: Range,
    pub message: String,
    pub kind: DiagnosticKind
}

#[derive(Eq)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Note
}

impl PartialEq<Self> for Diagnostic {
    fn eq(&self, other: &Self) -> bool {
        return self.range == other.range
            && self.message == other.message
    }
}

impl PartialEq<Self> for DiagnosticKind {
    fn eq(&self, other: &Self) -> bool {
        return std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

