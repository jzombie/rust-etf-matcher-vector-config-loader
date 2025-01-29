# ETF Matcher Vector Config Loader
A Rust library for downloading and parsing [ETF Matcher](https://etfmatcher.com/) vector configurations for ticker similarity search downstream tasks.

## Examples

### Load an ETF Matcher Configuration by Key
Retrieve a specific ETF vector configuration (e.g., `"v5-sma-lstm-stacks"`):

```rust
use rust_etf_matcher_vector_config_loader::get_etf_matcher_config_by_key;

fn main() {
    let config = get_etf_matcher_config_by_key("v5-sma-lstm-stacks").unwrap();
    
    println!("Vector Config Path: {}", config.path);
    println!("Description: {:?}", config.description.unwrap_or("No description".to_string()));
    println!("Feature Count: {:?}", config.features.unwrap_or(0));
}
```
🔹 **Example Output:**
```
Vector Config Path: v5.SMA-LSTM-STACKS.autoencoder.ticker_vectors_collection.flatbuffers.bin
Description: "v5 SMA LSTM STACKS"
Feature Count: 158
```

---

### Fetch All ETF Matcher Configurations
Retrieve **all** available configurations and print their paths:

```rust
use rust_etf_matcher_vector_config_loader::get_all_etf_matcher_configs;

fn main() {
    let configs = get_all_etf_matcher_configs().unwrap();

    println!("Available Configurations:");
    for (key, config) in configs.iter() {
        println!("{} -> {}", key, config.path);
    }
}
```
🔹 **Example Output:**
```
Available Configurations:
default -> v5.SPY-CORR-NO-SCALE-2.ticker_vectors_collection.flatbuffers.bin
v5-sma-lstm-stacks -> v5.SMA-LSTM-STACKS.autoencoder.ticker_vectors_collection.flatbuffers.bin
```

---

### **Get the Fully Qualified URL for an ETF Matcher Dataset**
Construct a **fully qualified URL** to download a specific dataset:

```rust
use rust_etf_matcher_vector_config_loader::get_resource_url;

fn main() {
    let dataset_url = get_resource_url("v5.SPY-CORR-NO-SCALE-2.ticker_vectors_collection.flatbuffers.bin");
    println!("Download URL: {}", dataset_url);
}
```
🔹 **Example Output:**
```
Download URL: https://etfmatcher.com/data/v5.SPY-CORR-NO-SCALE-2.ticker_vectors_collection.flatbuffers.bin
```

---

### **Fetch and Use a Configuration to Download a File**
Dynamically fetch a **specific configuration** and construct its **download URL**:

```rust
use rust_etf_matcher_vector_config_loader::{get_etf_matcher_config_by_key, get_resource_url};

fn main() {
    let config = get_etf_matcher_config_by_key("default").unwrap();
    
    let dataset_url = get_resource_url(&config.path);
    println!("Download dataset from: {}", dataset_url);
}
```
🔹 **Example Output:**
```
Download dataset from: https://etfmatcher.com/data/v5.SPY-CORR-NO-SCALE-2.ticker_vectors_collection.flatbuffers.bin
```

---

### **Verify if a Specific ETF Matcher Config Exists**
Check if a given configuration **exists** before using it:

```rust
use rust_etf_matcher_vector_config_loader::{get_all_etf_matcher_configs, get_config_by_key};

fn main() {
    let configs = get_all_etf_matcher_configs().unwrap();

    let config_key = "v5-sma-lstm-stacks";
    if let Some(config) = get_config_by_key(&configs, config_key) {
        println!("Configuration '{}' found!", config_key);
        println!("Path: {}", config.path);
    } else {
        println!("Configuration '{}' does not exist.", config_key);
    }
}
```
🔹 **Example Output:**
```
Configuration 'v5-sma-lstm-stacks' found!
Path: v5.SMA-LSTM-STACKS.autoencoder.ticker_vectors_collection.flatbuffers.bin
```

## License
[MIT License](LICENSE) (c) 2025 Jeremy Harris.

