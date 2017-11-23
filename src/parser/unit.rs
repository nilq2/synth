use tokenizer::token::Token;
use tokenizer::tokenizer::Source;
use template::*;
use std::sync::Arc;



pub struct Node<'u> {
    name: &'u Token<'u>,
    variant: &'u Variant<'u>,

    tokens: Vec<&'u Token<'u>>,
    children: Vec<&'u Node<'u>>,
}

pub struct Unit<'u> {
    source: Source<'u>,
    template: Arc<Template<'u>>,
    ast: Option<Vec<Node<'u>>>,
}


impl<'u> Unit<'u> {
    pub fn new (source: Source<'u>, template: Arc<Template<'u>>) -> Unit<'u> {
        Unit { source, template, ast:None }
    }

    pub fn parse (&mut self) {

    }
}

