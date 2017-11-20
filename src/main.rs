use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::{Peekable, Enumerate};
use std::str::Chars;


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
pub struct Token<'a> {
    pub ttype: Type,
    pub line: usize,
    pub slice: (usize, usize),
    pub lexeme: Option<&'a str>,
}

impl<'a> Token<'a> {
    fn new (
        ttype: Type,
        line: usize,
        slice: (usize, usize),
        lexeme: Option<&'a str>
    ) -> Token {
        Token { ttype, line, slice, lexeme }
    }

    fn number (line: usize, slice: (usize, usize), lexeme: &'a str) -> Token<'a> {
        Token::new(Type::Number, line+1, slice, Some(lexeme))
    }

    fn string (line: usize, slice: (usize, usize), lexeme: &'a str) -> Token<'a> {
        Token::new(Type::String, line+1, slice, Some(lexeme))
    }

    fn word (line: usize, slice: (usize, usize), lexeme: &'a str) -> Token<'a> {
        Token::new(Type::Word, line+1, slice, Some(lexeme))
    }

    fn symbol (line: usize, slice: (usize, usize), lexeme: &'a str) -> Token<'a> {
        Token::new(Type::Symbol, line+1, slice, Some(lexeme))
    }

    fn indent (line: usize) -> Token<'a> {
        Token::new(Type::Indent, line+1, (0,0), None)
    }

    fn dedent (line: usize) -> Token<'a> {
        Token::new(Type::Dedent, line+1, (0,0), None)
    }

    fn newline (line: usize) -> Token<'a> {
        Token::new(Type::EOL, line+1, (0,0), None)
    }

    fn eof (line: usize) -> Token<'a> {
        Token::new(Type::EOF, line+1, (0,0), None)
    }
}




#[derive(Debug)]
pub struct Source<'a> {
    path: String,
    lines: Vec<String>,
    tokens: Option<Vec<Token<'a>>>,
    directives: Vec<(String, String)>, // control directives
}

impl<'a> Source<'a> {
    pub fn new (path: &str, ctrl_char: Option<&str>) -> Source<'a> {
        let f: File = match File::open(path) {
            Ok(v) => v,
            Err(_) => panic!("No such file: {}", &path),
        };

        let mut lines: Vec<String> = Vec::new();
        let mut directives: Vec<(String, String)> = Vec::new();

        let file = BufReader::new(&f);

        if let Some(ctrl) = ctrl_char {
            for line in file.lines() {
                let line = line.unwrap();

                if line.starts_with(ctrl) {
                    directives.push ((
                        line[ctrl.len()..line.find(" ").unwrap()].to_string(),
                        line[line.find(" ").unwrap()+1..].to_string()
                    ));
                    lines.push("".to_string());

                } else {
                    lines.push(line);
                }
            }

        } else {
            for line in file.lines() {
                let line = line.unwrap();
                lines.push(line);
            }
        }

        Source {
            path: path.to_string(),
            lines: lines,
            tokens: None,
            directives: directives,
        }
    }

    pub fn get_directive (&self, name: &str) -> Option<&String> {
        self.directives.iter().find(|n| n.0 == name).map(|n| &n.1)
    }
}



pub fn matches (iter: &mut Peekable<Enumerate<Chars>>, string: &[&str]) -> Option<String> {
    None
}


pub fn tokenize<'a> (src: &mut Source<'a>) {
    let mut indents = Vec::new();
    let mut tokens: Vec<Token<'a>> = Vec::new();

    for (l, line) in src.lines.iter().enumerate() {
        let mut indent = 0;
        let mut start = false;
        let mut comment = false;
        let mut iter = line.chars().enumerate().peekable();

        let mut string_d: Vec<&str>;
        let mut comment_d: Vec<&str>;


        if let Some(string_delim) = src.get_directive("string") {
            for delim in string_delim.split_whitespace() {
                string_d.push(delim);
            }
        }

        if let Some(comment_delim) = src.get_directive("comment") {
            for delim in comment_delim.split_whitespace() {
                comment_d.push(delim);
            }
        }


        while let Some((from, next)) = iter.next() {
            if !start && next.is_whitespace() {
                indent += 1;

            } else if !start {
                start = true;

                if indent < *indents.last().unwrap_or(&0) {
                    while indent < *indents.last().unwrap() {
                        tokens.push(Token::dedent(l));
                        indents.pop();
                    }

                } else if indent > *indents.last().unwrap_or(&0) {
                    indents.push(indent);
                    tokens.push(Token::indent(l));
                }
            }


            if start {
                if next.is_numeric() {
                    while let Some(&(_, next)) = iter.peek() {
                        if !next.is_numeric() {
                            break;
                        }
                        iter.next();
                    }
                    let to = iter.peek().map(|v| v.0).unwrap_or(line.len());
                    tokens.push(Token::number(l, (from, to), &line[from..to]));

                } else if next.is_alphabetic() {
                    while let Some(&(_, next)) = iter.peek() {
                        if !next.is_alphanumeric() {
                            break;
                        }
                        iter.next();
                    }
                    let to = iter.peek().map(|v| v.0).unwrap_or(line.len());
                    tokens.push(Token::word(l, (from, to), &line[from..to]));

                } else if let Some(delim) = matches(&mut iter.clone(), &string_d) {
                    let mut last = next;

                    while let Some(&(to, next)) = iter.peek() {
                        if last != '\\' && matches(&mut iter.clone(), &[&delim]) != None {
                            tokens.push(Token::string(l, (from+1, to), &line[from+1..to]));
                            iter.nth(delim.len()-1);
                            break;
                        }

                        last = next;
                        iter.next();
                    }

                } else if next.is_whitespace() {

                } else {
                    tokens.push(Token::symbol(l, (from, from+1), &line[from..from+1]));
                }
            }
        }


        if tokens.last().map(|t| t.ttype != Type::EOL).unwrap_or(false) {
            tokens.push(Token::newline(l));
        }

        println!("{:5}| {}", l+1, line);
    }

    for _ in indents {
        tokens.push(Token::dedent(src.lines.len()));
    }
    tokens.push(Token::eof(src.lines.len()));
    src.tokens = Some(tokens);
}



fn main () {
    let mut s = Source::new("../examples/expressions.pi", Some("//!"));

    tokenize(&mut s);

    for token in s.tokens.unwrap() {
        println!("{:#?}: {:?} ", token.ttype, token.lexeme);
    }
}
