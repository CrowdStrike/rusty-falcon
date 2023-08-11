use crate::easy::errors::CredentialsError;
use std::{env, str::FromStr};

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
            Self::Us1 => "api.crowdstrike.com",
            Self::Us2 => "api.us-2.crowdstrike.com",
            Self::Eu1 => "api.eu-1.crowdstrike.com",
            Self::UsGov1 => "api.laggar.gcw.crowdstrike.com",
        }
    }

    pub fn base_path(self) -> String {
        format!("https://{}", self.host())
    }

    pub fn from_env() -> Result<Self, CredentialsError> {
        let cloud_str = env::var("FALCON_CLOUD").map_err(|_| CredentialsError("Missing FALCON_CLOUD environment variable. Please provide your Falcon Cloud region".to_string()))?;
        Self::from_str(cloud_str.as_str())
    }
}

impl FromStr for FalconCloud {
    type Err = CredentialsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "us-1" => Ok(Self::Us1),
            "us-2" => Ok(Self::Us2),
            "eu-1" => Ok(Self::Eu1),
            "us-gov-1" => Ok(Self::UsGov1),
            _ => Err(CredentialsError(format!("Invalid FALCON_CLOUD specifier: '{s}'. Supported values are: us-1, us-2, eu-1, us-gov-1"))),
        }
    }
}
