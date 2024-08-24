use crate::expr::Expr;
use crate::token::Token;

/// assign is also an expr, the expr's value is assign.value, for example a = 33, the result will be 33.
/// so for print a = 33, the console output will be 33
#[derive(Debug, Clone)]
pub(crate) struct Assign {
    pub name: Token,
    pub value: Expr,
    pub distance: Option<usize>
}

impl Assign {

}