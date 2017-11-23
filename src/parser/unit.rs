use tokenizer::token::Token;
use tokenizer::tokenizer::Source;
use template::*;



pub struct Node<'unit> {
    name: &'unit Token<'unit>,
    variant: &'unit Variant<'unit>,

    tokens: Vec<&'unit Token<'unit>>,
    children: Vec<&'unit Node<'unit>>,
}

pub struct Unit<'unit> {
    source: Source<'unit>,
    template: Template<'unit>,
    ast: Option<Vec<Node<'unit>>>,
}


impl<'unit> Unit<'unit> {
    pub fn new<'source> (source: Source<'source>) {}

    pub fn parse () {}
}

