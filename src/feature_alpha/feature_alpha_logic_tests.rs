#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::cli_logic::Config;

    #[test]
    fn test_config_creation() {
        let config = Config {
            text: "Test".to_string(),
            style: "default".to_string(),
        };
        assert_eq!(config.text, "Test");
        assert_eq!(config.style, "default");
    }
}
