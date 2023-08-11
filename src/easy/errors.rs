#[derive(Debug, Clone)]
pub struct CredentialsError(pub String);

impl std::fmt::Display for CredentialsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Application Errors: {}", self.0)
    }
}

impl std::error::Error for CredentialsError {}
