#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::ops;

#[derive(Debug)]
pub enum Value {
    None,
    Int(i64),
    Float(f64),
    Char(char),
    String(String)
}

macro_rules! arif {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        match ($lhs, $rhs) {
            (Int(a), Int(b)) => Int(a $op b),
            (Int(a), Float(b)) => Float(a as f64 $op b),

            (Float(a), Int(b)) => Float(a $op b as f64),
            (Float(a), Float(b)) => Float(a $op b),

            _ => Value::None
        }
    };
}

macro_rules! operator {
    ($traitName:ident, $funcName:ident, $op:tt) => {
        impl ops::$traitName for Value {
            type Output = Value;

            fn $funcName(self, rhs: Value) -> Value {
                use Value::*;
                arif!(self, rhs, $op)
            }
        }
    };
}

operator!(Add, add, +);
operator!(Sub, sub, -);
operator!(Mul, mul, *);
operator!(Div, div, /);
operator!(Rem, rem, %);
