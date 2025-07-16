use super::consts::{DEXCOM_BASE_URL_NON_US, DEXCOM_BASE_URL_US};
use super::error::DexcomApiError;
use std::time::Duration;

/// Configuration for the Dexcom client
#[derive(Debug, Clone)]
pub struct DexcomConfig {
    pub timeout: Duration,
    pub retry_attempts: u32,
    pub user_agent: String,
}

impl Default for DexcomConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            retry_attempts: 3,
            user_agent: "dexrs/0.1.1".to_string(),
        }
    }
}

/// Builder for DexcomClient
pub struct DexcomClientBuilder {
    username: String,
    password: String,
    ous: bool,
    config: DexcomConfig,
}

impl DexcomClientBuilder {
    /// Create a new builder with username and password
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            ous: false,
            config: DexcomConfig::default(),
        }
    }

    /// Set whether the user is outside the US
    pub fn outside_us(mut self, ous: bool) -> Self {
        self.ous = ous;
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Set the number of retry attempts
    pub fn retry_attempts(mut self, attempts: u32) -> Self {
        self.config.retry_attempts = attempts;
        self
    }

    /// Set a custom user agent
    pub fn user_agent(mut self, user_agent: String) -> Self {
        self.config.user_agent = user_agent;
        self
    }

    /// Build the DexcomClient
    pub fn build(self) -> Result<DexcomClient, DexcomApiError> {
        DexcomClient::new_with_config(self.username, self.password, self.ous, self.config)
    }
}

#[derive(Debug)]
pub struct DexcomClient {
    pub username: String,
    pub password: String,
    pub base_url: &'static str,
    pub account_id: Option<String>,
    pub session_id: Option<String>,
    pub config: DexcomConfig,
    pub(crate) reqwest_client: reqwest::blocking::Client,
}

impl DexcomClient {
    /// Create a new DexcomClient with default configuration
    #[doc = "Create a new DexcomClient, provided a username, password, and whether or not the user is outside the US"]
    pub fn new(
        username: String,
        password: String,
        ous: bool,
    ) -> Result<DexcomClient, DexcomApiError> {
        Self::new_with_config(username, password, ous, DexcomConfig::default())
    }

    /// Create a new DexcomClient with custom configuration
    pub fn new_with_config(
        username: String,
        password: String,
        ous: bool,
        config: DexcomConfig,
    ) -> Result<DexcomClient, DexcomApiError> {
        // Validate inputs
        if username.trim().is_empty() {
            return Err(DexcomApiError::InvalidInput("Username cannot be empty".to_string()));
        }
        if password.trim().is_empty() {
            return Err(DexcomApiError::InvalidInput("Password cannot be empty".to_string()));
        }

        let base_url = if ous {
            DEXCOM_BASE_URL_NON_US
        } else {
            DEXCOM_BASE_URL_US
        };

        let reqwest_client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .map_err(|e| DexcomApiError::Other(format!("Failed to create HTTP client: {}", e)))?;

        let mut client = DexcomClient {
            username: username.trim().to_string(),
            password: password.trim().to_string(),
            base_url,
            session_id: None,
            account_id: None,
            config,
            reqwest_client,
        };

        // Initialize session
        client.create_session()?;

        Ok(client)
    }

    /// Create a builder for configuring the client
    pub fn builder(username: String, password: String) -> DexcomClientBuilder {
        DexcomClientBuilder::new(username, password)
    }

    /// Check if the client has a valid session
    pub fn has_valid_session(&self) -> bool {
        self.account_id.is_some() && self.session_id.is_some()
    }

    /// Refresh the session if needed
    pub fn refresh_session_if_needed(&mut self) -> Result<(), DexcomApiError> {
        if !self.has_valid_session() {
            self.create_session()?;
        }
        Ok(())
    }

    /// Get the current session info
    pub fn session_info(&self) -> Option<(String, String)> {
        match (&self.account_id, &self.session_id) {
            (Some(account_id), Some(session_id)) => {
                Some((account_id.clone(), session_id.clone()))
            }
            _ => None,
        }
    }
}
