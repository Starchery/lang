mod lang;

use lang::Token;
use lang::Whitespace::*;
use lang::Symbol::*;
use lang::Literal::*;

#[derive(Debug)]
pub(crate) 
struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub(crate) 
    fn new(program: &'a str) -> Lexer<'a> {
        Lexer {
            source: program,
            tokens: Vec::new(),
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        unimplemented!()
    }

    pub(crate) 
    fn print_greeting(&self) {
        lang::print_test()
    }
}

