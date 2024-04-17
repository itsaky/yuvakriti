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

use std::fmt::Write;
use std::ops::Add;

use crate::ast::visitor::ASTVisitor;
use crate::ast::ClassDecl;
use crate::ast::Decl;
use crate::ast::Expr;
use crate::ast::ExprStmt;
use crate::ast::ForStmt;
use crate::ast::FuncCallExpr;
use crate::ast::FuncDecl;
use crate::ast::IfStmt;
use crate::ast::MemberAccessExpr;
use crate::ast::PrintStmt;
use crate::ast::Program;
use crate::ast::ReturnStmt;
use crate::ast::Stmt;
use crate::ast::VarStmt;
use crate::ast::WhileStmt;
use crate::ast::{AssignExpr, UnaryExpr};
use crate::ast::{BinaryExpr, IdentifierExpr};
use crate::ast::{BlockStmt, LiteralExpr};

pub struct ASTPrinter<'a> {
    f: &'a mut dyn Write,
    pretty: bool,
}

impl<'a> ASTPrinter<'a> {
    pub fn new(f: &'a mut dyn Write, pretty: bool) -> Self {
        ASTPrinter { f, pretty }
    }

    fn indent(&mut self, indent_level: &usize) {
        if !self.pretty {
            return;
        }

        if indent_level == &0 {
            return;
        }

        for _ in 0..*indent_level {
            self.f.write_str("  ").unwrap();
        }
    }

    fn whitespace(&mut self) {
        self.f.write_char(' ').unwrap();
    }

    fn linefeed(&mut self, indent_level: &usize) {
        if !self.pretty {
            return;
        }

        self.f.write_char('\n').unwrap();
        self.indent(indent_level);
    }

    fn print_expr(&mut self, expr: &mut Expr, indent_level: &mut usize) {
        self.f.write_str("(").unwrap();
        match expr {
            Expr::Assign(assign_expr) => {
                self.f.write_str("assign ").unwrap();
                self.visit_assign_expr(assign_expr, indent_level);
            }
            Expr::Binary(binary_expr) => {
                self.f.write_str("binary ").unwrap();
                self.visit_binary_expr(binary_expr, indent_level);
            }
            Expr::Unary(unary_expr) => {
                self.f.write_str("unary ").unwrap();
                self.visit_unary_expr(unary_expr, indent_level);
            }
            Expr::FuncCall(func_call_expr) => {
                self.f.write_str("call ").unwrap();
                self.visit_func_call_expr(func_call_expr, indent_level);
            }
            Expr::MemberAccess(member_access_expr) => {
                self.f.write_str("member ").unwrap();
                self.visit_member_access_expr(member_access_expr, indent_level);
            }
            Expr::Identifier(ident) => {
                self.f.write_str("identifier ").unwrap();
                self.visit_identifier_expr(ident, indent_level);
            }
            Expr::Literal(literal) => {
                self.f.write_str("literal ").unwrap();
                self.visit_literal_expr(literal, indent_level);
            }
        }
        self.f.write_str(")").unwrap();
    }

    fn print_decl(&mut self, decl: &mut Decl, indent_level: &mut usize) {
        self.f.write_str("(decl ").unwrap();
        match decl {
            Decl::Class(class_decl) => {
                self.f.write_str("class ").unwrap();
                self.visit_class_decl(class_decl, indent_level);
            }
            Decl::Func(func_decl) => {
                self.f.write_str("fun ").unwrap();
                self.visit_func_decl(func_decl, indent_level);
            }
            Decl::Stmt(stmt) => {
                self.print_stmt(stmt, indent_level);
            }
        }
        self.f.write_str(")").unwrap();
        self.linefeed(indent_level);
    }

    fn print_stmt(&mut self, stmt: &mut Stmt, indent_level: &mut usize) {
        self.f.write_str("(stmt ").unwrap();
        match stmt {
            Stmt::Expr(expr_stmt) => {
                self.f.write_str("expr ").unwrap();
                self.visit_expr_stmt(expr_stmt, indent_level);
            }
            Stmt::For(for_stmt) => {
                self.f.write_str("for ").unwrap();
                self.visit_for_stmt(for_stmt, indent_level);
            }
            Stmt::If(if_stmt) => {
                self.f.write_str("if ").unwrap();
                self.visit_if_stmt(if_stmt, indent_level);
            }
            Stmt::Print(print_stmt) => {
                self.f.write_str("print ").unwrap();
                self.visit_print_stmt(print_stmt, indent_level);
            }
            Stmt::Return(return_stmt) => {
                self.f.write_str("return ").unwrap();
                self.visit_return_stmt(return_stmt, indent_level);
            }
            Stmt::While(while_stmt) => {
                self.f.write_str("while ").unwrap();
                self.visit_while_stmt(while_stmt, indent_level);
            }
            Stmt::Var(var_decl) => {
                self.f.write_str("var ").unwrap();
                self.visit_var_stmt(var_decl, indent_level);
            }
            Stmt::Block(block_stmt) => {
                self.f.write_str("block ").unwrap();
                self.visit_block_stmt(block_stmt, indent_level);
            }
        }
        self.f.write_str(")").unwrap();
    }
}

impl<'a> ASTVisitor<usize, ()> for ASTPrinter<'a> {
    fn visit_program(&mut self, program: &mut Program, indent_level: &mut usize) -> Option<()> {
        self.f.write_str("(program").unwrap();
        self.linefeed(&mut indent_level.add(1));
        for i in 0..program.decls.len() {
            self.print_decl(&mut program.decls[i], &mut indent_level.add(1));
        }
        for i in 0..program.stmts.len() {
            self.print_stmt(&mut program.stmts[i], &mut indent_level.add(1));
            self.linefeed(&mut indent_level.add(1));
        }
        self.indent(indent_level);
        self.f.write_str(")").unwrap();
        None
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt, p: &mut usize) -> Option<()> {
        self.print_stmt(stmt, p);
        None
    }

    fn visit_class_decl(
        &mut self,
        class_decl: &mut ClassDecl,
        indent_level: &mut usize,
    ) -> Option<()> {
        self.f.write_str(&class_decl.name.name).unwrap();
        if let Some(supercls) = &class_decl.supercls {
            self.f.write_str(" : ").unwrap();
            self.f.write_str(&supercls.name).unwrap();
        }
        self.f.write_str(" {").unwrap();
        for i in 0..class_decl.methods.len() {
            let method = &mut class_decl.methods[i];
            self.visit_func_decl(method, &mut indent_level.add(1));
        }
        self.f.write_str("}").unwrap();
        None
    }

    fn visit_func_decl(
        &mut self,
        func_decl: &mut FuncDecl,
        indent_level: &mut usize,
    ) -> Option<()> {
        self.f.write_str(&func_decl.name.name).unwrap();
        self.f.write_str("(").unwrap();
        let mut first = true;
        for param in &func_decl.params {
            if !first {
                self.f.write_str(", ").unwrap();
            }
            first = false;
            self.f.write_str(&param.name).unwrap();
        }
        self.f.write_str(") ").unwrap();
        self.visit_block_stmt(&mut func_decl.body, &mut indent_level.add(1));
        None
    }

    fn visit_var_stmt(&mut self, var_decl: &mut VarStmt, indent_level: &mut usize) -> Option<()> {
        self.f.write_str(&var_decl.name.name).unwrap();
        if let Some(initializer) = var_decl.initializer.as_mut() {
            self.f.write_str(" = ").unwrap();
            self.print_expr(initializer, indent_level);
        }
        None
    }

    fn visit_block_stmt(
        &mut self,
        block_stmt: &mut BlockStmt,
        indent_level: &mut usize,
    ) -> Option<()> {
        self.f.write_str("{").unwrap();
        self.linefeed(&mut indent_level.add(1));
        for i in 0..block_stmt.decls.len() {
            let decl = block_stmt.decls.get_mut(i).unwrap();
            self.visit_decl(decl, &mut indent_level.add(1));
        }
        self.f.write_str("}").unwrap();
        None
    }

    fn visit_expr_stmt(
        &mut self,
        expr_stmt: &mut ExprStmt,
        indent_level: &mut usize,
    ) -> Option<()> {
        self.print_expr(&mut expr_stmt.expr, indent_level);
        None
    }

    fn visit_for_stmt(&mut self, for_stmt: &mut ForStmt, indent_level: &mut usize) -> Option<()> {
        self.f.write_str("(").unwrap();
        if let Some(init) = for_stmt.init.as_mut() {
            self.visit_stmt(init, &mut indent_level.add(1));
        }
        self.f.write_str("; ").unwrap();
        if let Some(condition) = for_stmt.condition.as_mut() {
            self.visit_expr(condition, &mut indent_level.add(1));
        }
        self.f.write_str("; ").unwrap();
        if let Some(step) = for_stmt.step.as_mut() {
            self.visit_expr(step, &mut indent_level.add(1));
        }
        self.f.write_str(") ").unwrap();
        self.visit_block_stmt(&mut for_stmt.body, &mut indent_level.add(1));
        None
    }

    fn visit_if_stmt(&mut self, if_stmt: &mut IfStmt, indent_level: &mut usize) -> Option<()> {
        self.print_expr(&mut if_stmt.condition, indent_level);
        self.f.write_str(" ").unwrap();
        self.visit_block_stmt(&mut if_stmt.then_branch, &mut indent_level.add(1));
        if let Some(else_branch) = if_stmt.else_branch.as_mut() {
            self.f.write_str(" else ").unwrap();
            self.visit_block_stmt(else_branch, &mut indent_level.add(1));
        }
        None
    }

    fn visit_print_stmt(
        &mut self,
        print_stmt: &mut PrintStmt,
        _indent_level: &mut usize,
    ) -> Option<()> {
        self.whitespace();
        self.print_expr(&mut print_stmt.expr, &mut 0);
        None
    }

    fn visit_return_stmt(
        &mut self,
        return_stmt: &mut ReturnStmt,
        indent_level: &mut usize,
    ) -> Option<()> {
        self.print_expr(&mut return_stmt.expr, indent_level);
        None
    }

    fn visit_while_stmt(
        &mut self,
        while_stmt: &mut WhileStmt,
        indent_level: &mut usize,
    ) -> Option<()> {
        self.print_expr(&mut while_stmt.condition, indent_level);
        self.f.write_str(" ").unwrap();
        self.visit_block_stmt(&mut while_stmt.body, &mut indent_level.add(1));
        None
    }

    fn visit_assign_expr(
        &mut self,
        assign_expr: &mut AssignExpr,
        indent_level: &mut usize,
    ) -> Option<()> {
        self.print_expr(&mut assign_expr.target, &mut indent_level.add(1));
        self.f.write_str(" = ").unwrap();
        self.print_expr(&mut assign_expr.value, &mut indent_level.add(1));
        None
    }

    fn visit_binary_expr(
        &mut self,
        binary_expr: &mut BinaryExpr,
        indent_level: &mut usize,
    ) -> Option<()> {
        self.f.write_str(&format!("{:?} ", binary_expr.op)).unwrap();
        self.print_expr(&mut binary_expr.left, &mut indent_level.add(1));
        self.print_expr(&mut binary_expr.right, &mut indent_level.add(1));
        None
    }

    fn visit_unary_expr(
        &mut self,
        unary_expr: &mut UnaryExpr,
        indent_level: &mut usize,
    ) -> Option<()> {
        self.f.write_str(&format!("{:?} ", unary_expr.op)).unwrap();
        self.print_expr(&mut unary_expr.expr, &mut indent_level.add(1));
        None
    }

    fn visit_func_call_expr(
        &mut self,
        func_call_expr: &mut FuncCallExpr,
        indent_level: &mut usize,
    ) -> Option<()> {
        self.visit_expr(&mut func_call_expr.callee, &mut indent_level.add(1));
        self.f.write_str("(").unwrap();
        let mut first = true;
        for i in 0..func_call_expr.args.len() {
            let arg = func_call_expr.args.get_mut(i).unwrap();
            if !first {
                self.f.write_str(", ").unwrap();
            }
            first = false;
            self.print_expr(arg, indent_level);
        }
        self.f.write_str(")").unwrap();
        None
    }

    fn visit_member_access_expr(
        &mut self,
        member_access_expr: &mut MemberAccessExpr,
        indent_level: &mut usize,
    ) -> Option<()> {
        self.print_expr(&mut member_access_expr.receiver, indent_level);
        self.f.write_str(".").unwrap();
        self.f.write_str(&member_access_expr.member.name).unwrap();
        None
    }

    fn visit_identifier_expr(
        &mut self,
        identifier: &mut IdentifierExpr,
        _p: &mut usize,
    ) -> Option<()> {
        self.f.write_str(&identifier.name).unwrap();
        None
    }

    fn visit_literal_expr(&mut self, literal: &mut LiteralExpr, _p: &mut usize) -> Option<()> {
        self.f
            .write_str(&format!(
                "{}",
                match literal {
                    LiteralExpr::String((str, _)) => str.to_owned(),
                    LiteralExpr::Nil(_) => "nil".to_string(),
                    LiteralExpr::Bool((b, _)) => b.to_string(),
                    LiteralExpr::Number((n, _)) => n.to_string(),
                }
            ))
            .unwrap();

        None
    }
}
