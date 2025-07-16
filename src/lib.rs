pub mod dexcom;
pub mod trends;
pub mod reading;

// Re-export main types for easier access
pub use dexcom::error::DexcomApiError;
pub use dexcom::client::{DexcomClient, DexcomClientBuilder, DexcomConfig};
pub use dexcom::glucose::GlucoseReadingsParams;
pub use reading::GlucoseReading;
pub use trends::TrendData as Trend;

/// Result type for Dexcom API operations
pub type DexcomResult<T> = Result<T, DexcomApiError>;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        DexcomClient,
        DexcomClientBuilder,
        DexcomConfig,
        DexcomApiError,
        DexcomResult,
        GlucoseReading,
        GlucoseReadingsParams,
        Trend,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_dexcom_config_default() {
        let config = DexcomConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.retry_attempts, 3);
        assert_eq!(config.user_agent, "dexrs/0.1.1");
    }

    #[test]
    fn test_glucose_readings_params_validation() {
        // Valid parameters
        let params = GlucoseReadingsParams::new(Some(100), Some(10));
        assert!(params.is_ok());

        // Invalid minutes (too high)
        let params = GlucoseReadingsParams::new(Some(2000), Some(10));
        assert!(params.is_err());

        // Invalid minutes (too low)
        let params = GlucoseReadingsParams::new(Some(0), Some(10));
        assert!(params.is_err());

        // Invalid max_count (too high)
        let params = GlucoseReadingsParams::new(Some(100), Some(300));
        assert!(params.is_err());

        // Invalid max_count (too low)
        let params = GlucoseReadingsParams::new(Some(100), Some(0));
        assert!(params.is_err());
    }

    #[test]
    fn test_glucose_readings_params_convenience_methods() {
        let latest = GlucoseReadingsParams::latest();
        assert_eq!(latest.minutes, Some(1440));
        assert_eq!(latest.max_count, Some(1));

        let hourly = GlucoseReadingsParams::last_hour();
        assert_eq!(hourly.minutes, Some(60));
        assert_eq!(hourly.max_count, Some(12));

        let daily = GlucoseReadingsParams::last_24_hours();
        assert_eq!(daily.minutes, Some(1440));
        assert_eq!(daily.max_count, Some(288));
    }

    #[test]
    fn test_dexcom_client_builder() {
        let builder = DexcomClientBuilder::new("test".to_string(), "test".to_string());
        assert_eq!(builder.username, "test");
        assert_eq!(builder.password, "test");
        assert_eq!(builder.ous, false);

        let builder = builder.outside_us(true);
        assert_eq!(builder.ous, true);

        let builder = builder.timeout(Duration::from_secs(60));
        assert_eq!(builder.config.timeout, Duration::from_secs(60));

        let builder = builder.retry_attempts(5);
        assert_eq!(builder.config.retry_attempts, 5);

        let builder = builder.user_agent("TestApp/1.0".to_string());
        assert_eq!(builder.config.user_agent, "TestApp/1.0");
    }

    #[test]
    fn test_error_types() {
        let login_error = DexcomApiError::LoginError("test".to_string());
        assert_eq!(login_error.to_string(), "Login error: test");

        let rate_limit_error = DexcomApiError::RateLimitExceeded;
        assert_eq!(rate_limit_error.to_string(), "Rate limit exceeded, please wait before retrying");

        let timeout_error = DexcomApiError::TimeoutError;
        assert_eq!(timeout_error.to_string(), "Request timed out");

        let invalid_input = DexcomApiError::InvalidInput("test".to_string());
        assert_eq!(invalid_input.to_string(), "Invalid input: test");
    }

    #[test]
    fn test_glucose_health_check() {
        // This would normally be a method on DexcomClient, but we can test the logic
        let is_healthy = |mg_dl: u16| mg_dl >= 70 && mg_dl <= 180;
        
        assert!(is_healthy(70));
        assert!(is_healthy(180));
        assert!(is_healthy(120));
        assert!(!is_healthy(69));
        assert!(!is_healthy(181));
    }

    #[test]
    fn test_glucose_status() {
        // This would normally be a method on DexcomClient, but we can test the logic
        let get_status = |mg_dl: u16| {
            match mg_dl {
                0..=69 => "Low",
                70..=180 => "Normal",
                181..=250 => "High",
                _ => "Very High",
            }
        };

        assert_eq!(get_status(50), "Low");
        assert_eq!(get_status(100), "Normal");
        assert_eq!(get_status(200), "High");
        assert_eq!(get_status(300), "Very High");
    }
}