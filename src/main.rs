mod lexer;

fn main() {
    println!("Hello, world!");

    let mut lex = lexer::Lexer::new("3.14");
    lex.tokenize();
    println!("{:?}", lex);

    // lex.print_greeting();
}
