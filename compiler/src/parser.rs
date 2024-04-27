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

use std::io::Read;

use crate::ast::ArrayAccessExpr;
use crate::ast::ArrayExpr;
use crate::ast::AssignExpr;
use crate::ast::BinaryExpr;
use crate::ast::BinaryOp;
use crate::ast::BlockStmt;
use crate::ast::BreakStmt;
use crate::ast::CompoundAssignExpr;
use crate::ast::ContinueStmt;
use crate::ast::Decl;
use crate::ast::Expr;
use crate::ast::ExprStmt;
use crate::ast::ForStmt;
use crate::ast::FuncDecl;
use crate::ast::IdentifierExpr;
use crate::ast::IdentifierType;
use crate::ast::IfStmt;
use crate::ast::LiteralExpr;
use crate::ast::PrintStmt;
use crate::ast::Program;
use crate::ast::ReturnStmt;
use crate::ast::Spanned;
use crate::ast::SpannedMut;
use crate::ast::Stmt;
use crate::ast::UnaryExpr;
use crate::ast::UnaryOp;
use crate::ast::VarStmt;
use crate::ast::WhileStmt;
use crate::diagnostics::Diagnostic;
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
    has_error: bool,

    // parser state
    position: Position,
    current: Option<Token>,
    next: Option<Token>,
}

impl<R: Read> YKParser<'_, R> {

    /// Create a new [YKParser] instance using the given [YKLexer].
    pub fn new(lexer: YKLexer<R>) -> YKParser<R> {
        let mut parser = YKParser {
            lexer,
            has_error: false,
            position: Position::NO_POS,
            current: None,
            next: None,
        };

        // initialize the parser state
        // this initializes the first and second token in the source
        parser.advance();
        parser.advance();

        return parser;
    }

    fn report(&mut self, diagnostic_kind: DiagnosticKind, message: &str) {
        let is_error = diagnostic_kind == DiagnosticKind::Error;
        self.lexer.diagnostics
            .handle(self.create_diagnostic(diagnostic_kind, message));

        self.has_error = self.has_error || is_error;
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

    pub fn has_errors(&self) -> bool {
        return self.has_error;
    }
}

impl<R: Read> YKParser<'_, R> {
    /// Parses the input source and returns the resulting [Program].
    pub fn parse(&mut self) -> Program {
        let mut decls: Vec<Decl> = Vec::new();
        let mut stmts: Vec<Stmt> = Vec::new();
        loop {
            if self.peek().is_none() {
                // reached EOF
                break;
            }

            if let Some(node) = self.decl() {
                if let Decl::Stmt(stmt) = node {
                    stmts.push(stmt);
                } else {
                    decls.push(node)
                }
            }
        }

        return Program::new(decls, stmts, Range::NO_RANGE);
    }

    /// Returns the next declaration in the input source.
    fn decl(&mut self) -> Option<Decl> {
        let token = self.peek();

        if token.is_some_and(|t| &t.token_type == &TokenType::Semicolon) {
            self.advance();
            return None;
        }

        return match token {
            Some(token) => match token.token_type {
                TokenType::Fun => self.fun_decl(),
                _ => {
                    let stmt = self.try_stmt_decl();
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
    fn var_stmt(&mut self) -> Option<VarStmt> {
        let var = self.accept(TokenType::Var, &err_exp_kywrd("var"))?;
        let mut range = var.range;
        let var_name = self.accept(TokenType::Identifier, messages::PARS_EXPECTED_VAR_NAME)?;
        range.set_end(&var_name.range);

        let mut init: Option<Expr> = None;
        if self.tmatch(&TokenType::Eq) != None {
            init = self.expr();
            if let Some(expr) = &init {
                range.set_end(&expr.range());
            }
        }

        Some(VarStmt::new(
            IdentifierExpr::new(var_name.text, IdentifierType::VarName, var_name.range),
            init,
            range,
        ))
    }

    fn fun_decl(&mut self) -> Option<Decl> {
        let mut fun = self.accept(TokenType::Fun, &err_exp_kywrd("fun"))?;
        let fun_name = self.accept(TokenType::Identifier, messages::PARS_EXPECTED_FUN_NAME)?;
        let params = self.fun_params()?;
        let body = self.block()?;
        let end = body.range().end;

        Some(Decl::Func(FuncDecl::new(
            IdentifierExpr::new(fun_name.text, IdentifierType::FuncName, fun_name.range),
            params,
            body,
            fun.range.set_end_pos(&end),
        )))
    }

    fn fun_params(&mut self) -> Option<Vec<IdentifierExpr>> {
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
            params.push(IdentifierExpr::new(
                param.text,
                IdentifierType::ParamName,
                param.range,
            ));
            if self.tmatch(&TokenType::Comma).is_none() {
                break;
            }
        }

        self.accept(TokenType::RParen, &err_exp_sym(")"))?;

        return Some(params);
    }

    fn block(&mut self) -> Option<BlockStmt> {
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

        Some(BlockStmt::new(decls, start.set_end(&rbrace.range)))
    }

    fn try_stmt_decl(&mut self) -> Option<Decl> {
        let token = self.peek()?;
        let token_type = &token.token_type;

        let mut req_semi = false;

        // First check for statements which do not require semicolons
        let stmt = match token_type {
            TokenType::For => self.for_stmt().map(|stmt| Stmt::For(Box::new(stmt))),
            TokenType::If => self.if_stmt().map(|stmt| Stmt::If(stmt)),
            TokenType::While => self.while_stmt().map(|stmt| Stmt::While(stmt)),
            TokenType::LBrace => self.block().map(|stmt| Stmt::Block(stmt)),
            TokenType::Identifier => self.try_labeled_or_expr().or_else(|| {
                req_semi = true;
                self.expr().map(|expr| Stmt::Expr(ExprStmt::from(expr)))
            }),
            _ => {
                req_semi = true;
                match token_type {
                    TokenType::Print => self.print_stmt().map(|stmt| Stmt::Print(stmt)),
                    TokenType::Return => self.return_stmt().map(|stmt| Stmt::Return(stmt)),
                    TokenType::Var => self.var_stmt().map(|stmt| Stmt::Var(stmt)),
                    TokenType::Break => self._break().map(|stmt| Stmt::Break(stmt)),
                    TokenType::Continue => self._continue().map(|stmt| Stmt::Continue(stmt)),
                    _ => self.expr().map(|expr| Stmt::Expr(ExprStmt::from(expr))),
                }
            }
        }?;

        if req_semi {
            self.accept(TokenType::Semicolon, &err_exp_sym(";"))?;
        }

        return Some(Decl::Stmt(stmt));
    }

    fn try_labeled_or_expr(&mut self) -> Option<Stmt> {
        let curr = self.peek()?;
        let next = self.peek_next()?;

        // expected: 'label: <for>|<while>'
        if curr.token_type != TokenType::Identifier || next.token_type != TokenType::Colon {
            return None;
        }

        let label = match self.primary()? {
            Expr::Identifier(ident) => Some(ident),
            _ => return None,
        };

        self.tmatch(&TokenType::Colon).unwrap();

        let token = self.peek()?;

        match &token.token_type {
            TokenType::For => self.for_stmt().map(|mut stmt| {
                stmt.label = label;
                Stmt::For(Box::new(stmt))
            }),
            TokenType::While => self.while_stmt().map(|mut stmt| {
                stmt.label = label;
                Stmt::While(stmt)
            }),
            _ => None,
        }
    }

    fn _break(&mut self) -> Option<BreakStmt> {
        self._labeled_stmt(TokenType::Break, "break", &BreakStmt::new)
    }

    fn _continue(&mut self) -> Option<ContinueStmt> {
        self._labeled_stmt(TokenType::Continue, "continue", &ContinueStmt::new)
    }

    fn _labeled_stmt<LabeledT>(
        &mut self,
        tk: TokenType,
        name: &str,
        new: &dyn Fn(Option<IdentifierExpr>, Range) -> LabeledT,
    ) -> Option<LabeledT> {
        let token = self.accept(tk, &err_exp_kywrd(name))?;

        let mut label = None;

        if self
            .peek()
            .map(|t| t.token_type == TokenType::Identifier)
            .unwrap_or(false)
        {
            label = match self.primary().take() {
                Some(Expr::Identifier(ident)) => Some(ident),
                _ => {
                    self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_LABEL);
                    None
                }
            }
        }

        let mut range = token.range.clone();
        if let Some(l) = &label {
            range.set_end(l.range());
        }

        Some(new(label, range))
    }

    fn print_stmt(&mut self) -> Option<PrintStmt> {
        let token = self.accept(TokenType::Print, &err_exp_kywrd("print"))?;
        let expro = self.expr();
        if expro.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
            return None;
        }

        let expr = expro.unwrap();
        let range = token.range.clone().set_end(expr.range());

        Some(PrintStmt::new(expr, range))
    }

    fn for_stmt(&mut self) -> Option<ForStmt> {
        let start = self.accept(TokenType::For, &err_exp_kywrd("for"))?;
        self.accept(TokenType::LParen, &err_exp_sym("("))?;

        let token = self.peek()?;

        let init = if token.token_type == TokenType::Var {
            self.var_stmt().map(|var| Stmt::Var(var))
        } else {
            self.expr().map(|mut expr| {
                let range = expr.range_mut().to_owned();
                Stmt::Expr(ExprStmt::new(expr, range))
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

        let body = body.unwrap();
        let range = start.range.clone().set_end(body.range());

        return Some(ForStmt::new(None, init, condition, step, body, range));
    }

    fn if_stmt(&mut self) -> Option<IfStmt> {
        let token = self.accept(TokenType::If, &err_exp_kywrd("if"))?;
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

        let body = body.unwrap();
        let mut range = Range::new();
        range.update_range(body.range());

        if else_body.is_some() {
            range.update_range(else_body.as_ref().unwrap().range());
        }

        range.set_start(&token.range);

        return Some(IfStmt::new(condition.unwrap(), body, else_body, range));
    }

    fn while_stmt(&mut self) -> Option<WhileStmt> {
        let token = self.accept(TokenType::While, &err_exp_kywrd("while"))?;
        self.tmatch(&TokenType::LParen);
        let condition = self.expr();
        self.tmatch(&TokenType::RParen);

        let body = self.block();
        if body.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_BODY);
            return None;
        }

        let body = body.unwrap();
        let mut range = Range::from(body.range());
        range.set_start(&token.range);

        return Some(WhileStmt::new(None, condition.unwrap(), body, range));
    }

    fn return_stmt(&mut self) -> Option<ReturnStmt> {
        let token = self.accept(TokenType::Return, &err_exp_kywrd("return"))?;
        let expr = self.expr();
        if expr.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
            return None;
        }

        let expr = expr.unwrap();
        let mut range = Range::from(expr.range());
        range.set_start(&token.range);

        Some(ReturnStmt::new(expr, range))
    }

    fn expr(&mut self) -> Option<Expr> {
        self.assign()
    }

    fn assign(&mut self) -> Option<Expr> {
        let mut left = self.or()?;

        while let Some(tok) = self.tmatch_any(&[
            TokenType::Eq,
            TokenType::PlusEq,
            TokenType::MinusEq,
            TokenType::AsteriskEq,
            TokenType::SlashEq,
        ]) {
            let is_cassign = matches!(
                tok.token_type,
                TokenType::PlusEq | TokenType::MinusEq | TokenType::AsteriskEq | TokenType::SlashEq
            );
            let right = self.assign()?;
            let mut range = Range::new();
            range.set_start(&left.range());
            range.set_end(&right.range());

            left = if is_cassign {
                Expr::CompoundAssign(Box::from(CompoundAssignExpr::new(
                    left,
                    match &tok.token_type {
                        TokenType::PlusEq => BinaryOp::Plus,
                        TokenType::MinusEq => BinaryOp::Minus,
                        TokenType::AsteriskEq => BinaryOp::Mult,
                        TokenType::SlashEq => BinaryOp::Div,
                        _ => unreachable!(),
                    },
                    right,
                    range,
                )))
            } else {
                Expr::Assign(Box::from(AssignExpr::new(left, right, range)))
            };
        }

        return Some(left);
    }

    fn or(&mut self) -> Option<Expr> {
        self.gen_binary_expr(&Self::and, &TokenType::Or, &BinaryOp::Or, &Self::and)
    }

    fn and(&mut self) -> Option<Expr> {
        self.gen_binary_expr(
            &Self::equality,
            &TokenType::And,
            &BinaryOp::And,
            &Self::equality,
        )
    }

    fn equality(&mut self) -> Option<Expr> {
        self.gen_binary_expr_multi_op(
            &Self::comparison,
            &[TokenType::EqEq, TokenType::BangEq],
            &[BinaryOp::EqEq, BinaryOp::NotEq],
            &Self::comparison,
        )
    }

    fn comparison(&mut self) -> Option<Expr> {
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

    fn term(&mut self) -> Option<Expr> {
        self.gen_binary_expr_multi_op(
            &Self::factor,
            &[TokenType::Plus, TokenType::Minus],
            &[BinaryOp::Plus, BinaryOp::Minus],
            &Self::factor,
        )
    }

    fn factor(&mut self) -> Option<Expr> {
        self.gen_binary_expr_multi_op(
            &Self::unary,
            &[TokenType::Asterisk, TokenType::Slash],
            &[BinaryOp::Mult, BinaryOp::Div],
            &Self::unary,
        )
    }

    fn unary(&mut self) -> Option<Expr> {
        if let Some(op) = self.tmatch_any(&[TokenType::Bang, TokenType::Minus]) {
            return self.unary_op(&op);
        }

        self.array_access()
    }

    fn unary_op(&mut self, op: &Token) -> Option<Expr> {
        if let Some(expr) = self.unary() {
            let mut range = Range::new();
            range.set_start(&op.range);
            range.set_end(&expr.range());

            let unary = UnaryExpr::new(UnaryOp::from_token(&op).unwrap(), expr, range);

            return Some(Expr::Unary(Box::from(unary)));
        }

        self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
        return None;
    }

    fn array_access(&mut self) -> Option<Expr> {
        let expr = self.primary()?;

        if let Some(token) = self.tmatch(&TokenType::LBrack) {
            let mut range = token.range.clone();
            let idx = self.expr();
            if idx.is_none() {
                self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
                return None;
            }

            self.consume(TokenType::RBrack, &err_exp_sym("]"));

            let idx = idx.unwrap();
            range.set_end(idx.range());

            return Some(Expr::ArrayAccess(ArrayAccessExpr::new(
                Box::from(expr),
                Box::from(idx),
                range,
            )));
        }

        return Some(expr);
    }

    fn primary(&mut self) -> Option<Expr> {
        if let Some(token) = self.advance() {
            match token.token_type {
                TokenType::True => Some(Expr::Literal(LiteralExpr::Bool((true, token.range)))),
                TokenType::False => Some(Expr::Literal(LiteralExpr::Bool((false, token.range)))),
                TokenType::Null => Some(Expr::Literal(LiteralExpr::Null(((), token.range)))),
                TokenType::Number => Some(Expr::Literal(LiteralExpr::Number((
                    token.text.parse().unwrap(),
                    token.range,
                )))),
                TokenType::String => Some(Expr::Literal(LiteralExpr::String((
                    token.text,
                    token.range,
                )))),
                TokenType::This => Some(Expr::Identifier(IdentifierExpr::new(
                    String::from("this"),
                    IdentifierType::Keyword,
                    token.range,
                ))),
                TokenType::Identifier => Some(Expr::Identifier(IdentifierExpr::new(
                    token.text,
                    IdentifierType::Other,
                    token.range,
                ))),
                TokenType::LParen => self.grouping(),
                TokenType::LBrack => self.array(token),
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

    fn array(&mut self, token: Token) -> Option<Expr> {
        // empty array
        if self.tmatch(&TokenType::RBrack).is_some() {
            return Some(Expr::Array(ArrayExpr::new(vec![], token.range)));
        }

        let expr = self.expr();
        if expr.is_none() {
            self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
            return None;
        }

        let mut exprs = vec![expr.unwrap()];

        let mut comma_without_expr = false;
        while self.tmatch(&TokenType::Comma).is_some() {
            if self.tmatch(&TokenType::RBrack).is_some() {
                return Some(Expr::Array(ArrayExpr::new(exprs, token.range)));
            }

            let expr = self.expr();
            if expr.is_some() {
                exprs.push(expr.unwrap());
                continue;
            } else if comma_without_expr {
                self.report(DiagnosticKind::Error, messages::PARS_EXPECTED_EXPR);
            }

            comma_without_expr = true;
        }

        self.consume(TokenType::RBrack, &err_exp_sym("]"));

        return Some(Expr::Array(ArrayExpr::new(exprs, token.range)));
    }

    fn gen_binary_expr(
        &mut self,
        left_expr: &dyn Fn(&mut Self) -> Option<Expr>,
        token_op: &TokenType,
        binary_op: &BinaryOp,
        right_expr: &dyn Fn(&mut Self) -> Option<Expr>,
    ) -> Option<Expr> {
        let mut expr = left_expr(self)?;

        while self.tmatch(token_op).is_some() {
            expr = self.binary_expr(expr, binary_op, right_expr)?;
        }

        return Some(expr);
    }

    fn gen_binary_expr_multi_op(
        &mut self,
        left_expr: &dyn Fn(&mut Self) -> Option<Expr>,
        token_op: &[TokenType],
        binary_op: &[BinaryOp],
        right_expr: &dyn Fn(&mut Self) -> Option<Expr>,
    ) -> Option<Expr> {
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
            expr = self.binary_expr(expr, bop, right_expr)?;
        }

        return Some(expr);
    }

    fn binary_expr(
        &mut self,
        left: Expr,
        op_type: &BinaryOp,
        next_expr_fn: &dyn Fn(&mut Self) -> Option<Expr>,
    ) -> Option<Expr> {
        let right = next_expr_fn(self)?;
        let mut range = Range::new();
        range.set_start(&left.range());
        range.set_end(&right.range());

        let expr: Expr = Expr::Binary(Box::from(BinaryExpr::new(
            left,
            op_type.clone(),
            right,
            range,
        )));

        Some(expr)
    }

    fn grouping(&mut self) -> Option<Expr> {
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

        if self
            .peek()
            .map(|t| t.token_type == TokenType::Comment)
            .unwrap_or(false)
        {
            return self.advance();
        }

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
