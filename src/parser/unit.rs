use tokenizer::token::Token;
use tokenizer::tokenizer::Source;
use template::*;
use rule::*;
use alias::*;
use std::sync::Arc;



pub struct Node<'u, 't: 'u> {
    name: &'u Token<'t>,
    variant: &'u Variant<'u, 't>,

    tokens: Vec<&'u Token<'t>>,
    children: Vec<&'u Node<'u, 't>>,
}

pub struct Path<'t, 's: 't> {
    variant: Variant<'t, 's>,
    children: Vec<Path<'t, 's>>,
}

pub struct Unit<'u, 's: 'u, 't: 'u> {
    source: &'u Source<'s>,
    template: Arc<Template<'t, 't>>,
    ast: Option<Vec<Node<'u, 's>>>,
}


impl<'u, 's, 't> Unit<'u, 's, 't> {
    pub fn new (source: &'u Source<'s>, template: Arc<Template<'t, 't>>) -> Self  {
        Self { source, template, ast:None }
    }

    pub fn parse (&mut self) {

    }
}

