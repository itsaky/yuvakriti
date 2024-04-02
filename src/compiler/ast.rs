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

use std::fmt::{Display, Formatter};

use crate::compiler::ast::visitor::ASTVisitor;
use crate::compiler::location::Range;
use crate::compiler::tokens::{Token, TokenType};

pub(crate) mod visitor;
pub(crate) mod pretty;
pub(crate) mod macros;

pub(crate) type Spanned<T> = (T, Range);
pub(crate) type StmtS = Spanned<Stmt>;
pub(crate) type ExprS = Spanned<Expr>;
pub(crate) type DeclS = Spanned<Decl>;
pub(crate) type Identifier = Spanned<String>;

pub(crate) trait AstNode {
    fn accept<P, R>(&mut self, visitor: &mut impl ASTVisitor<P, R>, p: &P) -> Option<R>;
}

/// Program : (Declaration)*
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Program {
    pub(crate) decls: Vec<DeclS>
}

/// Decl : ClassDecl
///        | FuncDecl
///        | VarDecl
///        | Stmt
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Decl {
    Class(ClassDecl),
    Func(FuncDecl),
    Stmt(Stmt)
}

/// ClassDecl : "class" IDENTIFIER ( ":" IDENTIFIER )? "{" ( FuncDecl )* "}"
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ClassDecl {
    pub(crate) name: Identifier,
    pub(crate) supercls: Option<Identifier>,
    pub(crate) methods: Vec<Spanned<FuncDecl>>,
}

/// FuncDecl : "fun" IDENTIFIER "(" ( IDENTIFIER )? ")" BlockStmt
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FuncDecl {
    pub(crate) name: Identifier,
    pub(crate) params: Vec<Identifier>,
    pub(crate) body: Spanned<BlockStmt>,
}

/// VarDecl : "var" IDENTIFIER ("=" Expr)?
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VarStmt {
    pub(crate) name: Identifier,
    pub(crate) initializer: Option<ExprS>,
}

/// BlockStmt : "{" ( Decl )* "}"
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BlockStmt {
    pub(crate) decls: Vec<DeclS>,
}

/// Stmt : ExprStmt
///       | ForStmt
///       | IfStmt
///       | PrintStmt
///       | ReturnStmt
///       | WhileStmt
///       | Block
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Stmt {
    Expr(ExprStmt),
    For(Box<ForStmt>),
    If(IfStmt),
    Print(PrintStmt),
    Return(ReturnStmt),
    While(WhileStmt),
    Var(VarStmt),
    Block(Spanned<BlockStmt>),
}

/// ExprStmt : Expr
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ExprStmt {
    pub(crate) expr: ExprS
}

/// ForStmt : "for" "(" ( Expr | VarDecl )? ";" ( Expr )? ";" ( Expr )? ")" Stmt
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ForStmt {
    pub(crate) init: Option<StmtS>,
    pub(crate) condition: Option<ExprS>,
    pub(crate) step: Option<ExprS>,
    pub(crate) body: Spanned<BlockStmt>
}

/// IfStmt : "if" Expr Stmt ( "else" Stmt )?
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct IfStmt {
    pub(crate) condition: ExprS,
    pub(crate) then_branch: Spanned<BlockStmt>,
    pub(crate) else_branch: Option<Spanned<BlockStmt>>
}

/// PrintStmt : "print" Expr
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrintStmt {
    pub(crate) expr: ExprS
}

/// ReturnStmt : "return" Expr
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ReturnStmt {
    pub(crate) expr: ExprS
}

/// WhileStmt : "while" Expr BlockStmt
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WhileStmt {
    pub(crate) condition: ExprS,
    pub(crate) body: Spanned<BlockStmt>
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Expr {
    Binary(Box<BinaryExpr>),
    Unary(Box<UnaryExpr>),
    FuncCall(Box<FuncCallExpr>),
    MemberAccess(Box<MemberAccessExpr>),
    Primary(Box<Spanned<PrimaryExpr>>),
}

/// AssignExpr : IDENTIFIER "=" Expr
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AssignExpr {
    pub(crate) target: ExprS,
    pub(crate) value: ExprS,
}

/// BinaryExpr : Expr BinaryOp Expr
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BinaryExpr {
    pub(crate) left: ExprS,
    pub(crate) op: BinaryOp,
    pub(crate) right: ExprS,
}

/// UnaryExpr : ( "!" | "-" ) Expr
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UnaryExpr {
    pub(crate) op: UnaryOp,
    pub(crate) expr: ExprS
}

/// FuncCallExpr : IDENTIFIER ( "(" Expr? ")" )*
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FuncCallExpr {
    pub(crate) callee: ExprS,
    pub(crate) args: Vec<ExprS>
}

/// MemberAccessExpr : Expr "." IDENTIFIER
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct MemberAccessExpr {
    pub(crate) receiver: ExprS,
    pub(crate) member: Identifier
}

/// PrimaryExpr : "true"
///              | "false"
///              | "nil"
///              | "this"
///              | NUMBER
///              | STRING
///              | IDENTIFIER
///              | "(" Expr ")"
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PrimaryExpr {
    True,
    False,
    Nil,
    This,
    Number(f64),
    String(String),
    Identifier(String),
    Grouping(ExprS),
}

/// UnaryOp : "-" | "!"
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum UnaryOp {
    Negate,
    Not
}

/// BinaryOp : "+" | "-" | "*" | "/"
///          | "==" | "!=" | ">" | ">=" | "<" | "<="
///          | "and" | "or"
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum BinaryOp {
    Or,
    And,
    Eq,
    EqEq,
    NotEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Plus,
    Minus,
    Mult,
    Div
}

impl UnaryOp {
    pub(crate) fn from_token(token: &Token) -> Option<UnaryOp> {
        match token.token_type {
            TokenType::Bang => Some(UnaryOp::Not),
            TokenType::Minus => Some(UnaryOp::Negate),
            _ => None
        }
    }
}

impl BinaryOp {
    pub(crate) fn from_token(token: &Token) -> Option<BinaryOp> {
        match token.token_type {
            TokenType::Plus => Some(BinaryOp::Plus),
            TokenType::Minus => Some(BinaryOp::Minus),
            TokenType::Asterisk => Some(BinaryOp::Mult),
            TokenType::Slash => Some(BinaryOp::Div),
            TokenType::EqEq => Some(BinaryOp::EqEq),
            TokenType::BangEq => Some(BinaryOp::NotEq),
            TokenType::Gt => Some(BinaryOp::Gt),
            TokenType::GtEq => Some(BinaryOp::GtEq),
            TokenType::Lt => Some(BinaryOp::Lt),
            TokenType::LtEq => Some(BinaryOp::LtEq),
            TokenType::And => Some(BinaryOp::And),
            TokenType::Or => Some(BinaryOp::Or),
            _ => None
        }
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

