use crate::token::Token;

pub(crate) struct Scanner(String);
impl Scanner {
    pub fn new(source: String) -> Self {
        Self(source)
    }

    pub fn scan_tokens(&self) -> Vec<Token>{
        todo!()
    }
}