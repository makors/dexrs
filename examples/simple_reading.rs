use dexrs::dexcom::client::DexcomClient;
use std::env;

pub fn main() {
    let client = DexcomClient::new(env::var("DEXCOM_USERNAME").unwrap(), env::var("DEXCOM_PASSWORD").unwrap(), false).unwrap();

    let values = client.get_glucose_readings(None, None).unwrap();
    for v in values {
        println!("MG/DL: {}, Trend: {}, Time: {}", v.mg_dl, v.trend.arrow, v.datetime);
    }
}