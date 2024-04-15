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

use paste::paste;

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
use crate::ast::UnaryExpr;
use crate::ast::VarStmt;
use crate::ast::WhileStmt;
use crate::ast::{AssignExpr, Visitable};
use crate::ast::{BinaryExpr, IdentifierExpr};
use crate::ast::{BlockStmt, GroupingExpr, LiteralExpr};

/// ASTVisitor for visiting AST nodes. Methods in the visitor result an [Option<R>]. If the result
/// is [Some], then the child nodes of the AST node will not be visited.
pub trait ASTVisitor<P, R> {
    fn visit_program(&mut self, program: &Program, p: &mut P) -> Option<R> {
        self.default_visit_program(program, p, true, true)
    }

    fn default_visit_program(
        &mut self,
        program: &Program,
        p: &mut P,
        visit_decls: bool,
        visit_stmts: bool,
    ) -> Option<R> {
        let mut r: Option<R> = None;
        if visit_decls {
            for decl in &program.decls {
                r = self.visit_decl(&decl, p);
                if r.is_some() {
                    return r;
                }
            }
        }

        if visit_stmts {
            for stmt in &program.stmts {
                r = self.visit_stmt(&stmt, p);
                if r.is_some() {
                    return r;
                }
            }
        }

        r
    }

    fn visit_decl(&mut self, decl: &Decl, p: &mut P) -> Option<R> {
        self.default_visit_decl(decl, p)
    }
    fn default_visit_decl(&mut self, decl: &Decl, p: &mut P) -> Option<R> {
        match decl {
            Decl::Class(class_decl) => self.visit_class_decl(&class_decl, p),
            Decl::Func(func_decl) => self.visit_func_decl(&func_decl, p),
            Decl::Stmt(stmt) => self.visit_stmt(stmt, p),
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt, p: &mut P) -> Option<R> {
        self.default_visit_stmt(stmt, p)
    }
    fn default_visit_stmt(&mut self, stmt: &Stmt, p: &mut P) -> Option<R> {
        match stmt {
            Stmt::Expr(expr_stmt) => self.visit_expr_stmt(&expr_stmt, p),
            Stmt::For(for_stmt) => self.visit_for_stmt(&for_stmt, p),
            Stmt::If(if_stmt) => self.visit_if_stmt(&if_stmt, p),
            Stmt::Print(print_stmt) => self.visit_print_stmt(&print_stmt, p),
            Stmt::Return(return_stmt) => self.visit_return_stmt(&return_stmt, p),
            Stmt::While(while_stmt) => self.visit_while_stmt(&while_stmt, p),
            Stmt::Var(var_decl) => self.visit_var_stmt(&var_decl, p),
            Stmt::Block(block_stmt) => self.visit_block_stmt(&block_stmt, p),
        }
    }

    fn visit_expr(&mut self, expr: &Expr, p: &mut P) -> Option<R> {
        self.default_visit_expr(expr, p)
    }
    fn default_visit_expr(&mut self, expr: &Expr, p: &mut P) -> Option<R> {
        match expr {
            Expr::Assign(expr) => self.visit_assign_expr(&expr, p),
            Expr::Binary(binary_expr) => self.visit_binary_expr(&binary_expr, p),
            Expr::Unary(unary_expr) => self.visit_unary_expr(&unary_expr, p),
            Expr::FuncCall(func_call_expr) => self.visit_func_call_expr(&func_call_expr, p),
            Expr::MemberAccess(member_access_expr) => {
                self.visit_member_access_expr(&member_access_expr, p)
            }
            Expr::Identifier(exp) => self.visit_identifier_expr(&exp, p),
            Expr::Literal(exp) => self.visit_literal_expr(&exp, p),
            Expr::Grouping(exp) => self.visit_grouping_expr(&exp, p),
        }
    }

    fn visit_class_decl(&mut self, class_decl: &ClassDecl, p: &mut P) -> Option<R> {
        self.default_visit_class_decl(class_decl, p)
    }

    fn default_visit_class_decl(&mut self, class_decl: &ClassDecl, p: &mut P) -> Option<R> {
        let mut r: Option<R> = self.visit_identifier_expr(&class_decl.name, p);
        if r.is_some() {
            return r;
        }

        if let Some(supercls) = &class_decl.supercls {
            r = self.visit_identifier_expr(supercls, p);
            if r.is_some() {
                return r;
            }
        }

        for method in &class_decl.methods {
            r = self.visit_func_decl(method, p);

            if r.is_some() {
                break;
            }
        }

        r
    }

    fn visit_func_decl(&mut self, func_decl: &FuncDecl, p: &mut P) -> Option<R> {
        self.default_visit_func_decl(func_decl, p)
    }
    fn default_visit_func_decl(&mut self, func_decl: &FuncDecl, p: &mut P) -> Option<R> {
        let mut r = self.visit_identifier_expr(&func_decl.name, p);
        if r.is_some() {
            return r;
        }

        for param in &func_decl.params {
            r = self.visit_identifier_expr(&param, p);
            if r.is_some() {
                return r;
            }
        }

        self.visit_block_stmt(&func_decl.body, p)
    }

    fn visit_var_stmt(&mut self, var_decl: &VarStmt, p: &mut P) -> Option<R> {
        self.default_visit_var_stmt(var_decl, p)
    }
    fn default_visit_var_stmt(&mut self, var_decl: &VarStmt, p: &mut P) -> Option<R> {
        let r = self.visit_identifier_expr(&var_decl.name, p);
        if r.is_some() {
            return r;
        }

        if let Some(initializer) = &var_decl.initializer {
            return self.visit_expr(initializer, p);
        }

        None
    }

    fn visit_block_stmt(&mut self, block_stmt: &BlockStmt, p: &mut P) -> Option<R> {
        self.default_visit_block_stmt(block_stmt, p)
    }
    fn default_visit_block_stmt(&mut self, block_stmt: &BlockStmt, p: &mut P) -> Option<R> {
        let mut r: Option<R> = None;
        for decl in &block_stmt.decls {
            r = self.visit_decl(&decl, p);

            if r.is_some() {
                break;
            }
        }

        r
    }

    fn visit_expr_stmt(&mut self, expr_stmt: &ExprStmt, p: &mut P) -> Option<R> {
        self.default_visit_expr_stmt(expr_stmt, p)
    }
    fn default_visit_expr_stmt(&mut self, expr_stmt: &ExprStmt, p: &mut P) -> Option<R> {
        self.visit_expr(&expr_stmt.expr, p)
    }

    fn visit_for_stmt(&mut self, for_stmt: &ForStmt, p: &mut P) -> Option<R> {
        self.default_visit_for_stmt(for_stmt, p)
    }
    fn default_visit_for_stmt(&mut self, for_stmt: &ForStmt, p: &mut P) -> Option<R> {
        let mut r: Option<R> = None;
        if let Some(init) = &for_stmt.init {
            r = self.visit_stmt(&init, p);
        }

        if r.is_some() {
            return r;
        }

        if let Some(cond) = &for_stmt.condition {
            r = self.visit_expr(cond, p);
        }

        if r.is_some() {
            return r;
        }

        if let Some(step) = &for_stmt.step {
            r = self.visit_expr(step, p);
        }

        if r.is_some() {
            return r;
        }

        self.visit_block_stmt(&for_stmt.body, p)
    }

    fn visit_if_stmt(&mut self, if_stmt: &IfStmt, p: &mut P) -> Option<R> {
        self.default_visit_if_stmt(if_stmt, p)
    }
    fn default_visit_if_stmt(&mut self, if_stmt: &IfStmt, p: &mut P) -> Option<R> {
        let mut r = self.visit_expr(&if_stmt.condition, p);
        if r.is_some() {
            return r;
        }

        r = self.visit_block_stmt(&if_stmt.then_branch, p);
        if r.is_some() {
            return r;
        }

        if let Some(block) = &if_stmt.else_branch {
            r = self.visit_block_stmt(&block, p);
        }

        r
    }

    fn visit_print_stmt(&mut self, print_stmt: &PrintStmt, p: &mut P) -> Option<R> {
        self.default_visit_print_stmt(print_stmt, p)
    }
    fn default_visit_print_stmt(&mut self, print_stmt: &PrintStmt, p: &mut P) -> Option<R> {
        self.visit_expr(&print_stmt.expr, p)
    }

    fn visit_return_stmt(&mut self, return_stmt: &ReturnStmt, p: &mut P) -> Option<R> {
        self.default_visit_return_stmt(return_stmt, p)
    }
    fn default_visit_return_stmt(&mut self, return_stmt: &ReturnStmt, p: &mut P) -> Option<R> {
        self.visit_expr(&return_stmt.expr, p)
    }

    fn visit_while_stmt(&mut self, while_stmt: &WhileStmt, p: &mut P) -> Option<R> {
        self.default_visit_while_stmt(while_stmt, p)
    }
    fn default_visit_while_stmt(&mut self, while_stmt: &WhileStmt, p: &mut P) -> Option<R> {
        let r = self.visit_expr(&while_stmt.condition, p);
        if r.is_some() {
            return r;
        }
        self.visit_block_stmt(&while_stmt.body, p)
    }

    fn visit_assign_expr(&mut self, assign_expr: &AssignExpr, p: &mut P) -> Option<R> {
        self.default_visit_assign_expr(assign_expr, p)
    }
    fn default_visit_assign_expr(&mut self, assign_expr: &AssignExpr, p: &mut P) -> Option<R> {
        self.visit_expr(&assign_expr.value, p)
    }

    fn visit_binary_expr(&mut self, binary_expr: &BinaryExpr, p: &mut P) -> Option<R> {
        self.default_visit_binary_expr(binary_expr, p)
    }
    fn default_visit_binary_expr(&mut self, binary_expr: &BinaryExpr, p: &mut P) -> Option<R> {
        let r = self.visit_expr(&binary_expr.left, p);
        if r.is_some() {
            return r;
        }
        self.visit_expr(&binary_expr.right, p)
    }

    fn visit_unary_expr(&mut self, unary_expr: &UnaryExpr, p: &mut P) -> Option<R> {
        self.default_visit_unary_expr(unary_expr, p)
    }
    fn default_visit_unary_expr(&mut self, unary_expr: &UnaryExpr, p: &mut P) -> Option<R> {
        self.visit_expr(&unary_expr.expr, p)
    }

    fn visit_func_call_expr(&mut self, func_call_expr: &FuncCallExpr, p: &mut P) -> Option<R> {
        self.default_visit_func_call_expr(func_call_expr, p)
    }
    fn default_visit_func_call_expr(
        &mut self,
        func_call_expr: &FuncCallExpr,
        p: &mut P,
    ) -> Option<R> {
        let mut r = self.visit_expr(&func_call_expr.callee, p);
        if r.is_some() {
            return r;
        }
        for arg in &func_call_expr.args {
            r = self.visit_expr(arg, p);
            if r.is_some() {
                return r;
            }
        }
        None
    }

    fn visit_member_access_expr(
        &mut self,
        member_access_expr: &MemberAccessExpr,
        p: &mut P,
    ) -> Option<R> {
        self.default_visit_member_access_expr(member_access_expr, p)
    }
    fn default_visit_member_access_expr(
        &mut self,
        member_access_expr: &MemberAccessExpr,
        p: &mut P,
    ) -> Option<R> {
        let r = self.visit_identifier_expr(&member_access_expr.member, p);
        if r.is_some() {
            return r;
        }

        self.visit_expr(&member_access_expr.receiver, p)
    }

    fn visit_grouping_expr(&mut self, grouping: &GroupingExpr, _p: &mut P) -> Option<R> {
        self.default_visit_grouping(grouping, _p)
    }
    fn default_visit_grouping(&mut self, grouping: &GroupingExpr, _p: &mut P) -> Option<R> {
        self.visit_expr(&grouping.expr, _p)
    }

    fn visit_identifier_expr(&mut self, identifier: &IdentifierExpr, _p: &mut P) -> Option<R> {
        self.default_visit_identifier(identifier, _p)
    }
    fn default_visit_identifier(&mut self, _identifier: &IdentifierExpr, _p: &mut P) -> Option<R> {
        None
    }

    fn visit_literal_expr(&mut self, literal: &LiteralExpr, _p: &mut P) -> Option<R> {
        self.default_visit_literal(literal, _p)
    }
    fn default_visit_literal(&mut self, _literal_expr: &LiteralExpr, _p: &mut P) -> Option<R> {
        None
    }
}

macro_rules! impl_visitable {
    ($name:ty) => {
        paste! {
            impl Visitable for $name {
                fn accept<P, R>(&self, visitor: &mut (impl ASTVisitor<P, R> + ?Sized), p: &mut P) -> Option<R> {
                    visitor.[< visit_ $name:snake>](self, p)
                }
            }
        }
    };
}

macro_rules! impl_visitables {
    ($($name:ty $(,)?)+) => {
        $(impl_visitable!($name);)+
    };
}

impl_visitables!(
    Program,
    ClassDecl,
    FuncDecl,
    VarStmt,
    BlockStmt,
    ExprStmt,
    ForStmt,
    IfStmt,
    PrintStmt,
    ReturnStmt,
    WhileStmt,
    AssignExpr,
    BinaryExpr,
    UnaryExpr,
    FuncCallExpr,
    MemberAccessExpr,
    GroupingExpr,
    IdentifierExpr,
    LiteralExpr,
);
