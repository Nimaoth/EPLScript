#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
use token::*;

macro_rules! errtok(( $ line : expr , $ col : expr , $ format : expr ) => {
                    Some (
                    Token {
                    ttype : TokenType :: Error ( String :: from ( $ format ) )
                    , line : $ line , column : $ col } ) } ; (
                    $ line : expr , $ col : expr , $ format : expr , $ (
                    $ a : expr ) , * ) => {
                    Some (
                    Token {
                    ttype : TokenType :: Error (
                    format ! ( $ format , $ ( $ a ) , * ) ) , line : $ line ,
                    column : $ col } ) } ;);

pub struct Lexer {
    chars: Vec<char>,
    index: usize,
    line: usize,
    col: usize,

    buff: Option<Token>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Lexer {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Lexer {
            chars: ref __self_0_0,
            index: ref __self_0_1,
            line: ref __self_0_2,
            col: ref __self_0_3,
            buff: ref __self_0_4 } => {
                let mut builder = __arg_0.debug_struct("Lexer");
                let _ = builder.field("chars", &&(*__self_0_0));
                let _ = builder.field("index", &&(*__self_0_1));
                let _ = builder.field("line", &&(*__self_0_2));
                let _ = builder.field("col", &&(*__self_0_3));
                let _ = builder.field("buff", &&(*__self_0_4));
                builder.finish()
            }
        }
    }
}

impl Lexer {
    pub fn new(code: &str) -> Lexer {
        Lexer{chars: code.chars().collect(),
              index: 0,
              line: 1,
              col: 1,
              buff: None,}
    }

    fn peekChar(&self, index: i32) -> char {
        let i = (self.index as i32 + index) as usize;
        if i >= self.chars.len() { return '\u{0}'; }

        self.chars[i]
    }

    fn skipWC(&mut self, newLineIsWs: bool) {
        while self.index < self.chars.len() {
            let c = self.chars[self.index];

            match c {
                '/' if self.peekChar(1) == '/' => {
                    self.index += 1;
                    while self.index + 1 < self.chars.len() {
                        if self.peekChar(1) == '\n' { break ; }
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

    fn isDigit(c: char) -> bool { c >= '0' && c <= '9' }

    fn isBinDigit(c: char) -> bool { c >= '0' && c <= '1' }

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
                Some(Token{ttype: TokenType::NewLine,
                           line: line,
                           column: col,})
            }


            c if Self::isIdChar(c) => {
                let start = self.index - 1;
                while self.index < self.chars.len() {
                    let c = self.chars[self.index];
                    if !Self::isIdChar(c) && !Self::isDigit(c) { break ; }

                    self.index += 1;
                    self.col += 1;
                }
                let s = self.toString(start, self.index - start);
                let ttype =
                    match &s[..] {
                        "print" => TokenType::Kprint,
                        _ => TokenType::Identifier(s),
                    };
                Some(Token{ttype: ttype, line: line, column: col,})
            }


            c if Self::isDigit(c) => {
                enum State {
                    s0,
                    i1,
                    i2,
                    ix,
                    b1,
                    b2,
                    bx,
                    h1,
                    h2,
                    hx,
                    d1,
                    d2,
                    dx,
                    err(&'static str),
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::fmt::Debug for State {
                    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                     -> ::std::fmt::Result {
                        match (&*self,) {
                            (&State::s0,) => {
                                let mut builder = __arg_0.debug_tuple("s0");
                                builder.finish()
                            }
                            (&State::i1,) => {
                                let mut builder = __arg_0.debug_tuple("i1");
                                builder.finish()
                            }
                            (&State::i2,) => {
                                let mut builder = __arg_0.debug_tuple("i2");
                                builder.finish()
                            }
                            (&State::ix,) => {
                                let mut builder = __arg_0.debug_tuple("ix");
                                builder.finish()
                            }
                            (&State::b1,) => {
                                let mut builder = __arg_0.debug_tuple("b1");
                                builder.finish()
                            }
                            (&State::b2,) => {
                                let mut builder = __arg_0.debug_tuple("b2");
                                builder.finish()
                            }
                            (&State::bx,) => {
                                let mut builder = __arg_0.debug_tuple("bx");
                                builder.finish()
                            }
                            (&State::h1,) => {
                                let mut builder = __arg_0.debug_tuple("h1");
                                builder.finish()
                            }
                            (&State::h2,) => {
                                let mut builder = __arg_0.debug_tuple("h2");
                                builder.finish()
                            }
                            (&State::hx,) => {
                                let mut builder = __arg_0.debug_tuple("hx");
                                builder.finish()
                            }
                            (&State::d1,) => {
                                let mut builder = __arg_0.debug_tuple("d1");
                                builder.finish()
                            }
                            (&State::d2,) => {
                                let mut builder = __arg_0.debug_tuple("d2");
                                builder.finish()
                            }
                            (&State::dx,) => {
                                let mut builder = __arg_0.debug_tuple("dx");
                                builder.finish()
                            }
                            (&State::err(ref __self_0),) => {
                                let mut builder = __arg_0.debug_tuple("err");
                                let _ = builder.field(&&(*__self_0));
                                builder.finish()
                            }
                        }
                    }
                }

                let mut s = match c { '0' => State::s0, _ => State::i1, };
                // println!("{:?} -> {:?}", c, s);
                let startIdx = self.index - 1;

                // parse text, check if valid
                while self.index < self.chars.len() {
                    let c = self.chars[self.index];
                    self.index += 1;

                    // print!("{:?} + {:?} -> ", s, c);

                    s =
                        match (s, c) {
                            (State::s0, 'f') => State::dx,
                            (State::i1, 'f') => State::dx,
                            (State::i2, 'f') => State::dx,


                            (State::s0, 'x') => State::h1,
                            (State::h1, c) if Self::isHexDigit(c) =>
                            State::h2,
                            (State::h1, _) =>
                            State::err("Number literal 0x has to be followed by hexadecimal digits"),
                            (State::h2, c) if Self::isHexDigit(c) =>
                            State::h2,
                            (State::h2, _) => State::hx,


                            (State::s0, 'b') => State::b1,
                            (State::b1, c) if Self::isBinDigit(c) =>
                            State::b2,
                            (State::b1, _) =>
                            State::err("Number literal 0b has to be followed by binary digits"),
                            (State::b2, c) if Self::isBinDigit(c) =>
                            State::b2,
                            (State::b2, c) if Self::isDigit(c) =>
                            State::err("Number literal 0b has to be followed by binary digits"),
                            (State::b2, _) => State::bx,


                            (State::s0, '.') => State::d1,
                            (State::d1, c) if Self::isDigit(c) => State::d2,
                            (State::d1, _) =>
                            State::err("Number literal 0. has to be followed by decimal digits"),
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
                            _ => State::err("Unexpected character"),
                        };



                    // convert parsed text to number
                    // Err(e) => Some(Err(format!("'{}' is not a valid i64 literal: {}", s, e)))
                    // Err(e) => Some(Err(format!("'{}' is not a valid i64 literal: {}", s, e)))
                    // Err(e) => Some(Err(format!("'{}' is not a valid i64 literal: {}", s, e)))
                    // Err(e) => Some(Err(format!("'{}' is not a valid i64 literal: {}", s, e)))
                    // Some(format!("({}:{}) {}: {:?}", line, col, e, self.peekChar(-1)))
                    // Some(format!("({}:{}) Unexpected character in number literal: {:?}", line, col, self.peekChar(-1)))

                    // Some(format!("({}:{}) Unrecognized character: {}", line, col, c))





                    ::io::_print(::std::fmt::Arguments::new_v1_formatted(&["",
                                                                           "\n"],
                                                                         &match (&s,)
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
                    if let State::err(_) = s { break ; }
                }
                match s {
                    State::s0 | State::i1 | State::i2 | State::ix => {
                        let l =
                            if let State::ix = s {
                                self.index - 1
                            } else { self.index } - startIdx;
                        let s = self.toString(startIdx, l);
                        match i64::from_str_radix(&s, 10) {
                            Ok(i) => {
                                Some(Token{ttype: TokenType::IntLiteral(i),
                                           line: line,
                                           column: col,})
                            }
                            Err(e) => None,
                        }
                    }
                    State::d2 | State::dx => {
                        let l =
                            if let State::dx = s {
                                self.index - 1
                            } else { self.index } - startIdx;
                        let s = self.toString(startIdx, l);
                        match s.parse::<f64>() {
                            Ok(i) => {
                                Some(Token{ttype: TokenType::FloatLiteral(i),
                                           line: line,
                                           column: col,})
                            }
                            Err(e) => None,
                        }
                    }
                    State::h2 | State::hx => {
                        let l =
                            if let State::hx = s {
                                self.index - 1
                            } else { self.index } - startIdx - 2;
                        let s = self.toString(startIdx + 2, l);
                        match i64::from_str_radix(&s, 16) {
                            Ok(i) => {
                                Some(Token{ttype: TokenType::IntLiteral(i),
                                           line: line,
                                           column: col,})
                            }
                            Err(e) => None,
                        }
                    }
                    State::b2 | State::bx => {
                        let l =
                            if let State::bx = s {
                                self.index - 1
                            } else { self.index } - startIdx - 2;
                        let s = self.toString(startIdx + 2, l);
                        match i64::from_str_radix(&s, 2) {
                            Ok(i) => {
                                Some(Token{ttype: TokenType::IntLiteral(i),
                                           line: line,
                                           column: col,})
                            }
                            Err(e) => None,
                        }
                    }
                    State::err(ref e) => { None }
                    _ => { None }
                }
            }
            _ =>
            Some(Token{ttype:
                           TokenType::Error(::fmt::format(::std::fmt::Arguments::new_v1_formatted(&["Invalid character "],
                                                                                                  &match (&c,)
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
                                                                                                                                                                        ::std::fmt::rt::v1::Count::Implied,},}]))),
                       line: line,
                       column: col,}),
        }
    }
    pub fn peek(&mut self) -> Option<&Token> {
        if let Some(ref t) = self.buff { return Some(t); }
        match self.parseToken() {
            Some(t) => {
                self.buff = Some(t);
                Some(self.buff.as_ref().unwrap())
            }
            None => None,
        }
    }
}
impl Iterator for Lexer {
    type
    Item
    =
    Token;
    fn next(&mut self) -> Option<Token> {
        if let Some(t) = self.buff.take() { return Some(t); }
        self.parseToken()
    }
}
