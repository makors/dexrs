use reqwest::StatusCode;

use super::client::DexcomClient;

impl DexcomClient {
    pub(crate) fn post(
        &self,
        endpoint: &str,
        json: serde_json::Value,
        params: Vec<(&str, &str)>,
    ) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
        let resp = self.reqwest_client
            .post(format!("{}/{}", self.base_url, endpoint))
            .header("Accept-Encoding", "application/json")
            .json(&json)
            .query(&params)
            .send();

        match resp {
            Ok(r) => {
                if r.status() != StatusCode::OK {
                    // oh no, an error occured! (server-side)
                    // for our purposes, we can panic here
                    // but it would be best to use different error types
                    println!("Error: {:?}", r.text().unwrap());
                    return Err("server-side dexcom error".into());
                }

                Ok(r)
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}
