use tokenizer::token::{Type, Token, TokenIterator};
use tokenizer::token::PartialToken::*;
use tokenizer::token::Type::*;
use tokenizer::tokenizer::*;
use rule::*;
use alias::*;

use super::error::*;

#[derive(Debug)]
pub struct Template<'t, 's: 't> {
    pub source: &'t Source<'s>,
    pub rules: Option<Vec<Rule<'t, 's>>>,
}

impl<'t, 's: 't> Template<'t, 's> {
    pub fn new (source: &'t Source<'s>) -> Self {
        Self { source, rules:None }
    }

    pub fn find_rule (&self, name: &str) -> Option<&Rule> {
        for rule in self.rules.as_ref().unwrap().iter() {
            if rule.name.lexeme.unwrap() == name {
                return Some(rule)
            }
        }

        None
    }

    pub fn parse (&mut self) -> Outcome<()> {
        let mut error_flag = false;
        let tokens         = self.source.tokens.as_ref().unwrap();

        let mut iter = TokenIterator::new(tokens);
        let mut rules = Vec::new();

        while !iter.match_with(&[Type(EOF)]) {
            if iter.check(&[Type(Word), Pair(Symbol, ":"), Type(EOL)]) || iter.check(&[Type(Word), Pair(Symbol, "!"), Type(EOL)]) {
                let outcome = self.parse_rule(&mut iter);

                if outcome.is_error() {                    
                    outcome.dump(&self.source.lines);
                    error_flag = true;

                    break
                }
                
                rules.push(outcome.unwrap());
                println!()
            }
        }

        self.rules = Some(rules);
        
        if error_flag {
            Outcome::new((), Some(vec!(Response::Error(None, "aborting due to previous errors".to_owned()))))
        } else {
            Outcome::new((), None)
        }
    }

    fn parse_rule (&self, mut iter: &mut TokenIterator<'t, 's>) -> Outcome<Rule<'t, 's>> {
        let mut error_flag = false;
        let mut response   = Vec::new();

        let name = iter.next().unwrap();
        let is_matching = iter.check(&[Pair(Symbol, ":")]);
        iter.eat(2);

        let mut segments: Vec<Segment<'t, 's>> = Vec::new();
        let mut variants: Vec<Variant<'t, 's>> = Vec::new();

        if !iter.match_with(&[Type(Indent)]) {
            response.push(Response::Error(Some(Pos {line: name.line, slice: name.slice }), format!("empty rule: {}", name.lexeme.unwrap())))
        } else {
            println!(":: parsing rule {:?}", name.lexeme);

            while !iter.match_with(&[Type(Dedent)]) {
                if iter.check(&[Type(Word), Lexeme(":"), Lexeme("=")]) {
                    let outcome = self.parse_variant(&mut iter, name.lexeme.unwrap());

                    if outcome.is_error() {
                        outcome.dump(&self.source.lines);
                        error_flag = true;

                        response.push(Response::Error(None, "failed variant".to_owned()));

                        break
                    }

                    variants.push(outcome.unwrap())

                } else if iter.check(&[Lexeme("["), Type(Word), Lexeme("]"), Type(EOL)]) {
                    let outcome = self.parse_segment(&mut iter, name.lexeme.unwrap(), None);
                    
                    if outcome.is_error() {
                        outcome.dump(&self.source.lines);
                        error_flag = true;

                        response.push(Response::Error(None, "failed segment".to_owned()));

                        break
                    }

                    segments.push(outcome.unwrap())
                }
            }
        }
        
        if response.len() > 0 || error_flag {            
            Outcome::new(Rule::new(name, is_matching, variants, segments), Some(response))
        } else {
            Outcome::new(Rule::new(name, is_matching, variants, segments), None)
        }
    }

    fn parse_variant (
        &self,
        mut iter: &mut TokenIterator<'t, 's>,
        rule: &'t str,
    ) -> Outcome<Variant<'t, 's>> {
        
        let mut response = Vec::new();

        let name = iter.get(0).unwrap();
        iter.eat(3);

        println!("   := parsing variant {:?}", name.lexeme);

        let mut segments: Vec<Segment<'t, 's>> = Vec::new();
        let mut tokens: Vec<&'t Token<'s>> = Vec::new();
        let mut aliases: Vec<Alias<'t, 's>> = Vec::new();

        if iter.match_with(&[Type(EOL)]) {
            response.push(Response::Error(Some(Pos {line: name.line, slice: name.slice }), format!("variant has no pattern: {}", name.lexeme.unwrap())))
        }

        while !iter.match_with(&[Type(EOL)]) {
            if iter.check(&[Type(Word), Pair(Symbol, ":")]) {
                let alias_name = iter.next().unwrap();
                iter.next();

                if !iter.check(&[Type(Word)]) {
                    response.push(Response::Error(Some(Pos {line: name.line, slice: name.slice }), format!("can't alias non-word: {}", name.lexeme.unwrap())));
                    break
                }

                let elem = iter.get(0).unwrap().lexeme.unwrap();

                if elem.to_string().to_uppercase() == elem.to_string() && Type::from_str(elem) == None {
                    response.push(Response::Error(Some(Pos {line: name.line, slice: name.slice }), format!("undefined type: {}", name.lexeme.unwrap())));
                    break
                }

                //println!("   {}:{}", alias_name.lexeme.unwrap(), elem);

                aliases.push(Alias::new(alias_name, tokens.len()));
            }
            
            match iter.next() {
                Some(ref t) => tokens.push(t),
                None        => {
                    response.push(Response::Error(Some(Pos {line: name.line, slice: name.slice }), format!("missing token: {}", name.lexeme.unwrap())));
                    break
                }
            }
        }

        if iter.match_with(&[Type(Indent)]) {
            loop {
                if iter.check(&[Pair(Symbol, "["), Type(Word), Pair(Symbol, "]"), Type(EOL)]) {
                    print!("   ");
                    let outcome = self.parse_segment(&mut iter, &rule, Some(name.lexeme.unwrap()));
                    
                    if outcome.is_error() {
                        outcome.dump(&self.source.lines);
                    }

                    segments.push(outcome.unwrap());
                }

                if iter.match_with(&[Type(Dedent)]) {
                    break
                }
            }
        }

        if response.len() > 0 {            
            Outcome::new(Variant::new(name, &rule, tokens, segments, aliases), Some(response))
        } else {
            Outcome::new(Variant::new(name, &rule, tokens, segments, aliases), None)
        }
    }

    fn parse_segment (
        &self,
        iter: &mut TokenIterator<'t, 's>,
        rule: &'t str,
        variant: Option<&'t str>,
    ) -> Outcome<Segment<'t, 's>> {
        
        let mut response = Vec::new();

        let name = iter.get(1).unwrap();
        iter.eat(3);

        println!("   [] parsing segment {:?}", name.lexeme);

        if !iter.match_with(&[Type(EOL), Type(Indent)]) {
            response.push(Response::Error(Some(Pos {line: name.line, slice: name.slice }), format!("empty segment: {}", name.lexeme.unwrap())))
        }

        let mut tokens: Vec<&'t Token<'s>> = Vec::new();

        let mut dent = 1;

        while dent > 0 {
            tokens.push(iter.next().unwrap());

            if iter.check(&[Type(Indent)]) {
                dent += 1;

            } else if iter.check(&[Type(Dedent)]) {
                dent -= 1;
            }
        }

        iter.next();

        if response.len() > 0 {
            Outcome::new(Segment::new(name, rule, variant, tokens), Some(response))
        } else {
            Outcome::new(Segment::new(name, rule, variant, tokens), None)
        }
    }
}
