#![allow(dead_code)]

use logos::Logos;
use logos::source::Source;

use std::ops::Range;
use std::fmt;

trait Lexeme { }

#[derive(Logos, Debug, PartialEq)]
pub(crate)
enum Token<'src> {

    Sym(Symbol),

    // Key(Keyword),

    Lit(Literal<'src>),

    #[regex("[a-zA-Z_]+[a-zA-Z_-]*[?!]?",
            |lex| lex.slice())]
    Id(&'src str),

    #[regex(r"\p{White_Space}", logos::skip)]
    EOF,

    #[error]
    UnexpectedToken
}

// #[derive(Debug, PartialEq)]
// pub(crate)
// enum Keyword {
//     Type,
//     Where
//     , If,   Else
//     , Let,  Match
//     , In,   Trait
//     , Impl, For
//     , Then, Deriving
//     , As,   Alias
//     }

#[derive(Logos, Debug, PartialEq)]
pub(crate)
enum Literal<'src> {
    #[regex(r#""[^"\\]*(?:\\.[^"\\]*)*""#)]
    Str(&'src str),

    #[regex(r#"f"[^"\\]*(?:\\.[^"\\]*)*""#)]
    FStr(&'src str),

    #[regex("[1-9]+([0-9]|_[0-9])*([eE][+-]?[0-9]+)?",
      |lex| lex.slice().parse())]
    Int(i64),

    #[regex(r#"([0-9]|_[0-9])+\.([0-9]|_[0-9])*([eE][+-]?[0-9]+)?"#,
      |lex| lex.slice().parse())]
    Float(f64),

    #[error]
    UnexpectedLiteral,
}

#[derive(Logos, Debug, PartialEq)]
pub(crate)
enum Symbol {
    // delimiters
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LCurly,

    #[token("}")]
    RCurly,

    #[token("[")]
    LBrack,

    #[token("]")]
    RBrack,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    // single-char
    #[token("-")]
    Dash,

    #[token("=")]
    Equals,

    #[token("+")]
    Plus,

    #[token("\\")]
    Lambda,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("|")]
    Pipe,

    #[token("\"")]
    Quote,

    #[token(";")]
    Semi,

    #[token("'")]
    Apostrophe,

    #[token("#")]
    Hash,

    #[token(":")]
    Colon,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,

     // multi-char
    #[token("->")]
    Arrow,

    #[token("::")]
    Colons,

    #[token("==")]
    EqEq,

    #[token("<=")]
    Leq,

    #[token(">=")]
    Geq,

    #[token("--")]
    DashDash,

    #[token("=>")]
    FatArrow,

    #[error]
    UnexpectedSymbol,
}

impl Lexeme for Symbol {}
impl<'src> Lexeme for Literal<'src> {}
// impl Lexeme for Keyword {}
impl<'src> Lexeme for Token<'src> {}

pub(crate)
fn assert_lex<'a, Token>(
    source: &'a Token::Source,
    tokens: &[(Token, &'a <Token::Source as Source>::Slice, Range<usize>)],
) where
    Token: Logos<'a> + fmt::Debug + PartialEq,
{
    let mut lex = Token::lexer(source);

    for tuple in tokens {
        assert_eq!(&(lex.next().expect("Unexpected end"), lex.slice(), lex.span()), tuple);
    }

    assert_eq!(lex.next(), None);
}

// #[test]
// fn ident() {
//     assert_lex(
//         "test words",
//         &[ (Token::Id("test"), "test", 0..4)
//          , (Token::Id("words"), "words", 5..10)
//          ]
//     )
// }
