use crate::error::ParseError;
use crate::expr::Expr;
use crate::expr::Expr::Logical;
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
            if let Some(dec) = self.declaration() {
                statements.push(dec);
            }
        }
        return Ok(statements);
    }
    /// grammar expression → assignment;
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    /// declaration → varDecl | statement
    fn declaration(&mut self) -> Option<Stmt> {
        let res = if self.match_(&[VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        match res {
            Ok(stmt) => Some(stmt),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    /// statement → exprStmt | ifStmt | printStmt | block ;
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_(&[IF]) {
            self.if_statement()
        } else if self.match_(&[PRINT]) {
            self.print_statement()
        } else if self.match_(&[LEFT_BRACE]) {
            Ok(Stmt::block(self.block()?))
        } else {
            self.expression_statement()
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(LEFT_PAREN, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(RIGHT_PAREN, "Expect ')' after if condition.")?;
        let thenBranch = self.statement()?;
        let mut elseBranch = if self.match_(&[ELSE]) {
            Some(self.statement()?)
        } else {
            None
        };
        Ok(Stmt::r#if(condition, thenBranch, elseBranch))
    }

    /// printStmt → "print" expression ";" ;
    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::print(value))
    }

    /// varDecl → "var" IDENTIFIER ( "=" expression )? ";"
    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(IDENTIFIER, "Expect variable name.")?; // var had been match by its caller
        let initializer: Option<Expr> = if self.match_(&[EQUAL]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(SEMICOLON, "Expect ';' after variable declaration.")?;
        Ok(Stmt::var(name, initializer))
    }

    /// exprStmt → expression ";"
    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(SEMICOLON, "Expect ';' after expression.")?;
        Ok(Stmt::expression(expr))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];
        while !self.check(RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration().unwrap()); // TODO
        }
        self.consume(RIGHT_BRACE, "Expect '}' after block.")?;
        Ok(statements)
    }

    /// assignment → IDENTIFIER "=" assignment | logic_or
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;
        if self.match_(&[EQUAL]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            if let Expr::Variable(expr) = expr {
                let name = expr.name;
                return Ok(Expr::assign(name, value));
            } else {
                self.error(equals, "Invalid assignment target."); // TODO thorw?
            }
        }
        Ok(expr)
    }

    /// logic_or → logic_and ( "or" logic_and )* ;
    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;
        while self.match_(&[OR]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::logical(expr, operator, right);
        }
        Ok(expr)
    }

    /// logic_and → equality ( "and" equality )* ;
    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        while self.match_(&[AND]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::logical(expr, operator, right);
        }
        Ok(expr)
    }

    /// equality → comparison ( ( "!=" | "==" ) comparison )* ;
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

    /// To access a variable, we define a new kind of primary expressio
    ///
    /// primary → "true" | "false" | "nil"
    /// | NUMBER | STRING
    /// | "(" expression ")"
    /// | IDENTIFIER ;
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
        if self.match_(&[IDENTIFIER]) {
            return Ok(Expr::variable(self.previous().clone()));
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
