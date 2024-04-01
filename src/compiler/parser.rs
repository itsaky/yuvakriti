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

use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

use crate::yklang::compiler::ast::{BinaryExpr, ForStmt, IfStmt, ReturnStmt, Spanned, WhileStmt};
use crate::yklang::compiler::ast::BinaryOp;
use crate::yklang::compiler::ast::BlockStmt;
use crate::yklang::compiler::ast::Decl;
use crate::yklang::compiler::ast::Expr;
use crate::yklang::compiler::ast::ExprS;
use crate::yklang::compiler::ast::FuncDecl;
use crate::yklang::compiler::ast::PrimaryExpr;
use crate::yklang::compiler::ast::PrintStmt;
use crate::yklang::compiler::ast::Program;
use crate::yklang::compiler::ast::Stmt;
use crate::yklang::compiler::ast::UnaryExpr;
use crate::yklang::compiler::ast::UnaryOp;
use crate::yklang::compiler::ast::VarDecl;
use crate::yklang::compiler::diagnostics::{Diagnostic, DiagnosticHandler, DiagnosticKind};
use crate::yklang::compiler::lexer::YKLexer;
use crate::yklang::compiler::location::Range;
use crate::yklang::compiler::messages;
use crate::yklang::compiler::messages::{err_exp_kywrd, err_exp_sym};
use crate::yklang::compiler::tokens::{Token, TokenType};

pub(crate) struct YKParser<'a, R: Read> {
    lexer: YKLexer<'a, R>,
    diagnostics: Rc<RefCell<dyn DiagnosticHandler + 'a>>,

    // parser state
    current: Option<Token>,
    next: Option<Token>,
    
    has_error: bool,
}

impl <R: Read> YKParser<'_, R> {
    
    const LOOKAHEAD_SIZE: usize = 5;

    /// Create a new [YKParser] instance using the given [YKLexer].
    pub(crate) fn new(
        lexer: YKLexer<R>,
        diagnostics_handler: Rc<RefCell<dyn DiagnosticHandler>>,
    ) -> YKParser<R> {
        let mut parser = YKParser {
            lexer,
            diagnostics: diagnostics_handler,
            current: None,
            next: None,
            has_error: false,
        };
        
        // initialize the parser state
        // this initializes the first and second token in the source
        parser.advance();
        parser.advance();

        return parser;
    }
    
    fn report(
        &mut self,
        diagnostic_kind: DiagnosticKind,
        message: &str
    ) {
        self.diagnostics.borrow_mut().handle(self.create_diagnostic(diagnostic_kind, message));
    }

    fn create_diagnostic(
        &self,
        diagnostic_kind: DiagnosticKind,
        message: &str,
    ) -> Diagnostic {
        Diagnostic {
            range: self.peek().unwrap().range.clone(),
            message: String::from(message),
            kind: diagnostic_kind
        }
    }
}

impl <R: Read> YKParser<'_, R> {
    
    /// Parses the input source and returns the resulting [Program].
    pub(crate) fn parse(&mut self) -> Program {
        let declarations = self.decls();
        return Program {
            decls: declarations
        };
        
    }

    fn decls(&mut self) -> Vec<Decl> {
        let mut declarations: Vec<Decl> = Vec::new();
        loop {
            if self.peek().is_none() {
                // reached EOF
                break
            }

            if let Some(decl) = self.decl() {
                declarations.push(decl);
                continue;
            }

            break;
        }
        
        declarations
    }

    /// Returns the next declaration in the input source.
    fn decl(&mut self) -> Option<Decl> {
        let token = self.peek();
        return match token {
            Some(token) =>  match token.token_type {
                TokenType::Var => self.var_decl(),
                TokenType::Fun => self.fun_decl(),
                _ => {
                    let stmt = self.try_parse_stmt();
                    if stmt.is_none() {
                        self.report(DiagnosticKind::Error, messages::PARS_DECL_OR_STMT_EXPECTED);
                    }
                    stmt
                }
            },
            None => {
                self.report(DiagnosticKind::Error, messages::PARS_UNEXPECTED_EOF);
                None
            }
        };
    }
    
    /// Returns the next variable declaration in the input source.
    fn var_decl(&mut self) -> Option<Decl> {
        self.accept0(TokenType::Var, &err_exp_kywrd("var"))?;
        
        let var_name = self.accept(TokenType::Identifier, messages::PARS_EXPECTED_VAR_NAME)?;
        let mut init: Option<ExprS> = None;
        if self.tmatch(TokenType::Eq) != None {
            init = self.expr();
        }

        self.accept0(TokenType::Semicolon, &err_exp_sym(";"))?;
        
        Some(Decl::Var(VarDecl {
            name: var_name.text,
            initializer: init
        }))
    }
    
    fn fun_decl(&mut self) -> Option<Decl> {
        self.accept0(TokenType::Fun, &err_exp_kywrd("fun"))?;
        let fun_name = self.accept(TokenType::Identifier, messages::PARS_EXPECTED_FUN_NAME)?;
        let params = self.fun_params()?;
        let body = self.block(true)?;

        Some(Decl::Func(FuncDecl{
            name: fun_name.text,
            params,
            body
        }))
    }

    fn fun_params(&mut self) -> Option<Vec<String>> {
        self.accept(TokenType::LParen, &err_exp_sym("("))?;

        let mut params = Vec::new();
        if self.tmatch(TokenType::RParen).is_none() {
            loop {
                let param = self.accept(TokenType::Identifier, messages::PARS_EXPECTED_PARAM_NAME).unwrap();
                params.push(param.text);
                if self.tmatch(TokenType::Comma).is_none() {
                    break
                }
            }
        }

        self.accept(TokenType::RParen, &err_exp_sym(")"))?;

        return Some(params)
    }

    fn block(&mut self, require_lbrace: bool) -> Option<Spanned<BlockStmt>> {
        if require_lbrace {
            self.accept(TokenType::LBrace, &err_exp_sym("{"))?;
        }
        
        let mut decls = Vec::with_capacity(0);
        while let Some(peek) = self.peek() {
            if decls.capacity() == 0 {
                decls = Vec::with_capacity(1);
            }
            if peek.token_type == TokenType::RBrace {
                break
            }
            if let Some(decl) = self.decl() {
                decls.push(decl);
            }
        }
        
        let rbrace = self.accept(TokenType::RBrace, &err_exp_sym("}"))?;
        
        Some((BlockStmt {
            decls
        }, rbrace.range))
    }
    
    fn try_parse_stmt(&mut self) -> Option<Decl> {
        let token = self.peek()?;
        let token_type = &token.token_type;
        let mut range = token.range;
        let result = match token_type {
            TokenType::Print => self.print_stmt().map(|stmt| Stmt::Print(stmt)),
            TokenType::For => self.for_stmt().map(|stmt| { Stmt::For(stmt) }),
            TokenType::If => self.if_stmt().map(|stmt| { Stmt::If(stmt) }),
            TokenType::While => self.while_stmt().map(|stmt| { Stmt::While(stmt) }),
            TokenType::Return => self.return_stmt().map(|stmt| { Stmt::Return(stmt) }),
            TokenType::LBrace => self.block(true).map(|stmt| { Stmt::Block(stmt) }),
            _ => None
        }.map(|stmt| {
            let end = &match stmt {
                Stmt::Print(ref print) => print.expr.1,
                Stmt::For(ref for_stmt) => for_stmt.body.1,
                Stmt::Expr(ref expr) => expr.expr.1,
                Stmt::If(ref fi) => fi.else_branch.as_ref().unwrap_or(&fi.then_branch).1,
                Stmt::Return(ref ret) => ret.expr.1,
                Stmt::While(ref whil) => whil.body.1,
                Stmt::Block(ref blk) => blk.1,
            };

            return Decl::Stmt((stmt, range.set_end(end)))
        });

        self.accept0(TokenType::Semicolon, &err_exp_sym(";"))?;

        return result
    }

    fn print_stmt(&mut self) -> Option<PrintStmt> {
        self.accept0(TokenType::Print, &err_exp_kywrd("print"))?;
        let expro = self.expr();
        if expro.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
            return None
        }
        
        let expr = expro.unwrap();

        Some(PrintStmt {
            expr,
        })
    }
    
    fn for_stmt(&mut self) -> Option<ForStmt> {
        self.accept0(TokenType::For, &err_exp_kywrd("for"))?;
        self.accept0(TokenType::LParen, &err_exp_sym("("))?;
        
        let init = self.expr();
        self.accept0(TokenType::Semicolon, &err_exp_sym(";"))?;

        let condition = self.expr();
        self.accept0(TokenType::Semicolon, &err_exp_sym(";"))?;

        let step = self.expr();
        self.accept0(TokenType::LParen, &err_exp_sym(")"))?;
        
        let body = self.block(true);
        if body.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_BODY);
            return None
        }
        
        return Some(ForStmt {
            init,
            condition,
            step,
            body: body.unwrap()
        })
    }
    
    fn if_stmt(&mut self) -> Option<IfStmt> {
        self.accept0(TokenType::If, &err_exp_kywrd("if"))?;
        self.tmatch(TokenType::LParen);
        let condition = self.expr();
        self.tmatch(TokenType::RParen);
        
        let body = self.block(true);
        if body.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_BODY);
            return None
        }
        
        let mut else_body = None;
        if self.tmatch(TokenType::Else).is_some() {
            else_body = self.block(true);
            if else_body.is_none() {
                self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_BODY);
                return None
            }
        }
        
        return Some(IfStmt {
            condition: condition.unwrap(),
            then_branch: body.unwrap(),
            else_branch: else_body
        })
    }
    
    fn while_stmt(&mut self) -> Option<WhileStmt> {
        self.accept0(TokenType::While, &err_exp_kywrd("while"))?;
        self.tmatch(TokenType::LParen);
        let condition = self.expr();
        self.tmatch(TokenType::RParen);
        
        let body = self.block(true);
        if body.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_BODY);
            return None
        }
        
        return Some(WhileStmt {
            condition: condition.unwrap(),
            body: body.unwrap()
        })
    }
    
    fn return_stmt(&mut self) -> Option<ReturnStmt> {
        self.accept0(TokenType::Return, &err_exp_kywrd("return"))?;
        let expr = self.expr();
        if expr.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
            return None
        }
        
        Some(ReturnStmt {
            expr: expr.unwrap()
        })
    }

    fn expr(&mut self) -> Option<ExprS> {
        self.or()
    }

    fn or(&mut self) -> Option<ExprS> {
        let mut expr = self.and()?;

        while let Some(op) = self.tmatch_any(&[TokenType::Or]) {
            expr = self.binary_expr(expr, op, BinaryOp::Or, &Self::and)?;
        }

        Some(expr)
    }

    fn and(&mut self) -> Option<ExprS> {
        let mut expr = self.equality()?;

        while let Some(op) = self.tmatch_any(&[TokenType::And]) {
            expr = self.binary_expr(expr, op, BinaryOp::And, &Self::equality)?;
        }

        Some(expr)
    }

    fn equality(&mut self) -> Option<ExprS> {
        let mut expr = self.comparison()?;

        while let Some(op) = self.tmatch_any(&[TokenType::EqEq, TokenType::BangEq]) {
            let binary_op = if op.token_type == TokenType::EqEq {
                BinaryOp::EqEq
            } else {
                BinaryOp::NotEq
            };
            expr = self.binary_expr(expr, op, binary_op, &Self::comparison)?;
        }

        Some(expr)
    }

    fn comparison(&mut self) -> Option<ExprS> {
        let mut expr = self.term()?;

        while let Some(op) = self.tmatch_any(&[
            TokenType::Gt,
            TokenType::GtEq,
            TokenType::Lt,
            TokenType::LtEq,
        ]) {
            let binary_op = match op.token_type {
                TokenType::Gt => BinaryOp::Gt,
                TokenType::GtEq => BinaryOp::GtEq,
                TokenType::Lt => BinaryOp::Lt,
                TokenType::LtEq => BinaryOp::LtEq,
                _ => unreachable!(),
            };
            expr = self.binary_expr(expr, op, binary_op, &Self::term)?;
        }

        Some(expr)
    }

    fn term(&mut self) -> Option<ExprS> {
        let mut expr = self.factor()?;

        while let Some(op) = self.tmatch_any(&[TokenType::Plus, TokenType::Minus]) {
            let binary_op = if op.token_type == TokenType::Plus {
                BinaryOp::Plus
            } else {
                BinaryOp::Minus
            };
            expr = self.binary_expr(expr, op, binary_op, &Self::factor)?;
        }

        Some(expr)
    }

    fn factor(&mut self) -> Option<ExprS> {
        let mut expr = self.unary()?;

        while let Some(op) = self.tmatch_any(&[TokenType::Asterisk, TokenType::Slash]) {
            let binary_op = if op.token_type == TokenType::Asterisk {
                BinaryOp::Mult
            } else {
                BinaryOp::Div
            };
            expr = self.binary_expr(expr, op, binary_op, &Self::unary)?;
        }

        Some(expr)
    }

    fn unary(&mut self) -> Option<ExprS> {
        if let Some(op) = self.tmatch_any(&[TokenType::Bang, TokenType::Minus]) {
            return self.unary_op(&op);
        }

        self.primary()
    }

    fn primary(&mut self) -> Option<ExprS> {
        if let Some(token) = self.advance() {
            match token.token_type {
                TokenType::True => Some((Expr::Primary(Box::new((PrimaryExpr::True, token.range))), token.range)),
                TokenType::False => Some((Expr::Primary(Box::new((PrimaryExpr::False, token.range))), token.range)),
                TokenType::Nil => Some((Expr::Primary(Box::new((PrimaryExpr::Nil, token.range))), token.range)),
                TokenType::This => Some((Expr::Primary(Box::new((PrimaryExpr::This, token.range))), token.range)),
                TokenType::Number => Some((Expr::Primary(Box::new((PrimaryExpr::Number(token.text.parse().unwrap()), token.range))), token.range)),
                TokenType::String => Some((Expr::Primary(Box::new((PrimaryExpr::String(token.text), token.range))), token.range)),
                TokenType::Identifier => Some((Expr::Primary(Box::new((PrimaryExpr::Identifier(token.text), token.range))), token.range)),
                TokenType::LParen => self.grouping(),
                _ => {
                    self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
                    None
                }
            }
        } else {
            self.report(DiagnosticKind::Error, messages::PARS_UNEXPECTED_EOF);
            None
        }
    }

    fn binary_expr(
        &mut self,
        left: ExprS,
        _op: Token,
        op_type: BinaryOp,
        next_expr_fn: &dyn Fn(&mut Self) -> Option<ExprS>,
    ) -> Option<ExprS> {
        let (right, right_range) = next_expr_fn(self)?;
        let mut range = Range::new();
        range.set_start(&left.1);
        range.set_end(&right_range);

        let binary = BinaryExpr {
            left,
            op: op_type,
            right: (right, right_range),
        };

        Some((Expr::Binary(Box::new(binary)), range))
    }
    
    fn unary_op(&mut self, op: &Token) -> Option<ExprS> {
        if let Some(expr) = self.expr() {
            let mut range = Range::new();
            range.set_start(&op.range);
            range.set_end(&expr.1);

            let unary = UnaryExpr {
                op: UnaryOp::from_token(&op).unwrap(),
                expr
            };

            return Some((Expr::Unary(Box::new(unary)), range));
        }

        self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
        return None
    }
    
    fn grouping(&mut self) -> Option<ExprS> {
        if let Some(expr) = self.expr() {
            if self.tmatch(TokenType::RParen) != None {
                return Some(expr);
            }

            self.report(DiagnosticKind::Error, &err_exp_sym("}"));
            return None
        }

        self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
        return None
    }
    
    /// Consumes and returns the next token in the input source if it matches the given token type.
    /// Reports an error with the given error message otherwise.
    fn accept(&mut self, token: TokenType, err_msg: &str) -> Option<Token> {
        if self.peek()?.token_type == token {
            return self.advance();
        }
        
        self.report(DiagnosticKind::Error, err_msg);
        return None
    }

    fn accept0(&mut self, token: TokenType, err_msg: &String) -> Option<Token> {
        return self.accept(token, err_msg.as_str());
    }
    
    /// Similar to [YKParser::accept], but does not return the token.
    fn consume(&mut self, token: TokenType, err_msg: &str) {
        self.accept(token, err_msg);
    }

    fn consume0(&mut self, token: TokenType, err_msg: &String) {
        self.consume(token, err_msg.as_str());
    }
    
    /// Similar to [YKParser::accept], but does not report any error if the token does not match.
    fn tmatch(&mut self, token: TokenType) -> Option<Token> {
        if self.peek()?.token_type == token {
            return self.advance();
        }
        
        None
    }
    
    /// Similar to [YKParser::tmatch], but expects the next token to match any of the given token types.
    fn tmatch_any(&mut self, tokens: &[TokenType]) -> Option<Token> {
        let peek = self.peek()?;
        for token in tokens {
            if &peek.token_type == token {
                return self.advance();
            }
        }
        
        None
    }

    /// Returns the next token in the input source.
    fn advance(&mut self) -> Option<Token> {
        let result = self.current.take();
        self.current = self.next.take();
        self.next = self.lexer.next();
        return result;
    }
    
    fn peek(&self) -> Option<&Token> {
        return self.current.as_ref();
    }
    
    fn peek_next(&self) -> Option<&Token> {
        return self.next.as_ref();
    }
    
    /// Returns whether the current character represents an end-of-file (EOF).
    pub(crate) fn is_at_eof(&self) -> bool {
        return matches!(self.current, None);
    }
}