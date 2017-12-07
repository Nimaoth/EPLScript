#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use stmt::*;
use expr::*;

pub trait Visitor {
    type StmtReturnType;
    type ExprReturnType;

    fn visitStmt(&mut self, stmt: &mut Stmt) -> Self::StmtReturnType;
    fn visitExpr(&mut self, expr: &mut Expr) -> Self::ExprReturnType;
}
