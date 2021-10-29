use std::env;
use crate::easy::errors::{CredentialsError};

#[derive(Debug, Clone, Copy)]
pub enum FalconCloud {
    Us1,
    Us2,
    Eu1,
    UsGov1,
}

impl FalconCloud {
    fn host(self) -> &'static str {
        match self {
            FalconCloud::Us1 => "api.crowdstrike.com",
            FalconCloud::Us2 => "api.us-2.crowdstrike.com",
            FalconCloud::Eu1 => "api.eu-1.crowdstrike.com",
            FalconCloud::UsGov1 => "api.laggar.gcw.crowdstrike.com"
        }
    }
    pub fn base_path(self) -> String {
        return format!("https://{}", self.host());
    }

    pub fn from_env() -> Result<Self, CredentialsError> {
        let cloud_str = env::var("FALCON_CLOUD")
            .map_err(|_| CredentialsError("Missing FALCON_CLOUD environment variable. Please provide your Falcon Cloud region".to_string()))?;
        return Self::from_str(cloud_str);
    }

    pub fn from_str(cloud: String) -> Result<Self, CredentialsError> {
        return match cloud.as_str() {
            "us-1" => Ok(FalconCloud::Us1),
            "us-2" => Ok(FalconCloud::Us2),
            "eu-1" => Ok(FalconCloud::Eu1),
            "us-gov-1" => Ok(FalconCloud::UsGov1),
            _ => Err(CredentialsError(format!("Invalid FALCON_CLOUD specifier: '{}'. Supported values are: us-1, us-2, eu-1, us-gov-1", cloud))),
        }
    }
}
