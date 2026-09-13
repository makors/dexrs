# dexrs

A small Rust library for reading glucose data from the Dexcom Share API.
Uses blocking HTTP requests. Not affiliated with Dexcom.

## Install

```sh
cargo add dexrs
```

## Read the latest value

Set `DEXCOM_USERNAME` and `DEXCOM_PASSWORD` in your environment, then:

```rust
use dexrs::dexcom::client::DexcomClient;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DexcomClient::new(
        env::var("DEXCOM_USERNAME")?,
        env::var("DEXCOM_PASSWORD")?,
        false, // true for accounts outside the US
    )?;

    for reading in client.get_glucose_readings(None, None)? {
        println!(
            "{} mg/dL ({:.1} mmol/L) {} at {}",
            reading.mg_dl, reading.mmol_l, reading.trend.arrow, reading.datetime,
        );
    }

    Ok(())
}
```

`get_glucose_readings(minutes, max_count)` defaults to one reading from the last
24 hours. For more history, pass values such as `Some(60), Some(12)`. The limits
are 1–1440 minutes and 1–288 readings. The account needs Dexcom Share enabled.

The same example is in [`examples/simple_reading.rs`](examples/simple_reading.rs).
Run it with `cargo run --example simple_reading`.

## Development

The repo uses Rust 1.98.1 and the 2024 edition. Rustup picks up the toolchain from
`rust-toolchain.toml`.

```sh
cargo test --locked --all-targets
```

Issues and pull requests are welcome. See [RELEASING.md](RELEASING.md) for releases.

MIT licensed. See [LICENSE](LICENSE).
