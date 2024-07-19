use crate::lox::Lox;
use crate::token::token_type::TokenType;
use crate::token::token_type::TokenType::*;
use crate::token::Token;
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

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(EOF, "".into(), None, self.line));
        self.tokens.clone()
    }

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

            '!' => self.add_token(if self.match_('=') { BANG_EQUAL } else { BANG }),
            '=' => self.add_token(if self.match_('=') { EQUAL_EQUAL } else { EQUAL }),
            '<' => self.add_token(if self.match_('=') { LESS_EQUAL } else { LESS }),
            '>' => self.add_token(if self.match_('=') {
                GREATER_EQUAL
            } else {
                GREATER
            }),

            _ => Lox::error(self.line, "Unexpected character."),
        }
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

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token2(token_type, None);
    }
    fn add_token2(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.into(), literal, self.line));
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
