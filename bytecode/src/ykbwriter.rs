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

use compiler::ast;
use compiler::ast::AstNode;
use compiler::ast::ASTVisitor;
use compiler::ast::FuncDecl;
use compiler::ast::PrimaryExpr;
use compiler::ast::Program;

use crate::cp::ConstantEntry;
use crate::cp_info::NumberInfo;
use crate::cp_info::StringInfo;
use crate::cp_info::Utf8Info;
use crate::decls;
use crate::ykbfile::YKBFile;

/// Converts a program into a YKB file.
pub struct YKBFileWriter {
    file: YKBFile,
}

impl YKBFileWriter {
    pub fn new() -> Self {
        return YKBFileWriter {
            file: YKBFile::new(),
        };
    }

    pub fn file(&self) -> &YKBFile {
        return &self.file;
    }

    pub fn file_mut(&mut self) -> &mut YKBFile {
        return &mut self.file;
    }

    pub fn write(&mut self, program: &mut Program) -> &YKBFile {
        let mut fpv = FirstPassVisitor::new(&mut self.file);
        program.accept(&mut fpv, &());
        return self.file();
    }
}

struct FirstPassVisitor<'a> {
    file: &'a mut YKBFile,
}
impl <'a> FirstPassVisitor<'a> {
    fn new(file: &'a mut YKBFile) -> Self {
        return FirstPassVisitor { file };
    }
}

impl ASTVisitor<(), ()> for FirstPassVisitor<'_> {
    fn visit_class_decl(&mut self, class_decl: &ast::ClassDecl, _p: &()) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        let name_index =
            constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(&class_decl.name.0)));
        self.file.declarations_mut().push(Box::new(decls::ClassDecl::new(name_index)));
        None
    }

    fn visit_func_decl(&mut self, func_decl: &FuncDecl, _p: &()) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        let name_index = constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(&func_decl.name.0)));
        self.file.declarations_mut().push(Box::new(decls::FuncDecl::new(name_index)));
        None
    }

    fn visit_primary_expr(&mut self, _primary_expr: &PrimaryExpr, p: &()) -> Option<()> {
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
            _ => return self.default_visit_primary_expr(_primary_expr, p),
        };

        None
    }
}
