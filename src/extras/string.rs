pub trait StringExtras {
    fn is_uppercase (&self) -> bool;
    fn is_lowercase (&self) -> bool;
}

impl StringExtras for String {
    fn is_uppercase (&self) -> bool {
        &self.to_uppercase() == self
    }

    fn is_lowercase (&self) -> bool {
        &self.to_lowercase() == self
    }
}

impl<'a> StringExtras for &'a str {
    fn is_uppercase (&self) -> bool {
        self.to_uppercase() == self.to_string()
    }

    fn is_lowercase (&self) -> bool {
        self.to_lowercase() == self.to_string()
    }
}
