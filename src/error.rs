use crate::apis::oauth2_api::Oauth2AccessTokenError;

#[derive(Debug, thiserror::Error)]
pub enum CredentialsError {
    #[error(
        "Missing FALCON_CLIENT_ID environment variable. Please provide your OAuth2 API Client Secret for authentication with CrowdStrike Falcon platform. Establishing and retrieving OAuth2 API credentials can be performed at https://falcon.crowdstrike.com/support/api-clients-and-keys."
    )]
    ClientID,
    #[error(
        "Missing FALCON_CLIENT_SECRET environment variable. Please provide your OAuth2 API Client Secret for authentication with CrowdStrike Falcon platform. Establishing and retrieving OAuth2 API credentials can be performed at https://falcon.crowdstrike.com/support/api-clients-and-keys."
    )]
    Secret,
    #[error(
        "Invalid FALCON_CLOUD specifier: '{0}'. Supported values are: us-1, us-2, eu-1, us-gov-1"
    )]
    Cloud(String),
    #[error("FALCON_CLOUD env variable is not set")]
    CloudEnv,
    #[error("Oauth error: {0}")]
    Oauth(#[from] crate::apis::Error<Oauth2AccessTokenError>),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Credentials error: {0}")]
    Credentials(#[from] CredentialsError),
}
