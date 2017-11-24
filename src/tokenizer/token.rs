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

impl Type {
     pub fn from_str (s: &str) -> Option<Self> {
        match s {
            "Number" | "NUMBER" => Some(Type::Number),
            "String" | "STRING" => Some(Type::String),
            "Symbol" | "SYMBOL" => Some(Type::Symbol),
            "Word"   | "WORD"   => Some(Type::Word),

            "Indent" | "INDENT" => Some(Type::Indent),
            "Dedent" | "DEDENT" => Some(Type::Dedent),

            "EOL" => Some(Type::EOL),
            "EOF" => Some(Type::EOF),

            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum PartialToken<'s> {
    Type(Type),
    Lexeme(&'s str),
    Pair(Type, &'s str),
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
pub struct TokenIterator<'t, 's: 't> {
    tokens: &'t Vec<Token<'s>>,
    pub current: usize,
}

impl<'t, 's: 't> TokenIterator<'t, 's> {
    pub fn new (tokens: &'t Vec<Token<'s>>) -> Self {
        Self { tokens: tokens, current: 0 }
    }

    pub fn get (&self, offset: usize) -> Option<&'t Token<'s>> {
        if offset + self.current >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.current + offset])
        }
    }

    pub fn eat (&mut self, offset: usize) {
        if self.current + offset <= self.tokens.len() {
            self.current += offset;
        } else {
            self.current = self.tokens.len() - 1;
        }
    }

    pub fn next(&mut self) -> Option<&'t Token<'s>> {
        if self.current >= self.tokens.len() {
            None
        } else {
            self.current += 1;
            Some(&self.tokens[self.current - 1])
        }
    }

    pub fn check(&self, tokens: &[PartialToken]) -> bool {
        if tokens.len() + self.current > self.tokens.len() {
            return false
        }

        let mut offset = 0;

        for token in tokens {
            if ! match token {
                &PartialToken::Type(ref t)   =>
                    self.get(offset).unwrap().token_type == *t,
                &PartialToken::Lexeme(ref l) =>
                    self.get(offset).unwrap().lexeme == Some(*l),
                &PartialToken::Pair(ref t, ref l) => {
                    let tk = self.get(offset).unwrap();
                    tk.lexeme == Some(*l) && tk.token_type == *t
                },
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
