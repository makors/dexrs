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
