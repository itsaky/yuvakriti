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

use crate::yklang::compiler::location::Range;

pub trait DiagnosticHandler {
    fn handle(diagnostic: Diagnostic);
}

#[derive(Eq)]
pub struct Diagnostic {
    range: Range,
    message: String,
}

impl PartialEq<Self> for Diagnostic {
    fn eq(&self, other: &Self) -> bool {
        return self.range == other.range
            && self.message == other.message
    }
}