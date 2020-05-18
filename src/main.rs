mod token;
use logos::Logos;

fn main() {
    let mut litlex = token::Literal::lexer(r#"345 1.34 "Hello!" f"test""#);
    println!("Hello, world!");
    while let Some(tok) = litlex.next() {
        println!("{:?}", &(tok, litlex.span()));
    }
}

#[test]
fn ident() {
    use token::*;
    assert_lex(
        "test words",
        &[ (Token::Id("test"), "test", 0..4)
         , (Token::Id("words"), "words", 5..10)
         ]
    )
}

#[test]
fn strings() {
    use token::*;
    assert_lex(
        r#""Hello" "\"world\"!""#,
        &[ (Literal::Str("\"Hello\""), "\"Hello\"", 0..7)
         , (Literal::Str(r#""\"world\"!""#), r#""\"world\"!""#, 8..20)
         ]
    )
}
