#[derive(Debug, Clone)]
pub(super) 
enum Token {
    Symbol(Symbol),
    Literal(Literal),
    Keyword(Keyword),
    Identifier(&'static str),
    Whitespace(Whitespace),
    EOF,
}

#[derive(Debug, Clone)]
pub(super) 
enum Whitespace {
    Space(usize),
    Newline,
}

#[derive(Debug, Clone)]
pub(super) 
enum Keyword {
}

#[derive(Debug, Clone)]
pub(super) 
enum Symbol {
    /* delimiters */
    LParen, RParen,
    LBrace, RBrace,
    Comma, Dot,

    /* single-char */
    Lambda,
    Plus, Minus,

    /* multi-char */
    Assign,
}

#[derive(Debug, Clone)]
pub(super) 
enum Literal {
    Char(char),
    Int(i32),
    Float(f64),
}

pub(super) 
fn print_test() {
    println!("Hola from 'lang.rs'");
    use Symbol::*;
    let lparen = Token::Symbol(LParen);
    let lambda = Token::Symbol(Lambda);
    let x      = Token::Identifier("x");
    let dot    = Token::Symbol(Dot);
    let x2     = Token::Identifier("x");
    let rparen = Token::Symbol(RParen);
    let dot2   = Token::Symbol(Dot);
    let eof    = Token::EOF;
    let abstraction = vec![lparen, lambda, x, dot, x2, rparen, dot2, eof];
    println!("{:?}", abstraction);
}

