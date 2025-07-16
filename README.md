# dexrs 🩸🍭
A robust Rust library for interacting with the Dexcom Share API

> [!WARNING]
> `dexrs` is most definitely still a **work in progress**. If you notice a bug, please open an issue or PR. We are not affiliated with Dexcom in any way.

## Features ✨

- 🔐 **Secure Authentication**: Robust session management with automatic retry logic
- 📊 **Flexible Data Access**: Get glucose readings with customizable time ranges and parameters
- 🛡️ **Error Handling**: Comprehensive error types with detailed error messages
- ⚡ **Performance**: Efficient HTTP client with connection pooling and timeouts
- 🔧 **Configurable**: Builder pattern for easy client configuration
- 📈 **Analytics**: Built-in glucose analysis and health status checking
- 🌍 **Global Support**: Support for both US and non-US Dexcom servers

## Installation 📦

`dexrs` can be installed through Cargo:

```bash
cargo add dexrs
```

## Quick Start 🚀

### Basic Usage

```rust
use dexrs::prelude::*;
use std::env;

fn main() -> DexcomResult<()> {
    // Get credentials from environment variables
    let username = env::var("DEXCOM_USERNAME")?;
    let password = env::var("DEXCOM_PASSWORD")?;

    // Create client
    let client = DexcomClient::new(username, password, false)?;

    // Get latest reading
    if let Some(reading) = client.get_latest_reading()? {
        println!("Glucose: {} mg/dL", reading.mg_dl);
        println!("Trend: {}", reading.trend.arrow);
        println!("Time: {}", reading.datetime);
    }

    Ok(())
}
```

### Advanced Usage with Builder Pattern

```rust
use dexrs::prelude::*;
use std::time::Duration;

fn main() -> DexcomResult<()> {
    let client = DexcomClient::builder(username, password)
        .outside_us(false)  // Set to true if outside the US
        .timeout(Duration::from_secs(30))
        .retry_attempts(3)
        .user_agent("MyApp/1.0".to_string())
        .build()?;

    // Get readings for different time periods
    let latest = client.get_latest_reading()?;
    let hourly = client.get_readings_last_hour()?;
    let daily = client.get_readings_last_24_hours()?;

    // Get average glucose
    if let Some(avg) = client.get_average_glucose_24h()? {
        println!("24-hour average: {:.1} mg/dL", avg);
    }

    Ok(())
}
```

## API Reference 📚

### Client Creation

```rust
// Simple creation
let client = DexcomClient::new(username, password, outside_us)?;

// Builder pattern
let client = DexcomClient::builder(username, password)
    .outside_us(false)
    .timeout(Duration::from_secs(30))
    .retry_attempts(3)
    .build()?;
```

### Getting Glucose Readings

```rust
// Latest reading
let latest = client.get_latest_reading()?;

// Predefined time ranges
let hourly = client.get_readings_last_hour()?;
let daily = client.get_readings_last_24_hours()?;

// Custom parameters
let params = GlucoseReadingsParams::new(Some(120), Some(10))?; // 2 hours, max 10 readings
let custom = client.get_glucose_readings_with_params(params)?;

// Since specific time
let since = Utc::now() - Duration::hours(6);
let since_readings = client.get_readings_since(since)?;
```

### Analysis and Utilities

```rust
// Check if glucose is healthy (70-180 mg/dL)
let is_healthy = client.is_glucose_healthy(reading.mg_dl);

// Get glucose status
let status = client.get_glucose_status(reading.mg_dl); // "Low", "Normal", "High", "Very High"

// Get 24-hour average
let avg = client.get_average_glucose_24h()?;

// Session management
if client.has_valid_session() {
    let (account_id, session_id) = client.session_info().unwrap();
}
```

## Error Handling 🛡️

The library provides comprehensive error handling with specific error types:

```rust
use dexrs::DexcomApiError;

match client.get_latest_reading() {
    Ok(Some(reading)) => println!("Got reading: {}", reading.mg_dl),
    Ok(None) => println!("No readings available"),
    Err(DexcomApiError::LoginError(msg)) => println!("Login failed: {}", msg),
    Err(DexcomApiError::RateLimitExceeded) => println!("Rate limit exceeded, please wait"),
    Err(DexcomApiError::NetworkError(msg)) => println!("Network error: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```

## Examples 📖

See the [`examples/`](https://github.com/makors/dexrs/tree/main/examples) directory for complete examples:

- [`simple_reading.rs`](examples/simple_reading.rs) - Basic usage example
- [`advanced_usage.rs`](examples/advanced_usage.rs) - Advanced features demonstration

Run examples with:

```bash
# Set your credentials
export DEXCOM_USERNAME="your_username"
export DEXCOM_PASSWORD="your_password"

# Run examples
cargo run --example simple_reading
cargo run --example advanced_usage
```

## Configuration ⚙️

### Environment Variables

- `DEXCOM_USERNAME` - Your Dexcom username/email
- `DEXCOM_PASSWORD` - Your Dexcom password

### Client Configuration

```rust
let config = DexcomConfig {
    timeout: Duration::from_secs(30),
    retry_attempts: 3,
    user_agent: "MyApp/1.0".to_string(),
};

let client = DexcomClient::new_with_config(username, password, false, config)?;
```

## Error Types 🚨

| Error Type | Description |
|------------|-------------|
| `LoginError` | Authentication failed |
| `SessionError` | Session management issues |
| `NotSharer` | User is not a Dexcom sharer |
| `HttpError` | HTTP request failures |
| `ParseError` | JSON parsing errors |
| `RateLimitExceeded` | API rate limit hit |
| `NetworkError` | Network connectivity issues |
| `TimeoutError` | Request timeout |
| `InvalidInput` | Invalid parameters |

## Contributing 🤝

If you wish to contribute improvements, bug fixes, or new features, feel free to open a PR. *Everyone* is welcome to contribute!

### Development Setup

```bash
git clone https://github.com/makors/dexrs.git
cd dexrs
cargo build
cargo test
```

## License 📄

Everything is licensed under MIT. See [LICENSE](LICENSE) for more details.

## Disclaimer ⚠️

This library is not affiliated with Dexcom in any way. Use at your own risk and ensure you comply with Dexcom's terms of service.
