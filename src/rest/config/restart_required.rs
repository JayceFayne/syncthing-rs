use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct RestartRequired {
    /// Whether a restart of Syncthing is required for the current config to take effect.
    pub requires_restart: bool,
}

#[cfg(test)]
mod tests {
    use crate::rest::config::test_helper;

    use super::*;

    #[test]
    fn restart_required_deserializes() {
        let deserialized = test_helper::deserialize::<RestartRequired>(RESTART_REQUIRED);
        assert!(deserialized.requires_restart);

        const RESTART_REQUIRED: &str = r#"
        {
            "requiresRestart": true
        }
        "#;
    }
}
