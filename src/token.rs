#![allow(dead_code)]
#![allow(non_snake_case)]

#[derive(Debug)]
pub enum TokenType 
{
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
    Kprint,
    
    // operators
    Plus,
    Minus,
    Slash,
    Asterisk,
    Percent,

    UnaryMinus,
    
    // delimiters
    OpenParen,      // (
    ClosingParen,   // )
    OpenBrace,      // {
    ClosingBrace,   // }
    OpenBracket,    // [
    ClosingBracket, // ]
    NewLine
}

#[derive(Debug)]
pub struct Token
{
    pub ttype: TokenType,
    pub line: usize,
    pub column: usize
}
