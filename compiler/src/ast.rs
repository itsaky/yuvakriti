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

use std::fmt::Display;
use std::fmt::Formatter;

pub use arithemetic::ArithmeticASTPrinter;
pub use pretty::ASTPrinter;
pub use visitor::ASTVisitor;

use crate::location::Range;
use crate::tokens::Token;
use crate::tokens::TokenType;

mod arithemetic;
mod pretty;
mod visitor;

pub type Spanned<T> = (T, Range);
pub type StmtS = Spanned<Stmt>;
pub type ExprS = Spanned<Expr>;
pub type DeclS = Spanned<Decl>;
pub type Identifier = Spanned<String>;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    Program,
    ClassDecl,
    FuncDecl,
    ExprStmt,
    ForStmt,
    WhileStmt,
    IfStmt,
    PrintStmt,
    ReturnStmt,
    VarStmt,
    BlockStmt,
    AssignExpr,
    BinaryExpr,
    UnaryExpr,
    FuncCallExpr,
    MemberAccessExpr,
    PrimaryExpr,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DeclType {
    /// Top-level declarations, defined directly in the file.
    TopLevel,

    /// Class-level declarations, defined within a class body.
    ClassLevel,

    /// Method level declarations, defined within a method body.
    MethodLevel,
}

/// An AST node.
pub trait AstNode {
    fn typ(self: &Self) -> NodeType;
}

/// An [ASTNode] which can be visited
pub trait Visitable {
    fn accept<P, R>(self: &mut Self, visitor: &mut impl ASTVisitor<P, R>, p: &mut P) -> Option<R>;
}

/// Program : (Declaration)*
#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub decls: Vec<DeclS>,
    pub stmts: Vec<StmtS>,
}

/// Decl : ClassDecl
///        | FuncDecl
///        | VarDecl
///        | Stmt
#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
    Class(ClassDecl),
    Func(FuncDecl),
    Stmt(Stmt),
}

/// ClassDecl : "class" IDENTIFIER ( ":" IDENTIFIER )? "{" ( FuncDecl )* "}"
#[derive(Clone, Debug, PartialEq)]
pub struct ClassDecl {
    pub name: Identifier,
    pub supercls: Option<Identifier>,
    pub methods: Vec<Spanned<FuncDecl>>,
    pub decl_type: DeclType,
}

/// FuncDecl : "fun" IDENTIFIER "(" ( IDENTIFIER )? ")" BlockStmt
#[derive(Clone, Debug, PartialEq)]
pub struct FuncDecl {
    pub name: Identifier,
    pub params: Vec<Identifier>,
    pub body: Spanned<BlockStmt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclStmt {
    pub decl: DeclS,
    pub typ: DeclType,
}

/// Stmt : ExprStmt
///       | ForStmt
///       | IfStmt
///       | PrintStmt
///       | ReturnStmt
///       | WhileStmt
///       | Block
#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(ExprStmt),
    For(Box<ForStmt>),
    If(IfStmt),
    Print(PrintStmt),
    Return(ReturnStmt),
    While(WhileStmt),
    Var(VarStmt),
    Block(Spanned<BlockStmt>),
}

/// VarDecl : "var" IDENTIFIER ("=" Expr)?
#[derive(Clone, Debug, PartialEq)]
pub struct VarStmt {
    pub name: Identifier,
    pub initializer: Option<ExprS>,
}

/// BlockStmt : "{" ( Decl )* "}"
#[derive(Clone, Debug, PartialEq)]
pub struct BlockStmt {
    pub decls: Vec<DeclS>,
}

/// ExprStmt : Expr
#[derive(Clone, Debug, PartialEq)]
pub struct ExprStmt {
    pub expr: ExprS,
}

/// ForStmt : "for" "(" ( Expr | VarDecl )? ";" ( Expr )? ";" ( Expr )? ")" Stmt
#[derive(Clone, Debug, PartialEq)]
pub struct ForStmt {
    pub init: Option<StmtS>,
    pub condition: Option<ExprS>,
    pub step: Option<ExprS>,
    pub body: Spanned<BlockStmt>,
}

/// IfStmt : "if" Expr Stmt ( "else" Stmt )?
#[derive(Clone, Debug, PartialEq)]
pub struct IfStmt {
    pub condition: ExprS,
    pub then_branch: Spanned<BlockStmt>,
    pub else_branch: Option<Spanned<BlockStmt>>,
}

/// PrintStmt : "print" Expr
#[derive(Clone, Debug, PartialEq)]
pub struct PrintStmt {
    pub expr: ExprS,
}

/// ReturnStmt : "return" Expr
#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStmt {
    pub expr: ExprS,
}

/// WhileStmt : "while" Expr BlockStmt
#[derive(Clone, Debug, PartialEq)]
pub struct WhileStmt {
    pub condition: ExprS,
    pub body: Spanned<BlockStmt>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary(Box<BinaryExpr>),
    Unary(Box<UnaryExpr>),
    FuncCall(Box<FuncCallExpr>),
    MemberAccess(Box<MemberAccessExpr>),
    Primary(Box<Spanned<PrimaryExpr>>),
}

/// AssignExpr : IDENTIFIER "=" Expr
#[derive(Clone, Debug, PartialEq)]
pub struct AssignExpr {
    pub target: ExprS,
    pub value: ExprS,
}

/// BinaryExpr : Expr BinaryOp Expr
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpr {
    pub left: ExprS,
    pub op: BinaryOp,
    pub right: ExprS,
}

/// UnaryExpr : ( "!" | "-" ) Expr
#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: ExprS,
}

/// FuncCallExpr : IDENTIFIER ( "(" Expr? ")" )*
#[derive(Clone, Debug, PartialEq)]
pub struct FuncCallExpr {
    pub callee: ExprS,
    pub args: Vec<ExprS>,
}

/// MemberAccessExpr : Expr "." IDENTIFIER
#[derive(Clone, Debug, PartialEq)]
pub struct MemberAccessExpr {
    pub receiver: ExprS,
    pub member: Identifier,
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
pub enum PrimaryExpr {
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
pub enum UnaryOp {
    Negate,
    Not,
}

/// BinaryOp : "+" | "-" | "*" | "/"
///          | "==" | "!=" | ">" | ">=" | "<" | "<="
///          | "and" | "or"
#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
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
    Div,
}

impl UnaryOp {
    pub fn sym(&self) -> &'static str {
        match self {
            UnaryOp::Negate => "-",
            UnaryOp::Not => "!",
        }
    }

    pub fn from_token(token: &Token) -> Option<UnaryOp> {
        match token.token_type {
            TokenType::Bang => Some(UnaryOp::Not),
            TokenType::Minus => Some(UnaryOp::Negate),
            _ => None,
        }
    }
}

impl BinaryOp {
    pub fn sym(&self) -> &'static str {
        match self {
            BinaryOp::Or => "or",
            BinaryOp::And => "and",
            BinaryOp::Eq => "==",
            BinaryOp::EqEq => "===",
            BinaryOp::NotEq => "!=",
            BinaryOp::Gt => ">",
            BinaryOp::GtEq => ">=",
            BinaryOp::Lt => "<",
            BinaryOp::LtEq => "<=",
            BinaryOp::Plus => "+",
            BinaryOp::Minus => "-",
            BinaryOp::Mult => "*",
            BinaryOp::Div => "/",
        }
    }

    pub fn precedence(&self) -> i32 {
        match self {
            BinaryOp::Or | BinaryOp::And => 6,

            BinaryOp::Eq
            | BinaryOp::EqEq
            | BinaryOp::NotEq
            | BinaryOp::Gt
            | BinaryOp::GtEq
            | BinaryOp::Lt
            | BinaryOp::LtEq => 5,

            BinaryOp::Plus | BinaryOp::Minus => 4,

            BinaryOp::Mult | BinaryOp::Div => 3,
        }
    }

    pub fn from_token(token: &Token) -> Option<BinaryOp> {
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
            _ => None,
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

impl BinaryExpr {
    /// Check if the binary expression has only primary expressions as operands.
    pub fn has_primary_operands(&self) -> bool {
        if matches!(&self.left.0, Expr::Primary(_)) && matches!(&self.right.0, Expr::Primary(_)) {
            return true;
        }
        false
    }

    /// Check if the binary expression has only primary number expressions as operands.
    pub fn has_num_operands(&self) -> bool {
        self.get_num_operands().is_some()
    }

    /// Get the operands of the binary expression as numbers if the binary expression has [PrimaryExpr::Number] as its operands.
    pub fn get_num_operands(&self) -> Option<(&f64, &f64)> {
        match (&self.left.0, &self.right.0) {
            (Expr::Primary(left), Expr::Primary(right)) => match (&left.0, &right.0) {
                (PrimaryExpr::Number(l), PrimaryExpr::Number(r)) => Some((l, r)),
                _ => None,
            },
            _ => None,
        }
    }
}

macro_rules! impl_node {
    ($node_type:ident) => {
        impl AstNode for $node_type {
            fn typ(&self) -> NodeType {
                NodeType::$node_type
            }
        }
    };
}

impl_node!(Program);
impl_node!(ClassDecl);
impl_node!(FuncDecl);
impl_node!(ExprStmt);
impl_node!(ForStmt);
impl_node!(WhileStmt);
impl_node!(IfStmt);
impl_node!(PrintStmt);
impl_node!(ReturnStmt);
impl_node!(VarStmt);
impl_node!(BlockStmt);
impl_node!(AssignExpr);
impl_node!(BinaryExpr);
impl_node!(UnaryExpr);
impl_node!(FuncCallExpr);
impl_node!(MemberAccessExpr);
impl_node!(PrimaryExpr);
