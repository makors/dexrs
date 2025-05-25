use serde_json::json;

use crate::{
    reading::GlucoseReading,
    trends::get_trend,
};

use super::{client::DexcomClient, consts::DEXCOM_GLUCOSE_DATA_ENDPOINT};
use super::error::DexcomApiError;

use serde::Deserialize;

#[derive(Deserialize)]
struct GlucoseResp {
    #[serde(rename = "DT")]
    dt: String,
    #[serde(rename = "Value")]
    value: u16,
    #[serde(rename = "Trend")]
    trend: String,
}

impl DexcomClient {
    #[doc = "Gets glucose readings, returns a vector, max minutes is 1440 and max count is 288"]
    pub fn get_glucose_readings(
        &self,
        mut minutes: Option<i16>,
        mut max_count: Option<i16>,
    ) -> Result<Vec<GlucoseReading>, DexcomApiError> {
        // guard clauses
        if minutes > Some(1440) || minutes < Some(1) || minutes.is_none() {
            minutes = Some(1440);
        }

        if max_count > Some(288) || max_count < Some(1) || max_count.is_none() {
            max_count = Some(1);
        }

        if self.account_id.is_none() || self.session_id.is_none() {
            return Err(DexcomApiError::SessionError("Session ID or Account ID is missing".to_string()));
        }

        let minutes = minutes.unwrap().to_string();
        let max_count = max_count.unwrap().to_string();

        let mut readings: Vec<GlucoseReading> = vec![];
        let params = vec![
            ("sessionId", self.session_id.as_ref().unwrap().as_str()),
            ("minutes", minutes.as_str()),
            ("maxCount", max_count.as_str()),
        ];

        let resp: Vec<GlucoseResp> = self
            .post(DEXCOM_GLUCOSE_DATA_ENDPOINT, json!({}), params)
            .map_err(|e| DexcomApiError::HttpError(e.to_string()))?
            .json()
            .map_err(|e| DexcomApiError::ParseError(e.to_string()))?;

        for reading in resp {
            readings.push(
                GlucoseReading::new(reading.value, get_trend(reading.trend), reading.dt)
                    .map_err(|e| DexcomApiError::ParseError(e.to_string()))?,
            );
        }

        Ok(readings)
    }
}
