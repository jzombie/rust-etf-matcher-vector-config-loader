# ETF Matcher Vector Config Loader
A Rust library for downloading and parsing [ETF Matcher](https://etfmatcher.com/) vector configurations for ticker similarity search downstream tasks.

## Examples

### Fetch All ETF Matcher Configurations
Retrieve **all** available configurations and print their paths:

```rust
use etf_matcher_vector_config_loader::get_all_etf_matcher_configs;

fn main() {
    let configs = get_all_etf_matcher_configs().unwrap();

    println!("Available Configurations:");
    for (key, config) in configs.iter() {
        println!("{} -> {}", key, config.path);
    }
}
```
🔹 **Example Output:**
```text
Available Configurations:
default -> v5.SPY-CORR-NO-SCALE-2.ticker_vectors_collection.flatbuffers.bin
v5-sma-lstm-stacks -> v5.SMA-LSTM-STACKS.autoencoder.ticker_vectors_collection.flatbuffers.bin
```

---

### Load an ETF Matcher Configuration by Key
Retrieve a specific ETF vector configuration (e.g., `"v5-sma-lstm-stacks"`):

```rust
use etf_matcher_vector_config_loader::get_etf_matcher_config_by_key;

fn main() {
    let config = get_etf_matcher_config_by_key("v5-sma-lstm-stacks").unwrap();
    
    println!("Vector Config Path: {}", config.path);
    println!("Description: {:?}", config.description.unwrap_or("No description".to_string()));
    println!("Feature Count: {:?}", config.features.unwrap_or(0));
}
```
🔹 **Example Output:**
```text
Vector Config Path: v5.SMA-LSTM-STACKS.autoencoder.ticker_vectors_collection.flatbuffers.bin
Description: "v5 SMA LSTM STACKS"
Feature Count: 158
```

---

### Load Default Ticker Vectors Collection and Symbol Map
Retrieve the **ticker vectors** collection and symbol map, then initialize repositories.

```rust
use etf_matcher_vector_config_loader::{get_ticker_vectors_collection_by_key, get_symbol_map};

fn main() {
    // Load ticker vectors collection
    // Where `default` represents the key of the default configuration
    let ticker_vectors_collection_bytes: Vec<u8> = get_ticker_vectors_collection_by_key("default")
        .map_err(|err| format!("Error when loading ticker vectors collection. {:?}", err))
        .expect("Failed to load ticker vectors collection");

    // Load ticker symbol map
    // In most cases, the ticker symbol map includes all available ticker vector 
    // collections. However, some entities may not be mapped if the chosen ticker 
    // vector collection has not been recently updated.
    let ticker_symbol_map_bytes: Vec<u8> = get_symbol_map()
                .map_err(|err| format!("Error when loading ticker symbol map. {:?}", err))
        .expect("Failed to load ticker symbol map");

    // Example usage
    println!("Ticker vector repository and symbol mapper initialized successfully.");
}
```
🔹 **Example Output:**
```text
Ticker vector repository and symbol mapper initialized successfully.
```

## License
[MIT License](LICENSE) (c) 2025 Jeremy Harris.

