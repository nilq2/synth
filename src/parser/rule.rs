use tokenizer::token::Token;
use alias::Alias;


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



impl<'t, 's: 't> Rule<'t, 's> {
    pub fn new (name: &'t Token<'s>, variants: Vec<Variant<'t, 's>>, segments: Vec<Segment<'t, 's>>) -> Self {
        Self { name, variants, segments }
    }
}

impl<'t, 's: 't> Variant<'t, 's> {
    pub fn new (
        name: &'t Token<'s>,
        tokens: Vec<&'t Token<'s>>,
        segments: Vec<Segment<'t, 's>>,
        aliases: Vec<Alias<'t, 's>>
    ) -> Self {
        Self { name, tokens, segments, aliases }
    }
}

impl<'t, 's: 't> Segment<'t, 's> {
    pub fn new (name: &'t Token<'s>, tokens: Vec<&'t Token<'s>>) -> Self {
        Self { name, tokens }
    }
}

