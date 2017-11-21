use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::{Peekable, Enumerate};
use std::str::Chars;
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

    fn number (line: usize, slice: (usize, usize), lexeme: &str) -> Token {
        Token::new(Type::Number, line+1, slice, Some(Rc::new(lexeme.to_string())))
    }

    fn string (line: usize, slice: (usize, usize), lexeme: &str) -> Token {
        Token::new(Type::String, line+1, slice, Some(Rc::new(lexeme.to_string())))
    }

    fn word (line: usize, slice: (usize, usize), lexeme: &str) -> Token {
        Token::new(Type::Word, line+1, slice, Some(Rc::new(lexeme.to_string())))
    }

    fn symbol (line: usize, slice: (usize, usize), lexeme: &str) -> Token {
        Token::new(Type::Symbol, line+1, slice, Some(Rc::new(lexeme.to_string())))
    }

    fn indent (line: usize) -> Token {
        Token::new(Type::Indent, line+1, (0,0), None)
    }

    fn dedent (line: usize) -> Token {
        Token::new(Type::Dedent, line+1, (0,0), None)
    }

    fn newline (line: usize) -> Token {
        Token::new(Type::EOL, line+1, (0,0), None)
    }

    fn eof (line: usize) -> Token {
        Token::new(Type::EOF, line+1, (0,0), None)
    }
}

#[derive(Debug)]
pub struct Source {
    path:       Rc<String>,
    lines:      Vec<Rc<String>>,
    tokens:     Option<Vec<Token>>,
    directives: Vec<(Rc<String>, Rc<String>)>,
}

impl Source {
    pub fn new (path: &str, ctrl_char: Option<&str>) -> Source {
        let f: File = match File::open(path) {
            Ok(v) => v,
            Err(_) => panic!("No such file: {}", &path),
        };

        let mut lines: Vec<Rc<String>> = Vec::new();
        let mut directives: Vec<(Rc<String>, Rc<String>)> = Vec::new();

        let file = BufReader::new(&f);

        if let Some(ctrl) = ctrl_char {
            for line in file.lines() {
                let line = line.unwrap();

                if line.starts_with(ctrl) {
                    directives.push ((
                        Rc::new(line[ctrl.len() .. line.find(" ").unwrap()].to_string()),
                        Rc::new(line[line.find(" ").unwrap()+1..].to_string()),
                    ));
                    lines.push(Rc::new("".to_string()));

                } else {
                    lines.push(Rc::new(line));
                }
            }

        } else {
            for line in file.lines() {
                let line = line.unwrap();
                lines.push(Rc::new(line));
            }
        }

        Source {
            path:       Rc::new(path.to_string()),
            lines:      lines,
            tokens:     None,
            directives: directives,
        }
    }

    pub fn get_directive (&self, name: &str) -> Option<Rc<String>> {
        match self.directives.iter().find(|n| &**n.0 == name).map(|n| &n.1) {
            Some(n) => Some(n.to_owned()),
            None    => None,
        }
    }
}

pub fn matches (first: char, iter: &mut Peekable<Enumerate<Chars>>, delims: &Vec<Rc<String>>) -> Option<Rc<String>> {
    let mut matched = true;

    for delim in delims {
        println!("{:#?}", first);
        let mut chars = delim.chars();

        if chars.next().unwrap() != first {
            matched = false;
        }

        for ch in chars {
            if ch != iter.peek().unwrap_or(&(0,' ')).1 {
                matched = false;
            }
            iter.next();
        }

        if matched {
            println!("maatch");
            return Some((*delim).clone());
        } else {
            matched = true;
        }
    }

    None
}

pub fn tokenize(src: &mut Source) {
    let mut indents = Vec::new();
    let mut tokens  = Vec::new();

    for (l, line) in src.lines.iter().enumerate() {
        let mut indent = 0;
        let mut start = false;
        let mut comment = false;
        let mut iter = line.chars().enumerate().peekable();

        let mut string_d:  Vec<Rc<String>> = Vec::new();
        let mut comment_d: Vec<Rc<String>> = Vec::new();

        if let Some(string_delim) = src.get_directive("string") {
            for delim in string_delim.split_whitespace() {
                string_d.push(Rc::new(delim.to_string()));
            }
        }

        if let Some(comment_delim) = src.get_directive("comment") {
            for delim in comment_delim.split_whitespace() {
                comment_d.push(Rc::new(delim.to_string()));
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

                } else if let Some(delim) = matches(next, &mut iter.clone(), &string_d) {
                    let mut last = next;

                    println!("MATCH");

                    while let Some(&(to, next)) = iter.peek() {
                        if last != '\\' && matches(next, &mut iter.clone(), &vec![delim.clone()]) != None {
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


        if tokens.last().map(|t| t.token_type != Type::EOL).unwrap_or(false) {
            tokens.push(Token::newline(l));
        }

        println!("{:5}| {}", l+1, line);
    }

    for _ in indents {
        tokens.push(Token::dedent(src.lines.len()));
    }

    tokens.push(Token::eof(src.lines.len()));
    src.tokens = Some(tokens)
}

fn main() {
    let mut s = Source::new("../examples/expressions.pi", Some("//!"));

    tokenize(&mut s);

    for token in s.tokens.unwrap() {
        println!("{:#?}: {:?} ", token.token_type, token.lexeme);
    }
}
