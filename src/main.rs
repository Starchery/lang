mod token;

fn main() {
    println!("Hello, world!");
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
