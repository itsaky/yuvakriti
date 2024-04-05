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

use crate::ast::ASTVisitor;
use crate::ast::BinaryExpr;
use crate::ast::Expr;
use crate::ast::PrimaryExpr;
use crate::ast::UnaryExpr;

pub struct ArithmeticASTPrinter<'a> {
    f: &'a mut dyn Write,
}

impl<'a> ArithmeticASTPrinter<'a> {
    pub fn new(f: &'a mut dyn Write) -> Self {
        ArithmeticASTPrinter { f }
    }
}

impl<'a> ASTVisitor<(), ()> for ArithmeticASTPrinter<'a> {
    fn visit_binary_expr(&mut self, binary_expr: &BinaryExpr, _p: &()) -> Option<()> {
        self.f.write_str("(").unwrap();
        self.visit_expr(&binary_expr.left.0, &());
        self.f
            .write_str(&format!(" {} ", binary_expr.op.sym()))
            .unwrap();
        self.visit_expr(&binary_expr.right.0, &());
        self.f.write_str(")").unwrap();
        None
    }

    fn visit_unary_expr(&mut self, unary_expr: &UnaryExpr, _p: &()) -> Option<()> {
        self.f
            .write_str(&format!("{}", unary_expr.op.sym()))
            .unwrap();
        self.visit_expr(&unary_expr.expr.0, &());
        None
    }

    fn visit_primary_expr(&mut self, _primary_expr: &PrimaryExpr, _p: &()) -> Option<()> {
        match _primary_expr {
            PrimaryExpr::Number(num) => {
                let _ = self.f.write_str(&format!("{}", num)).unwrap();
            }
            PrimaryExpr::Identifier(name) => {
                let _ = self.f.write_str(&format!("{}", name)).unwrap();
            }
            PrimaryExpr::Grouping(expr) => {
                let _ = self.visit_expr(&expr.0, _p);
            }
            _ => panic!("Not an arithemetic expression"),
        };

        None
    }

    fn visit_expr(&mut self, expr: &Expr, _p: &()) -> Option<()> {
        match expr {
            Expr::Binary(binary_expr) => self.visit_binary_expr(binary_expr, &()),
            Expr::Unary(unary_expr) => self.visit_unary_expr(unary_expr, &()),
            Expr::Primary(primary_expr) => self.visit_primary_expr(&primary_expr.0, &()),
            _ => panic!("Not an arithemetic expression"),
        };

        None
    }
}
