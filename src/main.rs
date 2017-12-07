#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

mod lexer;
mod token;
mod expr;
mod stmt;
mod visitor;
mod value;
mod parser;

use lexer::*;
use token::*;
use expr::*;
use stmt::*;
use visitor::*;
use value::*;
use parser::*;

macro_rules! mExprBinary {
    ($self:ident, $a:ident $op:tt $b:ident) => {
        {
            let va = $self.visitExpr($a);
            let vb = $self.visitExpr($b);
            va $op vb
        }
    };

    ($name:ident, $self:ident, $a:ident $op:tt $b:ident) => {
        &mut $name(ref mut $a, ref mut $b) => mExprBinary!(self, $a - $b),
    };
}

struct SimpleInterpreter {
}

impl Visitor for SimpleInterpreter {
    type StmtReturnType = ();
    type ExprReturnType = Value;

    fn visitStmt(&mut self, stmt: &mut Stmt) {
        use Stmt::*;
        match stmt {
            &mut Print(ref mut expr) => {
                println!("{:?}", self.visitExpr(expr));
            },
            _ => ()
        }
    }

    fn visitExpr(&mut self, expr: &mut Expr) -> Value {
        use Expr::*;
        match expr {
            &mut Int(i) => Value::Int(i),

            &mut Add(ref mut a, ref mut b) => mExprBinary!(self, a + b),
            &mut Sub(ref mut a, ref mut b) => mExprBinary!(self, a - b),
            &mut Mul(ref mut a, ref mut b) => mExprBinary!(self, a * b),
            &mut Div(ref mut a, ref mut b) => mExprBinary!(self, a / b),
            &mut Rem(ref mut a, ref mut b) => mExprBinary!(self, a % b),

            _ => Value::None 
        }
    }
}

fn main() {
    let mut lex = Lexer::new("print 6");
    let mut parser = Parser::new(lex);

    let print = parser.parsePrintStmt();

    let mut inter = SimpleInterpreter{};

    match print {
        Ok(mut stmt) => {
            println!("{:?}", stmt);
            inter.visitStmt(&mut stmt);
        },
        Err(err) => {
            println!("[ERR] {}", err);
        }
    }
}
