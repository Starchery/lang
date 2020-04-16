mod lang;

use lang::Token as T;
use lang::Whitespace::*;
use lang::Symbol::*;
use lang::Literal::*;

#[derive(Debug)]
pub(crate) 
struct Lexer<'a> {
    source: &'a str,
    stack: Stack,
    tokens: Vec<T>,
}

#[derive(Debug)]
struct Stack {
    values: String,
    current: Option<T>
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
                    Some(T::Literal(Int(_)))
                  | Some(T::Literal(Float(_)))
                  | Some(T::Identifier(_)) => (),

                    _ => {
                        self.clear_stack();
                        self.stack.current = Some(T::Literal(Int(c.1.to_digit(10).unwrap() as i32)));
                    },
                }
            },
            (pos, '.') => {
                match self.stack.current {
                    Some(T::Literal(Float(_))) | Some(T::Literal(Int(_))) => { 
                        match self.source.chars().nth(pos + 1) {
                            Some(digit) if digit.is_ascii_digit() => {
                                match self.stack.current {
                                    Some(T::Literal(Float(_))) => (),
                                    Some(T::Literal(Int(_))) => { 
                                        self.stack.current = Some(T::Literal(Float(self.source.get(pos - 1..pos + 2).unwrap().parse::<f64>().unwrap())));
                                    },
                                    _ => (),
                                }
                            },
                            _ => { 
                                self.clear_stack();
                                self.stack.current = Some(T::Symbol(Dot)); 
                            },
                        }
                    },
                    _ => { 
                        self.clear_stack();
                        self.stack.current = Some(T::Symbol(Dot)); 
                    },
                }
            },
            (_, '+') if !matches!(self.stack.current, Some(T::Symbol(Plus))) => {
                self.clear_stack();
                self.stack.current = Some(T::Symbol(Plus));
            },
            (_, '-') if !matches!(self.stack.current, Some(T::Symbol(Minus))) => {
                self.clear_stack();
                self.stack.current = Some(T::Symbol(Minus));
            },
            (_, '(') if !matches!(self.stack.current, Some(T::Symbol(LParen))) => {
                self.clear_stack();
                self.stack.current = Some(T::Symbol(LParen));
            },
            (_, ')') if !matches!(self.stack.current, Some(T::Symbol(RParen))) => {
                self.clear_stack();
                self.stack.current = Some(T::Symbol(RParen));
            },
            (_, '{') if !matches!(self.stack.current, Some(T::Symbol(LBrace))) => {
                self.clear_stack();
                self.stack.current = Some(T::Symbol(LParen));
            },
            (_, '}') if !matches!(self.stack.current, Some(T::Symbol(RBrace))) => {
                self.clear_stack();
                self.stack.current = Some(T::Symbol(RParen));
            },
            (_, ',') if !matches!(self.stack.current, Some(T::Symbol(Comma))) => {
                self.clear_stack();
                self.stack.current = Some(T::Symbol(Comma));
            },
            (_, '\n') => {
                match self.stack.current {
                    Some(T::Whitespace(Newline)) => (),
                    _ => {
                        self.clear_stack();
                        self.stack.current = Some(T::Whitespace(Newline));
                    },
                }
            },

            (_, s) if s.is_ascii_whitespace() => {
                match self.stack.current {
                    Some(T::Whitespace(Space(_))) => (),
                    _ => {
                        self.clear_stack();
                        self.stack.current = Some(T::Whitespace(Space(1)));
                    },
                }
            },

            _ => {
                self.clear_stack();
                self.stack.current = Some(T::Literal(Char(c.1)));
            },
        }; self.stack.values.push(c.1); }).last();

        self.clear_stack();
        self.tokens.push(T::EOF);
    }

    fn clear_stack(&mut self) {
        // println!("entering clear_stack with:");
        // println!("{:?}", self);
        if let Some(t) = &self.stack.current {
            self.tokens.push(
                match t {
                    T::Literal(Int(_)) => {
                        T::Literal(Int(
                            self.stack.values.parse::<i32>().unwrap()))
                    },
                    T::Literal(Float(_)) => {
                        T::Literal(Float(
                            self.stack.values.parse::<f64>().unwrap()))
                    },
                    T::Whitespace(Space(_)) => T::Whitespace(Space(self.stack.values.len())),
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
            self.source .replace(" ", "_") .replace("\n", "\\n"),
            self.tokens)
    }
}
