use crate::error::ParseError;
use crate::expr::binary::Binary;
use crate::expr::grouping::Grouping;
use crate::expr::literal::Literal;
use crate::expr::unary::Unary;
use crate::expr::Expr;
use crate::lox::Lox;
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

    /// When a syntax error does occur, this method returns null. That’s OK. The
    /// parser promises not to crash or hang on invalid syntax, but it doesn’t promise to return a usable syntax tree if an error is found.
    pub(crate) fn parse<E: Expr>(&mut self) -> Option<E> {
        match self.expression::<E>() {
            Ok(e) => Some(e),
            _ => None,
        }
    }
    /// grammar expression → equality;
    fn expression<E: Expr>(&mut self) -> Result<E, ParseError> {
        self.equality::<E>()
    }

    /// grammar: equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality<E: Expr>(&mut self) -> Result<E, ParseError> {
        let mut expr = self.comparison();
        while self.match_(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Binary::new(expr, operator, right);
        }
        return Ok(expr);
    }

    fn comparison<E: Expr>(&mut self) -> E {
        let mut expr = self.term();
        while self.match_(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Binary::new(expr, operator, right);
        }
        return expr;
    }

    fn term<E: Expr>(&mut self) -> E {
        let mut expr = self.factor();
        while self.match_(&[MINUS, PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Binary::new(expr, operator, right);
        }
        return expr;
    }

    fn factor<E: Expr>(&mut self) -> E {
        let mut expr = self.unary();
        while self.match_(&[SLASH, STAR]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Binary::new(expr, operator, right);
        }
        return expr;
    }

    fn unary<E: Expr>(&mut self) -> E {
        if self.match_(&[BANG, MINUS]) {
            let operator = self.previous().clone(); // TODO
            let right = self.unary();
            return Unary::new(operator, right);
        }
        return self.primary();
    }

    fn primary<E: Expr>(&mut self) -> Result<E, ParseError> {
        if self.match_(&[FALSE]) {
            return Ok(Literal::new(Some(false)));
        }
        if self.match_(&[TRUE]) {
            return Ok(Literal::new(Some(true)));
        }
        if self.match_(&[NIL]) {
            return Ok(Literal::new(None));
        }
        if self.match_(&[NUMBER, STRING]) {
            return Ok(Literal::new(Some(self.previous().literal.clone())));
        }

        if self.match_(&[LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Grouping::new(expr));
        } else {
            return self.error(self.peek(), "Expect expression.");
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance().clone()); // TODO
        } else {
            self.error(self.peek(), msg)
        }
    }
    fn error(&self, token: &Token, msg: &str) -> Result<Token, ParseError> {
        Lox::error_(token, msg);
        Err(ParseError)
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
