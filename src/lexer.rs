#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use token::*;

macro_rules! errtok {
    ($line:expr, $col:expr, $format:expr) => {
        Some(Token {
            ttype: TokenType::Error(format!("{}", $format)),
            line: $line,
            column: $col
        })
    };
    ($line:expr, $col:expr, $format:expr, $($a:expr),*) => {
        Some(Token {
            ttype: TokenType::Error(format!($format, $($a),*)),
            line: $line,
            column: $col
        })
    };
}

#[derive(Debug)]
pub struct Lexer
{
    chars: Vec<char>,
    index: usize,
    line: usize,
    col: usize,

    buff: Option<Token>
}

impl Lexer
{
    pub fn new(code: &str) -> Lexer
    {
        Lexer {
            chars: code.chars().collect(),
            index: 0,
            line: 1,
            col: 1,
            buff: None
        }
    }

    fn peekChar(&self, index: i32) -> char {
        let i = (self.index as i32 + index) as usize;
        if i >= self.chars.len() {
            return '\0';
        }

        self.chars[i]
    }

    fn skipWC(&mut self, newLineIsWs: bool)
    {
        while self.index < self.chars.len() {
            let c = self.chars[self.index];

            match c {
                '/' if self.peekChar(1) == '/' => {
                    self.index += 1;
                    while self.index + 1 < self.chars.len() {
                        if self.peekChar(1) == '\n' {
                            break;
                        }
                        self.index += 1;
                    }
                },
                '\n' if newLineIsWs => {
                    self.incLine();
                },
                '\n' => {
                    break;
                },
                _ if c.is_whitespace() => {
                    self.col += 1;
                },
                _ => {
                    break;
                }
            }

            self.index += 1;
        }
    }

    fn incLine(&mut self) {
        self.line += 1;
        self.col = 1;
    }

    fn isDigit(c: char) -> bool {
        c >= '0' && c <= '9'
    }
    
    fn isBinDigit(c: char) -> bool {
        c >= '0' && c <= '1'
    }

    fn isHexDigit(c: char) -> bool {
        Self::isDigit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
    }

    fn toString(&self, index: usize, len: usize) -> String {
        self.chars.iter().skip(index).take(len).collect()
    }

    fn isIdChar(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
    }

    fn parseToken(&mut self) -> Option<Token> {
        self.skipWC(false);

        if self.index >= self.chars.len() {
            return None;
        }

        let c = self.chars[self.index];

        let line = self.line;
        let col = self.col;

        self.index += 1;
        self.col += 1;

        match c {
            '\n' => {
                self.incLine();
                self.skipWC(true);
                Some(Token {
                    ttype: TokenType::NewLine,
                    line: line,
                    column: col
                })
            },

            c if Self::isIdChar(c) => {
                let start = self.index - 1;
                while self.index < self.chars.len() {
                    let c = self.chars[self.index];
                    if !Self::isIdChar(c) && !Self::isDigit(c) {
                        break;
                    }

                    self.index += 1;
                    self.col += 1;
                }
                let s = self.toString(start, self.index - start);
                let ttype = match &s[..] {
                    "print" => TokenType::Kprint,
                    _ => TokenType::Identifier(s)
                };
                Some(Token {
                    ttype: ttype,
                    line: line,
                    column: col
                })
            },

            c if Self::isDigit(c) => {
                #[derive(Debug)]
                enum State {
                    S0,
                    I1, I2, Ix,
                    B1, B2, Bx,
                    H1, H2, Hx,
                    D1, D2, Dx,
                    Err(&'static str)
                }
                
                let mut s = match c {
                    '0' => State::S0,
                    _ => State::I1
                };
                // println!("{:?} -> {:?}", c, s);
                let startIdx = self.index - 1;

                // parse text, check if valid
                while self.index < self.chars.len() {
                    let c = self.chars[self.index];
                    self.index += 1;

                    // print!("{:?} + {:?} -> ", s, c);

                    s = match (s, c) {
                        (State::S0, 'f') => State::Dx,
                        (State::I1, 'f') => State::Dx,
                        (State::I2, 'f') => State::Dx,

                        (State::S0, 'x') => State::H1,
                        (State::H1, c) if Self::isHexDigit(c) => State::H2,
                        (State::H1, _) => State::Err("Number literal 0x has to be followed by hexadecimal digits"),
                        (State::H2, c) if Self::isHexDigit(c) => State::H2,
                        (State::H2, _) => State::Hx,

                        (State::S0, 'b') => State::B1,
                        (State::B1, c) if Self::isBinDigit(c) => State::B2,
                        (State::B1, _) => State::Err("Number literal 0b has to be followed by binary digits"),
                        (State::B2, c) if Self::isBinDigit(c) => State::B2,
                        (State::B2, c) if Self::isDigit(c) => State::Err("Number literal 0b has to be followed by binary digits"),
                        (State::B2, _) => State::Bx,

                        (State::S0, '.') => State::D1,
                        (State::D1, c) if Self::isDigit(c) => State::D2,
                        (State::D1, _) => State::Err("Number literal 0. has to be followed by decimal digits"),
                        (State::D2, c) if Self::isDigit(c) => State::D2,
                        (State::D2, _) => State::Dx,


                        (State::S0, c) if Self::isDigit(c) => State::I1,
                        (State::I1, '.') => State::D1,
                        (State::I1, c) if Self::isDigit(c) => State::I2,
                        (State::I1, _) => State::Ix,
                        (State::I2, c) if Self::isDigit(c) => State::I2,
                        (State::I2, '.') => State::D1,
                        (State::I2, _) => State::Ix,

                        (State::S0, _) => State::Ix,
                        _ => State::Err("Unexpected character")
                    };

                    // println!("{:?}", s);

                    if let State::Err(_) = s {
                        break;
                    }
                }

                // convert parsed text to number
                match s {
                    State::S0 | State::I1 | State::I2 | State::Ix => {
                        let l = if let State::Ix = s { self.index - 1 } else { self.index } - startIdx;
                        let s = self.toString(startIdx, l);
                        match i64::from_str_radix(&s, 10) {
                            Ok(i) => {
                                Some(Token {
                                    ttype: TokenType::IntLiteral(i),
                                    line: line,
                                    column: col
                                })
                            },
                            Err(e) => errtok!(line, col, "'{}' is not a valid i64 literal: {}", s, e)
                        }
                    },
                    State::D2 | State::Dx => {
                        let l = if let State::Dx = s { self.index - 1 } else { self.index } - startIdx;
                        let s = self.toString(startIdx, l);
                        match s.parse::<f64>() {
                            Ok(i) => {
                                Some(Token {
                                    ttype: TokenType::FloatLiteral(i),
                                    line: line,
                                    column: col
                                })
                            },
                            Err(e) => errtok!(line, col, "'{}' is not a valid f64 literal: {}", s, e)
                        }
                    },
                    State::H2 | State::Hx => {
                        let l = if let State::Hx = s { self.index - 1 } else { self.index } - startIdx - 2;
                        let s = self.toString(startIdx + 2, l);
                        match i64::from_str_radix(&s, 16) {
                            Ok(i) => {
                                Some(Token {
                                    ttype: TokenType::IntLiteral(i),
                                    line: line,
                                    column: col
                                })
                            },
                            Err(e) => errtok!(line, col, "'{}' is not a valid i64 literal: {}", s, e)
                        }
                    },
                    State::B2 | State::Bx => {
                        let l = if let State::Bx = s { self.index - 1 } else { self.index } - startIdx - 2;
                        let s = self.toString(startIdx + 2, l);
                        match i64::from_str_radix(&s, 2) {
                            Ok(i) => {
                                Some(Token {
                                    ttype: TokenType::IntLiteral(i),
                                    line: line,
                                    column: col
                                })
                            },
                            Err(e) => errtok!(line, col, "'{}' is not a valid i64 literal: {}", s, e)
                        }
                    },
                    State::Err(ref e) => errtok!(line, col, "{}: {}", e, self.peekChar(-1)),
                        // Some(format!("({}:{}) {}: {:?}", line, col, e, self.peekChar(-1)))
                    _ => errtok!(line, col, "Unexpected character in number literal: {:?}", self.peekChar(-1))
                }
            },
            
            _ => errtok!(line, col, "Invalid character {:?}", c)
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if let Some(ref t) = self.buff {
            return Some(t);
        }

        match self.parseToken() {
            Some(t) => {
                self.buff = Some(t);
                Some(self.buff.as_ref().unwrap())
            },
            None => None
        }
    }
}

impl Iterator for Lexer
{
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if let Some(t) = self.buff.take() {
            return Some(t);
        }

        self.parseToken()
    }
}