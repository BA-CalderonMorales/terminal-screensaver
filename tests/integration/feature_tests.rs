use terminal_screensaver::features::{get_available_features, get_feature_description};

#[test]
fn test_get_available_features() {
    let features = get_available_features();

    assert!(features.len() > 0, "Should have at least one feature");
    assert!(features.contains(&"text_display"), "Should include text_display");
    assert!(features.contains(&"bouncing_logo"), "Should include bouncing_logo");
}

#[test]
fn test_get_feature_description_valid() {
    let desc = get_feature_description("text_display");
    assert!(desc.is_some(), "text_display should have a description");

    let text_desc = desc.unwrap();
    assert!(!text_desc.is_empty(), "Description should not be empty");
    assert!(text_desc.contains("text") || text_desc.contains("display"), "Description should be relevant");
}

#[test]
fn test_get_feature_description_invalid() {
    let desc = get_feature_description("nonexistent_feature");
    assert!(desc.is_none(), "Invalid feature should return None");
}

#[test]
fn test_all_available_features_have_descriptions() {
    let features = get_available_features();

    for feature_name in features {
        let desc = get_feature_description(feature_name);
        assert!(desc.is_some(), "Feature {} should have a description", feature_name);
        assert!(!desc.unwrap().is_empty(), "Feature {} description should not be empty", feature_name);
    }
}
