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

use crate::ast::BlockStmt;
use crate::ast::ClassDecl;
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
use crate::ast::NodeType;
use crate::ast::PrintStmt;
use crate::ast::Program;
use crate::ast::ReturnStmt;
use crate::ast::Stmt;
use crate::ast::UnaryExpr;
use crate::ast::UnaryOp;
use crate::ast::VarStmt;
use crate::ast::Visitable;
use crate::ast::WhileStmt;
use crate::ast::{ASTVisitor, BreakStmt, ContinueStmt};
use crate::ast::{ArrayAccessExpr, BinaryOp};
use crate::ast::{ArrayExpr, BinaryExpr};
use crate::ast::{AssignExpr, CompoundAssignExpr};
use crate::ast::{AstNode, EmptyStmt};
use crate::location::Range;

pub type Matcher = dyn ASTVisitor<(), bool>;

macro_rules! mtch {
    ($e:expr, $m:expr, $er:literal) => {
        $e.accept($m, &mut ()).map(|r| assert!(r)).expect($er)
    };
}

macro_rules! mtch_o {
    ($e:expr, $m:expr, $er:literal) => {
        $e.and_then(|m| m.accept($m, &mut ()).map(|r| assert!(r)))
            .expect($er);
    };
}

pub struct AssertingAstMatcher {
    typ: NodeType,
    nested: Vec<Box<Matcher>>,
}

impl AssertingAstMatcher {
    pub fn new(typ: NodeType, nested: Vec<Box<Matcher>>) -> AssertingAstMatcher {
        return AssertingAstMatcher { typ, nested };
    }
}

pub struct IdentifierMatcher {
    name: String,
}

impl IdentifierMatcher {
    pub fn new(name: String) -> IdentifierMatcher {
        return IdentifierMatcher { name };
    }
}
impl ASTVisitor<(), bool> for IdentifierMatcher {
    fn visit_identifier_expr(
        &mut self,
        identifier: &mut IdentifierExpr,
        _p: &mut (),
    ) -> Option<bool> {
        Some(self.name == identifier.name)
    }
}

pub struct LiteralMatcher {
    value: LiteralExpr,
}
impl LiteralMatcher {
    pub fn new(value: LiteralExpr) -> LiteralMatcher {
        return LiteralMatcher { value };
    }
}
impl ASTVisitor<(), bool> for LiteralMatcher {
    fn visit_literal_expr(&mut self, literal: &mut LiteralExpr, _p: &mut ()) -> Option<bool> {
        let result = match (&self.value, &literal) {
            (LiteralExpr::Null(_), LiteralExpr::Null(_)) => true,
            (LiteralExpr::Bool(f), LiteralExpr::Bool(s)) => &f.0 == &s.0,
            (LiteralExpr::Number(f), LiteralExpr::Number(s)) => &f.0 == &s.0,
            (LiteralExpr::String(f), LiteralExpr::String(s)) => &f.0 == &s.0,
            _ => false,
        };

        if !result {
            println!("Expected {}, got {}", self.value, &literal);
            assert!(false);
        }

        Some(true)
    }
}

pub struct NoOpMatcher {}

impl NoOpMatcher {
    pub fn new() -> NoOpMatcher {
        return NoOpMatcher {};
    }
}

impl ASTVisitor<(), bool> for NoOpMatcher {
    #[allow(unused_variables)]
    fn visit_program(&mut self, program: &mut Program, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_decl(&mut self, decl: &mut Decl, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_stmt(&mut self, stmt: &mut Stmt, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_expr(&mut self, expr: &mut Expr, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_class_decl(&mut self, class_decl: &mut ClassDecl, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_func_decl(&mut self, func_decl: &mut FuncDecl, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_var_stmt(&mut self, var_decl: &mut VarStmt, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_block_stmt(&mut self, block_stmt: &mut BlockStmt, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_expr_stmt(&mut self, expr_stmt: &mut ExprStmt, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_for_stmt(&mut self, for_stmt: &mut ForStmt, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_break_stmt(&mut self, break_stmt: &mut BreakStmt, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_continue_stmt(
        &mut self,
        continue_stmt: &mut ContinueStmt,
        p: &mut (),
    ) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_if_stmt(&mut self, if_stmt: &mut IfStmt, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_print_stmt(&mut self, print_stmt: &mut PrintStmt, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_return_stmt(&mut self, return_stmt: &mut ReturnStmt, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_while_stmt(&mut self, while_stmt: &mut WhileStmt, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_empty_stmt(&mut self, empty_stmt: &mut EmptyStmt, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_assign_expr(&mut self, assign_expr: &mut AssignExpr, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_compound_assign_expr(
        &mut self,
        compound_assign_expr: &mut CompoundAssignExpr,
        p: &mut (),
    ) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_binary_expr(&mut self, binary_expr: &mut BinaryExpr, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_unary_expr(&mut self, unary_expr: &mut UnaryExpr, p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_func_call_expr(
        &mut self,
        func_call_expr: &mut FuncCallExpr,
        p: &mut (),
    ) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_member_access_expr(
        &mut self,
        member_access_expr: &mut MemberAccessExpr,
        p: &mut (),
    ) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_grouping_expr(&mut self, grouping: &mut GroupingExpr, _p: &mut ()) -> Option<bool> {
        Some(true)
    }
    #[allow(unused_variables)]
    fn visit_identifier_expr(
        &mut self,
        identifier: &mut IdentifierExpr,
        _p: &mut (),
    ) -> Option<bool> {
        Some(true)
    }

    #[allow(unused_variables)]
    fn visit_literal_expr(&mut self, literal: &mut LiteralExpr, _p: &mut ()) -> Option<bool> {
        Some(true)
    }

    #[allow(unused_variables)]
    fn visit_array_expr(&mut self, array_expr: &mut ArrayExpr, p: &mut ()) -> Option<bool> {
        Some(true)
    }
}

pub struct BinaryMatcher {
    pub op: Option<BinaryOp>,
    pub nested: Vec<Box<Matcher>>,
}

impl BinaryMatcher {
    pub fn new(op: Option<BinaryOp>, nested: Vec<Box<Matcher>>) -> BinaryMatcher {
        return BinaryMatcher { op, nested };
    }
}

impl ASTVisitor<(), bool> for BinaryMatcher {
    fn visit_binary_expr(&mut self, binary_expr: &mut BinaryExpr, _p: &mut ()) -> Option<bool> {
        if let Some(op) = self.op.as_ref() {
            assert_eq!(op, &binary_expr.op);
        }

        if let Some(matcher) = self.nested.get_mut(0) {
            mtch!(
                binary_expr.left,
                matcher.as_mut(),
                "Failed to match binary left"
            );
        }

        if let Some(matcher) = self.nested.get_mut(1) {
            mtch!(
                binary_expr.right,
                matcher.as_mut(),
                "Failed to match binary right"
            );
        }

        Some(true)
    }
}

pub struct UnaryMatcher {
    pub op: Option<UnaryOp>,
    pub matcher: Option<Box<Matcher>>,
}

impl UnaryMatcher {
    pub fn new(op: Option<UnaryOp>, matcher: Option<Box<Matcher>>) -> UnaryMatcher {
        return UnaryMatcher { op, matcher };
    }
}

impl ASTVisitor<(), bool> for UnaryMatcher {
    fn visit_unary_expr(&mut self, unary_expr: &mut UnaryExpr, _p: &mut ()) -> Option<bool> {
        if let Some(op) = self.op.as_ref() {
            assert_eq!(op, &unary_expr.op);
        }

        if let Some(matcher) = self.matcher.as_mut() {
            mtch!(unary_expr.expr, matcher.as_mut(), "Failed to match expr");
        }

        Some(true)
    }
}

pub struct CompoundAssignmentMatcher {
    pub op: Option<BinaryOp>,
    pub nested: Vec<Box<Matcher>>,
}

impl CompoundAssignmentMatcher {
    pub fn new(op: Option<BinaryOp>, nested: Vec<Box<Matcher>>) -> CompoundAssignmentMatcher {
        return CompoundAssignmentMatcher { op, nested };
    }
}

impl ASTVisitor<(), bool> for CompoundAssignmentMatcher {
    fn visit_compound_assign_expr(
        &mut self,
        compound_assign_expr: &mut CompoundAssignExpr,
        _p: &mut (),
    ) -> Option<bool> {
        if let Some(op) = self.op.as_ref() {
            assert_eq!(op, &compound_assign_expr.op);
        }

        if let Some(matcher) = self.nested.get_mut(0) {
            mtch!(
                compound_assign_expr.target,
                matcher.as_mut(),
                "Failed to match target"
            );
        }

        if let Some(matcher) = self.nested.get_mut(1) {
            mtch!(
                compound_assign_expr.value,
                matcher.as_mut(),
                "Failed to match value"
            );
        }

        Some(true)
    }
}

#[allow(non_snake_case)]
pub fn Program(decls: Vec<Box<Matcher>>, stmts: Vec<Box<Matcher>>) -> AssertingAstMatcher {
    let mut nested: Vec<Box<Matcher>> = vec![];
    nested.extend(decls);
    nested.extend(stmts);
    return AssertingAstMatcher::new(NodeType::Program, nested);
}

#[allow(non_snake_case, unused)]
pub fn Node(typ: NodeType, nested: Vec<Box<Matcher>>) -> AssertingAstMatcher {
    return AssertingAstMatcher::new(typ, nested);
}

#[allow(non_snake_case, unused)]
pub fn Empty() -> AssertingAstMatcher {
    return AssertingAstMatcher::new(NodeType::EmptyStmt, vec![]);
}

#[allow(non_snake_case, unused)]
pub fn Array(nested: Vec<Box<Matcher>>) -> AssertingAstMatcher {
    return AssertingAstMatcher::new(NodeType::ArrayExpr, nested);
}

#[allow(non_snake_case, unused)]
pub fn Any() -> NoOpMatcher {
    return NoOpMatcher::new();
}

#[allow(non_snake_case, unused)]
pub fn Identifier(name: &str) -> IdentifierMatcher {
    return IdentifierMatcher::new(name.to_string());
}

#[allow(non_snake_case, unused)]
pub fn Number(value: f64) -> LiteralMatcher {
    return LiteralMatcher::new(LiteralExpr::Number((value, Range::NO_RANGE)));
}

#[allow(non_snake_case, unused)]
pub fn Bool(value: bool) -> LiteralMatcher {
    return LiteralMatcher::new(LiteralExpr::Bool((value, Range::NO_RANGE)));
}

#[allow(non_snake_case, unused)]
pub fn String(value: &str) -> LiteralMatcher {
    return LiteralMatcher::new(LiteralExpr::String((value.to_string(), Range::NO_RANGE)));
}
#[allow(non_snake_case, unused)]
pub fn Null() -> LiteralMatcher {
    return Literal(LiteralExpr::Null(((), Range::NO_RANGE)));
}
#[allow(non_snake_case, unused)]
pub fn Literal(value: LiteralExpr) -> LiteralMatcher {
    return LiteralMatcher::new(value);
}

#[allow(non_snake_case, unused)]
pub fn Binary(op: BinaryOp, nested: Vec<Box<Matcher>>) -> BinaryMatcher {
    return BinaryMatcher::new(Some(op), nested);
}

#[allow(non_snake_case, unused)]
pub fn Unary(op: UnaryOp, expr: Box<Matcher>) -> UnaryMatcher {
    return UnaryMatcher::new(Some(op), Some(expr));
}

#[allow(non_snake_case, unused)]
pub fn CompoundAssigment(op: BinaryOp, nested: Vec<Box<Matcher>>) -> CompoundAssignmentMatcher {
    return CompoundAssignmentMatcher::new(Some(op), nested);
}

impl ASTVisitor<(), bool> for AssertingAstMatcher {
    fn visit_program(&mut self, program: &mut Program, _p: &mut ()) -> Option<bool> {
        assert_eq!(&self.typ, &program.typ());
        let mut index = 0;
        for i in 0..program.decls.len() {
            let matcher = &mut self.nested[index];
            mtch!(
                &mut program.decls[i],
                matcher.as_mut(),
                "Failed to match decl"
            );
            index += 1;
        }

        for i in 0..program.stmts.len() {
            let matcher = &mut self.nested[index];
            mtch!(
                &mut program.stmts[i],
                matcher.as_mut(),
                "Failed to match stmt"
            );
            index += 1;
        }

        Some(true)
    }

    fn visit_func_decl(&mut self, func_decl: &mut FuncDecl, _p: &mut ()) -> Option<bool> {
        assert_eq!(&self.typ, &func_decl.typ());

        let mut idx = 0;
        if let Some(matcher) = self.nested.get_mut(idx) {
            mtch!(func_decl.name, matcher.as_mut(), "Failed to match name");
        }

        for i in 0..func_decl.params.len() {
            idx += 1;
            if let Some(matcher) = self.nested.get_mut(idx) {
                mtch!(
                    func_decl.params[i],
                    matcher.as_mut(),
                    "Failed to match param"
                );
            }
        }

        idx += 1;
        if let Some(matcher) = self.nested.get_mut(idx) {
            mtch!(func_decl.body, matcher.as_mut(), "Failed to match body");
        }

        Some(true)
    }

    fn visit_var_stmt(&mut self, var_decl: &mut VarStmt, _p: &mut ()) -> Option<bool> {
        assert_eq!(&self.typ, &var_decl.typ());
        let mut idx = 0;
        if let Some(matcher) = self.nested.get_mut(idx) {
            mtch!(var_decl.name, matcher.as_mut(), "Failed to match name");
        }

        idx += 1;
        if let Some(matcher) = self.nested.get_mut(idx) {
            mtch_o!(
                var_decl.initializer.as_mut(),
                matcher.as_mut(),
                "Failed to match expr"
            );
        }

        Some(true)
    }

    fn visit_block_stmt(&mut self, block_stmt: &mut BlockStmt, _p: &mut ()) -> Option<bool> {
        assert_eq!(&self.typ, &block_stmt.typ());

        let mut idx = 0;
        for i in 0..block_stmt.decls.len() {
            let decl = block_stmt.decls.get_mut(i).unwrap();
            if let Some(matcher) = self.nested.get_mut(idx) {
                mtch!(decl, matcher.as_mut(), "Failed to match decl");
            }
            idx += 1;
        }

        Some(true)
    }

    fn visit_for_stmt(&mut self, for_stmt: &mut ForStmt, _p: &mut ()) -> Option<bool> {
        assert_eq!(self.typ, for_stmt.typ());
        let mut idx = 0;

        if let Some(label) = for_stmt.label.as_mut() {
            if let Some(matcher) = self.nested.get_mut(idx) {
                mtch!(label, matcher.as_mut(), "Failed to match for label");
            }
        }

        idx += 1;
        if let Some(matcher) = self.nested.get_mut(idx) {
            mtch_o!(
                for_stmt.init.as_mut(),
                matcher.as_mut(),
                "Failed to match for init"
            );
        }

        idx += 1;
        if let Some(matcher) = self.nested.get_mut(idx) {
            mtch_o!(
                for_stmt.condition.as_mut(),
                matcher.as_mut(),
                "Failed to match for condition"
            );
        }

        idx += 1;
        if let Some(matcher) = self.nested.get_mut(idx) {
            mtch_o!(
                for_stmt.step.as_mut(),
                matcher.as_mut(),
                "Failed to match for step"
            );
        }

        Some(true)
    }

    fn visit_break_stmt(&mut self, break_stmt: &mut BreakStmt, _p: &mut ()) -> Option<bool> {
        assert_eq!(&self.typ, &break_stmt.typ());
        if let Some(matcher) = self.nested.get_mut(0) {
            mtch_o!(
                break_stmt.label.as_mut(),
                matcher.as_mut(),
                "Failed to match break label"
            );
        }
        Some(true)
    }

    fn visit_continue_stmt(
        &mut self,
        continue_stmt: &mut ContinueStmt,
        _p: &mut (),
    ) -> Option<bool> {
        assert_eq!(&self.typ, &continue_stmt.typ());
        if let Some(matcher) = self.nested.get_mut(0) {
            mtch_o!(
                continue_stmt.label.as_mut(),
                matcher.as_mut(),
                "Failed to match break label"
            );
        }
        Some(true)
    }

    fn visit_print_stmt(&mut self, print_stmt: &mut PrintStmt, _p: &mut ()) -> Option<bool> {
        assert_eq!(&self.typ, &print_stmt.typ());
        if let Some(matcher) = self.nested.get_mut(0) {
            mtch!(
                print_stmt.expr,
                matcher.as_mut(),
                "Failed to match print expr"
            );
        }
        Some(true)
    }

    fn visit_while_stmt(&mut self, while_stmt: &mut WhileStmt, _p: &mut ()) -> Option<bool> {
        assert_eq!(self.typ, while_stmt.typ());
        let mut idx = 0;

        if let Some(label) = while_stmt.label.as_mut() {
            if let Some(matcher) = self.nested.get_mut(idx) {
                mtch!(label, matcher.as_mut(), "Failed to match while label");
            }
        }

        idx += 1;
        if let Some(matcher) = self.nested.get_mut(idx) {
            mtch!(
                while_stmt.condition,
                matcher.as_mut(),
                "Failed to match while condition"
            );
        }

        idx += 1;
        if let Some(matcher) = self.nested.get_mut(idx) {
            mtch!(
                while_stmt.body,
                matcher.as_mut(),
                "Failed to match while body"
            );
        }

        Some(true)
    }

    fn visit_empty_stmt(&mut self, empty_stmt: &mut EmptyStmt, _p: &mut ()) -> Option<bool> {
        assert_eq!(&self.typ, &empty_stmt.typ());
        Some(true)
    }

    fn visit_assign_expr(&mut self, assign_expr: &mut AssignExpr, _p: &mut ()) -> Option<bool> {
        assert_eq!(self.typ, assign_expr.typ());
        if let Some(matcher) = self.nested.get_mut(0) {
            mtch!(
                assign_expr.target,
                matcher.as_mut(),
                "Failed to match assign target"
            );
        }
        if let Some(matcher) = self.nested.get_mut(1) {
            mtch!(
                assign_expr.value,
                matcher.as_mut(),
                "Failed to match assign value"
            );
        }

        Some(true)
    }

    fn visit_grouping_expr(&mut self, grouping: &mut GroupingExpr, _p: &mut ()) -> Option<bool> {
        assert_eq!(self.typ, grouping.typ());
        if let Some(matcher) = self.nested.get_mut(0) {
            mtch!(
                grouping.expr,
                matcher.as_mut(),
                "Failed to match grouping expr"
            );
        }

        Some(true)
    }

    fn visit_array_expr(&mut self, array_expr: &mut ArrayExpr, _p: &mut ()) -> Option<bool> {
        assert_eq!(&self.typ, &array_expr.typ());
        for i in 0..array_expr.elements.len() {
            if let Some(matcher) = self.nested.get_mut(i) {
                mtch_o!(
                    array_expr.elements.get_mut(i),
                    matcher.as_mut(),
                    "Failed to match element"
                );
            }
        }
        Some(true)
    }

    fn visit_array_access_expr(
        &mut self,
        array_expr: &mut ArrayAccessExpr,
        _p: &mut (),
    ) -> Option<bool> {
        assert_eq!(&self.typ, &array_expr.typ());
        if let Some(matcher) = self.nested.get_mut(0) {
            mtch!(
                &mut array_expr.array,
                matcher.as_mut(),
                "Failed to match array access array"
            );
        }
        if let Some(matcher) = self.nested.get_mut(1) {
            mtch!(
                &mut array_expr.index,
                matcher.as_mut(),
                "Failed to match array access array"
            );
        }
        Some(true)
    }
}
