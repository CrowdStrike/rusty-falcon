use crate::apis::configuration::Configuration;
use crate::apis::oauth2_api::{oauth2_access_token, Oauth2AccessTokenError};
use crate::apis::Error;
use crate::easy::cloud::FalconCloud;
use crate::easy::errors::CredentialsError;
use std::env;

pub struct FalconHandle {
    pub creds: Credentials,
    pub cfg: Configuration,
}

impl FalconHandle {
    pub async fn from_cfg(creds: Credentials) -> Result<Self, Error<Oauth2AccessTokenError>> {
        let configuration = Configuration {
            base_path: creds.falcon_cloud.base_path(),
            ..Default::default()
        };

        let mut handle = FalconHandle { creds: creds, cfg: configuration };
        handle.authenticate().await?;

        return Ok(handle);
    }

    pub async fn authenticate(&mut self) -> Result<(), Error<Oauth2AccessTokenError>> {
        let response = oauth2_access_token(&self.cfg, &self.creds.falcon_client_id, &self.creds.falcon_client_secret, self.creds.falcon_member_cid.as_ref().map(String::as_ref)).await?;
        self.cfg.oauth_access_token = Some(response.access_token);
        return Ok(());
    }
}

pub struct Credentials {
    falcon_cloud: FalconCloud,
    falcon_client_id: String,
    falcon_client_secret: String,
    falcon_member_cid: Option<String>,
}

impl Credentials {
    pub fn from_env() -> Result<Self, CredentialsError> {
        let client_id = env::var("FALCON_CLIENT_ID").map_err(|_| {
            CredentialsError(
                "Missing FALCON_CLIENT_ID environment variable. Please provide your OAuth2 API Client Secret for authentication with CrowdStrike Falcon platform. Establishing and retrieving OAuth2 API credentials can be performed at https://falcon.crowdstrike.com/support/api-clients-and-keys."
                    .to_string(),
            )
        })?;

        let client_secret = env::var("FALCON_CLIENT_SECRET").map_err(|_| {
            CredentialsError(
                "Missing FALCON_CLIENT_SECRET environment variable. Please provide your OAuth2 API Client Secret for authentication with CrowdStrike Falcon platform. Establishing and retrieving OAuth2 API credentials can be performed at https://falcon.crowdstrike.com/support/api-clients-and-keys."
                    .to_string(),
            )
        })?;
        let member_cid = env::var("FALCON_MEMBER_CID").ok();

        return Ok(Credentials {
            falcon_cloud: FalconCloud::from_env()?,
            falcon_client_id: client_id,
            falcon_client_secret: client_secret,
            falcon_member_cid: member_cid,
        });
    }
}
