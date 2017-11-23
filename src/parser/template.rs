use tokenizer::token::Token;
use tokenizer::tokenizer::Source;


pub struct Alias<'template> {
    name: Token<'template>,
    token: usize,
}

pub struct Rule<'template> {
    name: Token<'template>,
    variants: Vec<Variant<'template>>,
    segments: Vec<Segment<'template>>,
}

pub struct Variant<'template> {
    name: Token<'template>,
    rule: Rule<'template>,

    tokens: Vec<Token<'template>>,
    segments: Vec<Segment<'template>>,
    alias: Vec<Alias<'template>>,
}

pub struct Segment<'template> {
    name: Token<'template>,

    variant: Variant<'template>,
    tokens: Vec<Token<'template>>,
}

pub struct Path<'template> {
    variant: Variant<'template>,
    children: Vec<Path<'template>>,
}

pub struct Template<'template> {
    source: Source<'template>,
    rules: Option<Vec<Rule<'template>>>,
}


impl<'template> Template<'template> {
    pub fn new<'source> (source: Source<'source>) {}

    pub fn parse () {}
}

