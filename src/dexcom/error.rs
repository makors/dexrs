use std::fmt;

#[derive(Debug)]
pub enum DexcomApiError {
    LoginError(String),
    SessionError(String),
    NotSharer(String),
    HttpError(String),
    ParseError(String),
    MissingCredentials,
    InvalidAccountId,
    InvalidSessionId,
    Other(String),
}

impl fmt::Display for DexcomApiError {
    // our function to actually print it out
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
            DexcomApiError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for DexcomApiError {}
