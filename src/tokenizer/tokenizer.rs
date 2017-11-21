use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::{Peekable, Enumerate};
use std::str::Chars;
use std::rc::Rc;

use super::token::*;

#[derive(Debug)]
pub struct Source {
    pub path:       Rc<String>,
    pub lines:      Vec<Rc<String>>,
    pub tokens:     Option<Vec<Token>>,
    pub directives: Vec<(Rc<String>, Rc<String>)>,
}

impl Source {
    pub fn new (path: &str, ctrl_char: Option<&str>) -> Source {
        let f: File = match File::open(path) {
            Ok(v) => v,
            Err(_) => panic!("No such file: {}", &path),
        };

        let mut lines: Vec<Rc<String>>                    = Vec::new();
        let mut directives: Vec<(Rc<String>, Rc<String>)> = Vec::new();

        let file = BufReader::new(&f);

        if let Some(ctrl) = ctrl_char {
            for line in file.lines() {
                let line = line.unwrap();

                if line.starts_with(ctrl) {
                    directives.push ((
                        Rc::new(line[ctrl.len() .. line.find(" ").unwrap()].to_string()),
                        Rc::new(line[line.find(" ").unwrap() + 1..].to_string()),
                    ));
                    lines.push(Rc::new("".to_string()));

                } else {
                    lines.push(Rc::new(line));
                }
            }

        } else {
            for line in file.lines() {
                let line = line.unwrap();
                lines.push(Rc::new(line))
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
        // clone so the original is not exhausted if doesn't match
        let mut it = iter.clone();
        let mut chars = delim.chars();

        // workaround because first char is eaten before
        if chars.next().unwrap() != first {
            matched = false;
        }

        for ch in chars {
            if ch != it.peek().unwrap_or(&(0,' ')).1 {
                matched = false;
            }
            it.next();
        }

        if matched {
            return Some((*delim).clone())

        } else {
            matched = true
        }
    }

    None
}

pub fn tokenize(src: &mut Source) {
    let mut indents = Vec::new();
    let mut tokens  = Vec::new();

    let mut comment = 0; // start of block comment

    for (mut l, line) in src.lines.iter().enumerate() {
        l += 1; // line offset

        let mut indent = 0;
        let mut start = false; // start of content (after indent)
        let mut iter = line.chars().enumerate().peekable();

        let mut string_d:  Vec<Rc<String>> = Vec::new();
        let mut comment_d: Vec<Rc<String>> = Vec::new();

        // directive for string parsing
        if let Some(string_delim) = src.get_directive("string") {
            for delim in string_delim.split_whitespace() {
                string_d.push(Rc::new(delim.to_string()));
            }
        }

        // directive for comment parsing
        if let Some(comment_delim) = src.get_directive("comment") {
            for delim in comment_delim.split_whitespace() {
                comment_d.push(Rc::new(delim.to_string()));
            }
            if comment_d.len() > 3 {
                panic!("too many comment delimiters are defined at line {}", l);
            }
        }

        while let Some((from, next)) = iter.next() {
            if !start && next.is_whitespace() {
                indent += 1;

            } else if !start {
                start = true;

                if indent < *indents.last().unwrap_or(&0) {
                    while indent < *indents.last().unwrap_or(&0) {
                        if comment == 0 {
                            tokens.push(Token::dedent(l));
                            indents.pop();
                        }
                    }

                } else if indent > *indents.last().unwrap_or(&0) {
                    if comment == 0 {
                        indents.push(indent);
                        tokens.push(Token::indent(l));
                    }
                }
            }

            if start {
                if comment == 0 {
                    if next.is_numeric() {
                        while let Some(&(_, next)) = iter.peek() {
                            if !next.is_numeric() {
                                break;
                            }
                            iter.next();
                        }

                        let to = iter.peek().map(|v| v.0).unwrap_or(line.len());
                        tokens.push(Token::number(l, (from, to), &line[from..to]))

                    } else if next.is_alphabetic() {
                        while let Some(&(_, next)) = iter.peek() {
                            if !next.is_alphanumeric() {
                                break;
                            }
                            iter.next();
                        }

                        let to = iter.peek().map(|v| v.0).unwrap_or(line.len());
                        tokens.push(Token::word(l, (from, to), &line[from..to]))

                    } else if let Some(delim) = matches(next, &mut iter.clone(), &string_d) {
                        let mut last = next;

                        while let Some(&(to, next)) = iter.peek() {
                            if last != '\\' && matches(next, &mut iter.clone(), &vec![delim.clone()]) != None {
                                tokens.push(Token::string(l, (from+1, to), &line[from+1..to]));
                                iter.nth(delim.len()-1);
                                break;
                            }

                            last = next;
                            iter.next();
                        }

                    } else if let Some(delim) = matches(next, &mut iter.clone(), &comment_d) {
                        iter.nth(delim.len()-1);

                        match comment_d.len() {
                            1 => { // single line
                                break; // skip the rest of the line
                            },

                            2 => { // block
                                if comment_d[1] == delim {
                                    panic!("unexpected block comment terminator at line {}", l)
                                } else {
                                    comment = l // block comment
                                }
                            },

                            3 => { // block and single line
                                if comment_d[1] == delim {
                                    panic!("unexpected block comment terminator at line {}", l)
                                } else if comment_d[0] == delim {
                                    comment = l; // block comment
                                    break

                                } else {
                                    break // single line
                                }
                            },

                            _ => {}
                        }

                    } else if !next.is_whitespace()  {
                        tokens.push(Token::symbol(l, (from, from+1), &line[from..from+1]));
                    }

                } else {
                    if let Some(delim) = matches(next, &mut iter.clone(), &comment_d) {
                        iter.nth(delim.len()-1);

                        match comment_d.len() {
                            2 | 3 => if comment_d[1] == delim { comment = 0 },
                            _ => (),
                        }
                    }
                }
            }
        }

        if comment == 0 && tokens.last().map(|t| t.token_type != Type::EOL).unwrap_or(false) {
            tokens.push(Token::newline(l))
        }

        println!("{:5}| {}", l, line)
    }

    if comment != 0 {
        panic!("block comment not terminated at line {}", comment)
    }

    for _ in indents {
        tokens.push(Token::dedent(src.lines.len()))
    }

    tokens.push(Token::eof(src.lines.len()));
    src.tokens = Some(tokens)
}
