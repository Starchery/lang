#[derive(Debug)]
enum Token {
    DigitLiteral(u8),
    CharLiteral(char),
    Delimiter(char),
    Identifier(char),
    Symbol(char),
    EOF,
}

pub fn print_test() {
    println!("Hola from 'lexer.rs'");
    let lparen = Token::Delimiter('(');
    let lambda = Token::Symbol('\\');
    let x      = Token::Identifier('x');
    let dot    = Token::Symbol('.');
    let x2     = Token::Identifier('x');
    let rparen = Token::Delimiter(')');
    let dot2   = Token::Symbol('.');
    let eof    = Token::EOF;
    let abstraction = vec![lparen, lambda, x, dot, x2, rparen, dot2, eof];
    println!("{:?}", abstraction);
}

