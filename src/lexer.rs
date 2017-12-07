use token::*;

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
        if i < 0 || i >= self.chars.len() {
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

    fn parseToken(&mut self) -> Option<Result<Token, String>> {
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
                Some(Ok(Token {
                    ttype: TokenType::NewLine,
                    line: line,
                    column: col
                }))
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
                Some(Ok(Token {
                    ttype: ttype,
                    line: line,
                    column: col
                }))
            },

            c if Self::isDigit(c) => {
                #[derive(Debug)]
                enum State {
                    s0,
                    i1, i2, ix,
                    b1, b2, bx,
                    h1, h2, hx,
                    d1, d2, dx,
                    err(&'static str)
                }
                
                let mut s = match c {
                    '0' => State::s0,
                    _ => State::i1
                };
                // println!("{:?} -> {:?}", c, s);
                let startIdx = self.index - 1;

                // parse text, check if valid
                while self.index < self.chars.len() {
                    let c = self.chars[self.index];
                    self.index += 1;

                    // print!("{:?} + {:?} -> ", s, c);

                    s = match (s, c) {
                        (State::s0, 'f') => State::dx,
                        (State::i1, 'f') => State::dx,
                        (State::i2, 'f') => State::dx,

                        (State::s0, 'x') => State::h1,
                        (State::h1, c) if Self::isHexDigit(c) => State::h2,
                        (State::h1, _) => State::err("Number literal 0x has to be followed by hexadecimal digits"),
                        (State::h2, c) if Self::isHexDigit(c) => State::h2,
                        (State::h2, _) => State::hx,

                        (State::s0, 'b') => State::b1,
                        (State::b1, c) if Self::isBinDigit(c) => State::b2,
                        (State::b1, _) => State::err("Number literal 0b has to be followed by binary digits"),
                        (State::b2, c) if Self::isBinDigit(c) => State::b2,
                        (State::b2, c) if Self::isDigit(c) => State::err("Number literal 0b has to be followed by binary digits"),
                        (State::b2, _) => State::bx,

                        (State::s0, '.') => State::d1,
                        (State::d1, c) if Self::isDigit(c) => State::d2,
                        (State::d1, _) => State::err("Number literal 0. has to be followed by decimal digits"),
                        (State::d2, c) if Self::isDigit(c) => State::d2,
                        (State::d2, _) => State::dx,


                        (State::s0, c) if Self::isDigit(c) => State::i1,
                        (State::i1, '.') => State::d1,
                        (State::i1, c) if Self::isDigit(c) => State::i2,
                        (State::i1, _) => State::ix,
                        (State::i2, c) if Self::isDigit(c) => State::i2,
                        (State::i2, '.') => State::d1,
                        (State::i2, _) => State::ix,

                        (State::s0, _) => State::ix,
                        _ => State::err("Unexpected character")
                    };

                    println!("{:?}", s);

                    if let State::err(_) = s {
                        break;
                    }
                }

                // convert parsed text to number
                match s {
                    State::s0 | State::i1 | State::i2 | State::ix => {
                        let l = if let State::ix = s { self.index - 1 } else { self.index } - startIdx;
                        let s = self.toString(startIdx, l);
                        match i64::from_str_radix(&s, 10) {
                            Ok(i) => {
                                Some(Ok(Token {
                                    ttype: TokenType::IntLiteral(i),
                                    line: line,
                                    column: col
                                }))
                            },
                            Err(e) => Some(Err(format!("'{}' is not a valid i64 literal: {}", s, e)))
                        }
                    },
                    State::d2 | State::dx => {
                        let l = if let State::dx = s { self.index - 1 } else { self.index } - startIdx;
                        let s = self.toString(startIdx, l);
                        match s.parse::<f64>() {
                            Ok(i) => {
                                Some(Ok(Token {
                                    ttype: TokenType::FloatLiteral(i),
                                    line: line,
                                    column: col
                                }))
                            },
                            Err(e) => Some(Err(format!("'{}' is not a valid i64 literal: {}", s, e)))
                        }
                    },
                    State::h2 | State::hx => {
                        let l = if let State::hx = s { self.index - 1 } else { self.index } - startIdx - 2;
                        let s = self.toString(startIdx + 2, l);
                        match i64::from_str_radix(&s, 16) {
                            Ok(i) => {
                                Some(Ok(Token {
                                    ttype: TokenType::IntLiteral(i),
                                    line: line,
                                    column: col
                                }))
                            },
                            Err(e) => Some(Err(format!("'{}' is not a valid i64 literal: {}", s, e)))
                        }
                    },
                    State::b2 | State::bx => {
                        let l = if let State::bx = s { self.index - 1 } else { self.index } - startIdx - 2;
                        let s = self.toString(startIdx + 2, l);
                        match i64::from_str_radix(&s, 2) {
                            Ok(i) => {
                                Some(Ok(Token {
                                    ttype: TokenType::IntLiteral(i),
                                    line: line,
                                    column: col
                                }))
                            },
                            Err(e) => Some(Err(format!("'{}' is not a valid i64 literal: {}", s, e)))
                        }
                    },
                    State::err(ref e) => {
                        Some(Err(format!("({}:{}) {}: {:?}", line, col, e, self.peekChar(-1))))
                    }
                    _ => {
                        Some(Err(format!("({}:{}) Unexpected character in number literal: {:?}", line, col, self.peekChar(-1))))
                    }
                }
            },
            
            _ => Some(Err(format!("({}:{}) Unrecognized character: {}", self.line, self.col, c)))
        }
    }

    pub fn peek(&mut self) -> Option<Result<&Token, String>> {
        if let Some(ref t) = self.buff {
            return Some(Ok(t));
        }

        match self.parseToken() {
            Some(r) => {
                match r {
                    Ok(t) => {
                        self.buff = Some(t);
                        Some(Ok(self.buff.as_ref().unwrap()))
                    },
                    Err(e) => Some(Err(e))
                }
            },
            None => None
        }
    }
}

impl Iterator for Lexer
{
    type Item = Result<Token, String>;

    

    fn next(&mut self) -> Option<Result<Token, String>> {
        if let Some(t) = self.buff.take() {
            return Some(Ok(t));
        }

        self.parseToken()
    }
}