use std::{str::FromStr, sync::OnceLock};

use regex::Regex;

use crate::dexcom::consts;
use crate::dexcom::error::DexcomApiError;

use super::trends::TrendData;

static TIMEDATE_REGEX: OnceLock<Regex> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct GlucoseReading<'a> {
    pub mg_dl: u16,
    pub mmol_l: f32,
    pub trend: TrendData<'a>,
    pub datetime: String,
}

impl GlucoseReading<'_> {
    pub fn new<'a>(
        mg_dl: u16,
        trend: TrendData<'a>,
        datetime: String,
    ) -> Result<GlucoseReading<'a>, DexcomApiError> {
        let time_regex = TIMEDATE_REGEX.get_or_init(|| {
            Regex::new(r"Date\((?P<timestamp>\d+)(?P<timezone>[+-]\d{4})\)")
                .expect("Failed to compile datetime regex")
        });

        match time_regex.captures(&datetime) {
            Some(c) => {
                // amount of seconds (converting from ms to s)
                let timestamp_str = c.name("timestamp")
                    .ok_or_else(|| DexcomApiError::ParseError("Missing timestamp in datetime".to_string()))?
                    .as_str();
                
                let seconds = timestamp_str.parse::<i64>()
                    .map_err(|e| DexcomApiError::ParseError(format!("Failed to parse timestamp: {}", e)))?
                    / 1000;

                // UTC offset (ex. -0400 for EDT)
                let timezone = c.name("timezone")
                    .ok_or_else(|| DexcomApiError::ParseError("Missing timezone in datetime".to_string()))?
                    .as_str();

                // the main timestamp (in UTC, without the offset applied yet)
                let timestamp = chrono::DateTime::from_timestamp(seconds, 0)
                    .ok_or_else(|| DexcomApiError::ParseError("Invalid timestamp".to_string()))?;

                // Parse mmol/l value with proper error handling
                let mmol_l = format!("{:.1}", mg_dl as f32 * consts::MMOL_CONVERSION_FACTOR)
                    .parse::<f32>()
                    .map_err(|e| DexcomApiError::ParseError(format!("Failed to parse mmol/l value: {}", e)))?;

                // Parse timezone offset
                let timezone_offset = chrono::FixedOffset::from_str(timezone)
                    .map_err(|e| DexcomApiError::ParseError(format!("Failed to parse timezone offset: {}", e)))?;

                Ok(GlucoseReading {
                    mg_dl,
                    mmol_l,
                    trend,
                    datetime: timestamp
                        .with_timezone(&timezone_offset)
                        .format("%Y-%m-%dT%H:%M:%S%Z")
                        .to_string(), // readable format for JS
                })
            }
            None => Err(DexcomApiError::ParseError("invalid datetime format".to_string())),
        }
    }
}
