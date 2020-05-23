mod token;

use token::*;

enum Error {
    UnexpectedChar(char),
    UnexpectedEOF
}

type LexResult = Result<Lexer, Error>;

struct Lexer<'src> {
    source: &'src str,
    tokens: Vec<Token>
}

