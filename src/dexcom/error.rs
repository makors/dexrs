use std::fmt;

#[derive(Debug, Clone)]
pub enum DexcomApiError {
    LoginError(String),
    SessionError(String),
    NotSharer(String),
    HttpError(String),
    ParseError(String),
    MissingCredentials,
    InvalidAccountId,
    InvalidSessionId,
    RateLimitExceeded,
    NetworkError(String),
    TimeoutError,
    InvalidInput(String),
    Other(String),
}

impl fmt::Display for DexcomApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DexcomApiError::LoginError(msg) => write!(f, "Login error: {}", msg),
            DexcomApiError::SessionError(msg) => write!(f, "Session error: {}", msg),
            DexcomApiError::NotSharer(msg) => write!(f, "Not a sharer: {}", msg),
            DexcomApiError::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            DexcomApiError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DexcomApiError::MissingCredentials => write!(f, "Missing username or password"),
            DexcomApiError::InvalidAccountId => write!(f, "Invalid account ID returned by API"),
            DexcomApiError::InvalidSessionId => write!(f, "Invalid session ID returned by API"),
            DexcomApiError::RateLimitExceeded => write!(f, "Rate limit exceeded, please wait before retrying"),
            DexcomApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            DexcomApiError::TimeoutError => write!(f, "Request timed out"),
            DexcomApiError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            DexcomApiError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for DexcomApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<reqwest::Error> for DexcomApiError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            DexcomApiError::TimeoutError
        } else if err.is_connect() {
            DexcomApiError::NetworkError("Connection failed".to_string())
        } else {
            DexcomApiError::HttpError(err.to_string())
        }
    }
}

impl From<serde_json::Error> for DexcomApiError {
    fn from(err: serde_json::Error) -> Self {
        DexcomApiError::ParseError(err.to_string())
    }
}

impl From<std::io::Error> for DexcomApiError {
    fn from(err: std::io::Error) -> Self {
        DexcomApiError::NetworkError(err.to_string())
    }
}
