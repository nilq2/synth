use tokenizer::token::{Token, TokenIterator};
use tokenizer::token::PartialToken::*;
use tokenizer::token::Type::*;
use tokenizer::tokenizer::*;


pub struct Alias<'t, 's: 't> {
    name: &'t Token<'s>,
    token: usize,
}

pub struct Rule<'t, 's: 't> {
    name: &'t Token<'s>,
    variants: Vec<Variant<'t, 's>>,
    segments: Vec<Segment<'t, 's>>,
}

pub struct Variant<'t, 's: 't> {
    name: &'t Token<'s>,
    tokens: Vec<&'t Token<'s>>,
    segments: Vec<Segment<'t, 's>>,
    aliases: Vec<Alias<'t, 's>>,
}

pub struct Segment<'t, 's: 't> {
    name: &'t Token<'s>,
    tokens: Vec<&'t Token<'s>>,
}

pub struct Path<'t, 's: 't> {
    variant: Variant<'t, 's>,
    children: Vec<Path<'t, 's>>,
}

pub struct Template<'t, 's: 't> {
    pub source: &'t Source<'s>,
    pub rules: Option<Vec<Rule<'t, 's>>>,
}


impl<'t, 's: 't> Template<'t, 's> {
    pub fn new (source: &'t Source<'s>) -> Template<'t, 's> {
        Template { source, rules:None }
    }

    pub fn parse (&mut self) {
        let tokens: &'t Vec<Token<'s>> = &self.source.tokens.unwrap();

        let mut iter = TokenIterator::new(&tokens);

        let mut rules: Vec<Rule<'t, 's>> = Vec::new();

        while iter.get(0) != None {
            if iter.check(&[Type(Word), Lexeme(":"), Type(EOL)]) {
                rules.push(self.parse_rule(&mut iter));
            }
            iter.next();
        }

        self.rules = Some(rules);
    }

    fn parse_rule (&mut self, iter: &'s mut TokenIterator) -> Rule<'t, 's> {
        let name = iter.get(0).unwrap();
        iter.eat(3);

        let mut segments: Vec<Segment<'t, 's>> = Vec::new();
        let mut variants: Vec<Variant<'t, 's>> = Vec::new();

        if !iter.match_with(&[Type(Indent)]) {
            panic!("empty rule");
        }

        while !iter.match_with(&[Type(Dedent)]) {
            if iter.check(&[Type(Word), Lexeme(":"), Lexeme("=")]) {
                variants.push(self.parse_variant(&mut iter));
            } else if iter.check(&[Lexeme("["), Type(Word), Lexeme("]"), Type(EOL)]) {
                segments.push(self.parse_segment(&mut iter));
            }
        }

        Rule { name, segments, variants }
    }

    fn parse_variant (&mut self, iter: &'s mut TokenIterator) -> Variant<'t, 's> {
        let name = iter.get(0).unwrap();
        iter.eat(3);

        let mut segments: Vec<Segment<'t, 's>> = Vec::new();
        let mut tokens: Vec<&'t Token<'s>> = Vec::new();
        let mut aliases: Vec<Alias<'t, 's>> = Vec::new();

        if iter.match_with(&[Type(EOL)]) {
            panic!("empty variant");
        }

        Variant { name, segments, aliases, tokens }
    }

    fn parse_segment (&mut self, iter: &'s mut TokenIterator) -> Segment<'t, 's> {
        let name = iter.get(1).unwrap();
        iter.eat(3);

        if !iter.match_with(&[Type(Indent)]) {
            panic!("empty segment");
        }

        let mut tokens: Vec<&'t Token<'s>> = Vec::new();

        Segment { name, tokens }
    }
}

