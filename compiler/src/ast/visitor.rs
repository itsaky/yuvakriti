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

use paste::paste;

use crate::ast::Decl;
use crate::ast::Expr;
use crate::ast::ExprStmt;
use crate::ast::ForStmt;
use crate::ast::FuncCallExpr;
use crate::ast::FuncDecl;
use crate::ast::GroupingExpr;
use crate::ast::IdentifierExpr;
use crate::ast::IfStmt;
use crate::ast::LiteralExpr;
use crate::ast::MemberAccessExpr;
use crate::ast::PrintStmt;
use crate::ast::Program;
use crate::ast::ReturnStmt;
use crate::ast::Stmt;
use crate::ast::UnaryExpr;
use crate::ast::VarStmt;
use crate::ast::Visitable;
use crate::ast::WhileStmt;
use crate::ast::{ArrayExpr, ClassDecl};
use crate::ast::{AssignExpr, BreakStmt, ContinueStmt};
use crate::ast::{BinaryExpr, CompoundAssignExpr};
use crate::ast::{BlockStmt, EmptyStmt};

/// ASTVisitor for visiting AST nodes. Methods in the visitor result an [Option<R>]. If the result
/// is [Some], then the child nodes of the AST node will not be visited.
pub trait ASTVisitor<P, R> {
    fn visit_program(&mut self, program: &mut Program, p: &mut P) -> Option<R> {
        self.default_visit_program(program, p, true, true)
    }

    fn default_visit_program(
        &mut self,
        program: &mut Program,
        p: &mut P,
        visit_decls: bool,
        visit_stmts: bool,
    ) -> Option<R> {
        let mut r: Option<R> = None;
        if visit_decls {
            for i in 0..program.decls.len() {
                let decl = program.decls.get_mut(i).unwrap();
                r = self.visit_decl(decl, p);
                if r.is_some() {
                    return r;
                }
            }
        }

        if visit_stmts {
            for i in 0..program.stmts.len() {
                let stmt = program.stmts.get_mut(i).unwrap();
                r = self.visit_stmt(stmt, p);
                if r.is_some() {
                    return r;
                }
            }
        }

        r
    }

    fn visit_decl(&mut self, decl: &mut Decl, p: &mut P) -> Option<R> {
        self.default_visit_decl(decl, p)
    }
    fn default_visit_decl(&mut self, decl: &mut Decl, p: &mut P) -> Option<R> {
        match decl {
            Decl::Class(class_decl) => self.visit_class_decl(class_decl, p),
            Decl::Func(func_decl) => self.visit_func_decl(func_decl, p),
            Decl::Stmt(stmt) => self.visit_stmt(stmt, p),
        }
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt, p: &mut P) -> Option<R> {
        self.default_visit_stmt(stmt, p)
    }
    fn default_visit_stmt(&mut self, stmt: &mut Stmt, p: &mut P) -> Option<R> {
        match stmt {
            Stmt::Expr(expr_stmt) => self.visit_expr_stmt(expr_stmt, p),
            Stmt::For(for_stmt) => self.visit_for_stmt(for_stmt, p),
            Stmt::If(if_stmt) => self.visit_if_stmt(if_stmt, p),
            Stmt::Print(print_stmt) => self.visit_print_stmt(print_stmt, p),
            Stmt::Return(return_stmt) => self.visit_return_stmt(return_stmt, p),
            Stmt::While(while_stmt) => self.visit_while_stmt(while_stmt, p),
            Stmt::Var(var_decl) => self.visit_var_stmt(var_decl, p),
            Stmt::Block(block_stmt) => self.visit_block_stmt(block_stmt, p),
            Stmt::Break(br) => self.visit_break_stmt(br, p),
            Stmt::Continue(cont) => self.visit_continue_stmt(cont, p),
            Stmt::Empty(empty) => self.visit_empty_stmt(empty, p),
        }
    }

    fn visit_expr(&mut self, expr: &mut Expr, p: &mut P) -> Option<R> {
        self.default_visit_expr(expr, p)
    }
    fn default_visit_expr(&mut self, expr: &mut Expr, p: &mut P) -> Option<R> {
        match expr {
            Expr::Assign(expr) => self.visit_assign_expr(expr, p),
            Expr::CompoundAssign(expr) => self.visit_compound_assign_expr(expr, p),
            Expr::Binary(bin) => self.visit_binary_expr(bin, p),
            Expr::Unary(un) => self.visit_unary_expr(un, p),
            Expr::FuncCall(func) => self.visit_func_call_expr(func, p),
            Expr::MemberAccess(acc) => self.visit_member_access_expr(acc, p),
            Expr::Identifier(exp) => self.visit_identifier_expr(exp, p),
            Expr::Literal(exp) => self.visit_literal_expr(exp, p),
            Expr::Array(arr) => self.visit_array_expr(arr, p),
        }
    }

    fn visit_class_decl(&mut self, class_decl: &mut ClassDecl, p: &mut P) -> Option<R> {
        self.default_visit_class_decl(class_decl, p)
    }

    fn default_visit_class_decl(&mut self, class_decl: &mut ClassDecl, p: &mut P) -> Option<R> {
        let mut r: Option<R> = self.visit_identifier_expr(&mut class_decl.name, p);
        if r.is_some() {
            return r;
        }

        if let Some(supercls) = class_decl.supercls.as_mut() {
            r = self.visit_identifier_expr(supercls, p);
            if r.is_some() {
                return r;
            }
        }

        for i in 0..class_decl.methods.len() {
            let method = class_decl.methods.get_mut(i).unwrap();
            r = self.visit_func_decl(method, p);

            if r.is_some() {
                break;
            }
        }

        r
    }

    fn visit_func_decl(&mut self, func_decl: &mut FuncDecl, p: &mut P) -> Option<R> {
        self.default_visit_func_decl(func_decl, p)
    }
    fn default_visit_func_decl(&mut self, func_decl: &mut FuncDecl, p: &mut P) -> Option<R> {
        let mut r = self.visit_identifier_expr(&mut func_decl.name, p);
        if r.is_some() {
            return r;
        }

        for i in 0..func_decl.params.len() {
            let param = func_decl.params.get_mut(i).unwrap();
            r = self.visit_identifier_expr(param, p);
            if r.is_some() {
                return r;
            }
        }

        self.visit_block_stmt(&mut func_decl.body, p)
    }

    fn visit_var_stmt(&mut self, var_decl: &mut VarStmt, p: &mut P) -> Option<R> {
        self.default_visit_var_stmt(var_decl, p)
    }
    fn default_visit_var_stmt(&mut self, var_decl: &mut VarStmt, p: &mut P) -> Option<R> {
        let r = self.visit_identifier_expr(&mut var_decl.name, p);
        if r.is_some() {
            return r;
        }

        if let Some(initializer) = var_decl.initializer.as_mut() {
            return self.visit_expr(initializer, p);
        }

        None
    }

    fn visit_block_stmt(&mut self, block_stmt: &mut BlockStmt, p: &mut P) -> Option<R> {
        self.default_visit_block_stmt(block_stmt, p)
    }
    fn default_visit_block_stmt(&mut self, block_stmt: &mut BlockStmt, p: &mut P) -> Option<R> {
        let mut r: Option<R> = None;
        for i in 0..block_stmt.decls.len() {
            let decl = block_stmt.decls.get_mut(i).unwrap();
            r = self.visit_decl(decl, p);

            if r.is_some() {
                break;
            }
        }

        r
    }

    fn visit_expr_stmt(&mut self, expr_stmt: &mut ExprStmt, p: &mut P) -> Option<R> {
        self.default_visit_expr_stmt(expr_stmt, p)
    }
    fn default_visit_expr_stmt(&mut self, expr_stmt: &mut ExprStmt, p: &mut P) -> Option<R> {
        self.visit_expr(&mut expr_stmt.expr, p)
    }

    fn visit_for_stmt(&mut self, for_stmt: &mut ForStmt, p: &mut P) -> Option<R> {
        self.default_visit_for_stmt(for_stmt, p)
    }
    fn default_visit_for_stmt(&mut self, for_stmt: &mut ForStmt, p: &mut P) -> Option<R> {
        let mut r: Option<R> = None;
        if let Some(label) = for_stmt.label.as_mut() {
            r = self.visit_identifier_expr(label, p);
        }

        if r.is_some() {
            return r;
        }

        if let Some(init) = for_stmt.init.as_mut() {
            r = self.visit_stmt(init, p);
        }

        if r.is_some() {
            return r;
        }

        if let Some(cond) = for_stmt.condition.as_mut() {
            r = self.visit_expr(cond, p);
        }

        if r.is_some() {
            return r;
        }

        if let Some(step) = for_stmt.step.as_mut() {
            r = self.visit_expr(step, p);
        }

        if r.is_some() {
            return r;
        }

        self.visit_block_stmt(&mut for_stmt.body, p)
    }

    fn visit_break_stmt(&mut self, break_stmt: &mut BreakStmt, p: &mut P) -> Option<R> {
        self.default_visit_break_stmt(break_stmt, p)
    }

    fn default_visit_break_stmt(&mut self, break_stmt: &mut BreakStmt, p: &mut P) -> Option<R> {
        if let Some(label) = break_stmt.label.as_mut() {
            return self.visit_identifier_expr(label, p);
        }
        None
    }

    fn visit_continue_stmt(&mut self, continue_stmt: &mut ContinueStmt, p: &mut P) -> Option<R> {
        self.default_visit_continue_stmt(continue_stmt, p)
    }

    fn default_visit_continue_stmt(
        &mut self,
        continue_stmt: &mut ContinueStmt,
        p: &mut P,
    ) -> Option<R> {
        if let Some(label) = continue_stmt.label.as_mut() {
            return self.visit_identifier_expr(label, p);
        }
        None
    }

    fn visit_if_stmt(&mut self, if_stmt: &mut IfStmt, p: &mut P) -> Option<R> {
        self.default_visit_if_stmt(if_stmt, p)
    }
    fn default_visit_if_stmt(&mut self, if_stmt: &mut IfStmt, p: &mut P) -> Option<R> {
        let mut r = self.visit_expr(&mut if_stmt.condition, p);
        if r.is_some() {
            return r;
        }

        r = self.visit_block_stmt(&mut if_stmt.then_branch, p);
        if r.is_some() {
            return r;
        }

        if let Some(block) = if_stmt.else_branch.as_mut() {
            r = self.visit_block_stmt(block, p);
        }

        r
    }

    fn visit_print_stmt(&mut self, print_stmt: &mut PrintStmt, p: &mut P) -> Option<R> {
        self.default_visit_print_stmt(print_stmt, p)
    }
    fn default_visit_print_stmt(&mut self, print_stmt: &mut PrintStmt, p: &mut P) -> Option<R> {
        self.visit_expr(&mut print_stmt.expr, p)
    }

    fn visit_return_stmt(&mut self, return_stmt: &mut ReturnStmt, p: &mut P) -> Option<R> {
        self.default_visit_return_stmt(return_stmt, p)
    }
    fn default_visit_return_stmt(&mut self, return_stmt: &mut ReturnStmt, p: &mut P) -> Option<R> {
        self.visit_expr(&mut return_stmt.expr, p)
    }

    fn visit_while_stmt(&mut self, while_stmt: &mut WhileStmt, p: &mut P) -> Option<R> {
        self.default_visit_while_stmt(while_stmt, p)
    }
    fn default_visit_while_stmt(&mut self, while_stmt: &mut WhileStmt, p: &mut P) -> Option<R> {
        let mut r;
        if let Some(label) = while_stmt.label.as_mut() {
            r = self.visit_identifier_expr(label, p);
            if r.is_some() {
                return r;
            }
        }

        r = self.visit_expr(&mut while_stmt.condition, p);
        if r.is_some() {
            return r;
        }

        self.visit_block_stmt(&mut while_stmt.body, p)
    }

    fn visit_empty_stmt(&mut self, empty_stmt: &mut EmptyStmt, p: &mut P) -> Option<R> {
        self.default_visit_empty_stmt(empty_stmt, p)
    }
    fn default_visit_empty_stmt(&mut self, _empty_stmt: &mut EmptyStmt, _p: &mut P) -> Option<R> {
        None
    }

    fn visit_assign_expr(&mut self, assign_expr: &mut AssignExpr, p: &mut P) -> Option<R> {
        self.default_visit_assign_expr(assign_expr, p)
    }
    fn default_visit_assign_expr(&mut self, assign_expr: &mut AssignExpr, p: &mut P) -> Option<R> {
        let r = self.visit_expr(&mut assign_expr.target, p);
        if r.is_some() {
            return r;
        }

        self.visit_expr(&mut assign_expr.value, p)
    }

    fn visit_compound_assign_expr(
        &mut self,
        compound_assign_expr: &mut CompoundAssignExpr,
        p: &mut P,
    ) -> Option<R> {
        self.default_visit_compound_assign_expr(compound_assign_expr, p)
    }
    fn default_visit_compound_assign_expr(
        &mut self,
        assign_expr: &mut CompoundAssignExpr,
        p: &mut P,
    ) -> Option<R> {
        let r = self.visit_expr(&mut assign_expr.target, p);
        if r.is_some() {
            return r;
        }
        self.visit_expr(&mut assign_expr.value, p)
    }

    fn visit_binary_expr(&mut self, binary_expr: &mut BinaryExpr, p: &mut P) -> Option<R> {
        self.default_visit_binary_expr(binary_expr, p)
    }
    fn default_visit_binary_expr(&mut self, binary_expr: &mut BinaryExpr, p: &mut P) -> Option<R> {
        let r = self.visit_expr(&mut binary_expr.left, p);
        if r.is_some() {
            return r;
        }
        self.visit_expr(&mut binary_expr.right, p)
    }

    fn visit_unary_expr(&mut self, unary_expr: &mut UnaryExpr, p: &mut P) -> Option<R> {
        self.default_visit_unary_expr(unary_expr, p)
    }
    fn default_visit_unary_expr(&mut self, unary_expr: &mut UnaryExpr, p: &mut P) -> Option<R> {
        self.visit_expr(&mut unary_expr.expr, p)
    }

    fn visit_func_call_expr(&mut self, func_call_expr: &mut FuncCallExpr, p: &mut P) -> Option<R> {
        self.default_visit_func_call_expr(func_call_expr, p)
    }
    fn default_visit_func_call_expr(
        &mut self,
        func_call_expr: &mut FuncCallExpr,
        p: &mut P,
    ) -> Option<R> {
        let mut r = self.visit_expr(&mut func_call_expr.callee, p);
        if r.is_some() {
            return r;
        }
        for i in 0..func_call_expr.args.len() {
            let arg = func_call_expr.args.get_mut(i).unwrap();
            r = self.visit_expr(arg, p);
            if r.is_some() {
                return r;
            }
        }
        None
    }

    fn visit_member_access_expr(
        &mut self,
        member_access_expr: &mut MemberAccessExpr,
        p: &mut P,
    ) -> Option<R> {
        self.default_visit_member_access_expr(member_access_expr, p)
    }
    fn default_visit_member_access_expr(
        &mut self,
        member_access_expr: &mut MemberAccessExpr,
        p: &mut P,
    ) -> Option<R> {
        let r = self.visit_identifier_expr(&mut member_access_expr.member, p);
        if r.is_some() {
            return r;
        }

        self.visit_expr(&mut member_access_expr.receiver, p)
    }

    fn visit_grouping_expr(&mut self, grouping: &mut GroupingExpr, _p: &mut P) -> Option<R> {
        self.default_visit_grouping(grouping, _p)
    }
    fn default_visit_grouping(&mut self, grouping: &mut GroupingExpr, _p: &mut P) -> Option<R> {
        self.visit_expr(&mut grouping.expr, _p)
    }

    fn visit_identifier_expr(&mut self, identifier: &mut IdentifierExpr, _p: &mut P) -> Option<R> {
        self.default_visit_identifier(identifier, _p)
    }
    fn default_visit_identifier(
        &mut self,
        _identifier: &mut IdentifierExpr,
        _p: &mut P,
    ) -> Option<R> {
        None
    }

    fn visit_literal_expr(&mut self, literal: &mut LiteralExpr, _p: &mut P) -> Option<R> {
        self.default_visit_literal(literal, _p)
    }
    fn default_visit_literal(&mut self, _literal_expr: &mut LiteralExpr, _p: &mut P) -> Option<R> {
        None
    }

    fn visit_array_expr(&mut self, array_expr: &mut ArrayExpr, p: &mut P) -> Option<R> {
        self.default_visit_array_expr(array_expr, p)
    }
    fn default_visit_array_expr(&mut self, array_expr: &mut ArrayExpr, p: &mut P) -> Option<R> {
        for i in 0..array_expr.elements.len() {
            let element = array_expr.elements.get_mut(i).unwrap();
            let r = self.visit_expr(element, p);
            if r.is_some() {
                return r;
            }
        }
        None
    }
}

macro_rules! impl_visitable {
    ($name:ty) => {
        paste! {
            impl Visitable for $name {
                fn accept<P, R>(&mut self, visitor: &mut (impl ASTVisitor<P, R> + ?Sized), p: &mut P) -> Option<R> {
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
    BreakStmt,
    ContinueStmt,
    EmptyStmt,
    AssignExpr,
    CompoundAssignExpr,
    BinaryExpr,
    UnaryExpr,
    FuncCallExpr,
    MemberAccessExpr,
    GroupingExpr,
    IdentifierExpr,
    LiteralExpr,
    ArrayExpr,
);
