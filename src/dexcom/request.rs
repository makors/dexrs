use super::client::DexcomClient;
use super::error::DexcomApiError;

impl DexcomClient {
    pub(crate) fn post(
        &self,
        endpoint: &str,
        json: serde_json::Value,
        params: Vec<(&str, &str)>,
    ) -> Result<reqwest::blocking::Response, DexcomApiError> {
        let resp = self.reqwest_client
            .post(format!("{}/{}", self.base_url, endpoint))
            .header("Accept-Encoding", "application/json")
            .json(&json)
            .query(&params)
            .send();
        match resp {
            Ok(r) => {
                if r.status() != reqwest::StatusCode::OK {
                    let err_text = r.text().unwrap_or_else(|_| "Unknown server error".to_string());
                    return Err(DexcomApiError::HttpError(err_text));
                }
                Ok(r)
            },
            Err(e) => Err(DexcomApiError::HttpError(e.to_string())),
        }
    }
}
