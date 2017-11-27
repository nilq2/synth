#[derive(Debug)]
pub enum TokenizerErrorValue {
    Constant(String),
}

#[derive(Debug)]
pub struct TokenizerError {
    pub value:    TokenizerErrorValue,
    pub position: Option<(usize, usize)>,
}

#[allow(dead_code)]
impl TokenizerError {
    pub fn new(value: &str) -> TokenizerError {
        TokenizerError {
            value: TokenizerErrorValue::Constant(value.to_owned()),
            position: None,
        }
    }

    pub fn new_pos(position: (usize, usize), value: &str) -> TokenizerError {
        TokenizerError {
            value: TokenizerErrorValue::Constant(value.to_string()),
            position: Some(position),
        }
    }
}
