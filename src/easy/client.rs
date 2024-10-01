use crate::apis::configuration::Configuration;
use crate::apis::oauth2_api::{oauth2_access_token, Oauth2AccessTokenError};
use crate::apis::Error;
use crate::easy::cloud::FalconCloud;
use crate::error::CredentialsError;
use std::env;

#[derive(Clone)]
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

        let mut handle = FalconHandle {
            creds,
            cfg: configuration,
        };
        handle.authenticate().await?;

        Ok(handle)
    }

    pub async fn authenticate(&mut self) -> Result<(), Error<Oauth2AccessTokenError>> {
        let response = oauth2_access_token(
            &self.cfg,
            &self.creds.falcon_client_id,
            &self.creds.falcon_client_secret,
            self.creds.falcon_member_cid.as_ref().map(String::as_ref),
        )
        .await?;
        self.cfg.oauth_access_token = Some(response.access_token.unwrap_or_default());
        Ok(())
    }

    pub async fn from_env() -> Result<Self, CredentialsError> {
        Ok(FalconHandle::from_cfg(Credentials::from_env()?).await?)
    }
}

#[derive(Clone)]
pub struct Credentials {
    falcon_cloud: FalconCloud,
    falcon_client_id: String,
    falcon_client_secret: String,
    falcon_member_cid: Option<String>,
}

impl Credentials {
    pub fn from_env() -> Result<Self, CredentialsError> {
        let client_id = env::var("FALCON_CLIENT_ID").map_err(|_| CredentialsError::ClientID)?;

        let client_secret =
            env::var("FALCON_CLIENT_SECRET").map_err(|_| CredentialsError::Secret)?;
        let member_cid = env::var("FALCON_MEMBER_CID").ok();

        Ok(Credentials {
            falcon_cloud: FalconCloud::from_env()?,
            falcon_client_id: client_id,
            falcon_client_secret: client_secret,
            falcon_member_cid: member_cid,
        })
    }
}
