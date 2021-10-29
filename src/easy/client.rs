use crate::apis::{Error};
use crate::apis::configuration::{Configuration};
use crate::apis::oauth2_api::{oauth2_access_token, Oauth2AccessTokenError};

pub async fn new_client(creds: Credentials) -> Result<Configuration, Error<Oauth2AccessTokenError>> {
    let mut configuration = Configuration {
        ..Default::default()
    };

    let response = oauth2_access_token(&configuration, &creds.falcon_client_id, &creds.falcon_client_secret, None).await?;

    configuration.oauth_access_token = Some(response.access_token);
    return Ok(configuration);
}

pub struct Credentials {
    pub falcon_client_id: String,
    pub falcon_client_secret: String,
}
