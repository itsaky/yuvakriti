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

use crate::compiler::location::Range;

pub(crate) trait DiagnosticHandler {

    fn handle(&mut self, diagnostic: Diagnostic);
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub(crate) struct NoOpDiagnosticHandler {}

impl NoOpDiagnosticHandler {
    pub(crate) fn instance() -> &'static NoOpDiagnosticHandler {
        static INSTANCE: OnceLock<NoOpDiagnosticHandler> = OnceLock::new();
        return INSTANCE.get_or_init(|| NoOpDiagnosticHandler {})
    }
}

impl DiagnosticHandler for NoOpDiagnosticHandler {
    fn handle(&mut self, _diagnostic: Diagnostic) {}
}

pub(crate) fn no_op_handler() -> &'static NoOpDiagnosticHandler {
    return NoOpDiagnosticHandler::instance();
}


#[derive(Debug)]
pub(crate) struct CollectingDiagnosticHandler {
    pub(crate) diagnostics: Vec<Diagnostic>
}

impl CollectingDiagnosticHandler {
    pub(crate) fn new() -> CollectingDiagnosticHandler {
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

pub(crate) fn collecting_handler() -> CollectingDiagnosticHandler {
    return CollectingDiagnosticHandler::new();
}

#[derive(Eq, Debug, Clone)]
pub(crate) struct Diagnostic {
    pub(crate) range: Range,
    pub(crate) message: String,
    pub(crate) kind: DiagnosticKind
}

#[derive(Eq, Debug, Clone)]
pub(crate) enum DiagnosticKind {
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

