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

#[derive(Debug)]
pub enum PartialToken<'pt> {
    Type(Type),
    Lexeme(Option<&'pt str>),
    Token(Token<'pt>),
}

#[derive(Debug, PartialEq)]
pub struct Token<'t> {
    pub token_type: Type,
    pub line:       usize,
    pub slice:      (usize, usize),
    pub lexeme:     Option<&'t str>,
}


impl<'t> Token<'t> {
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


#[derive(Debug)]
pub struct TokenIterator<'ti> {
    tokens: &'ti Vec<Token<'ti>>,
    current: usize,
}

impl<'ti> TokenIterator<'ti> {
    pub fn new (tokens: &'ti Vec<Token<'ti>>) -> TokenIterator<'ti> {
        TokenIterator { tokens: tokens, current: 0 }
    }

    pub fn get (&self, offset: usize) -> &Token<'ti> {
        &self.tokens[self.current + offset]
    }

    pub fn next(&mut self) -> &Token<'ti> {
        self.current += 1;
        &self.tokens[self.current - 1]
    }

    pub fn check(&self, tokens: &[PartialToken]) -> bool {
        let mut offset = 0;

        for token in tokens {
            if ! match token {
                &PartialToken::Type(ref t)   => self.get(offset).token_type == *t,
                &PartialToken::Lexeme(ref l) => self.get(offset).lexeme == *l,
                &PartialToken::Token(ref tk) => self.get(offset) == tk,
            } {
                return false
            }

            offset += 1;
        }

        true
    }

    pub fn match_with(&mut self, tokens: &[PartialToken]) -> bool {
        if self.check(tokens) {
            self.current += tokens.len();
            true

        } else {
            false
        }
    }
}
