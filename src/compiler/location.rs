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

#[derive(Eq, Clone, Copy, Debug)]
pub(crate) struct Range {
    pub(crate) start: Position,
    pub(crate) end: Position
}

impl Range {
    pub(crate) const NO_RANGE: Range = Range {
        start: Position::NO_POS,
        end: Position::NO_POS
    };
    
    /// Create a new range.
    pub(crate) fn new() -> Range {
        return Self::NO_RANGE.clone();
    }
    
    /// Set the end of this range to the end of the given range.
    pub(crate) fn set_end(&mut self, end: &Range) -> Self {
        self.end = end.end.clone();
        *self
    }

    /// Set the end of this range to the given position.
    pub(crate) fn set_end_pos(&mut self, end: &Position) -> Self {
        self.end = end.clone();
        *self
    }

    /// Set the start of this range to the start of the given range.
    pub(crate) fn set_start(&mut self, start: &Range) -> Self {
        self.start = start.start.clone();
        *self
    }

    /// Set the start of this range to the given position.
    pub(crate) fn set_start_pos(&mut self, start: &Position) -> Self {
        self.start = start.clone();
        *self
    }
}

impl PartialEq<Self> for Range {
    fn eq(&self, other: &Self) -> bool {
        return self.start == other.start
            && self.end == other.end
    }
}

#[derive(Eq, Clone, Copy, Debug)]
pub(crate) struct Position {
    pub(crate) line: i32,
    pub(crate) column: i32,
    pub(crate) index: i64
}

impl Position {
    pub(crate) const NO_POS: Position = Position {
        line: -1,
        column: -1,
        index: -1
    };
}

impl PartialEq<Self> for Position {
    fn eq(&self, other: &Self) -> bool {
        return self.line == other.line
            && self.column == other.column
            && self.index == other.index
    }
}