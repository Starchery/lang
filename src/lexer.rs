mod token;

use token::Token;

enum Error {
    UnexpectedChar(char),
    UnexpectedEOF,
    InvalidToken(token::Error)
}

type LexResult = Result<Lexer, Error>;

struct Lexer<'src> {
    source: Chars,
    tokens: Vec<Token>
}

impl<'src> Lexer {
    fn new(input: &'src str) -> Lexer<'src> {
        Lexer {
            source: input.chars().peekable(),
            tokens: Vec::new()
        }
    }

    fn consume(&mut self, c: char) -> LexResult {
        self.tokens.push(Token::from(self.source.next().ok_or(Error::UnexpectedEOF))?)
    }

    // fn expect(self, c: char) -> LexResult {
    //     match self.source.peek() {
    //         Some(x) if x == c => Ok(self.consume(Token::from(c))),
    //         Some(x) => Err(UnexpectedChar(x)),
    //         _ => Err(UnexpectedEOF)
    //     }
    // }

    fn run(self) -> LexResult {
        loop {
            self.drop_while(char::is_ascii_whitespace);
            match self.source.peek().unwrap_or('\0') {
                '\0' => break,
                i if i.could_start_identifier() => 
                    self.take_ident()?,
                d if d.is_digit() => 
                    self.take_num()?,
                o if o.could_start_operator() => 
                    self.take_op()?,
                '"' => 
                    self.take_string()?,
                '\'' => 
                    self.take_char()?,
                c => 
                    self.consume(c)?,
            } // match
        } // loop

        self.tokens.push(Token::EOF);
        Ok(self)
    } // fn run

    fn drop_while(&mut self, f: impl Fn(char) -> bool) -> () {
        unimplemented!()
    }

    fn take_string(&mut self) -> LexResult {
        unimplemented!()
    }

    fn take_ident(&mut self) -> LexResult {
        unimplemented!()
    }

    fn take_char(&mut self) -> LexResult {
        unimplemented!()
    }

    fn take_num(&mut self) -> LexResult {
        unimplemented!()
    }

    fn take_op(&mut self) -> LexResult {
        unimplemented!()
    }

    // fn string(self) -> LexResult {
    //     let mut strbuilder = String::new();
    //     for c in self.expect('"')?.source {
    //         match c {
    //             '\\' => strbuilder.push(self.alt('n', '\\', '"')?),
    //             '"' => break,
    //             x => strbuilder.push(x)
    //         }
    //     }

    //     self.expect('"')?;
    //     Ok(self.consume(Token::String(strbuilder.as_str())))
    // }
}

impl char {
    fn could_start_identifier(&self) -> bool {
        unimplemented!()
    }

    fn could_start_operator(&self) -> bool {
        unimplemented!()
    }
}
