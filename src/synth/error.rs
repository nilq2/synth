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

    pub fn dump(&self, lines: &Vec<String>) -> &Self {
        if self.response.is_some() {
            for value in self.response.clone().unwrap().iter() {
                match *value {
                    Response::Error(ref v, ref message) => {
                        match *v {
                            Some(ref token) => {
                                match *token {
                                    PartialToken::Full(ref token) => {
                                        println!("{}: {}", "error".red().bold(), message.white().bold());

                                        let line = format!("{} |", token.line).blue().bold();
                                        println!("{}{}", line, lines.get(token.line - 1).unwrap());

                                        for _ in 0 .. line.len() + token.slice.0 {
                                            print!(" ")
                                        }

                                        for _ in 0 .. token.slice.1 - token.slice.0 {
                                            print!("{}", "^".red().bold())
                                        }

                                        println!()
                                    },

                                    PartialToken::PosLexeme {ref line, ref slice, ..} => {
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
                                    },
                                    
                                    _ => (),
                                }
                            },

                            None => (),
                        }
                    },
                    
                    Response::Warning(ref v, ref message) => {
                        match *v {
                            Some(ref token) => {
                                match *token {
                                    PartialToken::Full(ref token) => {
                                        println!("{}: {}", "warning".yellow().bold(), message.white().bold());

                                        let line = format!("{} |", token.line).blue().bold();
                                        println!("{}{}", line, lines.get(token.line - 1).unwrap());

                                        for _ in 0 .. line.len() + token.slice.0 {
                                            print!(" ")
                                        }

                                        for _ in 0 .. token.slice.1 - token.slice.0 {
                                            print!("{}", "^".yellow().bold())
                                        }
                                        
                                        println!()
                                    },

                                    PartialToken::PosLexeme {ref line, ref slice, ..} => {
                                        println!("{}: {}", "warning".yellow().bold(), message.white().bold());

                                        let ln = format!("{} |", line).blue().bold();
                                        println!("{}{}", ln, lines.get(line - 1).unwrap());

                                        for _ in 0 .. ln.len() + slice.0 {
                                            print!(" ")
                                        }

                                        for _ in 0 .. slice.1 - slice.0 {
                                            print!("{}", "^".yellow().bold())
                                        }
                                        
                                        println!()
                                    },
                                    
                                    _ => (),
                                }
                            },
                            
                            None => (),
                        }
                    },

                    Response::Note(ref v, ref message) => {
                        match *v {
                            Some(ref token) => {
                                match *token {
                                    PartialToken::Full(ref token) => {
                                        println!("{}: {}", "note".white().bold(), message.white().bold());

                                        let line = format!("{} |", token.line).blue().bold();
                                        println!("{}{}", line, lines.get(token.line - 1).unwrap());

                                        for _ in 0 .. line.len() + token.slice.0 {
                                            print!(" ")
                                        }

                                        for _ in 0 .. token.slice.1 - token.slice.0 {
                                            print!("{}", "^".white().bold())
                                        }
                                        
                                        println!()
                                    },

                                    PartialToken::PosLexeme {ref line, ref slice, ..} => {
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
                                    },

                                    _ => (),
                                }
                            },
                            
                            None => (),
                        }
                    },
                }
            }
        }

        &self
    }
}
