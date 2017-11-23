use tokenizer::token::*;
use tokenizer::tokenizer::*;
use std::sync::Arc;


pub struct Alias<'t> {
    name: &'t Token<'t>,
    token: usize,
}

pub struct Rule<'t> {
    name: &'t Token<'t>,
    variants: Vec<Variant<'t>>,
    segments: Vec<Segment<'t>>,
}

pub struct Variant<'t> {
    name: &'t Token<'t>,
    rule: &'t Rule<'t>,

    tokens: Vec<&'t Token<'t>>,
    segments: Vec<Segment<'t>>,
    aliases: Vec<Alias<'t>>,
}

pub struct Segment<'t> {
    name: &'t Token<'t>,

    variant: &'t Variant<'t>,
    tokens: Vec<&'t Token<'t>>,
}

pub struct Path<'t> {
    variant: Variant<'t>,
    children: Vec<Path<'t>>,
}

pub struct Template<'t> {
    pub source: Source<'t>,
    pub rules: Option<Vec<Rule<'t>>>,
}


impl<'t> Template<'t> {
    pub fn new (source: Source<'t>) -> Template<'t> {
        Template { source, rules:None }
    }

    pub fn parse (&mut self) {
        let tokens = self.source.tokens.as_ref();

        let mut iter = TokenIterator::new(tokens.unwrap());
    }

    fn parse_rule () {}
    fn parse_variant () {}
    fn parse_segment () {}
}

