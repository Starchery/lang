#![allow(dead_code)]

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate)
enum Token<'src> {
    // special
    EOF,
    UnexpectedSymbol,
    Identifier(&'src str),
    Operator(&'src str),

    // literals
    Int(i64),
    Rational(i64, i64),
    Float(f64),
    Str(&'src str),
    FStr(&'src str),
    Char(char),

    // delimiters
    LParen, RParen,
    LCurly, RCurly,
    LBrack, RBrack,
    Comma,
    Dot,

    // single-char
    Equals,
    Lambda,
    Pipe,
    Quote, SingleQuote,
    Semi,
    Colon,

     // multi-char
    Arrow,
    ColCol,
    DashDash,
    PipeDash,
    FatArrow,
}
