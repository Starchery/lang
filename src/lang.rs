#[derive(Debug)]
enum Token {
    Symbol(Symbol),
    Literal(Literal),
    Keyword(Keyword),
    Identifier(&'static str),
    Whitespace(Whitespace),
    EOF,
}

#[derive(Debug)]
enum Whitespace {
    Space,
    Newline,
}

#[derive(Debug)]
enum Keyword {
}

#[derive(Debug)]
enum Symbol {
    /* delimiters */
    LParen, RParen,
    LBrace, RBrace,

    /* single-char */
    Dot, Comma,
    Lambda,
    Plus, Minus,

    /* multi-char */
    Assign,
}

#[derive(Debug)]
enum Literal {
    Char(char),
    Int(i32),
    Float(f64),
}

pub fn print_test() {
    println!("Hola from 'lexer.rs'");
    let lparen = Token::Symbol(Symbol::LParen);
    let lambda = Token::Symbol(Symbol::Lambda);
    let x      = Token::Identifier("x");
    let dot    = Token::Symbol(Symbol::Dot);
    let x2     = Token::Identifier("x");
    let rparen = Token::Symbol(Symbol::RParen);
    let dot2   = Token::Symbol(Symbol::Dot);
    let eof    = Token::EOF;
    let abstraction = vec![lparen, lambda, x, dot, x2, rparen, dot2, eof];
    println!("{:?}", abstraction);
}

