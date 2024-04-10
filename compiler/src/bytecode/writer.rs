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

use util::matches_any;

use crate::ast;
use crate::ast::ASTVisitor;
use crate::ast::BinaryExpr;
use crate::ast::BinaryOp;
use crate::ast::FuncDecl;
use crate::ast::PrimaryExpr;
use crate::ast::PrintStmt;
use crate::ast::Program;
use crate::ast::Visitable;
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
        program.accept(&mut fpv, &());
    }
}

struct CodeGen<'a> {
    file: &'a mut YKBFile,
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
    fn visit_program(&mut self, program: &Program, p: &()) -> Option<()> {
        if self
            .file
            .attributes()
            .iter()
            .find(|attr| attr.name() == attrs::CODE)
            .is_some()
        {
            panic!("A YKBFile cannot have multiple Code attributes")
        }

        self.code = Some(attrs::Code::new());

        self.default_visit_program(program, p, true, false);
        for stmt in &program.stmts {
            self.visit_stmt(&stmt.0, p);
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

    fn visit_class_decl(&mut self, class_decl: &ast::ClassDecl, _p: &()) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        let name_index =
            constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(&class_decl.name.0)));
        self.file
            .declarations_mut()
            .push(Box::new(decls::ClassDecl::new(name_index)));
        None
    }

    fn visit_func_decl(&mut self, func_decl: &FuncDecl, p: &()) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        let name_index = constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(&func_decl.name.0)));
        self.file
            .declarations_mut()
            .push(Box::new(decls::FuncDecl::new(name_index)));

        self.default_visit_func_decl(func_decl, p)
    }

    fn visit_print_stmt(&mut self, print_stmt: &PrintStmt, p: &()) -> Option<()> {
        self.visit_expr(&print_stmt.expr.0, p);
        let code = self.code.as_mut().unwrap();
        code.push_insns_0(OpCode::Print);
        None
    }

    fn visit_binary_expr(&mut self, binary_expr: &BinaryExpr, p: &()) -> Option<()> {
        if self.features.const_folding
            && matches_any!(
                &binary_expr.op,
                BinaryOp::Plus | BinaryOp::Minus | BinaryOp::Mult | BinaryOp::Div
            )
        {
            if let Some((l, r)) = binary_expr.get_num_operands() {
                let constant_pool = self.file.constant_pool_mut();
                let index = constant_pool.push(ConstantEntry::Number(NumberInfo::from(
                    &apply_binary_op(&binary_expr.op, l, r),
                )));
                let code = self.code.as_mut().unwrap();
                code.push_insns_1_16(OpCode::Ldc, index);
                return None;
            }
        }

        self.visit_expr(&binary_expr.left.0, p);
        self.visit_expr(&binary_expr.right.0, p);

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

    fn visit_primary_expr(&mut self, _primary_expr: &PrimaryExpr, p: &()) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        let code = self.code.as_mut().unwrap();
        let _: () = match _primary_expr {
            PrimaryExpr::Number(num) => {
                let idx = constant_pool.push(ConstantEntry::Number(NumberInfo::from(num)));
                code.push_insns_1_16(OpCode::Ldc, idx);
            }
            PrimaryExpr::String(str) => {
                let str = &str[1..str.len() - 1]; // remove double quotes
                let idx = constant_pool.push_str(str);
                code.push_insns_1_16(OpCode::Ldc, idx);
            }
            PrimaryExpr::False => code.push_insns_0(OpCode::BPush0),
            PrimaryExpr::True => code.push_insns_0(OpCode::BPush1),
            PrimaryExpr::Identifier(ident) => {
                constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(ident)));
            }
            _ => return self.default_visit_primary_expr(_primary_expr, p),
        };

        None
    }
}

fn apply_binary_op(op: &BinaryOp, l: &f64, r: &f64) -> f64 {
    match op {
        BinaryOp::Plus => l + r,
        BinaryOp::Minus => l - r,
        BinaryOp::Mult => l * r,
        BinaryOp::Div => l / r,
        _ => panic!("Unsupported binary operator: {}", op.sym()),
    }
}
