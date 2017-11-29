use colored::*;

use super::tokenizer::token::PartialToken;

#[derive(Debug, Clone)]
pub enum Response<'r> {
    Note(Option<PartialToken<'r>>,    String),
    Warning(Option<PartialToken<'r>>, String),
    Error(Option<PartialToken<'r>>,   String),
}

#[derive(Debug, Clone)]
pub struct Outcome<'o, T> {
    pub value: T,
    pub response: Option<Vec<Response<'o>>>,
}

#[allow(dead_code)]
impl<'o, T: Clone> Outcome<'o, T> {
    pub fn new(value: T, response: Option<Vec<Response<'o>>>) -> Self {
        Self {
            value,
            response,
        }
    }

    pub fn new_pos(value: T, response: Option<Vec<Response<'o>>>) -> Self {
        Self {
            value,
            response,
        }
    }
    
    pub fn is_error(&self) -> bool {
        if self.response.is_some() {
            for i in self.response.clone().unwrap().iter() {
                match *i {
                    Response::Error(_, _) => return true,
                    _                     => (),
                }
            }
        }

        false
    }

    pub fn unwrap(&self) -> T {
        self.value.clone()
    }
    
    pub fn error(message: &str, line: usize, slice: (usize, usize), lines: &Vec<&str>) {
        println!("{}: {}", "error".red().bold(), message.white().bold());

        let ln = format!("{} |", line).blue().bold();
        println!("{}{}", ln, lines.get(line - 1).unwrap());

        for _ in 0 .. ln.len() + slice.0 {
            print!(" ")
        }

        for _ in 0 .. slice.1 - slice.0 {
            print!("{}", "^".red().bold())
        }
        
        println!()
    }
    
    pub fn warning(message: &str, line: usize, slice: (usize, usize), lines: &Vec<&str>) {
        println!("{}: {}", "warning".red().bold(), message.white().bold());

        let ln = format!("{} |", line).blue().bold();
        println!("{}{}", ln, lines.get(line - 1).unwrap());

        for _ in 0 .. ln.len() + slice.0 {
            print!(" ")
        }

        for _ in 0 .. slice.1 - slice.0 {
            print!("{}", "^".yellow().bold())
        }
        
        println!()
    }

    pub fn note(message: &str, line: usize, slice: (usize, usize), lines: &Vec<&str>) {
        println!("{}: {}", "note".white().bold(), message.white().bold());

        let ln = format!("{} |", line).blue().bold();
        println!("{}{}", ln, lines.get(line - 1).unwrap());

        for _ in 0 .. ln.len() + slice.0 {
            print!(" ")
        }

        for _ in 0 .. slice.1 - slice.0 {
            print!("{}", "^".white().bold())
        }
        
        println!()
    }

    pub fn dump(&self, lines: &Vec<&str>) -> &Self {
        if self.response.is_some() {
            for value in self.response.as_ref().unwrap().iter() {
                match *value {
                    Response::Error(ref v, ref message) => match *v {
                        Some(ref token) => match *token {
                            PartialToken::Ref(ref token)            => Self::error(message, token.line, token.slice, lines),
                            PartialToken::Pos {ref line, ref slice} => Self::error(message, *line, *slice, lines),
                            _ => (),
                        },

                        None => (),
                    },
                    
                    Response::Warning(ref v, ref message) => match *v {
                        Some(ref token) => match *token {
                            PartialToken::Ref(ref token)            => Self::warning(message, token.line, token.slice, lines),
                            PartialToken::Pos {ref line, ref slice} => Self::warning(message, *line, *slice, lines),
                            _ => (),
                        },
                        
                        None => (),
                    },

                    Response::Note(ref v, ref message) => match *v {
                        Some(ref token) => match *token {
                            PartialToken::Ref(ref token)            => Self::note(message, token.line, token.slice, lines),
                            PartialToken::Pos {ref line, ref slice} => Self::note(message, *line, *slice, lines),
                            _ => (),
                        },
                        
                        None => (),
                    },
                }
            }
        }

        &self
    }
}
