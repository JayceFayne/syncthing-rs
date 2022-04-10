pub mod devices;
pub mod folders;

pub type FolderId = String;

#[cfg(test)]
mod test_helper {
    use serde::{Deserialize, Serialize};

    pub fn deserialize<'a, T: Deserialize<'a>>(json: &'a str) -> T {
        serde_json::from_str(json).unwrap()
    }

    pub fn serialize<T: Serialize>(value: T) -> String {
        serde_json::to_string(&value).unwrap()
    }

    /// Deserialize `json` into `T`, then serialize it, deserialize it again into a
    /// [`serde_json::Value`] and compare it with a directly [`serde_json::Value`] normalized
    /// representation.
    pub fn normalization_roundtrip_test<T: Serialize + for<'a> Deserialize<'a>>(json: &str) {
        let value: T = deserialize(json);
        let serialized = serialize(value);
        let normalized: serde_json::Value = deserialize(&serialized);
        let normalized_expected: serde_json::Value = deserialize(json);
        assert_eq!(normalized, normalized_expected)
    }
}
