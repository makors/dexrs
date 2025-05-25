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
            Regex::new(r"Date\((?P<timestamp>\d+)(?P<timezone>[+-]\d{4})\)").unwrap()
        });

        match time_regex.captures(&datetime) {
            Some(c) => {
                // amount of seconds (converting from ms to s)
                let seconds = c
                    .name("timestamp")
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap()
                    / 1000;

                // UTC offset (ex. -0400 for EDT)
                let timezone = c.name("timezone").unwrap().as_str();

                // the main timestamp (in UTC, without the offset applied yet)
                let timestamp = chrono::DateTime::from_timestamp(seconds, 0).unwrap();

                Ok(GlucoseReading {
                    mg_dl,
                    mmol_l: format!("{:.1}", mg_dl as f32 * consts::MMOL_CONVERSION_FACTOR) // Round to one decimal place
                        .parse() // turn it into a f32
                        .unwrap(), // panic on error
                    trend,
                    datetime: timestamp
                        .with_timezone(&chrono::FixedOffset::from_str(timezone).unwrap())
                        .format("%Y-%m-%dT%H:%M:%S%Z")
                        .to_string(), // readable format for JS
                })
            }
            None => Err(DexcomApiError::ParseError("invalid datetime format".to_string())),
        }
    }
}
