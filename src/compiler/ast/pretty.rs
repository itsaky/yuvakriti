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

use crate::yklang::compiler::ast::AssignExpr;
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
use crate::yklang::compiler::ast::VarDecl;
use crate::yklang::compiler::ast::visitor::ASTVisitor;
use crate::yklang::compiler::ast::WhileStmt;

pub struct ASTPrinter<'a> {
    f: &'a mut dyn Write,
}

impl<'a> ASTPrinter<'a> {
    
    pub fn new(f: &'a mut dyn Write) -> Self {
        ASTPrinter {
            f,
        }
    }

    fn indent(&mut self, indent_level: &usize) {
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

    fn linefeed(&mut self) {
        self.f.write_char('\n').unwrap();
    }

    fn print_expr(&mut self, expr: &Expr, indent_level: &usize) {
        self.indent(indent_level);
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
            Expr::Primary(primary_expr) => {
                self.f.write_str("primary ").unwrap();
                self.visit_primary_expr(&primary_expr.0, indent_level);
            }
        }
        self.f.write_str(")").unwrap();
        self.linefeed();
    }

    fn print_decl(&mut self, decl: &Decl, indent_level: &usize) {
        self.indent(indent_level);
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
            Decl::Var(var_decl) => {
                self.f.write_str("var ").unwrap();
                self.visit_var_decl(var_decl, indent_level);
            }
            Decl::Stmt((stmt, _)) => {
                self.print_stmt(stmt, indent_level);
            }
        }
        self.linefeed();
        self.indent(indent_level);
        self.f.write_str(")").unwrap();
        self.linefeed();
    }

    fn print_stmt(&mut self, stmt: &Stmt, indent_level: &usize) {
        self.indent(indent_level);
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
            Stmt::Block(block_stmt) => {
                self.f.write_str("block ").unwrap();
                self.visit_block_stmt(&block_stmt.0, indent_level);
            }
        }
        self.indent(indent_level);
        self.f.write_str(")").unwrap();
        self.linefeed();
    }
}

impl<'a> ASTVisitor<usize, ()> for ASTPrinter<'a> {
    
    fn visit_program(&mut self, program: &Program, indent_level: &usize) -> Option<()> {
        self.indent(indent_level);
        self.f.write_str("(program\n").unwrap();
        for decl in &program.decls {
            self.print_decl(decl, &(indent_level + 1));
        }
        self.indent(indent_level);
        self.f.write_str(")").unwrap();
        None
    }

    fn visit_class_decl(&mut self, class_decl: &ClassDecl, indent_level: &usize) -> Option<()> {
        self.f.write_str(&class_decl.name).unwrap();
        if let Some(supercls) = &class_decl.supercls {
            self.f.write_str(" : ").unwrap();
            self.f.write_str(supercls).unwrap();
        }
        self.f.write_str(" {").unwrap();
        for (method, _) in &class_decl.methods {
            self.visit_func_decl(method, &(indent_level + 1));
        }
        self.indent(indent_level);
        self.f.write_str("}").unwrap();
        None
    }

    fn visit_func_decl(&mut self, func_decl: &FuncDecl, indent_level: &usize) -> Option<()> {
        self.f.write_str(&func_decl.name).unwrap();
        self.f.write_str("(").unwrap();
        let mut first = true;
        for param in &func_decl.params {
            if !first {
                self.f.write_str(", ").unwrap();
            }
            first = false;
            self.f.write_str(param).unwrap();
        }
        self.f.write_str(") ").unwrap();
        self.visit_block_stmt(&func_decl.body.0, &(indent_level + 1));
        None
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl, indent_level: &usize) -> Option<()> {
        self.f.write_str(&var_decl.name).unwrap();
        if let Some((initializer, _)) = &var_decl.initializer {
            self.f.write_str(" = ").unwrap();
            self.print_expr(initializer, indent_level);
        }
        None
    }

    fn visit_block_stmt(&mut self, block_stmt: &BlockStmt, indent_level: &usize) -> Option<()> {
        self.f.write_str("{").unwrap();
        self.linefeed();
        for decl in &block_stmt.decls {
            self.visit_decl(decl, &(indent_level + 1));
        }
        self.indent(indent_level);
        self.f.write_str("}").unwrap();
        None
    }

    fn visit_expr_stmt(&mut self, expr_stmt: &ExprStmt, indent_level: &usize) -> Option<()> {
        self.print_expr(&expr_stmt.expr.0, indent_level);
        None
    }

    fn visit_for_stmt(&mut self, for_stmt: &ForStmt, indent_level: &usize) -> Option<()> {
        self.f.write_str("(").unwrap();
        if let Some((init, _)) = &for_stmt.init {
            self.visit_expr(init, &(indent_level + 1));
        }
        self.f.write_str("; ").unwrap();
        if let Some((condition, _)) = &for_stmt.condition {
            self.visit_expr(condition, &(indent_level + 1));
        }
        self.f.write_str("; ").unwrap();
        if let Some((step, _)) = &for_stmt.step {
            self.visit_expr(step, &(indent_level + 1));
        }
        self.f.write_str(") ").unwrap();
        self.visit_block_stmt(&for_stmt.body.0, &(indent_level + 1));
        None
    }

    fn visit_if_stmt(&mut self, if_stmt: &IfStmt, indent_level: &usize) -> Option<()> {
        self.print_expr(&if_stmt.condition.0, indent_level);
        self.f.write_str(" ").unwrap();
        self.visit_block_stmt(&if_stmt.then_branch.0, &(indent_level + 1));
        if let Some(else_branch) = &if_stmt.else_branch {
            self.f.write_str(" else ").unwrap();
            self.visit_block_stmt(&else_branch.0, &(indent_level + 1));
        }
        None
    }

    fn visit_print_stmt(&mut self, print_stmt: &PrintStmt, _indent_level: &usize) -> Option<()> {
        self.whitespace();
        self.print_expr(&print_stmt.expr.0, &0);
        None
    }

    fn visit_return_stmt(&mut self, return_stmt: &ReturnStmt, indent_level: &usize) -> Option<()> {
        self.print_expr(&return_stmt.expr.0, indent_level);
        None
    }

    fn visit_while_stmt(&mut self, while_stmt: &WhileStmt, indent_level: &usize) -> Option<()> {
        self.print_expr(&while_stmt.condition.0, indent_level);
        self.f.write_str(" ").unwrap();
        self.visit_block_stmt(&while_stmt.body.0, &(indent_level + 1));
        None
    }

    fn visit_assign_expr(&mut self, assign_expr: &AssignExpr, indent_level: &usize) -> Option<()> {
        self.f.write_str(&assign_expr.name).unwrap();
        self.print_expr(&assign_expr.value.0, &(indent_level + 1));
        None
    }

    fn visit_binary_expr(&mut self, binary_expr: &BinaryExpr, indent_level: &usize) -> Option<()> {
        self.f.write_str(&format!("{:?} ", binary_expr.op)).unwrap();
        self.print_expr(&binary_expr.left.0, &(indent_level + 1));
        self.print_expr(&binary_expr.right.0, &(indent_level + 1));
        None
    }

    fn visit_func_call_expr(&mut self, func_call_expr: &FuncCallExpr, indent_level: &usize) -> Option<()> {
        self.visit_expr(&func_call_expr.callee.0, &(indent_level + 1));
        self.f.write_str("(").unwrap();
        let mut first = true;
        for (arg, _) in &func_call_expr.args {
            if !first {
                self.f.write_str(", ").unwrap();
            }
            first = false;
            self.print_expr(arg, indent_level);
        }
        self.f.write_str(")").unwrap();
        None
    }

    fn visit_member_access_expr(&mut self, member_access_expr: &MemberAccessExpr, indent_level: &usize) -> Option<()> {
        self.print_expr(&member_access_expr.receiver.0, indent_level);
        self.f.write_str(".").unwrap();
        self.f.write_str(&member_access_expr.member).unwrap();
        None
    }

    fn visit_primary_expr(&mut self, primary_expr: &PrimaryExpr, _indent_level: &usize) -> Option<()> {
        self.f.write_str(&format!("{:?}", primary_expr)).unwrap();
        None
    }

    fn visit_stmt(&mut self, stmt: &Stmt, p: &usize) -> Option<()> {
        self.print_stmt(stmt, p);
        None
    }
}