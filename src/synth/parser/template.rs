use tokenizer::token::{Type, Token, TokenIterator};
use tokenizer::token::PartialToken::*;
use tokenizer::token::Type::*;
use tokenizer::tokenizer::*;
use rule::*;
use alias::*;

use std::string;

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
        let mut response = Vec::new();
        let tokens = self.source.tokens.as_ref().unwrap();

        let mut iter = TokenIterator::new(tokens);
        let mut rules = Vec::new();

        while !iter.match_with(&[Type(EOF)]) {
            if iter.check(&[Type(Word), Pair(Symbol, ":"), Type(EOL)]) || iter.check(&[Type(Word), Pair(Symbol, "!"), Type(EOL)]) {
                rules.push(self.parse_rule(&mut iter).dump(&self.source.lines).unwrap());
                println!()
            }
        }

        self.rules = Some(rules);

        if response.len() > 0 {
            Outcome::new((), Some(response))
        } else {
            Outcome::new((), None)
        }
    }

    fn parse_rule (&self, mut iter: &mut TokenIterator<'t, 's>) -> Outcome<Rule<'t, 's>> {
        let mut response = Vec::new();
        
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
                    variants.push(self.parse_variant(&mut iter, name.lexeme.unwrap()));

                } else if iter.check(&[Lexeme("["), Type(Word), Lexeme("]"), Type(EOL)]) {
                    segments.push(self.parse_segment(&mut iter, name.lexeme.unwrap(), None));
                }
            }
        }

        if response.len() > 0 {
            Outcome::new(Rule::new(name, is_matching, variants, segments), Some(response))
        } else {
            Outcome::new(Rule::new(name, is_matching, variants, segments), Some(response))
        }
    }

    fn parse_variant (
        &self,
        mut iter: &mut TokenIterator<'t, 's>,
        rule: &'t str,
    ) -> Variant<'t, 's> {

        let name = iter.get(0).unwrap();
        iter.eat(3);

        println!("   := parsing variant {:?}", name.lexeme);

        let mut segments: Vec<Segment<'t, 's>> = Vec::new();
        let mut tokens: Vec<&'t Token<'s>> = Vec::new();
        let mut aliases: Vec<Alias<'t, 's>> = Vec::new();

        if iter.match_with(&[Type(EOL)]) {
            panic!("variant has no pattern");
        }

        while !iter.match_with(&[Type(EOL)]) {
            if iter.check(&[Type(Word), Pair(Symbol, ":")]) {
                let alias_name = iter.next().unwrap();
                iter.next();

                if !iter.check(&[Type(Word)]) {
                    panic!("can't alias non-word");
                }

                let elem = iter.get(0).unwrap().lexeme.unwrap();

                if elem.to_string().to_uppercase() == elem.to_string() && Type::from_str(elem) == None {
                    panic!("undefined type");
                }

                //println!("   {}:{}", alias_name.lexeme.unwrap(), elem);

                aliases.push(Alias::new(alias_name, tokens.len()));
            }

            tokens.push(iter.next().unwrap());
        }

        if iter.match_with(&[Type(Indent)]) {
            loop {
                if iter.check(&[Pair(Symbol, "["), Type(Word), Pair(Symbol, "]"), Type(EOL)]) {
                    print!("   ");
                    segments.push(self.parse_segment(&mut iter, &rule, Some(name.lexeme.unwrap())));
                }

                if iter.match_with(&[Type(Dedent)]) {
                    break
                }
            }
        }

        Variant::new ( name, &rule, tokens, segments, aliases )
    }

    fn parse_segment (
        &self,
        iter: &mut TokenIterator<'t, 's>,
        rule: &'t str,
        variant: Option<&'t str>,
    ) -> Segment<'t, 's> {

        let name = iter.get(1).unwrap();
        iter.eat(3);

        println!("   [] parsing segment {:?}", name.lexeme);

        if !iter.match_with(&[Type(EOL), Type(Indent)]) {
            panic!("empty segment");
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

        Segment::new ( name, rule, variant, tokens )
    }
}
