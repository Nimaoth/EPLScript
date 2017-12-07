#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use expr::*;

#[derive(Debug)]
pub enum Stmt {
    Print(Expr)
}