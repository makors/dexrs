# dexrs 🩸🍭
Rust library for interacting with the Dexcom Share API

> [!WARNING]
> `dexrs` is most definitely still a **work in progress**. If you notice a bug, please open an issue or PR.

## Installation
`dexrs` can be installed through Cargo:
```
cargo add dexrs
```

## Usage
You can use `dexrs` by implementing the code below. See [/examples](https://github.com/makors/dexrs/tree/main/examples) for more examples.
```rust
use dexrs::dexcom::client::DexcomClient;
use std::env;

pub fn main() {
    let client = DexcomClient::new(env::var("DEXCOM_USERNAME").unwrap(), env::var("DEXCOM_PASSWORD").unwrap(), false).unwrap();

    let values = client.get_glucose_readings(None, None).unwrap();
    for v in values {
        println!("MG/DL: {}, Trend: {}, Time: {}", v.mg_dl, v.trend.arrow, v.datetime);
    }
}
```

## Contributing
If you wish to contribute improvements, bug fixes, or even new features, feel free to open a PR. *Everyone* is welcome to contribute.

## License
Everything is licensed under MIT. See [LICENSE](https://github.com/makors/dexrs/tree/main/LICENSE) for more.
