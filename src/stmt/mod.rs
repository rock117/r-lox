trait Stmt {
    fn accept<R, V: Visitor<R>>(visitor: &V) -> R;
}

trait Visitor<R> {
    fn visit_block_stmt(stmt: &str) -> R; //
}
