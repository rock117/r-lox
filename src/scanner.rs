use crate::lox::Lox;
use crate::object::Object;
use crate::token::token_type::TokenType;
use crate::token::token_type::TokenType::*;
use crate::token::Token;
use map_macro::hash_map;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEY_WORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    hash_map! {
        "and" => AND,
        "class" => CLASS,
        "else" => ELSE,
        "false" => FALSE,
        "for" => FOR,
        "fun" => FUN,
        "if" => IF,
        "nil" => NIL,
        "or" => OR,
        "print" => PRINT,
        "return" => RETURN,
        "super" => SUPER,
        "this" => THIS,
        "true" => TRUE,
        "var" => VAR,
        "while" => WHILE,
    }
});

pub(crate) struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(EOF, "".into(), None, self.line));
        self.tokens
    }

    /// scan character, if a token found, add to self.tokens
    fn scan_token(&mut self) {
        let Some(c) = self.advance() else {
            return;
        };

        match c {
            '(' => self.add_token(LEFT_PAREN),
            ')' => self.add_token(RIGHT_PAREN),
            '{' => self.add_token(LEFT_BRACE),

            '}' => self.add_token(RIGHT_BRACE),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),

            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),

            '!' => {
                let token_type = if self.match_('=') { BANG_EQUAL } else { BANG };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_('=') { EQUAL_EQUAL } else { EQUAL };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_('=') { LESS_EQUAL } else { LESS };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_('=') {
                    GREATER_EQUAL
                } else {
                    GREATER
                };
                self.add_token(token_type);
            }

            '/' => {
                if self.match_('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH);
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace.
            }
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(Some(c)) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    Lox::error(self.line, "Unexpected character.")
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let type_ = KEY_WORDS.get(text).map(|v| *v).unwrap_or(IDENTIFIER);
        self.add_token(type_);
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: Option<char>) -> bool {
        c.map(|c| self.is_alpha(c)).unwrap_or(false) || self.is_digit(c)
    }
    fn is_digit(&self, c: Option<char>) -> bool {
        c.map(|c| c >= '0' && c <= '9').unwrap_or(false)
    }
    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == Some('.') && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token2(
            NUMBER,
            Some(Object::number(
                self.source[self.start..self.current]
                    .parse::<f64>()
                    .unwrap_or(f64::NAN),
            )),
        );
    }
    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            Some('\0')
        } else {
            self.source.chars().nth(self.current + 1)
        }
    }
    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Lox::error(self.line, "Unterminated string.");
            return;
        }

        /// The closing ".
        self.advance();
        // Trim the surrounding quotes.
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token2(STRING, Some(Object::string(value.into())));
    }
    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            Some('\0')
        } else {
            self.source.chars().nth(self.current)
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token2(token_type, None);
    }
    fn add_token2(&mut self, token_type: TokenType, literal: Option<Object>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.into(), literal, self.line));
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
