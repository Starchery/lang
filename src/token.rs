

enum Token<'T>
    { Sym(Symbol)
    , Key(Keyword)
    , Lit(Literal)
    , Id(&'T str)
    , EOF
    }

enum Keyword
    { Type, Where
    , If,   Else
    , Let,  Match
    , In,   Trait
    , Impl, For
    , Then, Deriving
    , As,   Alias
    }

enum Literal<'L>
    { Str(&'L str)
    , Int(i64)
    , Float(f64)
    }

enum Symbol
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
   , EqEq,  Leq, Geq
}

