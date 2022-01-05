#[derive(Debug, Clone)]
pub struct CredentialsError(pub String);

impl std::fmt::Display for CredentialsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Application Errors: {}", self.to_string())
    }
}

impl std::error::Error for CredentialsError {}
