mod lexer;

fn main() {
    println!("Hello, world!");

    let lex = lexer::Lexer::new("3.");
    println!("{:?}", lex);

    lex.print_greeting();
}
