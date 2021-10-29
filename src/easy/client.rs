use std::env;
use crate::apis::{Error};
use crate::apis::configuration::{Configuration};
use crate::apis::oauth2_api::{oauth2_access_token, Oauth2AccessTokenError};
use crate::easy::errors::{CredentialsError};
use crate::easy::cloud::{FalconCloud};

pub async fn new_client(creds: Credentials) -> Result<Configuration, Error<Oauth2AccessTokenError>> {
    let mut configuration = Configuration {
        base_path: creds.falcon_cloud.base_path(),
        ..Default::default()
    };

    let response = oauth2_access_token(&configuration, &creds.falcon_client_id, &creds.falcon_client_secret, None).await?;

    configuration.oauth_access_token = Some(response.access_token);
    return Ok(configuration);
}

pub struct Credentials {
    falcon_cloud: FalconCloud,
    falcon_client_id: String,
    falcon_client_secret: String,
}

impl Credentials {
    pub fn from_env() -> Result<Self, CredentialsError> {
        let client_id = env::var("FALCON_CLIENT_ID")
            .map_err(|_| CredentialsError("Missing FALCON_CLIENT_ID environment variable. Please provide your OAuth2 API Client Secret for authentication with CrowdStrike Falcon platform. Establishing and retrieving OAuth2 API credentials can be performed at https://falcon.crowdstrike.com/support/api-clients-and-keys.".to_string()))?;

        let client_secret = env::var("FALCON_CLIENT_SECRET")
            .map_err(|_| CredentialsError("Missing FALCON_CLIENT_SECRET environment variable. Please provide your OAuth2 API Client Secret for authentication with CrowdStrike Falcon platform. Establishing and retrieving OAuth2 API credentials can be performed at https://falcon.crowdstrike.com/support/api-clients-and-keys.".to_string()))?;

        return Ok(Credentials{
            falcon_cloud: FalconCloud::from_env()?,
            falcon_client_id: client_id,
            falcon_client_secret: client_secret,
        });
    }
}
