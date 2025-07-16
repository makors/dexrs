use serde_json::json;

use crate::{
    reading::GlucoseReading,
    trends::get_trend,
};

use super::{client::DexcomClient, consts::DEXCOM_GLUCOSE_DATA_ENDPOINT};
use super::error::DexcomApiError;

use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Deserialize)]
struct GlucoseResp {
    #[serde(rename = "DT")]
    dt: String,
    #[serde(rename = "Value")]
    value: u16,
    #[serde(rename = "Trend")]
    trend: String,
}

/// Parameters for glucose readings request
#[derive(Debug, Clone)]
pub struct GlucoseReadingsParams {
    /// Number of minutes to look back (1-1440, default: 1440)
    pub minutes: Option<i16>,
    /// Maximum number of readings to return (1-288, default: 1)
    pub max_count: Option<i16>,
}

impl Default for GlucoseReadingsParams {
    fn default() -> Self {
        Self {
            minutes: Some(1440),
            max_count: Some(1),
        }
    }
}

impl GlucoseReadingsParams {
    /// Create new parameters with validation
    pub fn new(minutes: Option<i16>, max_count: Option<i16>) -> Result<Self, DexcomApiError> {
        let minutes = minutes.unwrap_or(1440);
        let max_count = max_count.unwrap_or(1);

        if minutes < 1 || minutes > 1440 {
            return Err(DexcomApiError::InvalidInput(
                "Minutes must be between 1 and 1440".to_string()
            ));
        }

        if max_count < 1 || max_count > 288 {
            return Err(DexcomApiError::InvalidInput(
                "Max count must be between 1 and 288".to_string()
            ));
        }

        Ok(Self {
            minutes: Some(minutes),
            max_count: Some(max_count),
        })
    }

    /// Get the latest reading only
    pub fn latest() -> Self {
        Self {
            minutes: Some(1440),
            max_count: Some(1),
        }
    }

    /// Get readings for the last hour
    pub fn last_hour() -> Self {
        Self {
            minutes: Some(60),
            max_count: Some(12),
        }
    }

    /// Get readings for the last 24 hours
    pub fn last_24_hours() -> Self {
        Self {
            minutes: Some(1440),
            max_count: Some(288),
        }
    }
}

impl DexcomClient {
    /// Get glucose readings with default parameters (latest reading)
    pub fn get_glucose_readings(&self) -> Result<Vec<GlucoseReading>, DexcomApiError> {
        self.get_glucose_readings_with_params(GlucoseReadingsParams::default())
    }

    /// Get glucose readings with custom parameters
    pub fn get_glucose_readings_with_params(
        &self,
        params: GlucoseReadingsParams,
    ) -> Result<Vec<GlucoseReading>, DexcomApiError> {
        if !self.has_valid_session() {
            return Err(DexcomApiError::SessionError("No valid session".to_string()));
        }

        let minutes = params.minutes.unwrap_or(1440).to_string();
        let max_count = params.max_count.unwrap_or(1).to_string();

        let query_params = vec![
            ("sessionId", self.session_id.as_ref().unwrap().as_str()),
            ("minutes", minutes.as_str()),
            ("maxCount", max_count.as_str()),
        ];

        let resp: Vec<GlucoseResp> = self
            .post(DEXCOM_GLUCOSE_DATA_ENDPOINT, json!({}), query_params)?
            .json()?;

        let mut readings = Vec::new();
        for reading in resp {
            let glucose_reading = GlucoseReading::new(
                reading.value,
                get_trend(reading.trend),
                reading.dt,
            )?;
            readings.push(glucose_reading);
        }

        Ok(readings)
    }

    /// Get the latest glucose reading
    pub fn get_latest_reading(&self) -> Result<Option<GlucoseReading>, DexcomApiError> {
        let readings = self.get_glucose_readings_with_params(GlucoseReadingsParams::latest())?;
        Ok(readings.into_iter().next())
    }

    /// Get glucose readings for the last hour
    pub fn get_readings_last_hour(&self) -> Result<Vec<GlucoseReading>, DexcomApiError> {
        self.get_glucose_readings_with_params(GlucoseReadingsParams::last_hour())
    }

    /// Get glucose readings for the last 24 hours
    pub fn get_readings_last_24_hours(&self) -> Result<Vec<GlucoseReading>, DexcomApiError> {
        self.get_glucose_readings_with_params(GlucoseReadingsParams::last_24_hours())
    }

    /// Get glucose readings since a specific time
    pub fn get_readings_since(
        &self,
        since: DateTime<Utc>,
    ) -> Result<Vec<GlucoseReading>, DexcomApiError> {
        let now = Utc::now();
        let duration = now.signed_duration_since(since);
        let minutes = duration.num_minutes() as i16;
        
        if minutes <= 0 {
            return Ok(Vec::new());
        }

        let params = GlucoseReadingsParams::new(Some(minutes), Some(288))?;
        self.get_glucose_readings_with_params(params)
    }

    /// Get average glucose for the last 24 hours
    pub fn get_average_glucose_24h(&self) -> Result<Option<f64>, DexcomApiError> {
        let readings = self.get_readings_last_24_hours()?;
        
        if readings.is_empty() {
            return Ok(None);
        }

        let sum: u32 = readings.iter().map(|r| r.mg_dl as u32).sum();
        let average = sum as f64 / readings.len() as f64;
        
        Ok(Some(average))
    }

    /// Check if glucose is in a healthy range (70-180 mg/dL)
    pub fn is_glucose_healthy(&self, mg_dl: u16) -> bool {
        mg_dl >= 70 && mg_dl <= 180
    }

    /// Get glucose status based on value
    pub fn get_glucose_status(&self, mg_dl: u16) -> &'static str {
        match mg_dl {
            0..=69 => "Low",
            70..=180 => "Normal",
            181..=250 => "High",
            _ => "Very High",
        }
    }
}
