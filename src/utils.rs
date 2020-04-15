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

use serde::{Deserialize, Deserializer};

pub fn default_on_null<'de, D, T: Default + Deserialize<'de>>(
    deserializer: D,
) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
