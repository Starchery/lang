#![allow(dead_code)]

use logos::Logos;
use logos::source::Source;
use std::ops::Range;
use std::fmt;

trait Lexeme { }

#[derive(Logos, Debug, PartialEq)]
pub(crate) enum Token<'src> {

    Sym(Symbol),

    Key(Keyword),

    Lit(Literal<'src>),

    #[regex(r"[a-zA-Z]+")]
    Id(&'src str),

    EOF,

    #[error]
    InvalidToken
}

#[derive(Debug, PartialEq)]
pub(crate) enum Keyword {
    Type,
    Where
    , If,   Else
    , Let,  Match
    , In,   Trait
    , Impl, For
    , Then, Deriving
    , As,   Alias
    }

#[derive(Debug, PartialEq)]
pub(crate) enum Literal<'src>
    { Str(&'src str)
    , FStr(&'src str)
    , Int(i64)
    , Float(f64)
    }

#[derive(Debug, PartialEq)]
pub(crate) enum Symbol
    // delimiters
   { LParen,  RParen
   , LCurly,  RCurly
   , LBrack,  RBrack
   , Comma ,  Dot

    // single-char
   , Dash,  Equals
   , Plus,  Lambda
   , Star,  Slash
   , Pipe,  Quote
   , Semi,  Apostrophe
   , Hash,  Colon
   , Lt,    Gt

    // multi-char
   , Arrow, Colons
   , EqEq,  Leq
   , Geq,   DashDash
   , FatArrow
}

impl Lexeme for Symbol {}
impl<'src> Lexeme for Literal<'src> {}
impl Lexeme for Keyword {}
impl<'src> Lexeme for Token<'src> {}

pub(crate) fn assert_lex<'a, Token>(
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

#[test]
fn ident() {
    assert_lex(
        "test words",
        &[ (Token::Id("test"), "test", 0..4)
         , (Token::Id("words"), "words", 5..10)
         ]
    )
}
