use std::rc::Rc;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


#[derive(Debug)]
pub enum Type {
    Number,
    String,
    Word,
    Symbol,

    Indent,
    Dedent,
    Newline,

    EOF,
}



#[derive(Debug)]
pub struct Token {
    pub ttype: Type,
    pub line: usize,
    pub slice: (usize, usize),
    pub lexeme: Option<String>,
}

impl Token {
    fn new (
        ttype: Type,
        line: usize,
        slice: (usize, usize),
        lexeme: Option<String>
    ) -> Token {
        Token { ttype, line, slice, lexeme }
    }

    fn number (line: usize, slice: (usize, usize), lexeme: String) -> Token {
        Token::new(Type::Number, line+1, slice, Some(lexeme))
    }

    fn string (line: usize, slice: (usize, usize), lexeme: String) -> Token {
        Token::new(Type::String, line+1, slice, Some(lexeme))
    }

    fn word (line: usize, slice: (usize, usize), lexeme: String) -> Token {
        Token::new(Type::Word, line+1, slice, Some(lexeme))
    }

    fn symbol (line: usize, slice: (usize, usize), lexeme: String) -> Token {
        Token::new(Type::Symbol, line+1, slice, Some(lexeme))
    }

    fn indent (line: usize) -> Token {
        Token::new(Type::Indent, line+1, (0,0), None)
    }

    fn dedent (line: usize) -> Token {
        Token::new(Type::Dedent, line+1, (0,0), None)
    }

    fn newline (line: usize) -> Token {
        Token::new(Type::Newline, line+1, (0,0), None)
    }

    fn eof (line: usize) -> Token {
        Token::new(Type::EOF, line+1, (0,0), None)
    }

}



#[derive(Debug)]
pub struct Alias {
    name: Token,
    id: u32,
}


#[derive(Debug)]
pub struct Segment {
    name: Token,
    variant: Rc<Variant>,
    tokens: Vec<Rc<Token>>,
}


#[derive(Debug)]
pub struct Variant {
    name: Token,
    rule: Rule,

    tokens: Vec<Rc<Token>>,
    segments: Vec<Segment>,
    alias: Vec<Alias>,

}


#[derive(Debug)]
pub struct Rule {
    name: Token,
    variants: Vec<Variant>,
}



#[derive(Debug)]
pub struct Source {
    path: String,
    lines: Vec<String>,
    tokens: Option<Vec<Token>>,
    directives: Vec<(String, String)>, // control directives
}

impl Source {
    pub fn new (path: &str, ctrl_char: Option<&str>) -> Source {
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



pub fn tokenize (src: &mut Source) {
    let mut indents = vec![0];
    let mut tokens = Vec::new();

    for (l, line) in src.lines.iter().enumerate() {
        let mut indent = 0;
        let mut start = false;
        let mut iter = line.chars().enumerate().peekable();

        let mut sdelim = Vec::new();

        if let Some(string_delim) = src.get_directive("string") {
            for bit in string_delim.split_whitespace() {
                sdelim.push(bit.chars().next().unwrap());
            }
        }


        while let Some((from, next)) = iter.next() {
            if !start && next.is_whitespace() {
                indent += 1;

            } else if !start {
                start = true;

                if indent < *indents.last().unwrap() {
                    while indent < *indents.last().unwrap() {
                        tokens.push(Token::dedent(l));
                        indents.pop();
                    }

                } else if indent > *indents.last().unwrap() {
                    indents.push(indent);
                    tokens.push(Token::indent(l));
                }
            }


            if start {
                if next.is_numeric() {
                    while let Some(&(to, next)) = iter.peek() {
                        if !next.is_numeric() {
                            tokens.push(Token::number(l, (from, to), String::from(&line[from..to])));
                            break;
                        }

                        iter.next();
                    }

                } else if next.is_alphabetic() {
                    while let Some(&(to, next)) = iter.peek() {
                        if !next.is_alphanumeric() {
                            tokens.push(Token::word(l, (from, to), String::from(&line[from..to])));
                            break;
                        }

                        iter.next();
                    }

                } else if let Some(delim) = sdelim.iter().find(|&&c| c == next) {
                        let mut last = next;

                        while let Some(&(to, next)) = iter.peek() {
                            if next == *delim && last != '\\' {
                                tokens.push(Token::string(l, (from+1, to), String::from(&line[from+1..to])));
                                break;
                            }

                            last = next;
                            iter.next();
                        }

                } else if next == '\n' {
                    tokens.push(Token::newline(l));

                } else if next.is_whitespace() {

                } else {
                    tokens.push(Token::symbol(l, (from, from+1), String::from(&line[from..from+1])));
                }
            }
        }

        println!("{:5}| {}", l+1, line);
    }
    tokens.push(Token::eof(src.lines.len()));
    src.tokens = Some(tokens);
}



fn main () {
    let mut s = Source::new("../examples/expressions.pi", Some("//!"));

    tokenize(&mut s);

    for token in s.tokens.unwrap() {
        print!("{:?} ", token.lexeme);
    }
}
