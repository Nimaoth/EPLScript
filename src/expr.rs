#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

#[derive(Debug)]
pub enum Expr {
    Int(i64),

    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Rem(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn CreateAdd(a: Expr, b: Expr) -> Expr {
        Expr::Add(Box::new(a), Box::new(b))
    }
    
    pub fn CreateSub(a: Expr, b: Expr) -> Expr {
        Expr::Sub(Box::new(a), Box::new(b))
    }
    
    pub fn CreateMul(a: Expr, b: Expr) -> Expr {
        Expr::Mul(Box::new(a), Box::new(b))
    }
    
    pub fn CreateDiv(a: Expr, b: Expr) -> Expr {
        Expr::Div(Box::new(a), Box::new(b))
    }
    
    pub fn CreateRem(a: Expr, b: Expr) -> Expr {
        Expr::Rem(Box::new(a), Box::new(b))
    }
}
