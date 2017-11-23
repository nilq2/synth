#[derive(Debug, PartialEq)]
pub enum Type {
    Number,
    String,
    Word,
    Symbol,

    Indent,
    Dedent,

    EOL,
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub token_type: Type,
    pub line:       usize,
    pub slice:      (usize, usize),
    pub lexeme:     Option<&'a str>,
}

}




impl<'a> Token<'a> {
    pub fn new (token_type: Type, line: usize, slice: (usize, usize), lexeme: Option<&str>) -> Token {
        Token { token_type, line, slice, lexeme }
    }

    pub fn number(line: usize, slice: (usize, usize), lexeme: &str) -> Token {
        Token::new(Type::Number, line, slice, Some(lexeme))
    }

    pub fn string(line: usize, slice: (usize, usize), lexeme: &str) -> Token {
        Token::new(Type::String, line, slice, Some(lexeme))
    }

    pub fn word(line: usize, slice: (usize, usize), lexeme: &str) -> Token {
        Token::new(Type::Word, line, slice, Some(lexeme))
    }

    pub fn symbol(line: usize, slice: (usize, usize), lexeme: &str) -> Token {
        Token::new(Type::Symbol, line, slice, Some(lexeme))
    }

    pub fn indent<'b>(line: usize) -> Token<'b> {
        Token::new(Type::Indent, line, (0,0), None)
    }

    pub fn dedent<'b>(line: usize) -> Token<'b> {
        Token::new(Type::Dedent, line, (0,0), None)
    }

    pub fn newline<'b>(line: usize) -> Token<'b> {
        Token::new(Type::EOL, line, (0,0), None)
    }

    pub fn eof<'b>(line: usize) -> Token<'b> {
        Token::new(Type::EOF, line, (0,0), None)
    }
}
