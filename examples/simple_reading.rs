use dexrs::prelude::*;
use std::env;
use std::time::Duration;

fn main() -> DexcomResult<()> {
    // Get credentials from environment variables
    let username = env::var("DEXCOM_USERNAME")
        .map_err(|_| DexcomApiError::InvalidInput("DEXCOM_USERNAME environment variable not set".to_string()))?;
    let password = env::var("DEXCOM_PASSWORD")
        .map_err(|_| DexcomApiError::InvalidInput("DEXCOM_PASSWORD environment variable not set".to_string()))?;

    // Create client using the builder pattern
    let client = DexcomClient::builder(username, password)
        .outside_us(false) // Set to true if outside the US
        .timeout(Duration::from_secs(30))
        .retry_attempts(3)
        .build()?;

    println!("Successfully connected to Dexcom API");
    
    // Get the latest reading
    match client.get_latest_reading()? {
        Some(reading) => {
            println!("Latest Reading:");
            println!("  Glucose: {} mg/dL", reading.mg_dl);
            println!("  Trend: {} ({})", reading.trend.arrow, reading.trend.description);
            println!("  Time: {}", reading.datetime);
            
            // Check if glucose is in healthy range
            let status = client.get_glucose_status(reading.mg_dl);
            let is_healthy = client.is_glucose_healthy(reading.mg_dl);
            println!("  Status: {} ({})", status, if is_healthy { "Healthy" } else { "Needs Attention" });
        }
        None => println!("No recent readings available"),
    }

    // Get readings for the last hour
    println!("\nReadings for the last hour:");
    let hourly_readings = client.get_readings_last_hour()?;
    for reading in hourly_readings.iter().take(5) { // Show first 5 readings
        println!("  {} mg/dL at {}", reading.mg_dl, reading.datetime.format("%H:%M"));
    }

    // Get average glucose for last 24 hours
    if let Some(avg) = client.get_average_glucose_24h()? {
        println!("\n24-hour average glucose: {:.1} mg/dL", avg);
    }

    Ok(())
}