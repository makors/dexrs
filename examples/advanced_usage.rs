use dexrs::prelude::*;
use std::env;
use std::time::Duration;
use chrono::{Utc, Duration as ChronoDuration};

fn main() -> DexcomResult<()> {
    // Get credentials from environment variables
    let username = env::var("DEXCOM_USERNAME")
        .map_err(|_| DexcomApiError::InvalidInput("DEXCOM_USERNAME environment variable not set".to_string()))?;
    let password = env::var("DEXCOM_PASSWORD")
        .map_err(|_| DexcomApiError::InvalidInput("DEXCOM_PASSWORD environment variable not set".to_string()))?;

    // Create client with custom configuration
    let mut client = DexcomClient::builder(username, password)
        .outside_us(false)
        .timeout(Duration::from_secs(45))
        .retry_attempts(5)
        .user_agent("MyDiabetesApp/1.0".to_string())
        .build()?;

    println!("=== Dexcom API Advanced Usage Example ===\n");

    // Check session status
    if client.has_valid_session() {
        if let Some((account_id, session_id)) = client.session_info() {
            println!("✓ Active session:");
            println!("  Account ID: {}", account_id);
            println!("  Session ID: {}", session_id);
        }
    }

    // Example 1: Get latest reading with detailed analysis
    println!("\n1. Latest Reading Analysis:");
    if let Some(reading) = client.get_latest_reading()? {
        analyze_reading(&client, &reading);
    }

    // Example 2: Get readings for different time periods
    println!("\n2. Time-based Readings:");
    
    // Last hour
    let hourly = client.get_readings_last_hour()?;
    println!("  Last hour: {} readings", hourly.len());
    
    // Last 24 hours
    let daily = client.get_readings_last_24_hours()?;
    println!("  Last 24 hours: {} readings", daily.len());

    // Custom time range (last 6 hours)
    let six_hours_ago = Utc::now() - ChronoDuration::hours(6);
    let custom = client.get_readings_since(six_hours_ago)?;
    println!("  Last 6 hours: {} readings", custom.len());

    // Example 3: Statistical analysis
    println!("\n3. Statistical Analysis:");
    if let Some(avg) = client.get_average_glucose_24h()? {
        println!("  24-hour average: {:.1} mg/dL", avg);
        
        let status = if avg < 70.0 {
            "Low"
        } else if avg <= 180.0 {
            "Normal"
        } else {
            "High"
        };
        println!("  Overall status: {}", status);
    }

    // Example 4: Custom parameters
    println!("\n4. Custom Parameters:");
    let custom_params = GlucoseReadingsParams::new(Some(120), Some(10))?; // Last 2 hours, max 10 readings
    let custom_readings = client.get_glucose_readings_with_params(custom_params)?;
    println!("  Custom query (2 hours, max 10): {} readings", custom_readings.len());

    // Example 5: Error handling demonstration
    println!("\n5. Error Handling:");
    let invalid_params = GlucoseReadingsParams::new(Some(2000), Some(1)); // Invalid minutes
    match invalid_params {
        Ok(_) => println!("  ✓ Parameters valid"),
        Err(e) => println!("  ✗ Parameter error: {}", e),
    }

    // Example 6: Session refresh
    println!("\n6. Session Management:");
    match client.refresh_session_if_needed() {
        Ok(_) => println!("  ✓ Session refreshed successfully"),
        Err(e) => println!("  ✗ Session refresh failed: {}", e),
    }

    println!("\n=== Example completed successfully ===");
    Ok(())
}

fn analyze_reading(client: &DexcomClient, reading: &GlucoseReading) {
    println!("  Glucose: {} mg/dL", reading.mg_dl);
    println!("  Trend: {} ({})", reading.trend.arrow, reading.trend.description);
    println!("  Time: {}", reading.datetime);
    
    let status = client.get_glucose_status(reading.mg_dl);
    let is_healthy = client.is_glucose_healthy(reading.mg_dl);
    
    println!("  Status: {} ({})", status, if is_healthy { "Healthy" } else { "Needs Attention" });
    
    // Trend analysis
    match reading.trend.arrow.as_str() {
        "↗" | "↘" => println!("  Trend: Stable"),
        "↑" | "↑↑" => println!("  Trend: Rising"),
        "↓" | "↓↓" => println!("  Trend: Falling"),
        "→" => println!("  Trend: Steady"),
        _ => println!("  Trend: Unknown"),
    }
}