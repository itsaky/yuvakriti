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

macro_rules! def_node {
    ($name:ident {
        $($prop:ident: $ty:ty $(,)?)*
    }) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name {
            $(pub $prop: $ty,)*
            range: Range
        }

        impl $name {
            pub fn new($($prop: $ty,)* range: Range) -> Self {
                Self { $($prop,)* range }
            }
            pub fn own_range(self) -> Range {
                self.range
            }
        }
    };
}

macro_rules! def_enum {
    ($name:ident {
        $($prop:ident $(: $ty:ty)?,)+
    }) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $name {
            $( $prop $(($ty))? ),+
        }

        #[allow(non_snake_case)]
        impl $name {
            $( pub fn $prop (&self) -> Option<&$($ty)?> {
                if let $name::$prop(node) = self {
                    return Some(node);
                }

                None
            })+
        }

        impl AstNode for $name {
            fn typ(&self) -> NodeType {
                match self {
                    $($name::$prop(node) => node.typ(),)*
                }
            }
        }

        impl Spanned for $name {
            fn range(&self) -> &Range {
                match self {
                    $($name::$prop(node) => node.range(),)*
                }
            }
        }

        impl SpannedMut for $name {
            fn range_mut(&mut self) -> &mut Range {
                match self {
                    $($name::$prop(node) => node.range_mut(),)*
                }
            }
        }

        impl Visitable for $name {
            fn accept<P, R>(&mut self, visitor: &mut (impl ASTVisitor<P, R> + ?Sized), p: &mut P) -> Option<R> {
                match self {
                    $($name::$prop(node) => node.accept(visitor, p),)*
                }
            }
        }
    };
}

macro_rules! impl_node {
    ($node_type:ident) => {
        impl AstNode for $node_type {
            fn typ(&self) -> NodeType {
                NodeType::$node_type
            }
        }

        impl Spanned for $node_type {
            fn range(&self) -> &Range {
                &self.range
            }
        }

        impl SpannedMut for $node_type {
            fn range_mut(&mut self) -> &mut Range {
                &mut self.range
            }
        }
    };
}

pub type SpannedNode<T> = (T, Range);

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
    BreakStmt,
    ContinueStmt,
    EmptyStmt,
    AssignExpr,
    CompoundAssignExpr,
    BinaryExpr,
    UnaryExpr,
    FuncCallExpr,
    MemberAccessExpr,
    IdentifierExpr,
    LiteralExpr,
    GroupingExpr,
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

pub trait Spanned {
    fn range(self: &Self) -> &Range;
}

pub trait SpannedMut {
    fn range_mut(self: &mut Self) -> &mut Range;
}

/// An AST node.
pub trait AstNode: Spanned {
    fn typ(self: &Self) -> NodeType;
}

/// An [ASTNode] which can be visited
pub trait Visitable {
    fn accept<P, R>(
        self: &mut Self,
        visitor: &mut (impl ASTVisitor<P, R> + ?Sized),
        p: &mut P,
    ) -> Option<R>;
}

def_node!(Program {
    decls: Vec<Decl>,
    stmts: Vec<Stmt>,
});

def_enum!(Decl {
    Class: ClassDecl,
    Func: FuncDecl,
    Stmt: Stmt,
});

def_node!(ClassDecl {
    name: IdentifierExpr,
    supercls: Option<IdentifierExpr>,
    methods: Vec<FuncDecl>,
    decl_type: DeclType,
});

def_node!(FuncDecl {
    name: IdentifierExpr,
    params: Vec<IdentifierExpr>,
    body: BlockStmt,
});

def_node!(DeclStmt {
    decl: Decl,
    typ: DeclType,
});

def_enum!(Stmt {
    Expr: ExprStmt,
    For: Box<ForStmt>,
    If: IfStmt,
    Print: PrintStmt,
    Return: ReturnStmt,
    While: WhileStmt,
    Var: VarStmt,
    Block: BlockStmt,
    Break: BreakStmt,
    Continue: ContinueStmt,
    Empty: EmptyStmt,
});

def_node!(EmptyStmt {});

def_node!(BreakStmt {
    label: Option<IdentifierExpr>
});

def_node!(ContinueStmt {
    label: Option<IdentifierExpr>
});

def_node!(VarStmt {
    name: IdentifierExpr,
    initializer: Option<Expr>,
});

def_node!(BlockStmt {
    decls: Vec<Decl>,
});

def_node!(ExprStmt { expr: Expr });

impl From<Expr> for ExprStmt {
    fn from(value: Expr) -> Self {
        let range = value.range().clone();
        return Self::new(value, range);
    }
}

def_node!(ForStmt {
    label: Option<IdentifierExpr>,
    init: Option<Stmt>,
    condition: Option<Expr>,
    step: Option<Expr>,
    body: BlockStmt,
});

def_node!(IfStmt {
    condition: Expr,
    then_branch: BlockStmt,
    else_branch: Option<BlockStmt>,
});

def_node!(PrintStmt { expr: Expr });

def_node!(ReturnStmt { expr: Expr });

def_node!(WhileStmt {
    label: Option<IdentifierExpr>,
    condition: Expr,
    body: BlockStmt,
});

def_enum!(Expr {
    Assign: Box<AssignExpr>,
    CompoundAssign: Box<CompoundAssignExpr>,
    Binary: Box<BinaryExpr>,
    Unary: Box<UnaryExpr>,
    FuncCall: Box<FuncCallExpr>,
    MemberAccess: Box<MemberAccessExpr>,
    Identifier: IdentifierExpr,
    Literal: LiteralExpr,
});

def_node!(AssignExpr {
    target: Expr,
    value: Expr,
});

def_node!(CompoundAssignExpr {
    target: Expr,
    op: BinaryOp,
    value: Expr,
});

def_node!(BinaryExpr {
    left: Expr,
    op: BinaryOp,
    right: Expr,
});

def_node!(UnaryExpr {
    op: UnaryOp,
    expr: Expr,
});

def_node!(FuncCallExpr {
    callee: Expr,
    args: Vec<Expr>,
});

def_node!(MemberAccessExpr {
    receiver: Expr,
    member: IdentifierExpr,
});

def_node!(IdentifierExpr {
    name: String,
    typ: IdentifierType
});

impl IdentifierExpr {
    pub fn ident_typ(&self) -> &IdentifierType {
        &self.typ
    }
}
#[derive(PartialEq, Clone, Debug)]
pub enum IdentifierType {
    ClassName,
    FuncName,
    ParamName,
    VarName,
    Keyword,
    Other,
}

impl IdentifierType {
    pub fn is_decl_name(&self) -> bool {
        match self {
            IdentifierType::ClassName
            | IdentifierType::FuncName
            | IdentifierType::ParamName
            | IdentifierType::VarName => true,

            IdentifierType::Keyword | IdentifierType::Other => false,
        }
    }
}

def_node!(GroupingExpr { expr: Box<Expr> });

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralExpr {
    Null(SpannedNode<()>),
    Bool(SpannedNode<bool>),
    Number(SpannedNode<f64>),
    String(SpannedNode<String>),
}

#[allow(non_snake_case)]
impl LiteralExpr {
    pub fn Bool(&self) -> Option<&SpannedNode<bool>> {
        if let LiteralExpr::Bool(s) = self {
            return Some(s);
        }
        None
    }

    pub fn Number(&self) -> Option<&SpannedNode<f64>> {
        if let LiteralExpr::Number(s) = self {
            return Some(s);
        }
        None
    }

    pub fn String(&self) -> Option<&SpannedNode<String>> {
        if let LiteralExpr::String(s) = self {
            return Some(s);
        }
        None
    }
}

impl AstNode for LiteralExpr {
    fn typ(&self) -> NodeType {
        NodeType::LiteralExpr
    }
}

impl Spanned for LiteralExpr {
    fn range(self: &Self) -> &Range {
        match self {
            LiteralExpr::Null(s) => &s.1,
            LiteralExpr::Bool(s) => &s.1,
            LiteralExpr::Number(s) => &s.1,
            LiteralExpr::String(s) => &s.1,
        }
    }
}

impl SpannedMut for LiteralExpr {
    fn range_mut(self: &mut Self) -> &mut Range {
        match self {
            LiteralExpr::Null(s) => &mut s.1,
            LiteralExpr::Bool(s) => &mut s.1,
            LiteralExpr::Number(s) => &mut s.1,
            LiteralExpr::String(s) => &mut s.1,
        }
    }
}

impl Display for LiteralExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralExpr::Null(_) => write!(f, "null"),
            LiteralExpr::Bool((b, _)) => write!(f, "{}", b),
            LiteralExpr::Number((n, _)) => write!(f, "{}", n),
            LiteralExpr::String((s, _)) => write!(f, "\"{}\"", s),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Or,
    And,
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
            BinaryOp::EqEq => "==",
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

            BinaryOp::EqEq
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

    /// Get the inverse of the binary comparison operator.
    pub fn inv_cmp(&self) -> Option<BinaryOp> {
        match self {
            BinaryOp::EqEq => Some(BinaryOp::NotEq),
            BinaryOp::NotEq => Some(BinaryOp::EqEq),
            BinaryOp::Gt => Some(BinaryOp::Lt),
            BinaryOp::GtEq => Some(BinaryOp::LtEq),
            BinaryOp::Lt => Some(BinaryOp::Gt),
            BinaryOp::LtEq => Some(BinaryOp::GtEq),
            _ => None,
        }
    }

    pub fn is_cmp(&self) -> bool {
        matches!(
            self,
            BinaryOp::EqEq
                | BinaryOp::NotEq
                | BinaryOp::Gt
                | BinaryOp::GtEq
                | BinaryOp::Lt
                | BinaryOp::LtEq
        )
    }

    pub fn is_cond(&self) -> bool {
        matches!(self, BinaryOp::And | BinaryOp::Or)
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
    /// Get the operands of the binary expression as numbers if the binary expression has [PrimaryExpr::Number] as its operands.
    pub fn get_num_operands(&self) -> Option<(&f64, &f64)> {
        match (&self.left, &self.right) {
            (Expr::Literal(left), Expr::Literal(right)) => match (&left, &right) {
                (LiteralExpr::Number((l, _)), LiteralExpr::Number((r, _))) => Some((l, r)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl_node!(Program);
impl_node!(ClassDecl);
impl_node!(FuncDecl);
impl_node!(ExprStmt);
impl_node!(ForStmt);
impl_node!(WhileStmt);
impl_node!(BreakStmt);
impl_node!(ContinueStmt);
impl_node!(IfStmt);
impl_node!(PrintStmt);
impl_node!(ReturnStmt);
impl_node!(VarStmt);
impl_node!(BlockStmt);
impl_node!(EmptyStmt);
impl_node!(AssignExpr);
impl_node!(CompoundAssignExpr);
impl_node!(BinaryExpr);
impl_node!(UnaryExpr);
impl_node!(FuncCallExpr);
impl_node!(MemberAccessExpr);
impl_node!(IdentifierExpr);
impl_node!(GroupingExpr);
