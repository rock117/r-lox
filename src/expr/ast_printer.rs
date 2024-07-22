use crate::expr::binary::Binary;
use crate::expr::grouping::Grouping;
use crate::expr::literal::Literal;
use crate::expr::unary::Unary;
use crate::expr::{Expr, Visitor};
use std::fmt::Display;

pub(crate) struct AstPrinter;

impl AstPrinter {
    pub(crate) fn new() -> Self {
        AstPrinter
    }

    pub(crate) fn print(&self, expr: &impl Expr) -> String {
        return expr.accept(self);
    }

    fn parenthesize(&self, name: &str, exprs: &[&impl Expr]) -> String {
        let mut strs = String::new();
        strs.push_str("(");
        strs.push_str(name);
        for expr in exprs {
            strs.push_str(" ");
            strs.push_str(&expr.accept(self));
        }
        strs.push_str(")");
        strs
    }
    fn parenthesize2<E1: Expr, E2: Expr>(&self, name: &str, e1: &E1, e2: &E2) -> String {
        format!("({} {} {})", name, e1.accept(self), e2.accept(self))
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr<L: Expr, R: Expr>(&self, expr: &Binary<L, R>) -> String {
        self.parenthesize2(&expr.operator.lexeme, &expr.left, &expr.right)
    }

    fn visit_grouping_expr<E: Expr>(&self, expr: &Grouping<E>) -> String {
        self.parenthesize("group", &[&expr.expression])
    }

    fn visit_literal_expr<E: Display>(&self, expr: &Literal<E>) -> String {
        match &expr.value {
            None => "nil".into(),
            Some(value) => value.to_string(),
        }
    }

    fn visit_unary_expr<E: Expr>(&self, expr: &Unary<E>) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::ast_printer::AstPrinter;
    use crate::expr::binary::Binary;
    use crate::expr::grouping::Grouping;
    use crate::expr::literal::Literal;
    use crate::expr::unary::Unary;
    use crate::token::token_type::TokenType;
    use crate::token::Token;

    #[test]
    fn test_print_ast() {
        let expr = Binary::new(
            Unary::new(
                Token::new(TokenType::MINUS, "-".into(), None, 1),
                Literal::new(Some(123)),
            ),
            Token::new(TokenType::STAR, "*".into(), None, 1),
            Grouping::new(Literal::new(Some(45.67))),
        );
        assert_eq!("(* (- 123) (group 45.67))", AstPrinter::new().print(&expr));
    }

    #[test]
    fn test_print_ast_by_parser() {}
}
