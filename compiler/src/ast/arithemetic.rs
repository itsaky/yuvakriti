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

use crate::ast::Expr;
use crate::ast::IdentifierExpr;
use crate::ast::UnaryExpr;
use crate::ast::{ASTVisitor, AssignExpr};
use crate::ast::{BinaryExpr, GroupingExpr, LiteralExpr};

pub struct ArithmeticASTPrinter<'a> {
    f: &'a mut dyn Write,
}

impl<'a> ArithmeticASTPrinter<'a> {
    pub fn new(f: &'a mut dyn Write) -> Self {
        ArithmeticASTPrinter { f }
    }
}

impl<'a> ASTVisitor<(), ()> for ArithmeticASTPrinter<'a> {
    fn visit_expr(&mut self, expr: &Expr, _p: &mut ()) -> Option<()> {
        match expr {
            Expr::Assign(assign_expr) => self.visit_assign_expr(assign_expr, _p),
            Expr::Binary(exp) => self.visit_binary_expr(exp, _p),
            Expr::Unary(exp) => self.visit_unary_expr(exp, _p),
            Expr::Literal(exp) => self.visit_literal_expr(exp, _p),
            Expr::Identifier(exp) => self.visit_identifier_expr(exp, _p),
            Expr::Grouping(exp) => self.visit_grouping_expr(exp, _p),
            _ => panic!("Not an arithemetic expression"),
        };

        None
    }

    fn visit_assign_expr(&mut self, assign_expr: &AssignExpr, p: &mut ()) -> Option<()> {
        self.f.write_str("assign ").unwrap();
        self.visit_expr(&assign_expr.target, p);
        self.f.write_str(" = ").unwrap();
        self.visit_expr(&assign_expr.value, p);
        None
    }

    fn visit_binary_expr(&mut self, binary_expr: &BinaryExpr, _p: &mut ()) -> Option<()> {
        self.f.write_str("(").unwrap();
        self.visit_expr(&binary_expr.left, _p);
        self.f
            .write_str(&format!(" {} ", binary_expr.op.sym()))
            .unwrap();
        self.visit_expr(&binary_expr.right, _p);
        self.f.write_str(")").unwrap();
        None
    }

    fn visit_unary_expr(&mut self, unary_expr: &UnaryExpr, _p: &mut ()) -> Option<()> {
        self.f
            .write_str(&format!("{}", unary_expr.op.sym()))
            .unwrap();
        self.visit_expr(&unary_expr.expr, _p);
        None
    }

    fn visit_grouping_expr(&mut self, grouping: &GroupingExpr, _p: &mut ()) -> Option<()> {
        self.f.write_str("(").unwrap();
        self.visit_expr(&grouping.expr, _p);
        self.f.write_str(")").unwrap();
        None
    }

    fn visit_identifier_expr(&mut self, _identifier: &IdentifierExpr, _p: &mut ()) -> Option<()> {
        self.f.write_str(&format!("{}", _identifier.name)).unwrap();
        None
    }

    fn visit_literal_expr(&mut self, literal: &LiteralExpr, _p: &mut ()) -> Option<()> {
        match literal {
            LiteralExpr::Nil(_) => self.f.write_str("nil").unwrap(),
            LiteralExpr::Bool((boo, _)) => self.f.write_str(&boo.to_string()).unwrap(),
            LiteralExpr::Number((num, _)) => self.f.write_str(&format!("{}", num)).unwrap(),
            LiteralExpr::String((str, _)) => self.f.write_str(&format!("\"{}\"", str)).unwrap(),
        }
        None
    }
}
