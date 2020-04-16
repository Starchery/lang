mod lexer;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let program = fs::read_to_string("tests/test1.lang")?;

    let mut lex = lexer::Lexer::new(&program);
    lex.tokenize();
    println!("{}", lex);

    Ok(())
    // lex.print_greeting();
}
