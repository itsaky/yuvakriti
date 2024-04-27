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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub const NO_RANGE: Range = Range {
        start: Position::NO_POS,
        end: Position::NO_POS,
    };

    /// Create a new range.
    pub fn new() -> Range {
        return Self::NO_RANGE.clone();
    }

    /// Set the start and end of this range.
    pub fn update_range(&mut self, range: &Range) {
        self.update(&range.start, &range.end);
    }

    /// Set the start and end of this range.
    pub fn update(&mut self, start: &Position, end: &Position) {
        self.start = start.clone();
        self.end = end.clone();
    }

    /// Set the end of this range to the end of the given range.
    pub fn set_end(&mut self, end: &Range) -> Self {
        self.end = end.end.clone();
        *self
    }

    /// Set the end of this range to the given position.
    pub fn set_end_pos(&mut self, end: &Position) -> Self {
        self.end = end.clone();
        *self
    }

    /// Set the start of this range to the start of the given range.
    pub fn set_start(&mut self, start: &Range) -> Self {
        self.start = start.start.clone();
        *self
    }

    /// Set the start of this range to the given position.
    pub fn set_start_pos(&mut self, start: &Position) -> Self {
        self.start = start.clone();
        *self
    }
}

impl From<&Range> for Range {
    fn from(range: &Range) -> Self {
        return range.clone();
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    pub line: i32,
    pub column: i32,
    pub index: i64,
}

impl Position {
    pub const NO_POS: Position = Position {
        line: -1,
        column: -1,
        index: -1,
    };

    pub fn new(line: i32, column: i32, index: i64) -> Position {
        Position {
            line,
            column,
            index,
        }
    }
}
