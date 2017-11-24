use tokenizer::token::Token;

pub struct Alias<'t, 's: 't> {
    name: &'t Token<'s>,
    token: usize,
}


impl<'t, 's: 't> Alias<'t, 's> {
    pub fn new(name: &'t Token<'s>, token: usize) -> Self {
        Self { name, token }
    }
}
