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

use crate::CpSize;

pub enum YKBDeclType {
    ClassDecl,
    FuncDecl,
}

pub trait YKBDecl {
    fn dtype(&self) -> &YKBDeclType;
    fn name_index(&self) -> &CpSize;
}

pub struct ClassDecl {
    pub name_index: CpSize,
    // TODO(itsaky): Add the fields, methods, etc. here.
}

impl ClassDecl {
    pub fn new(name_index: CpSize) -> ClassDecl {
        return ClassDecl { name_index };
    }
}

impl YKBDecl for ClassDecl {
    fn dtype(&self) -> &YKBDeclType {
        return &YKBDeclType::ClassDecl;
    }
    fn name_index(&self) -> &CpSize {
        return &self.name_index;
    }
}

pub struct FuncDecl {
    pub name_index: CpSize,
    // TODO(itsaky): Add the params, body, etc. here.
}

impl FuncDecl {
    pub fn new(name_index: CpSize) -> FuncDecl {
        return FuncDecl { name_index };
    }
}

impl YKBDecl for FuncDecl {
    fn dtype(&self) -> &YKBDeclType {
        return &YKBDeclType::FuncDecl;
    }
    fn name_index(&self) -> &CpSize {
        return &self.name_index;
    }
}
