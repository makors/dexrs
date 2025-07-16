use super::client::DexcomClient;
use super::error::DexcomApiError;
use std::time::Duration;

impl DexcomClient {
    /// Make a POST request with retry logic
    pub(crate) fn post(
        &self,
        endpoint: &str,
        json: serde_json::Value,
        params: Vec<(&str, &str)>,
    ) -> Result<reqwest::blocking::Response, DexcomApiError> {
        self.post_with_retry(endpoint, json, params, 0)
    }

    /// Make a POST request with retry logic
    fn post_with_retry(
        &self,
        endpoint: &str,
        json: serde_json::Value,
        params: Vec<(&str, &str)>,
        attempt: u32,
    ) -> Result<reqwest::blocking::Response, DexcomApiError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        let response = self.reqwest_client
            .post(&url)
            .header("Accept-Encoding", "application/json")
            .header("Content-Type", "application/json")
            .json(&json)
            .query(&params)
            .send();

        match response {
            Ok(resp) => {
                let status = resp.status();
                
                // Handle different status codes
                match status.as_u16() {
                    200 => Ok(resp),
                    401 => Err(DexcomApiError::LoginError("Unauthorized - check credentials".to_string())),
                    403 => Err(DexcomApiError::NotSharer("Access forbidden - user may not be a sharer".to_string())),
                    429 => {
                        if attempt < self.config.retry_attempts {
                            // Wait before retrying (exponential backoff)
                            let delay = Duration::from_secs(2_u64.pow(attempt));
                            std::thread::sleep(delay);
                            return self.post_with_retry(endpoint, json, params, attempt + 1);
                        } else {
                            Err(DexcomApiError::RateLimitExceeded)
                        }
                    }
                    500..=599 => {
                        if attempt < self.config.retry_attempts {
                            // Wait before retrying (exponential backoff)
                            let delay = Duration::from_secs(2_u64.pow(attempt));
                            std::thread::sleep(delay);
                            return self.post_with_retry(endpoint, json, params, attempt + 1);
                        } else {
                            let err_text = resp.text().unwrap_or_else(|_| "Server error".to_string());
                            Err(DexcomApiError::HttpError(format!("Server error ({}): {}", status, err_text)))
                        }
                    }
                    _ => {
                        let err_text = resp.text().unwrap_or_else(|_| "Unknown error".to_string());
                        Err(DexcomApiError::HttpError(format!("HTTP {}: {}", status, err_text)))
                    }
                }
            },
            Err(e) => {
                if attempt < self.config.retry_attempts && (e.is_timeout() || e.is_connect()) {
                    // Wait before retrying (exponential backoff)
                    let delay = Duration::from_secs(2_u64.pow(attempt));
                    std::thread::sleep(delay);
                    return self.post_with_retry(endpoint, json, params, attempt + 1);
                } else {
                    Err(e.into())
                }
            }
        }
    }

    /// Make a GET request (for future use)
    pub(crate) fn get(
        &self,
        endpoint: &str,
        params: Vec<(&str, &str)>,
    ) -> Result<reqwest::blocking::Response, DexcomApiError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        let response = self.reqwest_client
            .get(&url)
            .header("Accept-Encoding", "application/json")
            .query(&params)
            .send();

        match response {
            Ok(resp) => {
                let status = resp.status();
                
                match status.as_u16() {
                    200 => Ok(resp),
                    401 => Err(DexcomApiError::LoginError("Unauthorized - check credentials".to_string())),
                    403 => Err(DexcomApiError::NotSharer("Access forbidden - user may not be a sharer".to_string())),
                    429 => Err(DexcomApiError::RateLimitExceeded),
                    _ => {
                        let err_text = resp.text().unwrap_or_else(|_| "Unknown error".to_string());
                        Err(DexcomApiError::HttpError(format!("HTTP {}: {}", status, err_text)))
                    }
                }
            },
            Err(e) => Err(e.into())
        }
    }
}
