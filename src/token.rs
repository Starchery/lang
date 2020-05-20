#![allow(dead_code)]

trait Lexeme { }

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub(crate)
enum Symbol {
    //special
    EOF,
    Identifier,
    Int,
    Float,
    Str,
    FStr,
    Char,

    // delimiters
    LParen,
    RParen,
    LCurly,
    RCurly,
    LBrack,
    RBrack,
    Comma,
    Dot,

    // single-char
    Dash,
    Equals,
    Plus,
    Lambda,
    Star,
    Slash,
    Pipe,
    Quote,
    Semi,
    Apostrophe,
    Hash,
    Colon,
    Lt,
    Gt,

     // multi-char
    Arrow,
    Colons,
    EqEq,
    Leq,
    Geq,
    DashDash,
    FatArrow,
    UnexpectedSymbol,
}
