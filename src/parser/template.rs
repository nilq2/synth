use tokenizer::token::{Type, Token, TokenIterator};
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
        let tokens = self.source.tokens.as_ref().unwrap();

        let mut iter = TokenIterator::new(tokens);
        let mut rules = Vec::new();

        while !iter.match_with(&[Type(EOF)]) {
            if iter.check(&[Type(Word), Lexeme(":"), Type(EOL)]) {
                rules.push(self.parse_rule(&mut iter));
            }
            //iter.next();
        }

        self.rules = Some(rules);
    }

    fn parse_rule (&mut self, mut iter: &mut TokenIterator<'t, 's>) -> Rule<'t, 's> {
        let name = iter.get(0).unwrap();
        iter.eat(3);

        let mut segments: Vec<Segment<'t, 's>> = Vec::new();
        let mut variants: Vec<Variant<'t, 's>> = Vec::new();

        if !iter.match_with(&[Type(Indent)]) {
            panic!("empty rule");
        }

        println!(":: parsing rule {:?}", name.lexeme);

        while !iter.match_with(&[Type(Dedent)]) {
            if iter.check(&[Type(Word), Lexeme(":"), Lexeme("=")]) {
                variants.push(self.parse_variant(&mut iter));

            } else if iter.check(&[Lexeme("["), Type(Word), Lexeme("]"), Type(EOL)]) {
                segments.push(self.parse_segment(&mut iter));
            }
        }

        Rule { name, segments, variants }
    }

    fn parse_variant (&mut self, mut iter: &mut TokenIterator<'t, 's>) -> Variant<'t, 's> {
        let name = iter.get(0).unwrap();
        iter.eat(3);

        println!(":= parsing variant {:?}", name.lexeme);

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

                aliases.push(Alias { name: alias_name, token: tokens.len() });
            }

            tokens.push(iter.next().unwrap());
        }

        if iter.match_with(&[Type(Indent)]) {
            loop {
                if iter.check(&[Pair(Symbol, "["), Type(Word), Pair(Symbol, "]"), Type(EOL)]) {
                    segments.push(self.parse_segment(&mut iter));
                }

                if iter.match_with(&[Type(Dedent)]) {
                    break
                }
            }
        }

        Variant { name, segments, aliases, tokens }
    }

    fn parse_segment (&mut self, mut iter: &mut TokenIterator<'t, 's>) -> Segment<'t, 's> {
        let name = iter.get(1).unwrap();
        iter.eat(3);

        println!("[] parsing segment {:?}", name.lexeme);

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

        Segment { name, tokens }
    }
}
