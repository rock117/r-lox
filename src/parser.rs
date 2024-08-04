use crate::error::ParseError;
use crate::expr::Expr;
use crate::lox::Lox;
use crate::object::Object;
use crate::stmt::Stmt;
use crate::token::token_type::TokenType;
use crate::token::token_type::TokenType::*;
use crate::token::Token;

#[derive(Default)]
pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub(crate) fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        return Ok(statements);
    }
    /// grammar expression → equality;
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_(&[PRINT]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::print(value))
    }
    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(SEMICOLON, "Expect ';' after expression.")?;
        Ok(Stmt::expression(expr))
    }

    /// grammar: equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison();
        while self.match_(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Ok(Expr::binary(expr?, operator, right?));
        }
        return expr;
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term();
        while self.match_(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Ok(Expr::binary(expr?, operator, right?));
        }
        return expr;
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor();
        while self.match_(&[MINUS, PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Ok(Expr::binary(expr?, operator, right?));
        }
        return expr;
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary();
        while self.match_(&[SLASH, STAR]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Ok(Expr::binary(expr?, operator, right?));
        }
        return expr;
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_(&[BANG, MINUS]) {
            let operator = self.previous().clone(); // TODO
            let right = self.unary()?;
            return Ok(Expr::unary(operator, right));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_(&[FALSE]) {
            return Ok(Expr::literal(Some(Object::Boolean(false))));
        }
        if self.match_(&[TRUE]) {
            return Ok(Expr::literal(Some(Object::Boolean(true))));
        }
        if self.match_(&[NIL]) {
            return Ok(Expr::literal(None));
        }
        if self.match_(&[NUMBER, STRING]) {
            return Ok(Expr::literal(self.previous().literal.clone()));
        }

        if self.match_(&[LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Expr::grouping(expr));
        } else {
            return Err(self.error(self.peek().clone(), "Expect expression."));
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance().clone()); // TODO
        } else {
            Err(self.error(self.peek().clone(), msg))
        }
    }
    fn error(&self, token: Token, msg: &str) -> ParseError {
        Lox::error_(&token, msg);
        ParseError::new(token, msg.into())
    }

    /// discards tokens until found a statement boundary
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().r#type == SEMICOLON {
                return;
            }
            match self.peek().r#type {
                CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => return,
                _ => {}
            }
            self.advance();
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().r#type == token_type;
    }
    fn peek(&self) -> &Token {
        return self.tokens.get(self.current).unwrap(); // TODO
    }
    fn match_(&mut self, tokens: &[TokenType]) -> bool {
        for type_ in tokens {
            if self.check(*type_) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap() // TODO
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }
    fn is_at_end(&self) -> bool {
        self.peek().r#type == EOF
    }
}
