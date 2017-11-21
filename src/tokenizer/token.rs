use std::rc::Rc;

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
pub struct Token {
    pub token_type: Type,
    pub line:       usize,
    pub slice:      (usize, usize),
    pub lexeme:     Option<Rc<String>>,
}

impl Token {
    pub fn new(
        token_type: Type,
        line:       usize,
        slice:      (usize, usize),
        lexeme:     Option<Rc<String>>,
    ) -> Token {
        Token {
            token_type,
            line,
            slice,
            lexeme,
        }
    }

    pub fn number (line: usize, slice: (usize, usize), lexeme: &str) -> Token {
        Token::new(Type::Number, line, slice, Some(Rc::new(lexeme.to_string())))
    }

    pub fn string (line: usize, slice: (usize, usize), lexeme: &str) -> Token {
        Token::new(Type::String, line, slice, Some(Rc::new(lexeme.to_string())))
    }

    pub fn word (line: usize, slice: (usize, usize), lexeme: &str) -> Token {
        Token::new(Type::Word, line, slice, Some(Rc::new(lexeme.to_string())))
    }

    pub fn symbol (line: usize, slice: (usize, usize), lexeme: &str) -> Token {
        Token::new(Type::Symbol, line, slice, Some(Rc::new(lexeme.to_string())))
    }

    pub fn indent (line: usize) -> Token {
        Token::new(Type::Indent, line, (0,0), None)
    }

    pub fn dedent (line: usize) -> Token {
        Token::new(Type::Dedent, line, (0,0), None)
    }

    pub fn newline (line: usize) -> Token {
        Token::new(Type::EOL, line, (0,0), None)
    }

    pub fn eof (line: usize) -> Token {
        Token::new(Type::EOF, line, (0,0), None)
    }
}
