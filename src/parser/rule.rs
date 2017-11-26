use tokenizer::token::Token;
use alias::Alias;

#[derive(Debug)]
pub struct Rule<'t, 's: 't> {
    pub name: &'t Token<'s>,
    pub is_matching: bool,
    pub variants: Vec<Variant<'t, 's>>,
    pub segments: Vec<Segment<'t, 's>>,
}

#[derive(Debug)]
pub struct Variant<'t, 's: 't> {
    pub name: &'t Token<'s>,
    pub tokens: Vec<&'t Token<'s>>,
    pub segments: Vec<Segment<'t, 's>>,
    pub aliases: Vec<Alias<'t, 's>>,
}

#[derive(Debug)]
pub struct Segment<'t, 's: 't> {
    pub name: &'t Token<'s>,
    pub tokens: Vec<&'t Token<'s>>,
}



impl<'t, 's: 't> Rule<'t, 's> {
    pub fn new (name: &'t Token<'s>, is_matching: bool, variants: Vec<Variant<'t, 's>>, segments: Vec<Segment<'t, 's>>) -> Self {
        Self { name, is_matching, variants, segments }
    }

    pub fn variant (&self, name: &str) -> Option<&Variant<'t, 's>> {
        for var in self.variants.iter() {
            if var.name.lexeme.unwrap() == name {
                return Some(&var)
            }
        }

        None
    }

    pub fn segment (&self, name: &str) -> Option<&Segment<'t, 's>> {
        for var in self.segments.iter() {
            if var.name.lexeme.unwrap() == name {
                return Some(&var)
            }
        }

        None
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

    pub fn segment (&self, name: &str) -> Option<&Segment<'t, 's>> {
        for var in self.segments.iter() {
            if var.name.lexeme.unwrap() == name {
                return Some(&var)
            }
        }

        None
    }
}

impl<'t, 's: 't> Segment<'t, 's> {
    pub fn new (name: &'t Token<'s>, tokens: Vec<&'t Token<'s>>) -> Self {
        Self { name, tokens }
    }
}

