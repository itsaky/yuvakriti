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

use crate::ast::BinaryExpr;
use crate::ast::BinaryOp;
use crate::ast::BlockStmt;
use crate::ast::Decl;
use crate::ast::DeclS;
use crate::ast::Expr;
use crate::ast::ExprS;
use crate::ast::ExprStmt;
use crate::ast::ForStmt;
use crate::ast::FuncDecl;
use crate::ast::Identifier;
use crate::ast::IfStmt;
use crate::ast::PrimaryExpr;
use crate::ast::PrintStmt;
use crate::ast::Program;
use crate::ast::ReturnStmt;
use crate::ast::Spanned;
use crate::ast::Stmt;
use crate::ast::UnaryExpr;
use crate::ast::UnaryOp;
use crate::ast::VarStmt;
use crate::ast::WhileStmt;
use crate::diagnostics::Diagnostic;
use crate::diagnostics::DiagnosticHandler;
use crate::diagnostics::DiagnosticKind;
use crate::lexer::YKLexer;
use crate::location::Position;
use crate::location::Range;
use crate::messages;
use crate::messages::err_exp_kywrd;
use crate::messages::err_exp_sym;
use crate::tokens::Token;
use crate::tokens::TokenType;

pub struct YKParser<'a, R: Read> {
    lexer: YKLexer<'a, R>,
    diagnostics: Rc<RefCell<dyn DiagnosticHandler + 'a>>,

    // parser state
    position: Position,
    current: Option<Token>,
    next: Option<Token>,
}

impl<R: Read> YKParser<'_, R> {
    /// Create a new [YKParser] instance using the given [YKLexer].
    pub fn new(
        lexer: YKLexer<R>,
        diagnostics_handler: Rc<RefCell<dyn DiagnosticHandler>>,
    ) -> YKParser<R> {
        let mut parser = YKParser {
            lexer,
            diagnostics: diagnostics_handler,
            position: Position::NO_POS,
            current: None,
            next: None,
            // has_error: false,
        };

        // initialize the parser state
        // this initializes the first and second token in the source
        parser.advance();
        parser.advance();

        return parser;
    }

    fn report(&mut self, diagnostic_kind: DiagnosticKind, message: &str) {
        self.diagnostics
            .borrow_mut()
            .handle(self.create_diagnostic(diagnostic_kind, message));
    }

    fn create_diagnostic(&self, diagnostic_kind: DiagnosticKind, message: &str) -> Diagnostic {
        let range = self
            .peek()
            .map(|tkn| tkn.range)
            .or_else(|| {
                Some(
                    Range::new()
                        .set_start_pos(&self.position)
                        .set_end_pos(&self.position),
                )
            })
            .expect("Expected token");

        Diagnostic {
            range: range.clone(),
            message: String::from(message),
            kind: diagnostic_kind,
        }
    }
}

impl<R: Read> YKParser<'_, R> {
    /// Parses the input source and returns the resulting [Program].
    pub fn parse(&mut self) -> Program {
        let declarations = self.decls();
        return Program {
            decls: declarations,
        };
    }

    fn decls(&mut self) -> Vec<DeclS> {
        let mut declarations: Vec<DeclS> = Vec::new();
        loop {
            if self.peek().is_none() {
                // reached EOF
                break;
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
    fn decl(&mut self) -> Option<DeclS> {
        let token = self.peek();
        return match token {
            Some(token) => match token.token_type {
                TokenType::Fun => self.fun_decl(),
                _ => {
                    let stmt = self.try_parse_stmt_decl();
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

    fn var_stmt(&mut self) -> Option<VarStmt> {
        return self.var_stmts().map(|stmts| stmts.0);
    }

    /// Returns the next variable declaration in the input source.
    fn var_stmts(&mut self) -> Option<Spanned<VarStmt>> {
        let var = self.accept(TokenType::Var, &err_exp_kywrd("var"))?;
        let mut range = var.range;
        let var_name = self.accept(TokenType::Identifier, messages::PARS_EXPECTED_VAR_NAME)?;
        range.set_end(&var_name.range);

        let mut init: Option<ExprS> = None;
        if self.tmatch(&TokenType::Eq) != None {
            init = self.expr();
            if let Some(expr) = &init {
                range.set_end(&expr.1);
            }
        }

        Some((
            VarStmt {
                name: (var_name.text, var_name.range),
                initializer: init,
            },
            range,
        ))
    }

    fn fun_decl(&mut self) -> Option<DeclS> {
        let mut fun = self.accept(TokenType::Fun, &err_exp_kywrd("fun"))?;
        let fun_name = self.accept(TokenType::Identifier, messages::PARS_EXPECTED_FUN_NAME)?;
        let params = self.fun_params()?;
        let body = self.block()?;
        let end = body.1.end;

        Some((
            Decl::Func(FuncDecl {
                name: (fun_name.text, fun_name.range),
                params,
                body,
            }),
            fun.range.set_end_pos(&end),
        ))
    }

    fn fun_params(&mut self) -> Option<Vec<Identifier>> {
        self.accept(TokenType::LParen, &err_exp_sym("("))?;
        let mut params = Vec::new();
        if self.peek()?.token_type == TokenType::RParen {
            // no params in func
            self.tmatch(&TokenType::RParen);
            return Some(params);
        }

        loop {
            let param = self
                .accept(TokenType::Identifier, messages::PARS_EXPECTED_PARAM_NAME)
                .unwrap();
            params.push((param.text, param.range));
            if self.tmatch(&TokenType::Comma).is_none() {
                break;
            }
        }

        self.accept(TokenType::RParen, &err_exp_sym(")"))?;

        return Some(params);
    }

    fn block(&mut self) -> Option<Spanned<BlockStmt>> {
        let mut start = self.accept(TokenType::LBrace, &err_exp_sym("{"))?.range;

        let mut decls = Vec::with_capacity(0);
        while let Some(peek) = self.peek() {
            if decls.capacity() == 0 {
                decls = Vec::with_capacity(1);
            }
            if peek.token_type == TokenType::RBrace {
                break;
            }
            if let Some(decl) = self.decl() {
                decls.push(decl);
            }
        }

        let rbrace = self.accept(TokenType::RBrace, &err_exp_sym("}"))?;

        Some((BlockStmt { decls }, start.set_end(&rbrace.range)))
    }

    fn try_parse_stmt_decl(&mut self) -> Option<DeclS> {
        let token = self.peek()?;
        let token_type = &token.token_type;
        let mut range = token.range;

        let mut req_semi = false;

        // First check for statements which do not require semicolons
        let stmt = match token_type {
            TokenType::For => self.for_stmt().map(|stmt| Stmt::For(Box::new(stmt))),
            TokenType::If => self.if_stmt().map(|stmt| Stmt::If(stmt)),
            TokenType::While => self.while_stmt().map(|stmt| Stmt::While(stmt)),
            TokenType::LBrace => self.block().map(|stmt| Stmt::Block(stmt)),
            _ => {
                req_semi = true;
                match token_type {
                    TokenType::Print => self.print_stmt().map(|stmt| Stmt::Print(stmt)),
                    TokenType::Return => self.return_stmt().map(|stmt| Stmt::Return(stmt)),
                    TokenType::Var => self.var_stmt().map(|stmt| Stmt::Var(stmt)),
                    _ => self.expr().map(|expr| Stmt::Expr(ExprStmt { expr })),
                }
            }
        }?;

        let end = &match stmt {
            Stmt::Print(ref print) => print.expr.1,
            Stmt::For(ref for_stmt) => for_stmt.body.1,
            Stmt::Expr(ref expr) => expr.expr.1,
            Stmt::If(ref fi) => fi.else_branch.as_ref().unwrap_or(&fi.then_branch).1,
            Stmt::Return(ref ret) => ret.expr.1,
            Stmt::While(ref whil) => whil.body.1,
            Stmt::Var(ref var) => {
                if let Some(init) = &var.initializer {
                    init.1
                } else {
                    var.name.1
                }
            }
            Stmt::Block(ref blk) => blk.1,
        };

        if req_semi {
            self.accept(TokenType::Semicolon, &err_exp_sym(";"))?;
        }

        return Some((Decl::Stmt(stmt), range.set_end(end)));
    }

    fn print_stmt(&mut self) -> Option<PrintStmt> {
        self.accept(TokenType::Print, &err_exp_kywrd("print"))?;
        let expro = self.expr();
        if expro.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
            return None;
        }

        let expr = expro.unwrap();

        Some(PrintStmt { expr })
    }

    fn for_stmt(&mut self) -> Option<ForStmt> {
        self.accept(TokenType::For, &err_exp_kywrd("for"))?;
        self.accept(TokenType::LParen, &err_exp_sym("("))?;

        let token = self.peek()?;

        let init = if token.token_type == TokenType::Var {
            self.var_stmts().map(|var| (Stmt::Var(var.0), var.1))
        } else {
            self.expr().map(|expr| {
                let range = expr.1;
                (Stmt::Expr(ExprStmt { expr }), range)
            })
        };

        self.accept(TokenType::Semicolon, &err_exp_sym(";"))?;

        let condition = self.expr();
        self.accept(TokenType::Semicolon, &err_exp_sym(";"))?;

        let step = self.expr();
        self.accept(TokenType::RParen, &err_exp_sym(")"))?;

        let body = self.block();
        if body.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_BODY);
            return None;
        }

        return Some(ForStmt {
            init,
            condition,
            step,
            body: body.unwrap(),
        });
    }

    fn if_stmt(&mut self) -> Option<IfStmt> {
        self.accept(TokenType::If, &err_exp_kywrd("if"))?;
        self.tmatch(&TokenType::LParen);
        let condition = self.expr();
        self.tmatch(&TokenType::RParen);

        let body = self.block();
        if body.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_BODY);
            return None;
        }

        let mut else_body = None;
        if self.tmatch(&TokenType::Else).is_some() {
            else_body = self.block();
            if else_body.is_none() {
                self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_BODY);
                return None;
            }
        }

        return Some(IfStmt {
            condition: condition.unwrap(),
            then_branch: body.unwrap(),
            else_branch: else_body,
        });
    }

    fn while_stmt(&mut self) -> Option<WhileStmt> {
        self.accept(TokenType::While, &err_exp_kywrd("while"))?;
        self.tmatch(&TokenType::LParen);
        let condition = self.expr();
        self.tmatch(&TokenType::RParen);

        let body = self.block();
        if body.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_BODY);
            return None;
        }

        return Some(WhileStmt {
            condition: condition.unwrap(),
            body: body.unwrap(),
        });
    }

    fn return_stmt(&mut self) -> Option<ReturnStmt> {
        self.accept(TokenType::Return, &err_exp_kywrd("return"))?;
        let expr = self.expr();
        if expr.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
            return None;
        }

        Some(ReturnStmt {
            expr: expr.unwrap(),
        })
    }

    fn expr(&mut self) -> Option<ExprS> {
        self.assign()
    }

    fn assign(&mut self) -> Option<ExprS> {
        self.gen_binary_expr(&Self::or, &TokenType::Eq, &BinaryOp::Eq, &Self::assign)
    }

    fn or(&mut self) -> Option<ExprS> {
        self.gen_binary_expr(&Self::and, &TokenType::Or, &BinaryOp::Or, &Self::and)
    }

    fn and(&mut self) -> Option<ExprS> {
        self.gen_binary_expr(
            &Self::equality,
            &TokenType::And,
            &BinaryOp::And,
            &Self::equality,
        )
    }

    fn equality(&mut self) -> Option<ExprS> {
        self.gen_binary_expr_multi_op(
            &Self::comparison,
            &[TokenType::EqEq, TokenType::BangEq],
            &[BinaryOp::EqEq, BinaryOp::NotEq],
            &Self::comparison,
        )
    }

    fn comparison(&mut self) -> Option<ExprS> {
        self.gen_binary_expr_multi_op(
            &Self::term,
            &[
                TokenType::Gt,
                TokenType::GtEq,
                TokenType::Lt,
                TokenType::LtEq,
            ],
            &[BinaryOp::Gt, BinaryOp::GtEq, BinaryOp::Lt, BinaryOp::LtEq],
            &Self::term,
        )
    }

    fn term(&mut self) -> Option<ExprS> {
        self.gen_binary_expr_multi_op(
            &Self::factor,
            &[TokenType::Plus, TokenType::Minus],
            &[BinaryOp::Plus, BinaryOp::Minus],
            &Self::factor,
        )
    }

    fn factor(&mut self) -> Option<ExprS> {
        self.gen_binary_expr_multi_op(
            &Self::unary,
            &[TokenType::Asterisk, TokenType::Slash],
            &[BinaryOp::Mult, BinaryOp::Div],
            &Self::unary,
        )
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
                TokenType::True => Some((
                    Expr::Primary(Box::new((PrimaryExpr::True, token.range))),
                    token.range,
                )),
                TokenType::False => Some((
                    Expr::Primary(Box::new((PrimaryExpr::False, token.range))),
                    token.range,
                )),
                TokenType::Nil => Some((
                    Expr::Primary(Box::new((PrimaryExpr::Nil, token.range))),
                    token.range,
                )),
                TokenType::This => Some((
                    Expr::Primary(Box::new((PrimaryExpr::This, token.range))),
                    token.range,
                )),
                TokenType::Number => Some((
                    Expr::Primary(Box::new((
                        PrimaryExpr::Number(token.text.parse().unwrap()),
                        token.range,
                    ))),
                    token.range,
                )),
                TokenType::String => Some((
                    Expr::Primary(Box::new((PrimaryExpr::String(token.text), token.range))),
                    token.range,
                )),
                TokenType::Identifier => Some((
                    Expr::Primary(Box::new((PrimaryExpr::Identifier(token.text), token.range))),
                    token.range,
                )),
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

    fn gen_binary_expr(
        &mut self,
        left_expr: &dyn Fn(&mut Self) -> Option<ExprS>,
        token_op: &TokenType,
        binary_op: &BinaryOp,
        right_expr: &dyn Fn(&mut Self) -> Option<ExprS>,
    ) -> Option<ExprS> {
        let mut expr = left_expr(self)?;

        while let Some(eq) = self.tmatch(token_op) {
            expr = self.binary_expr(expr, eq, binary_op, right_expr)?;
        }

        return Some(expr);
    }

    fn gen_binary_expr_multi_op(
        &mut self,
        left_expr: &dyn Fn(&mut Self) -> Option<ExprS>,
        token_op: &[TokenType],
        binary_op: &[BinaryOp],
        right_expr: &dyn Fn(&mut Self) -> Option<ExprS>,
    ) -> Option<ExprS> {
        if token_op.len() != binary_op.len() {
            panic!("token_op and binary_op must have same length");
        }

        let mut expr = left_expr(self)?;
        while let Some(op) = self.tmatch_any(token_op) {
            let index = token_op
                .iter()
                .position(|typ| *typ == op.token_type)
                .unwrap();
            let bop = &binary_op[index];
            expr = self.binary_expr(expr, op, bop, right_expr)?;
        }

        return Some(expr);
    }

    fn binary_expr(
        &mut self,
        left: ExprS,
        _op: Token,
        op_type: &BinaryOp,
        next_expr_fn: &dyn Fn(&mut Self) -> Option<ExprS>,
    ) -> Option<ExprS> {
        let (right, right_range) = next_expr_fn(self)?;
        let mut range = Range::new();
        range.set_start(&left.1);
        range.set_end(&right_range);

        let binary = BinaryExpr {
            left,
            op: op_type.clone(),
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
                expr,
            };

            return Some((Expr::Unary(Box::new(unary)), range));
        }

        self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
        return None;
    }

    fn grouping(&mut self) -> Option<ExprS> {
        if let Some(expr) = self.expr() {
            if self.tmatch(&TokenType::RParen).is_some() {
                return Some(expr);
            }

            self.report(DiagnosticKind::Error, &err_exp_sym(")"));
            return None;
        }

        self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
        return None;
    }

    /// Consumes and returns the next token in the input source if it matches the given token type.
    /// Reports an error with the given error message otherwise.
    fn accept(&mut self, expected: TokenType, err_msg: &str) -> Option<Token> {
        if let Some(token) = self.peek() {
            if token.token_type == expected {
                return self.advance();
            }
        }

        self.report(DiagnosticKind::Error, err_msg);
        return None;
    }

    /// Similar to [YKParser::accept], but does not return the token.
    #[allow(unused)]
    fn consume(&mut self, token: TokenType, err_msg: &str) {
        self.accept(token, err_msg);
    }

    #[allow(unused)]
    fn consume0(&mut self, token: TokenType, err_msg: &String) {
        self.consume(token, err_msg.as_str());
    }

    /// Similar to [YKParser::accept], but does not report any error if the token does not match.
    fn tmatch(&mut self, token: &TokenType) -> Option<Token> {
        if &self.peek()?.token_type == token {
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
        self.position = result
            .as_ref()
            .map(|token| token.range.end)
            .unwrap_or(self.position);
        self.current = self.next.take();
        self.next = self.lexer.next();
        return result;
    }

    fn peek(&self) -> Option<&Token> {
        return self.current.as_ref();
    }

    #[allow(unused)]
    fn peek_next(&self) -> Option<&Token> {
        return self.next.as_ref();
    }

    /// Returns whether the current character represents an end-of-file (EOF).
    pub fn is_at_eof(&self) -> bool {
        return matches!(self.current, None);
    }
}
