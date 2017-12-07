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

    fn skipNewLine(&mut self) {
        let isNL = if let Some(Ok(t)) = self.lex.peek() {
            if let TokenType::NewLine = t.ttype { true } else { false }
        } else {
            false
        };
        if isNL {
            self.lex.next();
        }
    }

    pub fn parseStmt(&mut self) -> Result<Stmt, String> {
        match self.lex.next() {
            Some(result) => {
                match result {
                    Ok(token) => {
                        match token.ttype {
                            TokenType::Kprint => self.parsePrintStmt(&token),
                            _ => Err(format!("({}:{}) Failed to parse statement: unexpected token {:?}", token.line, token.column, token.ttype))
                        }
                    },
                    Err(err) => Err(format!("Failed to parse statement: {}", err))
                }
            },
            None => Err(format!("Failed to parse statement: reached end of file"))
        }
    }

    fn parsePrintStmt(&mut self, token: &Token) -> Result<Stmt, String> {
        self.skipNewLine();

        match self.parseExpr() {
            Ok(expr) => Ok(Stmt::Print(expr)),
            Err(err) => Err(format!("({}:{}) Failed to parse print statement: {}", token.line, token.column, err))
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
