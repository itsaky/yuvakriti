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

use std::sync::OnceLock;

use crate::location::Range;

pub trait DiagnosticHandler {
    fn handle(&mut self, diagnostic: Diagnostic);
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct NoOpDiagnosticHandler {}

impl NoOpDiagnosticHandler {
    pub fn instance() -> &'static NoOpDiagnosticHandler {
        static INSTANCE: OnceLock<NoOpDiagnosticHandler> = OnceLock::new();
        return INSTANCE.get_or_init(|| NoOpDiagnosticHandler {});
    }
}

impl DiagnosticHandler for NoOpDiagnosticHandler {
    fn handle(&mut self, _diagnostic: Diagnostic) {}
}

pub fn no_op_handler() -> &'static NoOpDiagnosticHandler {
    return NoOpDiagnosticHandler::instance();
}

#[derive(Debug)]
pub struct CollectingDiagnosticHandler {
    pub diagnostics: Vec<Diagnostic>,
}

impl CollectingDiagnosticHandler {
    pub fn new() -> CollectingDiagnosticHandler {
        return CollectingDiagnosticHandler {
            diagnostics: Vec::new(),
        };
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Diagnostic {
    pub range: Range,
    pub message: String,
    pub kind: DiagnosticKind,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Note,
}