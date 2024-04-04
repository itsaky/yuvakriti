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

use crate::compiler::ast::AstNode;
use crate::compiler::ast::ASTVisitor;
use crate::compiler::ast::PrimaryExpr;
use crate::compiler::ast::Program;
use crate::compiler::bytecode::cp::ConstantEntry;
use crate::compiler::bytecode::cp_info::NumberInfo;
use crate::compiler::bytecode::cp_info::StringInfo;
use crate::compiler::bytecode::cp_info::Utf8Info;
use crate::compiler::bytecode::YKBFile;

/// Converts a program into a YKB file.
pub(crate) struct YKBFileWriter {
    file: YKBFile,
}

impl YKBFileWriter {
    pub(crate) fn new() -> Self {
        return YKBFileWriter {
            file: YKBFile::new(),
        };
    }
    
    pub(crate) fn file(&self) -> &YKBFile {
        return &self.file;
    }

    pub(crate) fn file_mut(&mut self) -> &mut YKBFile {
        return &mut self.file;
    }

    pub(crate) fn write(&mut self, program: &mut Program) -> &YKBFile {
        program.accept(self, &());
        return self.file();
    }
}

impl ASTVisitor<(), u16> for YKBFileWriter {
    fn visit_primary_expr(
        &mut self,
        _primary_expr: &PrimaryExpr,
        file: &(),
    ) -> Option<u16> {
        let constant_pool = self.file.constant_pool_mut();
        match _primary_expr {
            PrimaryExpr::Number(num) => {
                constant_pool.push(ConstantEntry::Number(NumberInfo::from(num)))
            }
            PrimaryExpr::String(str) => {
                let str = &str[1..str.len() - 1]; // remove double quotes
                let utf8info = constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(str)));
                constant_pool.push(ConstantEntry::String(StringInfo {
                    string_index: utf8info,
                }))
            }
            PrimaryExpr::Identifier(ident) => {
                constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(ident)))
            }
            _ => return self.default_visit_primary_expr(_primary_expr, file),
        };

        None
    }
}
