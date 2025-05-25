use super::consts::{DEXCOM_BASE_URL_NON_US, DEXCOM_BASE_URL_US};
use super::error::DexcomApiError;

#[derive(Debug)]
pub struct DexcomClient {
    pub username: String,
    pub password: String,
    pub base_url: &'static str,
    pub account_id: Option<String>,
    pub session_id: Option<String>,
    pub(crate) reqwest_client: reqwest::blocking::Client,
}

impl DexcomClient {
    #[doc = "Create a new DexcomClient, provided a username, password, and whether or not the user is outside the US"]
    pub fn new(
        username: String,
        password: String,
        ous: bool,
    ) -> Result<DexcomClient, DexcomApiError> {
        if username.is_empty() || password.is_empty() {
            return Err(DexcomApiError::MissingCredentials);
        }

        let mut dclient = DexcomClient {
            username,
            password,
            base_url: if ous {
                DEXCOM_BASE_URL_NON_US
            } else {
                DEXCOM_BASE_URL_US
            },
            session_id: None,
            account_id: None,
            reqwest_client: reqwest::blocking::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap(),
        };

        dclient.create_session().unwrap();

        Ok(dclient)
    }
}
