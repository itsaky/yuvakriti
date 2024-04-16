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

use std::ops::Deref;

use crate::ast::ASTVisitor;
use crate::ast::BinaryOp;
use crate::ast::FuncDecl;
use crate::ast::IdentifierExpr;
use crate::ast::LiteralExpr;
use crate::ast::PrintStmt;
use crate::ast::Program;
use crate::ast::Visitable;
use crate::ast::{BinaryExpr, ClassDecl};
use crate::bytecode::attrs;
use crate::bytecode::cp::ConstantEntry;
use crate::bytecode::cp_info::NumberInfo;
use crate::bytecode::cp_info::Utf8Info;
use crate::bytecode::decls;
use crate::bytecode::file::YKBFile;
use crate::bytecode::opcode::OpCode;
use crate::features::CompilerFeatures;

/// Converts a program into a YKB file.
pub struct YKBFileWriter<'inst> {
    file: &'inst mut YKBFile,
    features: &'inst CompilerFeatures,
}

impl YKBFileWriter<'_> {
    pub fn new<'a>(file: &'a mut YKBFile, features: &'a CompilerFeatures) -> YKBFileWriter<'a> {
        return YKBFileWriter { file, features };
    }

    pub fn file(&self) -> &YKBFile {
        return &self.file;
    }

    pub fn file_mut(&mut self) -> &mut YKBFile {
        return &mut self.file;
    }

    pub fn write(&mut self, program: &mut Program) {
        let mut fpv = CodeGen::new(&mut self.file, &self.features);
        program.accept(&mut fpv, &mut ());
    }
}

struct CodeGen<'a> {
    file: &'a mut YKBFile,
    #[allow(unused)]
    features: &'a CompilerFeatures,
    code: Option<attrs::Code>,
}

impl<'a> CodeGen<'a> {
    fn new(file: &'a mut YKBFile, features: &'a CompilerFeatures) -> Self {
        return CodeGen {
            file,
            features,
            code: None,
        };
    }
}

impl ASTVisitor<(), ()> for CodeGen<'_> {
    fn visit_program(&mut self, program: &mut Program, p: &mut ()) -> Option<()> {
        if self
            .file
            .attributes()
            .iter()
            .find(|attr| attr.name() == attrs::CODE)
            .is_some()
        {
            panic!("A YKBFile cannot have multiple Code attributes")
        }

        self.code = Some(attrs::Code::new(0, 0, 0));

        self.default_visit_program(program, p, true, false);
        for i in 0..program.stmts.len() {
            let stmt = program.stmts.get_mut(i).unwrap();
            self.visit_stmt(stmt, p);
        }

        if self.code.as_ref().unwrap().instructions().len() > 0 {
            self.file
                .constant_pool_mut()
                .push(ConstantEntry::Utf8(Utf8Info::from(attrs::CODE)));

            self.file
                .attributes_mut()
                .push(attrs::Attr::Code(self.code.take().unwrap()));
        }

        self.code = None;
        None
    }

    fn visit_class_decl(&mut self, class_decl: &mut ClassDecl, _p: &mut ()) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        let name_index =
            constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(&class_decl.name.name)));
        self.file
            .declarations_mut()
            .push(Box::new(decls::ClassDecl::new(name_index)));
        None
    }

    fn visit_func_decl(&mut self, func_decl: &mut FuncDecl, p: &mut ()) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        let name_index =
            constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(&func_decl.name.name)));
        self.file
            .declarations_mut()
            .push(Box::new(decls::FuncDecl::new(name_index)));

        self.default_visit_func_decl(func_decl, p)
    }

    fn visit_print_stmt(&mut self, print_stmt: &mut PrintStmt, p: &mut ()) -> Option<()> {
        self.visit_expr(&mut print_stmt.expr, p);
        let code = self.code.as_mut().unwrap();
        code.push_insns_0(OpCode::Print);
        None
    }

    fn visit_binary_expr(&mut self, binary_expr: &mut BinaryExpr, p: &mut ()) -> Option<()> {
        self.visit_expr(&mut binary_expr.left, p);
        self.visit_expr(&mut binary_expr.right, p);

        let code = self.code.as_mut().unwrap();
        let opcode = match binary_expr.op {
            BinaryOp::Plus => OpCode::Add,
            BinaryOp::Minus => OpCode::Sub,
            BinaryOp::Mult => OpCode::Mult,
            BinaryOp::Div => OpCode::Div,
            _ => panic!("Unsupported binary operator: {}", binary_expr.op.sym()),
        };

        code.push_insns_0(opcode);

        None
    }

    fn visit_identifier_expr(
        &mut self,
        identifier: &mut IdentifierExpr,
        _p: &mut (),
    ) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(&identifier.name)));
        None
    }

    fn visit_literal_expr(&mut self, literal: &mut LiteralExpr, _p: &mut ()) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        let code = self.code.as_mut().unwrap();
        match literal {
            LiteralExpr::Nil(_) => {}
            LiteralExpr::Bool((boo, _)) => {
                code.push_insns_0(if *boo { OpCode::BPush1 } else { OpCode::BPush0 });
            }
            LiteralExpr::Number((num, _)) => {
                let idx = constant_pool.push(ConstantEntry::Number(NumberInfo::from(num.deref())));
                code.push_insns_1_16(OpCode::Ldc, idx);
            }
            LiteralExpr::String((str, _)) => {
                let str = &str[1..str.len() - 1]; // remove double quotes
                let idx = constant_pool.push_str(str);
                code.push_insns_1_16(OpCode::Ldc, idx);
            }
        }

        None
    }
}
