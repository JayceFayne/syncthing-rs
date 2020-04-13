pub struct QueryChars(bool);

impl QueryChars {
    pub fn new() -> Self {
        Self(false)
    }

    pub fn next_char(&mut self) -> char {
        match self.0 {
            true => '&',
            false => {
                self.0 = true;
                '?'
            }
        }
    }
}
