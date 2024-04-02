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

#[macro_export]
macro_rules! spanned {
    ($value:expr) => {
        ($value, $crate::compiler::location::Range::new())
    };
    ($value:expr, $range:expr) => {
        ($value, $range)
    };
}

#[macro_export]
macro_rules! identifier {
    ($value:expr $(, $range:expr)?) => {
        $crate::spanned!($value, $($range)?)
    };
}

#[macro_export]
macro_rules! program {
    ($($decl:expr),* $(,)?) => {
        $crate::compiler::ast::Program {
            decls: vec![$($decl),*]
        }
    };
}

#[macro_export]
macro_rules! decl {
    (class $name:expr, $($supercls:expr)? $(, $range:expr)? $(, $method:expr),+ $(,)?) => {
        $crate::compiler::ast::Decl::Class($crate::compiler::ast::ClassDecl {
            name: $name,
            supercls: $($supercls)?,
            methods: vec![$($crate::spanned!($method, $($range)?)),*],
        })
    };
    (func $name:expr, $body:expr $(, $range:expr)? $(, $param:expr),* $(,)?) => {
        $crate::compiler::ast::Decl::Func($crate::compiler::ast::FuncDecl {
            name: $name,
            params: vec![$($param),*],
            body: $crate::spanned!($body, $($range)?),
        })
    };
    ($stmt:expr) => {
        $crate::compiler::ast::Decl::Stmt($stmt)
    };
}

#[macro_export]
macro_rules! var_stmt {
    ($name:expr) => {
        $crate::compiler::ast::VarStmt {
            name: $name,
            initializer: None
        }
    };
    ($name:expr $(, $initializer:expr)?) => {
        $crate::compiler::ast::VarStmt {
            name: $name,
            initializer: $($initializer.map(|expr| $crate::spanned!(expr)))?
        }
    };
    ($name:expr $(, $initializer:expr)?, $range:expr) => {
        $crate::compiler::ast::VarStmt {
            name: $name,
            initializer: $($initializer.map(|expr| $crate::spanned!(expr, $range)))?
        }
    };
}

#[macro_export]
macro_rules! block_stmt {
    ($($decl:expr $(, $range:expr)?),* $(,)?) => {
        $crate::compiler::ast::BlockStmt {
            decls: vec![$($crate::spanned!($decl, $($range)?)),*]
        }
    };
}

#[macro_export]
macro_rules! stmt {
    (expr $expr:expr $(, $range:expr)?) => {
        $crate::compiler::ast::Stmt::Expr($crate::compiler::ast::ExprStmt {
            expr: $crate::spanned!($expr $($range)?)
        })
    };
    (for $($init:expr)?, $($condition:expr)?, $($step:expr)?, $body:expr $(, $range:expr)?) => {
        $crate::compiler::ast::Stmt::For(Box::new($crate::compiler::ast::ForStmt {
            init: $init.map(|stmt| $crate::spanned!(stmt, $($range)?)),
            condition: $condition.map(|expr| $crate::spanned!(expr, $($range)?)),
            step: $step.map(|expr| $crate::spanned!(expr, $($range)?)),
            body: $crate::spanned!($body, $($range)?),
        }))
    };
    (if $condition:expr $(, $range_condition:expr)?, $then_branch:expr $(, $range_then:expr)? $(, $else_branch:expr)? $(, $range_else:expr)?) => {
        $crate::compiler::ast::Stmt::If($crate::compiler::ast::IfStmt {
            condition: $crate::spanned!($condition, $($range_condition)?),
            then_branch: $crate::spanned!($then_branch, $($range_then)?),
            else_branch: $else_branch.map(|block| $crate::spanned!(block, $($range_else)?)),
        })
    };
    (print $expr:expr $(, $range:expr)?) => {
        $crate::compiler::ast::Stmt::Print($crate::compiler::ast::PrintStmt {
            expr: $crate::spanned!($expr, $($range)?)
        })
    };
    (return $expr:expr $(, $range:expr)?) => {
        $crate::compiler::ast::Stmt::Return($crate::compiler::ast::ReturnStmt {
            expr: $crate::spanned!($expr, $($range)?)
        })
    };
    (while $condition:expr $(, $range_condition:expr)?, $body:expr $(, $range_body:expr)?) => {
        $crate::compiler::ast::Stmt::While($crate::compiler::ast::WhileStmt {
            condition: $crate::spanned!($condition, $($range_condition)?),
            body: $crate::spanned!($body, $($range_body)?),
        })
    };
    (var $var:expr) => {
        $crate::compiler::ast::Stmt::Var($var)
    };
    (block $block:expr $(, $range:expr)?) => {
        $crate::compiler::ast::Stmt::Block($crate::spanned!($block, $($range)?))
    };
}

#[macro_export]
macro_rules! expr {
    (binary $left:expr $(, $range_left:expr)?, $op:ident, $right:expr $(, $range_right:expr)? $(, $range:expr)?) => {
        $crate::compiler::ast::Expr::Binary(Box::new($crate::compiler::ast::BinaryExpr {
            left: $crate::spanned!($left, $($range_left)?),
            op: $crate::compiler::ast::BinaryOp::$op,
            right: $crate::spanned!($right, $($range_right)?),
        }))
    };
    (unary $op:ident, $expr:expr $(, $range:expr)?) => {
        $crate::compiler::ast::Expr::Unary(Box::new($crate::compiler::ast::UnaryExpr {
            op: $crate::compiler::ast::UnaryOp::$op,
            expr: $crate::spanned!($expr $($range)?),
        }))
    };
    (call $callee:expr, $($arg:expr),* $(,)?) => {
        $crate::compiler::ast::Expr::FuncCall(Box::new($crate::compiler::ast::FuncCallExpr {
            callee: $callee,
            args: vec![$($arg),*],
        }))
    };
    (member $receiver:expr, $member:expr) => {
        $crate::compiler::ast::Expr::MemberAccess(Box::new($crate::compiler::ast::MemberAccessExpr {
            receiver: $receiver,
            member: $member,
        }))
    };
    (primary $value:expr) => {
        $crate::compiler::ast::Expr::Primary(Box::new($value))
    };
}

#[macro_export]
macro_rules! primary_expr {
    (true) => {
        spanned!($crate::compiler::ast::PrimaryExpr::True, $crate::compiler::location::Range::NO_RANGE)
    };
    (true $range:expr) => {
        spanned!($crate::compiler::ast::PrimaryExpr::True, $range)
    };
    (false) => {
        spanned!($crate::compiler::ast::PrimaryExpr::False, $crate::compiler::location::Range::NO_RANGE)
    };
    (false $range:expr) => {
        spanned!($crate::compiler::ast::PrimaryExpr::False, $range)
    };
    (nil) => {
        spanned!($crate::compiler::ast::PrimaryExpr::Nil, $crate::compiler::location::Range::NO_RANGE)
    };
    (nil $range:expr) => {
        spanned!($crate::compiler::ast::PrimaryExpr::Nil, $range)
    };
    (this) => {
        spanned!($crate::compiler::ast::PrimaryExpr::This, $crate::compiler::location::Range::NO_RANGE)
    };
    (this $range:expr) => {
        spanned!($crate::compiler::ast::PrimaryExpr::This, $range)
    };
    (num $num:expr) => {
        spanned!($crate::compiler::ast::PrimaryExpr::Number($num), $crate::compiler::location::Range::NO_RANGE)
    };
    (num $num:expr, $range:expr) => {
        spanned!($crate::compiler::ast::PrimaryExpr::Number($num), $range)
    };
    (str $str:expr) => {
        spanned!($crate::compiler::ast::PrimaryExpr::String($str), $crate::compiler::location::Range::NO_RANGE)
    };
    (str $str:expr, $range:expr) => {
        spanned!($crate::compiler::ast::PrimaryExpr::String($str), $range)
    };
    (ident $ident:expr) => {
        spanned!($crate::compiler::ast::PrimaryExpr::Identifier($ident), $crate::compiler::location::Range::NO_RANGE)
    };
    (ident $ident:expr, $range:expr) => {
        spanned!($crate::compiler::ast::PrimaryExpr::Identifier($ident), $range)
    };
    (group $expr:expr) => {
        spanned!($crate::compiler::ast::PrimaryExpr::Grouping($expr), $crate::compiler::location::Range::NO_RANGE)
    };
    (group $expr:expr, $range:expr) => {
        spanned!($crate::compiler::ast::PrimaryExpr::Grouping($expr), $range)
    };
}