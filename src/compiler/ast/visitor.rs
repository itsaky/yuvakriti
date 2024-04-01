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

use crate::yklang::compiler::ast::AssignExpr;
use crate::yklang::compiler::ast::AstNode;
use crate::yklang::compiler::ast::BinaryExpr;
use crate::yklang::compiler::ast::BlockStmt;
use crate::yklang::compiler::ast::ClassDecl;
use crate::yklang::compiler::ast::Decl;
use crate::yklang::compiler::ast::Expr;
use crate::yklang::compiler::ast::ExprStmt;
use crate::yklang::compiler::ast::ForStmt;
use crate::yklang::compiler::ast::FuncCallExpr;
use crate::yklang::compiler::ast::FuncDecl;
use crate::yklang::compiler::ast::IfStmt;
use crate::yklang::compiler::ast::MemberAccessExpr;
use crate::yklang::compiler::ast::PrimaryExpr;
use crate::yklang::compiler::ast::PrintStmt;
use crate::yklang::compiler::ast::Program;
use crate::yklang::compiler::ast::ReturnStmt;
use crate::yklang::compiler::ast::Stmt;
use crate::yklang::compiler::ast::UnaryExpr;
use crate::yklang::compiler::ast::VarDecl;
use crate::yklang::compiler::ast::WhileStmt;

/// ASTVisitor for visiting AST nodes. Methods in the visitor result an [Option<R>]. If the result
/// is [Some], then the child nodes of the AST node will not be visited.
pub trait ASTVisitor<P, R> {
    fn visit_program(&mut self, program: &Program, p: &P) -> Option<R> {
        let mut r: Option<R> = None;
        for decl in &program.decls {
            r = self.visit_decl(decl, p);

            if r.is_some() {
                break
            }
        }

        r
    }

    fn visit_class_decl(&mut self, class_decl: &ClassDecl, p: &P) -> Option<R> {
        let mut r: Option<R> = None;
        for (method, _) in &class_decl.methods {
            r = self.visit_func_decl(method, p);

            if r.is_some() {
                break
            }
        }

        r
    }

    fn visit_func_decl(&mut self, func_decl: &FuncDecl, p: &P) -> Option<R> {
        self.visit_block_stmt(&func_decl.body.0, p)
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl, p: &P) -> Option<R> {
        if let Some((initializer, _)) = &var_decl.initializer {
            return self.visit_expr(initializer, p)
        }

        None
    }

    fn visit_block_stmt(&mut self, block_stmt: &BlockStmt, p: &P) -> Option<R> {
        let mut r: Option<R> = None;
        for decl in &block_stmt.decls {
            r = self.visit_decl(decl, p);

            if r.is_some() {
                break
            }
        }

        r
    }

    fn visit_expr_stmt(&mut self, expr_stmt: &ExprStmt, p: &P) -> Option<R> {
        self.visit_expr(&expr_stmt.expr.0, p)
    }

    fn visit_for_stmt(&mut self, for_stmt: &ForStmt, p: &P) -> Option<R> {
        let mut r: Option<R> = None;
        if let Some((init, _)) = &for_stmt.init {
            r = self.visit_expr(&init, p);
        }

        if r.is_some() {
            return r
        }

        if let Some((cond, _)) = &for_stmt.condition {
            r = self.visit_expr(cond, p);
        }

        if r.is_some() {
            return r
        }

        if let Some((step, _)) = &for_stmt.step {
            r = self.visit_expr(step, p);
        }

        if r.is_some() {
            return r
        }

        self.visit_block_stmt(&for_stmt.body.0, p)
    }

    fn visit_if_stmt(&mut self, if_stmt: &IfStmt, p: &P) -> Option<R> {
        let mut r = self.visit_expr(&if_stmt.condition.0, p);
        if r.is_some() {
            return r
        }

        r = self.visit_block_stmt(&if_stmt.then_branch.0, p);
        if r.is_some() {
            return r
        }

        if let Some(block) = &if_stmt.else_branch {
            r = self.visit_block_stmt(&block.0, p);
        }

        r
    }

    fn visit_print_stmt(&mut self, print_stmt: &PrintStmt, p: &P) -> Option<R> {
        self.visit_expr(&print_stmt.expr.0, p)
    }

    fn visit_return_stmt(&mut self, return_stmt: &ReturnStmt, p: &P) -> Option<R> {
        self.visit_expr(&return_stmt.expr.0, p)
    }

    fn visit_while_stmt(&mut self, while_stmt: &WhileStmt, p: &P) -> Option<R> {
        let r = self.visit_expr(&while_stmt.condition.0, p);
        if r.is_some() {
            return r;
        }
        self.visit_block_stmt(&while_stmt.body.0, p)
    }

    fn visit_assign_expr(&mut self, assign_expr: &AssignExpr, p: &P) -> Option<R> {
        self.visit_expr(&assign_expr.value.0, p)
    }

    fn visit_binary_expr(&mut self, binary_expr: &BinaryExpr, p: &P) -> Option<R> {
        let r = self.visit_expr(&binary_expr.left.0, p);
        if r.is_some() {
            return r;
        }
        self.visit_expr(&binary_expr.right.0, p)
    }

    fn visit_unary_expr(&mut self, unary_expr: &UnaryExpr, p: &P) -> Option<R> {
        self.visit_expr(&unary_expr.expr.0, p)
    }

    fn visit_func_call_expr(&mut self, func_call_expr: &FuncCallExpr, p: &P) -> Option<R> {
        let mut r = self.visit_expr(&func_call_expr.callee.0, p);
        if r.is_some() {
            return r;
        }
        for (arg, _) in &func_call_expr.args {
            r = self.visit_expr(arg, p);
            if r.is_some() {
                return r;
            }
        }
        None
    }

    fn visit_member_access_expr(&mut self, member_access_expr: &MemberAccessExpr, p: &P) -> Option<R> {
        self.visit_expr(&member_access_expr.receiver.0, p)
    }

    fn visit_primary_expr(&mut self, _primary_expr: &PrimaryExpr, _p: &P) -> Option<R> {
        // Primary expressions are leaf nodes
        None
    }

    fn visit_decl(&mut self, decl: &Decl, p: &P) -> Option<R> {
        match decl {
            Decl::Class(class_decl) => self.visit_class_decl(&class_decl, p),
            Decl::Func(func_decl) => self.visit_func_decl(&func_decl, p),
            Decl::Var(var_decl) => self.visit_var_decl(&var_decl, p),
            Decl::Stmt ((stmt, _)) => self.visit_stmt(stmt, p),
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt, p: &P) -> Option<R> {
        match stmt {
            Stmt::Expr(expr_stmt) => self.visit_expr_stmt(&expr_stmt, p),
            Stmt::For(for_stmt) => self.visit_for_stmt(&for_stmt, p),
            Stmt::If(if_stmt) => self.visit_if_stmt(&if_stmt, p),
            Stmt::Print(print_stmt) => self.visit_print_stmt(&print_stmt, p),
            Stmt::Return(return_stmt) => self.visit_return_stmt(&return_stmt, p),
            Stmt::While(while_stmt) => self.visit_while_stmt(&while_stmt, p),
            Stmt::Block(block_stmt) => self.visit_block_stmt(&block_stmt.0, p),
        }
    }

    fn visit_expr(&mut self, expr: &Expr, p: &P) -> Option<R> {
        match expr {
            Expr::Assign(assign_expr) => self.visit_assign_expr(&assign_expr, p),
            Expr::Binary(binary_expr) => self.visit_binary_expr(&binary_expr, p),
            Expr::Unary(unary_expr) => self.visit_unary_expr(&unary_expr, p),
            Expr::FuncCall(func_call_expr) => self.visit_func_call_expr(&func_call_expr, p),
            Expr::MemberAccess(member_access_expr) => self.visit_member_access_expr(&member_access_expr, p),
            Expr::Primary(primary_expr) => self.visit_primary_expr(&primary_expr.0, p),
        }
    }
}

impl AstNode for Program {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_program(self, p)
    }
}

impl AstNode for ClassDecl {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_class_decl(self, p)
    }
}

impl AstNode for FuncDecl {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_func_decl(self, p)
    }
}

impl AstNode for VarDecl {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_var_decl(self, p)
    }
}

impl AstNode for BlockStmt {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_block_stmt(self, p)
    }
}

impl AstNode for ExprStmt {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_expr_stmt(self, p)
    }
}

impl AstNode for ForStmt {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_for_stmt(self, p)
    }
}

impl AstNode for IfStmt {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_if_stmt(self, p)
    }
}

impl AstNode for PrintStmt {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_print_stmt(self, p)
    }
}

impl AstNode for ReturnStmt {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_return_stmt(self, p)
    }
}

impl AstNode for WhileStmt {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_while_stmt(self, p)
    }
}

impl AstNode for AssignExpr {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_assign_expr(self, p)
    }
}

impl AstNode for BinaryExpr {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_binary_expr(self, p)
    }
}

impl AstNode for UnaryExpr {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_unary_expr(self, p)
    }
}

impl AstNode for FuncCallExpr {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_func_call_expr(self, p)
    }
}

impl AstNode for MemberAccessExpr {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_member_access_expr(self, p)
    }
}

impl AstNode for PrimaryExpr {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R> {
        visitor.visit_primary_expr(self, p)
    }
}