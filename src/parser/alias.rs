use tokenizer::token::Token;

#[derive(Debug)]
pub struct Alias<'t, 's: 't> {
    pub name: &'t Token<'s>,
    pub token: usize,
}


impl<'t, 's: 't> Alias<'t, 's> {
    pub fn new(name: &'t Token<'s>, token: usize) -> Self {
        Self { name, token }
    }
}
