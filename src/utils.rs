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

// https://github.com/serde-rs/serde/issues/1560
macro_rules! named_unit_variant {
    ($variant:ident) => {
        pub mod $variant {
            const VARIANT_STR: &str = stringify!($variant);

            pub fn serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(VARIANT_STR)
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V;
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = ();
                    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        f.write_str(concat!("\"", stringify!($variant), "\""))
                    }
                    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        if value == VARIANT_STR {
                            Ok(())
                        } else {
                            Err(E::invalid_value(serde::de::Unexpected::Str(value), &self))
                        }
                    }
                }
                deserializer.deserialize_str(V)
            }
        }
    };
}

pub(crate) use named_unit_variant;
