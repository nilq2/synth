use tokenizer::token::{Type, Token, TokenIterator};
use tokenizer::token::Type::*;
use tokenizer::token::PartialToken::*;
use tokenizer::tokenizer::Source;
use template::*;
use alias::*;
use rule::*;
use std::sync::Arc;

use extras::string::{StringExtras};
use compiler::compiler::AST;

use super::error::*;

#[derive(Debug, Clone)]
pub struct Node<'u> {
    pub variant: &'u Variant<'u, 'u>,
    pub tokens: Vec<Alias<'u, 'u>>,
    pub children: Vec<Node<'u>>,
}

#[derive(Debug)]
pub struct Path<'u> {
    variant: &'u Variant<'u, 'u>,
    children: Vec<Path<'u>>,
}

#[derive(Debug)]
pub struct Unit<'u> {
    source: &'u Source<'u>,
    template: Arc<Template<'u, 'u>>,
}

fn dump_path(path: &Path) {
    print!("{}", path.variant.name.lexeme.unwrap());
    for path in path.children.iter() {
        print!(" -> ");
        dump_path(&path);
    }
}

impl<'u> Unit<'u> {
    pub fn new (source: &'u Source<'u>, template: Arc<Template<'u,'u>>) -> Self  {
        Self { source, template }
    }

    pub fn parse (&mut self) -> Outcome<AST> {
        let mut response = Vec::new();
        
        let tokens = self.source.tokens.as_ref().unwrap();

        let mut source = TokenIterator::new(tokens);
        let mut paths: Vec<Path> = Vec::new();
        let mut nodes: Vec<Node> = Vec::new();

        while source.get(0) != None {
            let mut path: Option<Path> = None;

            for rule in self.template.rules.as_ref().unwrap().iter() {
                if rule.is_matching {
                    path = self.check_rule(&mut source, &rule);
                    if let Some(_) = path {
                        break
                    }
                }
            }

            if let None = path {
                if source.get(0).unwrap() == &Type(Type::EOF) {
                    break
                }

                let start = source.get(0).unwrap().slice.0;
                let end   = source.get(0).unwrap().slice.1;
                
                response.push(Response::Error(Some(Full(&source.get(0).unwrap())), format!("no path matches at token: {}", source.get(0).unwrap().lexeme.unwrap())));
                break
            }

            println!("   ++ matched {}", path.as_ref().unwrap().variant.name.lexeme.unwrap());
            paths.push(path.unwrap());
        }

        print!("\n:: PATHS ::\n   ");

        for path in paths.iter() {
            dump_path(&path);
            print!("\n   ");
        }

        println!("\n:: AST ::");
        source.current = 0;

        for path in paths.iter() {
            nodes.push(self.parse_path(&mut source, path));
        }
        
        if response.len() > 0 {
            Outcome::new(AST::new (self.source, nodes), Some(response))
        } else {
            Outcome::new(AST::new (self.source, nodes), None)
        }
    }

    fn check_rule (
        &self, mut source: &mut TokenIterator, rule: &'u Rule
    ) -> Option<Path> {
        for variant in &rule.variants {
            if let Some(path) = self.check_variant(&mut source, variant) {
                return Some(path)
            }
        }

        None
    }

    fn check_variant (
        &self, mut source: &mut TokenIterator, variant: &'u Variant
    ) -> Option<Path> {
        //println!("?? checking {}", &variant.name.lexeme.unwrap());

        let aliases = &variant.aliases;
        let tokens = &variant.tokens;

        let mut children: Vec<Path> = Vec::new();

        let reset = source.current;
        let mut alias = 0;
        let mut index = 0;

        while index < tokens.len() {
            //println!("{:?}", &tokens[index]);

            if tokens[index].lexeme.unwrap() == "\\" {
                index += 1;
            }

            if alias < aliases.len() && index == aliases[alias].token {
                if tokens[index].lexeme.unwrap().is_uppercase() {
                    if source.get(0).unwrap()
                    != &Type(Type::from_str(tokens[index].lexeme.unwrap()).unwrap()) {
                        //println!("   -- didn't match type {}", tokens[index].lexeme.unwrap());
                        source.current = reset;
                        return None

                    } else {
                        index += 1;
                        source.next();
                    }

                } else {
                    let rules = self.template.rules.as_ref();
                    let recurse = rules
                        .unwrap()
                        .iter()
                        .find(|&r| r.name.lexeme == tokens[index].lexeme)
                        .unwrap();

                    //println!("   @@ {:?}", recurse.name);

                    //source.eat(index);
                    let rule = self.check_rule(&mut source, recurse);

                    if let Some(r) = rule {
                        index += 1;
                        children.push(r);

                    } else {
                        //println!("   -- didn't match rule {}", recurse.name.lexeme.unwrap());
                        source.current = reset;
                        return None
                    }
                }
                alias += 1;

            } else {
                /*
                println!("   !! {:?} {}", tokens[index].lexeme, source.get(0).unwrap().lexeme.unwrap_or(
                   &source.get(0).unwrap().token_type.to_str()
                ));
                */
                if tokens[index].token_type == Word && tokens[index].lexeme.unwrap().is_uppercase() {
                    if source.get(0).unwrap()
                    != &Type(Type::from_str(tokens[index].lexeme.unwrap()).unwrap()) {
                        //println!("   -- didn't match type {}", tokens[index].lexeme.unwrap());
                        source.current = reset;
                        return None
                    }

                } else if source.get(0).unwrap() != &Lexeme(tokens[index].lexeme.unwrap()) {
                    /*
                    println!("   -- didn't match token {} at {}",
                        tokens[index].lexeme.unwrap(),
                        &source.get(0).unwrap().lexeme.unwrap_or(&source.get(0).unwrap().token_type.to_str())
                    );
                    */
                    source.current = reset;
                    return None
                }

                index += 1;
                source.next();
            }
        }

        //source.eat(index);
        Some(Path{ variant: variant, children: children })
    }

    fn parse_path (&self, mut source: &mut TokenIterator, path: &Path<'u>) -> Node {
        let variant = &path.variant;

        let mut children: Vec<Node> = Vec::new();
        let mut tokens: Vec<Alias> = Vec::new();

        let mut alias = 0;
        let mut index = 0;

        let mut pchildren = path.children.iter();

        while index < variant.tokens.len() {
            if variant.tokens[index].lexeme.unwrap() == "\\" {
                index += 1;
            }

            if alias < variant.aliases.len() && index == variant.aliases[alias].token {
                if variant.tokens[index].lexeme.unwrap().is_uppercase() {
                    tokens.push(Alias::new(variant.aliases[alias].name, source.current));
                    source.next();

                } else {
                    children.push(self.parse_path(&mut source, pchildren.next().unwrap()));
                }

                alias += 1;

            } else {
                source.next();
            }

            index += 1;
        }

        Node { variant, tokens, children }
    }
}
