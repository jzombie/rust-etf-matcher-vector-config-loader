use reqwest;
use serde::Deserialize;
use std::collections::HashMap;

static BASE_URL: &str = "https://etfmatcher.com/data/";

/// Represents the configuration for a ticker vector file.
/// This struct is deserialized from the TOML configuration file.
#[derive(Debug, Deserialize, Clone)]
pub struct TickerVectorConfig {
    /// File path of the ticker vector.
    pub path: String,
    /// Description of the dataset.
    pub description: Option<String>,
    /// Notebook used to generate the dataset.
    pub proto_noteboook: Option<String>,
    /// Timestamp of last training.
    pub last_training_time: Option<String>,
    /// Number of features used in the dataset.
    pub features: Option<u32>,
    /// Dimensionality of the vector representation.
    pub vector_dimensions: Option<u32>,
    /// Sequence length used in training.
    pub training_sequence_length: Option<u32>,
    /// List of data sources used for training.
    pub training_data_sources: Option<Vec<String>>,
}

/// Represents the structure of the entire TOML configuration file.
/// The configuration contains multiple named ticker vector configurations.
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "ticker_vector_config")]
    pub ticker_vector_config: HashMap<String, TickerVectorConfig>,
}

/// Fetches all ETF Matcher ticker vector configurations.
///
/// # Returns
/// * `Ok(HashMap<String, TickerVectorConfig>)` if the request succeeds.
/// * `Err(Box<dyn std::error::Error>)` if the request fails.
///
/// # Example
/// ```
/// use etf_matcher_vector_config_loader::get_all_etf_matcher_configs;
/// let configs = get_all_etf_matcher_configs().unwrap();
/// println!("Loaded {} configurations", configs.len());
/// ```
pub fn get_all_etf_matcher_configs(
) -> Result<HashMap<String, TickerVectorConfig>, Box<dyn std::error::Error>> {
    load_all_configs_from_url(&format!("{}ticker_vector_configs.toml", BASE_URL))
}

/// Fetches a specific ETF Matcher ticker vector configuration by key.
///
/// # Arguments
/// * `key` - The name of the configuration to retrieve.
///
/// # Returns
/// * `Ok(TickerVectorConfig)` if the key exists.
/// * `Err(Box<dyn std::error::Error>)` if the key is not found.
///
/// # Example
/// ```
/// use etf_matcher_vector_config_loader::get_etf_matcher_config_by_key;
/// let config = get_etf_matcher_config_by_key("default").unwrap();
/// println!("Config path: {}", config.path);
/// ```
pub fn get_etf_matcher_config_by_key(
    key: &str,
) -> Result<TickerVectorConfig, Box<dyn std::error::Error>> {
    let all_configs = get_all_etf_matcher_configs()?;

    let selected_config = get_config_by_key(&all_configs, key)
        .ok_or_else(|| format!("Config for key '{}' not found", key))?;

    Ok(selected_config.clone())
}

/// Retrieves the fully qualified URL for the ticker symbol map file.
///
/// # Returns
/// * A `String` containing the complete URL.
///
/// # Example
/// ```
/// use etf_matcher_vector_config_loader::get_symbol_map_url;
/// let url = get_symbol_map_url();
/// println!("Symbol map URL: {}", url);
/// ```
pub fn get_symbol_map_url() -> String {
    get_resource_url("ticker_symbol_map.flatbuffers.bin")
}

/// Constructs a fully qualified URL for a given filename.
///
/// # Arguments
/// * `filename` - The name of the file to create a full URL for.
///
/// # Returns
/// * A `String` containing the full URL.
///
/// # Example
/// ```
/// use etf_matcher_vector_config_loader::get_resource_url;
/// let url = get_resource_url("dataset.bin");
/// assert_eq!(url, "https://etfmatcher.com/data/dataset.bin");
/// ```
pub fn get_resource_url(filename: &str) -> String {
    format!("{}{}", BASE_URL, filename)
}

/// Fetches the ETF Matcher ticker vector configurations from a remote TOML file.
///
/// # Arguments
/// * `url` - The URL of the TOML configuration file.
///
/// # Returns
/// * `Ok(HashMap<String, TickerVectorConfig>)` on success.
/// * `Err(Box<dyn std::error::Error>)` if the request fails or the TOML parsing fails.
///
/// # Example
/// ```
/// use etf_matcher_vector_config_loader::load_all_configs_from_url;
/// let configs = load_all_configs_from_url("https://etfmatcher.com/data/ticker_vector_configs.toml").unwrap();
/// println!("Loaded {} configurations", configs.len());
/// ```
pub fn load_all_configs_from_url(
    url: &str,
) -> Result<HashMap<String, TickerVectorConfig>, Box<dyn std::error::Error>> {
    // Fetch the TOML file from the remote URL.
    let response = reqwest::blocking::get(url)?.text()?;

    // Parse the TOML content into a Config struct.
    let config: Config = toml::from_str(&response)?;

    // Return all configurations as a HashMap.
    Ok(config.ticker_vector_config)
}

/// Retrieves a specific configuration from the loaded ETF Matcher configurations.
///
/// # Arguments
/// * `configs` - A reference to the `HashMap` containing configurations.
/// * `key` - The key name of the configuration to retrieve.
///
/// # Returns
/// * `Some(&TickerVectorConfig)` if the key exists.
/// * `None` if the key does not exist.
///
/// # Example
/// ```
/// use etf_matcher_vector_config_loader::{load_all_configs_from_url, get_config_by_key};
/// let configs = load_all_configs_from_url("https://etfmatcher.com/data/ticker_vector_configs.toml").unwrap();
/// let config = get_config_by_key(&configs, "default");
/// assert!(config.is_some());
/// ```
pub fn get_config_by_key<'a>(
    configs: &'a HashMap<String, TickerVectorConfig>,
    key: &str,
) -> Option<&'a TickerVectorConfig> {
    configs.get(key)
}
