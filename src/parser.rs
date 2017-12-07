#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use lexer::Lexer;
use stmt::Stmt;
use expr::Expr;
use token::*;

#[derive(Debug)]
pub struct Parser {
    lex: Lexer
}

impl Parser {
    pub fn new(lex: Lexer) -> Parser {
        Parser {
            lex
        }
    }

    pub fn parsePrintStmt(&mut self) -> Result<Stmt, String> {
        match self.lex.next() {
            Some(result) => {
                match result {
                    Ok(token) => {
                        match token.ttype {
                            TokenType::Kprint => {
                                match self.parseExpr() {
                                    Ok(expr) => Ok(Stmt::Print(expr)),
                                    Err(err) => Err(format!("({}:{}) Failed to parse print statement: {}", token.line, token.column, err))
                                }
                            },
                            _ => Err(format!("({}:{}) Failed to parse print statement: unexpected token {:?}", token.line, token.column, token.ttype))
                        }
                    },
                    Err(err) => Err(format!("Failed to parse print statement: {}", err))
                }
            },
            None => Err(format!("Failed to parse print statement: reached end of file"))
        }
    }
    
    pub fn parseExpr(&mut self) -> Result<Expr, String> {
        self.parsePrimary()
    }

    pub fn parsePrimary(&mut self) -> Result<Expr, String> {
        use TokenType::*;
        match self.lex.next() {
            Some(result) => {
                match result {
                    Ok(token) => {
                        match token.ttype {
                            IntLiteral(i) => Ok(Expr::Int(i)),
                            _ => Err(format!("({}:{}) Failed to parse primary expression: unexpected token {:?}", token.line, token.column, token.ttype))
                        }
                    },
                    Err(err) => {
                        Err(format!("Failed to parse primary expression: {}", err))
                    }
                }
            },
            Option::None => {
                Err(format!("Failed to parse primary expression: reached end of file"))
            }
        }
    }
}
