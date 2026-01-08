use std::fmt;

/// Error type for SoundCloud API operations
#[derive(Debug)]
pub struct Error {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl Error {
    /// Create a new error from a message
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            source: None,
        }
    }

    /// Create an error from another error with a message
    pub fn from_error<E: std::error::Error + Send + Sync + 'static>(msg: impl Into<String>, source: E) -> Self {
        Self {
            message: msg.into(),
            source: Some(Box::new(source)),
        }
    }
}

// REQUIRED: std::error::Error requires Display to be implemented
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// REQUIRED: Makes Error usable in Result<T, E>
// OPTIONAL: source() method - we implement it to expose underlying errors for debugging
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

// OPTIONAL but CONVENIENT: Allows automatic conversion via ? operator
// Without these, you'd need: .map_err(|e| Error::new(format!("...")))? everywhere
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::from_error("HTTP request failed", err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::from_error("JSON parsing failed", err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::from_error("IO operation failed", err)
    }
}

// OPTIONAL: Convenience for creating errors from strings
impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Self::new(msg)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self::new(msg)
    }
}

