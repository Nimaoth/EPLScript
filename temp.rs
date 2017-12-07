#![feature(prelude_import)]
#![no_std]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;

mod lexer {








    // &mut Sub(ref mut a, ref mut b) => mExprBinary!(self, a - b),
    // &mut Mul(ref mut a, ref mut b) => mExprBinary!(self, a * b),
    // &mut Div(ref mut a, ref mut b) => mExprBinary!(self, a / b),
    // &mut Rem(ref mut a, ref mut b) => mExprBinary!(self, a % b),




    use token::*;
    pub struct Lexer {
        chars: Vec<char>,
        index: usize,
        line: usize,
        col: usize,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Lexer {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                Lexer {
                chars: ref __self_0_0,
                index: ref __self_0_1,
                line: ref __self_0_2,
                col: ref __self_0_3 } => {
                    let mut builder = __arg_0.debug_struct("Lexer");
                    let _ = builder.field("chars", &&(*__self_0_0));
                    let _ = builder.field("index", &&(*__self_0_1));
                    let _ = builder.field("line", &&(*__self_0_2));
                    let _ = builder.field("col", &&(*__self_0_3));
                    builder.finish()
                }
            }
        }
    }
    impl Lexer {
        pub fn new(code: &str) -> Lexer {
            Lexer{chars: code.chars().collect(), index: 0, line: 1, col: 1,}
        }
        fn peek(&self, index: usize) -> char {
            if self.index + index >= self.chars.len() { return '\u{0}'; }
            self.chars[self.index + index]
        }
        fn skipWC(&mut self, newLineIsWs: bool) {
            while self.index < self.chars.len() {
                let c = self.chars[self.index];
                match c {
                    '/' if self.peek(1) == '/' => {
                        self.index += 1;
                        while self.index + 1 < self.chars.len() {
                            if self.peek(1) == '\n' { break ; }
                            self.index += 1;
                        }
                    }
                    '\n' if newLineIsWs => { self.incLine(); }
                    '\n' => { break ; }
                    _ if c.is_whitespace() => { self.col += 1; }
                    _ => { break ; }
                }
                self.index += 1;
            }
        }
        fn incLine(&mut self) { self.line += 1; self.col = 1; }
    }
    impl Iterator for Lexer {
        type
        Item
        =
        Result<Token, String>;
        fn next(&mut self) -> Option<Result<Token, String>> {
            self.skipWC(false);
            if self.index >= self.chars.len() { return None; }
            let c = self.chars[self.index];
            let line = self.line;
            let col = self.col;
            self.index += 1;
            self.col += 1;
            match c {
                '\n' => {
                    self.incLine();
                    self.skipWC(true);
                    Some(Ok(Token{ttype: TokenType::NewLine,
                                  line: line,
                                  column: col,}))
                }
                _ =>
                Some(Err(::fmt::format(::std::fmt::Arguments::new_v1_formatted(&["(",
                                                                                 ":",
                                                                                 ") Unrecognized character: "],
                                                                               &match (&self.line,
                                                                                       &self.col,
                                                                                       &c)
                                                                                    {
                                                                                    (__arg0,
                                                                                     __arg1,
                                                                                     __arg2)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                 ::std::fmt::Display::fmt),
                                                                                     ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                 ::std::fmt::Display::fmt),
                                                                                     ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                 ::std::fmt::Display::fmt)],
                                                                                },
                                                                               &[::std::fmt::rt::v1::Argument{position:
                                                                                                                  ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                              format:
                                                                                                                  ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                     ' ',
                                                                                                                                                 align:
                                                                                                                                                     ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                 flags:
                                                                                                                                                     0u32,
                                                                                                                                                 precision:
                                                                                                                                                     ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                 width:
                                                                                                                                                     ::std::fmt::rt::v1::Count::Implied,},},
                                                                                 ::std::fmt::rt::v1::Argument{position:
                                                                                                                  ::std::fmt::rt::v1::Position::At(1usize),
                                                                                                              format:
                                                                                                                  ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                     ' ',
                                                                                                                                                 align:
                                                                                                                                                     ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                 flags:
                                                                                                                                                     0u32,
                                                                                                                                                 precision:
                                                                                                                                                     ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                 width:
                                                                                                                                                     ::std::fmt::rt::v1::Count::Implied,},},
                                                                                 ::std::fmt::rt::v1::Argument{position:
                                                                                                                  ::std::fmt::rt::v1::Position::At(2usize),
                                                                                                              format:
                                                                                                                  ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                     ' ',
                                                                                                                                                 align:
                                                                                                                                                     ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                 flags:
                                                                                                                                                     0u32,
                                                                                                                                                 precision:
                                                                                                                                                     ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                 width:
                                                                                                                                                     ::std::fmt::rt::v1::Count::Implied,},}])))),
            }
        }
    }
}
mod token {
    #![allow(dead_code)]
    #![allow(non_snake_case)]
    pub enum TokenType {
        None,
        IntLiteral(i64),
        FloatLiteral(f64),
        CharLiteral(char),
        StringLiteral(String),
        Identifier(String),
        Kfun,
        Kreturn,
        Kif,
        Kelse,
        Kfor,
        Kwhile,
        Kloop,
        Kbreak,
        Kcontinue,
        Plus,
        Minus,
        Slash,
        Asterisk,
        Percent,
        UnaryMinus,
        OpenParen,
        ClosingParen,
        OpenBrace,
        ClosingBrace,
        OpenBracket,
        ClosingBracket,
        NewLine,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for TokenType {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match (&*self,) {
                (&TokenType::None,) => {
                    let mut builder = __arg_0.debug_tuple("None");
                    builder.finish()
                }
                (&TokenType::IntLiteral(ref __self_0),) => {
                    let mut builder = __arg_0.debug_tuple("IntLiteral");
                    let _ = builder.field(&&(*__self_0));
                    builder.finish()
                }
                (&TokenType::FloatLiteral(ref __self_0),) => {
                    let mut builder = __arg_0.debug_tuple("FloatLiteral");
                    let _ = builder.field(&&(*__self_0));
                    builder.finish()
                }
                (&TokenType::CharLiteral(ref __self_0),) => {
                    let mut builder = __arg_0.debug_tuple("CharLiteral");
                    let _ = builder.field(&&(*__self_0));
                    builder.finish()
                }
                (&TokenType::StringLiteral(ref __self_0),) => {
                    let mut builder = __arg_0.debug_tuple("StringLiteral");
                    let _ = builder.field(&&(*__self_0));
                    builder.finish()
                }
                (&TokenType::Identifier(ref __self_0),) => {
                    let mut builder = __arg_0.debug_tuple("Identifier");
                    let _ = builder.field(&&(*__self_0));
                    builder.finish()
                }
                (&TokenType::Kfun,) => {
                    let mut builder = __arg_0.debug_tuple("Kfun");
                    builder.finish()
                }
                (&TokenType::Kreturn,) => {
                    let mut builder = __arg_0.debug_tuple("Kreturn");
                    builder.finish()
                }
                (&TokenType::Kif,) => {
                    let mut builder = __arg_0.debug_tuple("Kif");
                    builder.finish()
                }
                (&TokenType::Kelse,) => {
                    let mut builder = __arg_0.debug_tuple("Kelse");
                    builder.finish()
                }
                (&TokenType::Kfor,) => {
                    let mut builder = __arg_0.debug_tuple("Kfor");
                    builder.finish()
                }
                (&TokenType::Kwhile,) => {
                    let mut builder = __arg_0.debug_tuple("Kwhile");
                    builder.finish()
                }
                (&TokenType::Kloop,) => {
                    let mut builder = __arg_0.debug_tuple("Kloop");
                    builder.finish()
                }
                (&TokenType::Kbreak,) => {
                    let mut builder = __arg_0.debug_tuple("Kbreak");
                    builder.finish()
                }
                (&TokenType::Kcontinue,) => {
                    let mut builder = __arg_0.debug_tuple("Kcontinue");
                    builder.finish()
                }
                (&TokenType::Plus,) => {
                    let mut builder = __arg_0.debug_tuple("Plus");
                    builder.finish()
                }
                (&TokenType::Minus,) => {
                    let mut builder = __arg_0.debug_tuple("Minus");
                    builder.finish()
                }
                (&TokenType::Slash,) => {
                    let mut builder = __arg_0.debug_tuple("Slash");
                    builder.finish()
                }
                (&TokenType::Asterisk,) => {
                    let mut builder = __arg_0.debug_tuple("Asterisk");
                    builder.finish()
                }
                (&TokenType::Percent,) => {
                    let mut builder = __arg_0.debug_tuple("Percent");
                    builder.finish()
                }
                (&TokenType::UnaryMinus,) => {
                    let mut builder = __arg_0.debug_tuple("UnaryMinus");
                    builder.finish()
                }
                (&TokenType::OpenParen,) => {
                    let mut builder = __arg_0.debug_tuple("OpenParen");
                    builder.finish()
                }
                (&TokenType::ClosingParen,) => {
                    let mut builder = __arg_0.debug_tuple("ClosingParen");
                    builder.finish()
                }
                (&TokenType::OpenBrace,) => {
                    let mut builder = __arg_0.debug_tuple("OpenBrace");
                    builder.finish()
                }
                (&TokenType::ClosingBrace,) => {
                    let mut builder = __arg_0.debug_tuple("ClosingBrace");
                    builder.finish()
                }
                (&TokenType::OpenBracket,) => {
                    let mut builder = __arg_0.debug_tuple("OpenBracket");
                    builder.finish()
                }
                (&TokenType::ClosingBracket,) => {
                    let mut builder = __arg_0.debug_tuple("ClosingBracket");
                    builder.finish()
                }
                (&TokenType::NewLine,) => {
                    let mut builder = __arg_0.debug_tuple("NewLine");
                    builder.finish()
                }
            }
        }
    }
    pub struct Token {
        pub ttype: TokenType,
        pub line: usize,
        pub column: usize,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Token {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                Token {
                ttype: ref __self_0_0,
                line: ref __self_0_1,
                column: ref __self_0_2 } => {
                    let mut builder = __arg_0.debug_struct("Token");
                    let _ = builder.field("ttype", &&(*__self_0_0));
                    let _ = builder.field("line", &&(*__self_0_1));
                    let _ = builder.field("column", &&(*__self_0_2));
                    builder.finish()
                }
            }
        }
    }
}
mod expr {
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(unused_macros)]
    pub enum Expr {
        Int(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
        Mul(Box<Expr>, Box<Expr>),
        Div(Box<Expr>, Box<Expr>),
        Rem(Box<Expr>, Box<Expr>),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Expr {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match (&*self,) {
                (&Expr::Int(ref __self_0),) => {
                    let mut builder = __arg_0.debug_tuple("Int");
                    let _ = builder.field(&&(*__self_0));
                    builder.finish()
                }
                (&Expr::Add(ref __self_0, ref __self_1),) => {
                    let mut builder = __arg_0.debug_tuple("Add");
                    let _ = builder.field(&&(*__self_0));
                    let _ = builder.field(&&(*__self_1));
                    builder.finish()
                }
                (&Expr::Sub(ref __self_0, ref __self_1),) => {
                    let mut builder = __arg_0.debug_tuple("Sub");
                    let _ = builder.field(&&(*__self_0));
                    let _ = builder.field(&&(*__self_1));
                    builder.finish()
                }
                (&Expr::Mul(ref __self_0, ref __self_1),) => {
                    let mut builder = __arg_0.debug_tuple("Mul");
                    let _ = builder.field(&&(*__self_0));
                    let _ = builder.field(&&(*__self_1));
                    builder.finish()
                }
                (&Expr::Div(ref __self_0, ref __self_1),) => {
                    let mut builder = __arg_0.debug_tuple("Div");
                    let _ = builder.field(&&(*__self_0));
                    let _ = builder.field(&&(*__self_1));
                    builder.finish()
                }
                (&Expr::Rem(ref __self_0, ref __self_1),) => {
                    let mut builder = __arg_0.debug_tuple("Rem");
                    let _ = builder.field(&&(*__self_0));
                    let _ = builder.field(&&(*__self_1));
                    builder.finish()
                }
            }
        }
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
}
mod stmt {
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(unused_macros)]
    use expr::*;
    pub enum Stmt { Print(Expr), }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Stmt {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match (&*self,) {
                (&Stmt::Print(ref __self_0),) => {
                    let mut builder = __arg_0.debug_tuple("Print");
                    let _ = builder.field(&&(*__self_0));
                    builder.finish()
                }
            }
        }
    }
}
mod visitor {
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(unused_macros)]
    use stmt::*;
    use expr::*;
    pub trait Visitor {
        type
        StmtReturnType;
        type
        ExprReturnType;
        fn visitStmt(&mut self, stmt: &mut Stmt)
        -> Self::StmtReturnType;
        fn visitExpr(&mut self, expr: &mut Expr)
        -> Self::ExprReturnType;
    }
}
mod value {
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(unused_macros)]
    use std::ops;
    pub enum Value { None, Int(i64), Float(f64), Char(char), String(String), }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Value {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match (&*self,) {
                (&Value::None,) => {
                    let mut builder = __arg_0.debug_tuple("None");
                    builder.finish()
                }
                (&Value::Int(ref __self_0),) => {
                    let mut builder = __arg_0.debug_tuple("Int");
                    let _ = builder.field(&&(*__self_0));
                    builder.finish()
                }
                (&Value::Float(ref __self_0),) => {
                    let mut builder = __arg_0.debug_tuple("Float");
                    let _ = builder.field(&&(*__self_0));
                    builder.finish()
                }
                (&Value::Char(ref __self_0),) => {
                    let mut builder = __arg_0.debug_tuple("Char");
                    let _ = builder.field(&&(*__self_0));
                    builder.finish()
                }
                (&Value::String(ref __self_0),) => {
                    let mut builder = __arg_0.debug_tuple("String");
                    let _ = builder.field(&&(*__self_0));
                    builder.finish()
                }
            }
        }
    }
    macro_rules! arif(( $ lhs : expr , $ rhs : expr , $ op : tt ) => {
                      match ( $ lhs , $ rhs ) {
                      ( Int ( a ) , Int ( b ) ) => Int ( a $ op b ) , (
                      Int ( a ) , Float ( b ) ) => Float ( a as f64 $ op b ) ,
                      ( Float ( a ) , Int ( b ) ) => Float ( a $ op b as f64 )
                      , ( Float ( a ) , Float ( b ) ) => Float ( a $ op b ) ,
                      _ => Value :: None } } ;);
    macro_rules! operator((
                          $ traitName : ident , $ funcName : ident , $ op : tt
                          ) => {
                          impl ops :: $ traitName for Value {
                          type Output = Value ; fn $ funcName (
                          self , rhs : Value ) -> Value {
                          use Value :: * ; arif ! ( self , rhs , $ op ) } } }
                          ;);
    impl ops::Add for Value {
        type
        Output
        =
        Value;
        fn add(self, rhs: Value) -> Value {
            use Value::*;
            match (self, rhs) {
                (Int(a), Int(b)) => Int(a + b),
                (Int(a), Float(b)) => Float(a as f64 + b),
                (Float(a), Int(b)) => Float(a + b as f64),
                (Float(a), Float(b)) => Float(a + b),
                _ => Value::None,
            }
        }
    }
    impl ops::Sub for Value {
        type
        Output
        =
        Value;
        fn sub(self, rhs: Value) -> Value {
            use Value::*;
            match (self, rhs) {
                (Int(a), Int(b)) => Int(a - b),
                (Int(a), Float(b)) => Float(a as f64 - b),
                (Float(a), Int(b)) => Float(a - b as f64),
                (Float(a), Float(b)) => Float(a - b),
                _ => Value::None,
            }
        }
    }
    impl ops::Mul for Value {
        type
        Output
        =
        Value;
        fn mul(self, rhs: Value) -> Value {
            use Value::*;
            match (self, rhs) {
                (Int(a), Int(b)) => Int(a * b),
                (Int(a), Float(b)) => Float(a as f64 * b),
                (Float(a), Int(b)) => Float(a * b as f64),
                (Float(a), Float(b)) => Float(a * b),
                _ => Value::None,
            }
        }
    }
    impl ops::Div for Value {
        type
        Output
        =
        Value;
        fn div(self, rhs: Value) -> Value {
            use Value::*;
            match (self, rhs) {
                (Int(a), Int(b)) => Int(a / b),
                (Int(a), Float(b)) => Float(a as f64 / b),
                (Float(a), Int(b)) => Float(a / b as f64),
                (Float(a), Float(b)) => Float(a / b),
                _ => Value::None,
            }
        }
    }
    impl ops::Rem for Value {
        type
        Output
        =
        Value;
        fn rem(self, rhs: Value) -> Value {
            use Value::*;
            match (self, rhs) {
                (Int(a), Int(b)) => Int(a % b),
                (Int(a), Float(b)) => Float(a as f64 % b),
                (Float(a), Int(b)) => Float(a % b as f64),
                (Float(a), Float(b)) => Float(a % b),
                _ => Value::None,
            }
        }
    }
}
use lexer::*;
use token::*;
use expr::*;
use stmt::*;
use visitor::*;
use value::*;
macro_rules! mExprBinary(( $ self : ident , $ a : ident $ op : tt $ b : ident
                         ) => {
                         {
                         let va = $ self . visitExpr ( $ a ) ; let vb = $ self
                         . visitExpr ( $ b ) ; va $ op vb } } ; (
                         $ name : ident , $ self : ident , $ a : ident $ op :
                         tt $ b : ident ) => {
                         & mut $ name ( ref mut $ a , ref mut $ b ) =>
                         mExprBinary ! ( self , $ a - $ b ) , } ;);
struct SimpleInterpreter {
}
impl Visitor for SimpleInterpreter {
    type
    StmtReturnType
    =
    ();
    type
    ExprReturnType
    =
    Value;
    fn visitStmt(&mut self, stmt: &mut Stmt) {
        use Stmt::*;
        match stmt {
            &mut Print(ref mut expr) => {
                ::io::_print(::std::fmt::Arguments::new_v1_formatted(&["",
                                                                       "\n"],
                                                                     &match (&self.visitExpr(expr),)
                                                                          {
                                                                          (__arg0,)
                                                                          =>
                                                                          [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                       ::std::fmt::Debug::fmt)],
                                                                      },
                                                                     &[::std::fmt::rt::v1::Argument{position:
                                                                                                        ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                    format:
                                                                                                        ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                           ' ',
                                                                                                                                       align:
                                                                                                                                           ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                       flags:
                                                                                                                                           0u32,
                                                                                                                                       precision:
                                                                                                                                           ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                       width:
                                                                                                                                           ::std::fmt::rt::v1::Count::Implied,},}]));
            }
            _ => (),
        }
    }
    fn visitExpr(&mut self, expr: &mut Expr) -> Value {
        use Expr::*;
        match expr {
            &mut Int(i) => Value::Int(i),
            &mut Add(ref mut a, ref mut b) => {
                let va = self.visitExpr(a);
                let vb = self.visitExpr(b);
                va + vb
            }
        }
    }
}
fn main() {
    use Stmt::*;
    use Expr::*;
    let mut st = Stmt::Print(Expr::CreateSub(Int(1), Int(2)));
    let mut v = SimpleInterpreter{};
    v.visitStmt(&mut st);
}
