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
    current: Option<Token>
}

impl Stack {
    fn new() -> Stack {
        Stack {
            values: String::new(),
            current: None,
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
                    Some(Token::Literal(Int(_)))
                  | Some(Token::Literal(Float(_)))
                  | Some(Token::Identifier(_)) => (),

                    _ => {
                        self.clear_stack();
                        self.stack.current = Some(Token::Literal(Int(c.1.to_digit(10).unwrap() as i32)));
                    },
                }
            },
            (pos, '.') => {
                match self.stack.current {
                    Some(Token::Literal(Float(_))) | Some(Token::Literal(Int(_))) => { 
                        match self.source.chars().nth(pos + 1) {
                            Some(digit) if digit.is_ascii_digit() => {
                                match self.stack.current {
                                    Some(Token::Literal(Float(_))) => (),
                                    Some(Token::Literal(Int(_))) => { 
                                        // self.stack.current = Some(Token::Literal(Float(digit.to_digit(10).unwrap() as f64)));
                                        self.stack.current = Some(Token::Literal(Float(self.source.get(pos - 1..pos + 2).unwrap().parse::<f64>().unwrap())));
                                    },
                                    _ => (),
                                }
                            },
                            _ => { 
                                self.clear_stack();
                                self.stack.current = Some(Token::Symbol(Dot)); 
                            },
                        }
                    },
                    _ => { 
                        self.clear_stack();
                        self.stack.current = Some(Token::Symbol(Dot)); 
                    },
                }
            },
            _ => {
                self.clear_stack();
                self.stack.current = Some(Token::Literal(Char(c.1)));
            },
        }; self.stack.values.push(c.1); }).last();

        self.clear_stack();
        self.tokens.push(Token::EOF);
    }

    fn clear_stack(&mut self) {
        // println!("entering clear_stack with:");
        // println!("{:?}", self);
        if let Some(t) = &self.stack.current {
            self.tokens.push(
                match t {
                    Token::Literal(Int(_)) => {
                        Token::Literal(Int(
                            self.stack.values.parse::<i32>().unwrap()))
                    },
                    Token::Literal(Float(_)) => {
                        Token::Literal(Float(
                            self.stack.values.parse::<f64>().unwrap()))
                    },
                    _ => t.clone()
                }
            )
        }
        self.stack.values.clear();
    }

    pub(crate) 
    fn print_greeting(&self) {
        lang::print_test()
    }
}

impl<'a> std::fmt::Display for Lexer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\" -> {:#?}", 
               self.source
                    .chars()
                    .map(|c| 
                         if c.is_ascii_whitespace() { 
                             '_' 
                         } else { 
                             c 
                         }
                    ).collect::<String>(), 
                self.tokens)
    }
}
