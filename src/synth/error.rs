use colored::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Position {
    pub position: (usize, usize),
    pub span:     usize,
}

impl Position {
    pub fn new(position: (usize, usize), span: usize) -> Self {
        Self {
            position,
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Response<'r> {
    Note(&'r str,    Position),
    Warning(&'r str, Position),
    Error(&'r str,   Position),
}

#[derive(Debug, Clone)]
pub struct Outcome<'o, T> {
    pub value: T,
    pub response: Option<Vec<Response<'o>>>,
}

#[allow(dead_code)]
impl<'o, T> Outcome<'o, T> {
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
        for i in self.response.clone().unwrap().iter() {
            match *i {
                Response::Error(_, _) => return true,
                _                     => (),
            }
        }

        false
    }

    pub fn dump(&self, lines: &Vec<String>) -> &Self {
        if self.response.is_some() {
            for value in self.response.clone().unwrap().iter() {
                match *value {
                    Response::Error(ref v, ref pos) => {
                        println!("{}: {}", "error".red().bold(), v.white().bold());

                        let line = format!("{} |", pos.position.0).blue().bold();
                        println!("{}{}", line, lines.get(pos.position.0 - 1).unwrap());

                        for _ in 0 .. line.len() + pos.position.1 {
                            print!(" ")
                        }

                        for _ in 0 .. pos.span {
                            print!("{}", "^".red().bold())
                        }

                        println!()
                    },
                    
                    Response::Warning(ref v, ref pos) => {
                        println!("{}: {}", "warning".yellow().bold(), v.white().bold());

                        let line = format!("{} |", pos.position.0).blue().bold();
                        println!("{}{}", line, lines.get(pos.position.0 - 1).unwrap());

                        for _ in 0 .. line.len() + pos.position.1 {
                            print!(" ")
                        }

                        for _ in 0 .. pos.span {
                            print!("{}", "^".yellow().bold())
                        }
                    },
                    
                    Response::Note(ref v, ref pos) => {
                        println!("{}: {}", "note".white().bold(), v.white().bold());

                        let line = format!("{} |", pos.position.0).blue().bold();
                        println!("{}{}", line, lines.get(pos.position.0 - 1).unwrap());

                        for _ in 0 .. line.len() + pos.position.1 {
                            print!(" ")
                        }

                        for _ in 0 .. pos.span {
                            print!("{}", "^".white().bold())
                        }
                    },
                }
            }
        }
        
        &self
    }
}
