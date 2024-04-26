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

use log::trace;

use crate::ast::ASTVisitor;
use crate::ast::BinaryExpr;
use crate::ast::BinaryOp;
use crate::ast::BlockStmt;
use crate::ast::Decl;
use crate::ast::EmptyStmt;
use crate::ast::Expr;
use crate::ast::IfStmt;
use crate::ast::LiteralExpr;
use crate::ast::Spanned;
use crate::ast::Stmt;
use crate::ast::UnaryExpr;
use crate::ast::UnaryOp;

/// Helper for constant folding in the compiler.
pub struct ConstFold;

impl ConstFold {
    /// Create a new instance of [ConstFold].
    pub fn new() -> ConstFold {
        ConstFold {}
    }

    /// Try to fold the given statement and return a [Stmt] if the constant folding was
    /// successful.
    pub fn try_fold_stmt(&self, stmt: &Stmt) -> Option<Stmt> {
        match stmt {
            Stmt::If(_if) => self.fold_if(_if),
            Stmt::Block(block) => self.fold_block(block),
            _ => None,
        }
    }

    pub fn fold_block(&self, block: &BlockStmt) -> Option<Stmt> {
        let len = block.decls.len();
        if len == 0 {
            return Some(Stmt::Empty(EmptyStmt::new(block.range().clone())));
        } else if len == 1 {
            if let Decl::Stmt(stmt) = block.decls.get(0).unwrap() {
                if let Some(folded) = self.try_fold_stmt(stmt) {
                    return Some(folded);
                }
                return Some(stmt.clone());
            }
        }

        None
    }

    pub fn fold_if(&self, _if: &IfStmt) -> Option<Stmt> {
        if let Some(cond) = _if.condition.Literal().and_then(|l| l.Bool()).map(|b| b.0) {
            let mut stmt = if cond {
                Stmt::Block(_if.then_branch.clone())
            } else if let Some(else_branch) = &_if.else_branch {
                Stmt::Block(else_branch.clone())
            } else {
                Stmt::Empty(EmptyStmt::new(_if.range().clone()))
            };

            if let Some(folded) = self.try_fold_stmt(&stmt) {
                stmt = folded;
            }

            return Some(stmt);
        }

        None
    }

    /// Try to fold the given expression and return an [Expr] if the constant folding was
    /// successful.
    pub fn try_fold_expr(&self, expr: &Expr) -> Option<Expr> {
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
        let folded = self.try_fold_expr(expr);
        if let Some(exp) = &folded {
            expr = exp;
        }

        match &unary.op {
            UnaryOp::Negate => {
                if let Some((num, _)) = &expr.Literal().and_then(|l| l.Number()) {
                    trace!("[ConstFold] Negating {} to {}", num, -num);
                    return Some(Expr::Literal(LiteralExpr::Number((
                        -num,
                        unary.range().clone(),
                    ))));
                }
            }
            UnaryOp::Not => {
                if let Some((boo, _)) = &expr.Literal().and_then(|l| l.Bool()) {
                    trace!("[ConstFold] Negating {} to {}", boo, !boo);
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
        let left_folded = self.try_fold_expr(left);
        if let Some(exp) = &left_folded {
            left = exp;
        }

        let mut right = &binary.right;
        let right_folded = self.try_fold_expr(right);
        if let Some(exp) = &right_folded {
            right = exp;
        }

        match (left, right) {
            (Expr::Literal(l), Expr::Literal(r)) => match (l, r) {
                (LiteralExpr::Number((l, _)), LiteralExpr::Number((r, _))) => match &binary.op {
                    BinaryOp::Plus | BinaryOp::Minus | BinaryOp::Mult | BinaryOp::Div => {
                        let result = self.apply_arithmetic(&binary.op, l, r);
                        trace!(
                            "[ConstFold] Folding {} {} {} => {}",
                            l,
                            binary.op.sym(),
                            r,
                            &result
                        );
                        return Some(Expr::Literal(LiteralExpr::Number((
                            result,
                            binary.range().clone(),
                        ))));
                    }
                    BinaryOp::EqEq
                    | BinaryOp::NotEq
                    | BinaryOp::Gt
                    | BinaryOp::GtEq
                    | BinaryOp::Lt
                    | BinaryOp::LtEq => {
                        let result = self.apply_arithmetical_logic(&binary.op, l, r);
                        trace!(
                            "[ConstFold] Folding {} {} {} => {}",
                            l,
                            binary.op.sym(),
                            r,
                            &result
                        );
                        return Some(Expr::Literal(LiteralExpr::Bool((
                            result,
                            binary.range().clone(),
                        ))));
                    }
                    _ => {}
                },

                (LiteralExpr::Bool((l, _)), LiteralExpr::Bool((r, _))) => match &binary.op {
                    BinaryOp::And
                    | BinaryOp::Or
                    | BinaryOp::EqEq
                    | BinaryOp::NotEq
                    | BinaryOp::Gt
                    | BinaryOp::GtEq
                    | BinaryOp::Lt
                    | BinaryOp::LtEq => {
                        let result = self.apply_boolean_logic(&binary.op, l, r);
                        trace!(
                            "[ConstFold] Folding {} {} {} => {}",
                            l,
                            binary.op.sym(),
                            r,
                            &result
                        );
                        return Some(Expr::Literal(LiteralExpr::Bool((
                            result,
                            binary.range().clone(),
                        ))));
                    }
                    _ => {}
                },

                (LiteralExpr::Bool((b, _)), expr) | (expr, LiteralExpr::Bool((b, _))) => {
                    return self.fold_bool_binary_expr(b, binary, &|| Expr::Literal(expr.clone()));
                }

                _ => {}
            },

            (expr, Expr::Literal(l)) | (Expr::Literal(l), expr) => {
                if let Some((b, _)) = l.Bool() {
                    return self.fold_bool_binary_expr(b, binary, &|| expr.clone());
                }
            }
            _ => {}
        }

        None
    }

    fn fold_bool_binary_expr(
        &self,
        b: &bool,
        binary: &BinaryExpr,
        expr: &dyn Fn() -> Expr,
    ) -> Option<Expr> {
        match &binary.op {
            BinaryOp::And => {
                // b and <expr>
                // <expr> and b
                // ===>
                // false  -- if b == false
                // <expr> -- otherwise
                if !b {
                    return Some(Expr::Literal(LiteralExpr::Bool((
                        false,
                        binary.range().clone(),
                    ))));
                }
                return Some(expr());
            }
            BinaryOp::Or => {
                // b or <expr>
                // <expr> or b
                // ===>
                // true  -- if b == true
                // <expr> -- otherwise
                if *b {
                    return Some(Expr::Literal(LiteralExpr::Bool((
                        true,
                        binary.range().clone(),
                    ))));
                }
                return Some(expr());
            }

            BinaryOp::EqEq => {
                // b == <expr>
                // <expr> == b
                // ===>
                // <expr> -- if b == true
                // !<expr> -- if b == false
                if *b {
                    return Some(expr());
                }

                return Some(Expr::Unary(Box::from(UnaryExpr::new(
                    UnaryOp::Not,
                    expr(),
                    binary.range().clone(),
                ))));
            }
            BinaryOp::NotEq => {
                // b != <expr>
                // <expr> != b
                // ===>
                // !<expr> -- if b == true
                // <expr> -- if b == false

                if *b {
                    return Some(Expr::Unary(Box::from(UnaryExpr::new(
                        UnaryOp::Not,
                        expr(),
                        binary.range().clone(),
                    ))));
                }

                return Some(expr());
            }

            _ => {}
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
    fn visit_stmt(&mut self, stmt: &mut Stmt, p: &mut ()) -> Option<()> {
        if let Some(folded) = self.try_fold_stmt(stmt) {
            *stmt = folded;
        }
        self.default_visit_stmt(stmt, p)
    }
    fn visit_expr(&mut self, expr: &mut Expr, p: &mut ()) -> Option<()> {
        if let Some(folded) = self.try_fold_expr(expr) {
            *expr = folded;
        }
        self.default_visit_expr(expr, p)
    }
}
