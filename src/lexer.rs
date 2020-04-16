mod lang;

use lang::Token;
use lang::Whitespace::*;
use lang::Symbol::*;
use lang::Literal::*;

#[derive(Debug)]
pub(crate) 
struct Lexer<'a> {
    source: &'a str,
    stack: Stack,
    tokens: Vec<Token>,
}

#[derive(Debug)]
struct Stack {
    values: String,
    current: Token,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            values: String::new(),
            current: Token::EOF,
        }
    }
}

impl<'a> Lexer<'a> {
    pub(crate) 
    fn new(program: &'a str) -> Lexer<'a> {
        Lexer {
            source: program,
            stack: Stack::new(),
            tokens: Vec::new(),
        }
    }

    pub(crate)
    fn tokenize(&mut self) {
        self.source.char_indices().map(|c| { /* println!("dealing with {} now", c.1); */ match c {
            (_, '0'..='9') => {
                match self.stack.current {
                    Token::Literal(Int(_)) 
                  | Token::Literal(Float(_)) 
                  | Token::Identifier(_) => (),

                    _ => {
                        self.clear_stack();
                        self.stack.current = Token::Literal(Int(0));
                    },
                }
            },
            (pos, '.') => {
                match self.stack.current {
                    Token::Literal(Float(_)) | Token::Literal(Int(_)) => { 
                        match self.source.chars().nth(pos + 1) {
                            Some(digit) if digit.is_ascii_digit() => {
                                match self.stack.current {
                                    Token::Literal(Float(_)) => (),
                                    Token::Literal(Int(_)) => { 
                                        self.stack.current = Token::Literal(Float(0.0));
                                    },
                                    _ => (),
                                }
                            },
                            _ => { 
                                self.clear_stack();
                                self.stack.current = Token::Symbol(Dot); 
                            },
                        }
                    },
                    _ => { 
                        self.clear_stack();
                        self.stack.current = Token::Symbol(Dot); 
                    },
                }
            },
            _ => self.tokens.push(Token::Literal(Char(c.1))),
        }; self.stack.values.push(c.1)}).last();

        self.clear_stack();
        self.tokens.push(Token::EOF);
    }

    fn clear_stack(&mut self) {
        // println!("entering clear_stack with:");
        // println!("{:?}", self);
        self.tokens.push(
            match self.stack.current {
                Token::Literal(Int(_)) => {
                    Token::Literal(Int(
                        self.stack.values.parse::<i32>().unwrap()))
                },
                Token::Literal(Float(_)) => {
                    Token::Literal(Float(
                        self.stack.values.parse::<f64>().unwrap()))
                },
                _ => self.stack.current.clone()
            }
        );
        self.stack.values.clear();
    }

// let mut peeker = self.source.char_indices()
// .skip(pos)
// .take_while(|digit| digit.is_ascii_digit() || digit == '.')
// .map(|digit| match digit {
// (idx, '.') => match self.source.chars().nth(idx + 1) {
// Some('0'..='9') => self.tokens.push(
// Token::Literal(Float(
// self.source.get(pos..(
// self.source.char_indices()
// .skip(idx)
// .take_while(|frac|
// frac[1].is_ascii_digit()
// ).last()
// .unwrap()[0]))
// .parse::<f64>()
// .unwrap()))),
// (idx, '0'..='9') =>
// )),
// )

// });
// }

    pub(crate) 
    fn print_greeting(&self) {
        lang::print_test()
    }
}

