pub struct QueryChars(bool);

impl QueryChars {
    pub fn new() -> Self {
        Self(false)
    }

    pub fn next_char(&mut self) -> char {
        if self.0 {
            '&'
        } else {
            self.0 = true;
            '?'
        }
    }
}
