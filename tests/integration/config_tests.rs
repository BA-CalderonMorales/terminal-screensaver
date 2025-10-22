use terminal_screensaver::cli::cli_logic::{ActionConfig, Config};

#[test]
fn test_config_construction() {
    let config = Config {
        text: "Test Screensaver".to_string(),
        style: "default".to_string(),
        actions: vec![],
    };

    assert_eq!(config.text, "Test Screensaver");
    assert_eq!(config.style, "default");
    assert_eq!(config.actions.len(), 0);
}

#[test]
fn test_config_with_actions() {
    let action = ActionConfig {
        key: "h".to_string(),
        description: "Help".to_string(),
        command: "echo help".to_string(),
    };

    let config = Config {
        text: "Test".to_string(),
        style: "default".to_string(),
        actions: vec![action.clone()],
    };

    assert_eq!(config.actions.len(), 1);
    assert_eq!(config.actions[0].key, "h");
    assert_eq!(config.actions[0].description, "Help");
}

#[test]
fn test_action_config_fields() {
    let action = ActionConfig {
        key: "t".to_string(),
        description: "Test Action".to_string(),
        command: "bash -c 'echo test'".to_string(),
    };

    assert_eq!(action.key, "t");
    assert_eq!(action.description, "Test Action");
    assert!(action.command.contains("echo test"));
}
