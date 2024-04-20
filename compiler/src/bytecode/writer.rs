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

use crate::ast::BinaryOp;
use crate::ast::BlockStmt;
use crate::ast::ClassDecl;
use crate::ast::FuncDecl;
use crate::ast::IdentifierExpr;
use crate::ast::LiteralExpr;
use crate::ast::PrintStmt;
use crate::ast::Program;
use crate::ast::VarStmt;
use crate::ast::Visitable;
use crate::ast::{ASTVisitor, IdentifierType};
use crate::ast::{AssignExpr, BinaryExpr};
use crate::bytecode::attrs;
use crate::bytecode::cp::ConstantEntry;
use crate::bytecode::cp_info::NumberInfo;
use crate::bytecode::cp_info::Utf8Info;
use crate::bytecode::decls;
use crate::bytecode::file::YKBFile;
use crate::bytecode::opcode::OpCode;
use crate::features::CompilerFeatures;
use crate::messages;
use crate::scope::Scope;
use crate::symtab::VarSym;

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
        program.accept(&mut fpv, &mut Scope::new());
    }
}

struct CodeGen<'a> {
    file: &'a mut YKBFile,
    #[allow(unused)]
    features: &'a CompilerFeatures,
    scope: Option<Scope<'a>>,
    code: Option<attrs::Code>,
}

impl<'a> CodeGen<'a> {
    fn new(file: &'a mut YKBFile, features: &'a CompilerFeatures) -> Self {
        return CodeGen {
            file,
            features,
            scope: None,
            code: None,
        };
    }
}

impl ASTVisitor<Scope<'_>, ()> for CodeGen<'_> {
    fn visit_program(&mut self, program: &mut Program, scope: &mut Scope) -> Option<()> {
        if self
            .file
            .attributes()
            .iter()
            .find(|attr| attr.name() == attrs::CODE)
            .is_some()
        {
            panic!("A YKBFile cannot have multiple Code attributes")
        }

        self.scope = Some(Scope::new());
        self.code = Some(attrs::Code::new(0, 0, 0));

        self.default_visit_program(program, scope, true, false);
        for i in 0..program.stmts.len() {
            let stmt = program.stmts.get_mut(i).unwrap();
            self.visit_stmt(stmt, scope);
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
        self.scope = None;
        None
    }

    fn visit_class_decl(&mut self, class_decl: &mut ClassDecl, _scope: &mut Scope) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        let name_index =
            constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(&class_decl.name.name)));
        self.file
            .declarations_mut()
            .push(Box::new(decls::ClassDecl::new(name_index)));
        None
    }

    fn visit_func_decl(&mut self, func_decl: &mut FuncDecl, scope: &mut Scope) -> Option<()> {
        let constant_pool = self.file.constant_pool_mut();
        let name_index =
            constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(&func_decl.name.name)));
        self.file
            .declarations_mut()
            .push(Box::new(decls::FuncDecl::new(name_index)));

        self.default_visit_func_decl(func_decl, scope)
    }

    fn visit_var_stmt(&mut self, var_decl: &mut VarStmt, scope: &mut Scope<'_>) -> Option<()> {
        // Visit the initializer first, if there is one
        // this ensures that the result of the initializer is pushed onto the stack before
        // the variable is declared
        if let Some(expr) = var_decl.initializer.as_mut() {
            self.visit_expr(expr, scope);
        }

        let code = self
            .code
            .as_mut()
            .expect("Code must be set before visiting a VarStmt");
        let var_name = &var_decl.name.name.clone();
        let var_idx = match scope.push_var(VarSym::new(var_name.clone())) {
            // This duplicate variable error must have been handled during the attribution phase
            // if it wansn't somehow reported at that point, then we panic
            Err(_) => panic!("{}", &messages::err_dup_var(&var_name)),
            Ok(index) => index,
        };

        let opcode = match var_idx {
            0 => OpCode::Store0,
            1 => OpCode::Store1,
            2 => OpCode::Store2,
            3 => OpCode::Store3,
            _ => OpCode::Store,
        };

        if opcode != OpCode::Store {
            code.push_insns_0(opcode);
        } else {
            code.push_insns_1_16(opcode, var_idx);
        }

        // Update the max locals to account for the new variable
        code.update_max_locals(1);

        None
    }

    fn visit_assign_expr(
        &mut self,
        assign_expr: &mut AssignExpr,
        scope: &mut Scope<'_>,
    ) -> Option<()> {
        if let Some(identifier) = assign_expr.target.Identifier() {
            self.visit_expr(&mut assign_expr.value, scope);

            if let Some(idx) = scope.get_var_idx(&identifier.name) {
                let code = self
                    .code
                    .as_mut()
                    .expect("Code must be set before visiting an AssignExpr");
                let opcode = match &idx {
                    0 => OpCode::Store0,
                    1 => OpCode::Store1,
                    2 => OpCode::Store2,
                    3 => OpCode::Store3,
                    _ => OpCode::Store,
                };

                if opcode != OpCode::Store {
                    code.push_insns_0(opcode);
                } else {
                    code.push_insns_1_16(opcode, idx.clone());
                }

                return None;
            }
        }

        // TODO: Support more assign expressions
        panic!("Unsupported assign expr: {:?}", assign_expr);
    }

    fn visit_block_stmt(&mut self, block_stmt: &mut BlockStmt, scope: &mut Scope) -> Option<()> {
        let mut new = Scope::new();
        new.parent = Some(&scope);
        self.default_visit_block_stmt(block_stmt, &mut new)
    }

    fn visit_print_stmt(&mut self, print_stmt: &mut PrintStmt, scope: &mut Scope) -> Option<()> {
        self.visit_expr(&mut print_stmt.expr, scope);
        let code = self.code.as_mut().unwrap();
        code.push_insns_0(OpCode::Print);
        None
    }

    fn visit_binary_expr(&mut self, binary_expr: &mut BinaryExpr, scope: &mut Scope) -> Option<()> {
        self.visit_expr(&mut binary_expr.left, scope);
        self.visit_expr(&mut binary_expr.right, scope);

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
        scope: &mut Scope,
    ) -> Option<()> {
        let typ = identifier.ident_typ();

        if typ == &IdentifierType::ClassName {
            let constant_pool = self.file.constant_pool_mut();
            constant_pool.push(ConstantEntry::Utf8(Utf8Info::from(&identifier.name)));
        }

        if !typ.is_decl_name() && typ != &IdentifierType::Keyword {
            if let Some(idx) = scope.get_var_idx(&identifier.name) {
                let code = self.code.as_mut().unwrap();
                let opcode = match idx {
                    0 => OpCode::Load0,
                    1 => OpCode::Load1,
                    2 => OpCode::Load2,
                    3 => OpCode::Load3,
                    _ => OpCode::Load,
                };
                if opcode != OpCode::Load {
                    code.push_insns_0(opcode);
                } else {
                    code.push_insns_1_16(opcode, idx.clone());
                }
            } else {
                panic!("Variable not found: {}", &identifier.name);
            }
        }

        None
    }

    fn visit_literal_expr(&mut self, literal: &mut LiteralExpr, _scope: &mut Scope) -> Option<()> {
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
