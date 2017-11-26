use tokenizer::token::{Type, Token, TokenIterator};
use tokenizer::token::Type::*;
use tokenizer::token::PartialToken::*;
use tokenizer::tokenizer::Source;
use template::*;
use rule::*;
use alias::*;
use std::sync::Arc;


#[derive(Debug)]
pub struct Node<'u, 't: 'u, 'ts: 't> {
    name: &'u Token<'t>,
    variant: &'u Variant<'t, 'ts>,

    tokens: Vec<&'u Token<'t>>,
    children: Vec<&'u Node<'u, 't, 'ts>>,
}

#[derive(Debug)]
pub struct Path<'u, 't: 'u, 'ts: 't> {
    variant: &'u Variant<'t, 'ts>,
    children: Vec<Path<'u, 't, 'ts>>,
}

#[derive(Debug)]
pub struct Unit<'u, 's: 'u, 't: 'u, 'ts: 't> {
    source: &'u Source<'s>,
    template: Arc<Template<'t, 'ts>>,
    ast: Option<Vec<Node<'u, 't, 'ts>>>,
}


impl<'u, 's: 'u, 't: 'u, 'ts: 't> Unit<'u, 's, 't, 'ts> {
    pub fn new (source: &'u Source<'s>, template: Arc<Template<'t, 'ts>>) -> Self  {
        Self { source, template, ast:None }
    }

    pub fn parse (&mut self) {
        let tokens = self.source.tokens.as_ref().unwrap();

        let mut source = TokenIterator::new(tokens);
        let mut paths: Vec<Path> = Vec::new();

        //let mut ast = Vec::new();

        while source.get(0) != None {
            let mut path: Option<Path> = None;

            for rule in self.template.rules.as_ref().unwrap().iter() {
                path = self.check_rule(&mut source, &rule);
                println!("");
                if let Some(ref p) = path {
                    break
                }
            }

            if let None = path {
                panic!("no path matches at token {:?}", source.get(0));
            }

            paths.push(path.unwrap());
        }
    }

    fn check_rule (
        &self, mut source: &mut TokenIterator, rule: &'u Rule<'t, 'ts>
    ) -> Option<Path<'u, 't, 'ts>> {
        for variant in &rule.variants {
            if let Some(path) = self.check_variant(&mut source, variant) {
                return Some(path)
            }
        }

        None
    }

    fn check_variant (
        &self, mut source: &mut TokenIterator, variant: &'u Variant<'t, 'ts>
    ) -> Option<Path<'u, 't, 'ts>> {
        let aliases = &variant.aliases;
        let tokens = &variant.tokens;

        let mut children: Vec<Path<'u, 't, 'ts>> = Vec::new();

        let reset = source.current;
        let mut alias = 0;
        let mut index = 0;

        while index < tokens.len() {
            println!("{:?}", &tokens[index]);

            if tokens[index].lexeme.unwrap() == "\\" {
                index += 1;
            }

            if alias < aliases.len() && index == aliases[alias].token {
                if tokens[index].lexeme.unwrap().to_uppercase() == tokens[index].lexeme.unwrap() {
                    if source.get(index).unwrap()
                    != &Type(Type::from_str(tokens[index].lexeme.unwrap()).unwrap()) {
                        source.current = reset;
                        return None

                    } else {
                        index += 1;
                    }

                } else {
                    let rules = self.template.rules.as_ref();
                    let recurse = rules
                        .unwrap()
                        .iter()
                        .find(|&r| r.name.lexeme == tokens[index].lexeme)
                        .unwrap();

                    println!("{:?}", recurse);

                    let rule = self.check_rule(&mut source, recurse);

                    if let Some(r) = rule {
                        children.push(r);

                    } else {
                        source.current = reset;
                        return None
                    }
                }
            }
        }

        Some(Path{ variant: variant, children: children })
    }

    fn parse_path (&self, mut source: &mut TokenIterator, path: &Path<'u, 't, 's>) {

    }
}

