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

use crate::ast::ASTVisitor;
use crate::ast::BinaryExpr;
use crate::ast::BinaryOp;
use crate::ast::Expr;
use crate::ast::LiteralExpr;
use crate::ast::Spanned;
use crate::ast::UnaryExpr;
use crate::ast::UnaryOp;

/// Helper for constant folding in the compiler.
pub struct ConstFold;

impl ConstFold {
    /// Create a new instance of [ConstFold].
    pub fn new() -> ConstFold {
        ConstFold {}
    }

    /// Try to fold the given expression and return an [Expr] if the constant folding was
    /// successful.
    pub fn try_fold(&self, expr: &Expr) -> Option<Expr> {
        match expr {
            Expr::Binary(binary) => self.fold_binary(binary.as_ref()),
            Expr::Unary(unary) => self.fold_unary(unary.as_ref()),
            _ => None,
        }
    }

    /// Perform constant folding on the given unary expression. Returns an [Expr] if the constant
    /// folding was successful or [None] if it failed.
    pub fn fold_unary(&self, unary: &UnaryExpr) -> Option<Expr> {
        let mut expr = &unary.expr;
        let folded = self.try_fold(expr);
        if let Some(exp) = &folded {
            expr = exp;
        }

        match &unary.op {
            UnaryOp::Negate => {
                if let Some((num, _)) = &expr.Literal().and_then(|l| l.Number()) {
                    return Some(Expr::Literal(LiteralExpr::Number((
                        -num,
                        unary.range().clone(),
                    ))));
                }
            }
            UnaryOp::Not => {
                if let Some((boo, _)) = &expr.Literal().and_then(|l| l.Bool()) {
                    return Some(Expr::Literal(LiteralExpr::Bool((
                        !boo,
                        unary.range().clone(),
                    ))));
                }
            }
        }

        None
    }

    /// Perform constant folding on the given binary expression. Returns an [Expr] if the constant
    /// folding was successful or [None] if it failed.
    pub fn fold_binary(&self, binary: &BinaryExpr) -> Option<Expr> {
        let mut left = &binary.left;
        let left_folded = self.try_fold(left);
        if let Some(exp) = &left_folded {
            left = exp;
        }

        let mut right = &binary.right;
        let right_folded = self.try_fold(right);
        if let Some(exp) = &right_folded {
            right = exp;
        }

        if let (Some((l, _)), Some((r, _))) = (
            left.Literal().and_then(|l| l.Number()),
            right.Literal().and_then(|r| r.Number()),
        ) {
            match &binary.op {
                BinaryOp::Plus | BinaryOp::Minus | BinaryOp::Mult | BinaryOp::Div => {
                    return Some(Expr::Literal(LiteralExpr::Number((
                        self.apply_arithmetic(&binary.op, l, r),
                        binary.range().clone(),
                    ))));
                }
                BinaryOp::EqEq
                | BinaryOp::NotEq
                | BinaryOp::Gt
                | BinaryOp::GtEq
                | BinaryOp::Lt
                | BinaryOp::LtEq => {
                    return Some(Expr::Literal(LiteralExpr::Bool((
                        self.apply_arithmetical_logic(&binary.op, l, r),
                        binary.range().clone(),
                    ))))
                }
                _ => {}
            }
        }

        if let (Some((l, _)), Some((r, _))) = (
            left.Literal().and_then(|l| l.Bool()),
            right.Literal().and_then(|r| r.Bool()),
        ) {
            match &binary.op {
                BinaryOp::And
                | BinaryOp::Or
                | BinaryOp::EqEq
                | BinaryOp::NotEq
                | BinaryOp::Gt
                | BinaryOp::GtEq
                | BinaryOp::Lt
                | BinaryOp::LtEq => {
                    return Some(Expr::Literal(LiteralExpr::Bool((
                        self.apply_boolean_logic(&binary.op, l, r),
                        binary.range().clone(),
                    ))))
                }
                _ => {}
            }
        }

        None
    }

    fn apply_arithmetic(&self, op: &BinaryOp, l: &f64, r: &f64) -> f64 {
        match op {
            BinaryOp::Plus => l + r,
            BinaryOp::Minus => l - r,
            BinaryOp::Mult => l * r,
            BinaryOp::Div => l / r,
            _ => panic!("Unsupported arithmetic operator: {}", op.sym()),
        }
    }

    fn apply_arithmetical_logic(&self, op: &BinaryOp, l: &f64, r: &f64) -> bool {
        match op {
            BinaryOp::EqEq => l == r,
            BinaryOp::NotEq => l != r,
            BinaryOp::Gt => l > r,
            BinaryOp::GtEq => l >= r,
            BinaryOp::Lt => l < r,
            BinaryOp::LtEq => l <= r,
            _ => panic!("Unsupported logical operator: {}", op.sym()),
        }
    }

    fn apply_boolean_logic(&self, op: &BinaryOp, l: &bool, r: &bool) -> bool {
        match op {
            BinaryOp::EqEq => l == r,
            BinaryOp::NotEq => l != r,
            BinaryOp::Gt => l > r,
            BinaryOp::GtEq => l >= r,
            BinaryOp::Lt => l < r,
            BinaryOp::LtEq => l <= r,
            BinaryOp::And => *l && *r,
            BinaryOp::Or => *l || *r,
            _ => panic!("Unsupported logical operator: {}", op.sym()),
        }
    }
}

impl ASTVisitor<(), ()> for ConstFold {
    fn visit_expr(&mut self, expr: &mut Expr, p: &mut ()) -> Option<()> {
        if let Some(folded) = self.try_fold(expr) {
            *expr = folded;
        }
        self.default_visit_expr(expr, p)
    }
}
