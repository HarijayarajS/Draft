#[derive(Debug)]
pub enum RestError {
    Error(String),
}

impl RestError {
    pub fn error<T>(msg: &str) -> Result<T, Self> {
        Err(RestError::Error(msg.to_string()))
    }
}

// Optional alias for clarity
type RestResult<T> = Result<T, RestError>;