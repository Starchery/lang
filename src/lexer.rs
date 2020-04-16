mod lang;

use crate::lang::*;
use crate::lang::Whitespace::*;
use crate::lang::Symbol::*;
use crate::lang::Literal::*;

struct Lexer {
    source: &'static str,
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(program: &str) -> Lexer {
        Lexer {
            source: program,
            tokens: Vec::new(),
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        unimplemented!()
    }
}

