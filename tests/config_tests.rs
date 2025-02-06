use etf_matcher_vector_config_loader::*;
use std::collections::BTreeMap;

#[test]
fn test_get_all_etf_matcher_configs() {
    let configs = get_all_etf_matcher_configs();

    assert!(configs.is_ok(), "Failed to fetch configurations");

    let configs = configs.unwrap();
    assert!(
        !configs.is_empty(),
        "Expected some configurations but got an empty BTreeMap"
    );

    // Optionally check if a known key exists (if a specific key is expected)
    if let Some(default_config) = configs.get("default") {
        assert!(
            !default_config.path.is_empty(),
            "Expected default config to have a valid path"
        );
    }
}

#[test]
fn test_get_etf_matcher_config_by_key() {
    let result = get_etf_matcher_config_by_key("default");

    assert!(
        result.is_ok(),
        "Expected to fetch config for key 'default' but got an error"
    );

    let config = result.unwrap();
    assert!(
        !config.path.is_empty(),
        "Expected 'default' config to have a valid path"
    );

    // Test for a non-existent key
    let missing_result = get_etf_matcher_config_by_key("nonexistent_key");
    assert!(
        missing_result.is_err(),
        "Expected an error when fetching a non-existent key but got Ok"
    );
}

#[test]
fn test_get_resource_url() {
    let url = get_resource_url("test_file.bin");
    assert_eq!(url, "https://etfmatcher.com/data/test_file.bin");
}

#[test]
fn test_get_symbol_map_url() {
    let url = get_symbol_map_url();
    assert_eq!(
        url,
        "https://etfmatcher.com/data/ticker_symbol_map.flatbuffers.bin"
    );
}

#[test]
fn test_get_config_by_key() {
    let mut configs = BTreeMap::new();
    configs.insert(
        "test".to_string(),
        TickerVectorConfig {
            path: "test_path.bin".to_string(),
            description: Some("Test Config".to_string()),
            proto_noteboook: None,
            last_training_time: Some("2025-01-01T00:00:00Z".to_string()),
            features: Some(100),
            vector_dimensions: Some(200),
            training_sequence_length: Some(50),
            training_data_sources: Some(vec!["source1".to_string(), "source2".to_string()]),
        },
    );

    let config = get_config_by_key(&configs, "test");
    assert!(config.is_some());
    assert_eq!(config.unwrap().path, "test_path.bin");

    let missing_config = get_config_by_key(&configs, "nonexistent");
    assert!(missing_config.is_none());
}
