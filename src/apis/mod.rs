use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod alerts_api;
pub mod cloud_connect_aws_api;
pub mod cspm_registration_api;
pub mod custom_ioa_api;
pub mod d4c_registration_api;
pub mod detects_api;
pub mod device_control_policies_api;
pub mod discover_api;
pub mod event_streams_api;
pub mod falcon_complete_dashboard_api;
pub mod falcon_container_api;
pub mod falcon_container_cli_api;
pub mod falconx_sandbox_api;
pub mod filevantage_api;
pub mod firewall_management_api;
pub mod firewall_policies_api;
pub mod host_group_api;
pub mod hosts_api;
pub mod identity_protection_api;
pub mod incidents_api;
pub mod installation_tokens_api;
pub mod installation_tokens_settings_api;
pub mod intel_api;
pub mod ioa_exclusions_api;
pub mod ioc_api;
pub mod iocs_api;
pub mod kubernetes_protection_api;
pub mod malquery_api;
pub mod message_center_api;
pub mod ml_exclusions_api;
pub mod mobile_enrollment_api;
pub mod mssp_api;
pub mod oauth2_api;
pub mod ods_api;
pub mod overwatch_dashboard_api;
pub mod prevention_policies_api;
pub mod quarantine_api;
pub mod quick_scan_api;
pub mod real_time_response_admin_api;
pub mod real_time_response_api;
pub mod recon_api;
pub mod report_executions_api;
pub mod response_policies_api;
pub mod sample_uploads_api;
pub mod scheduled_reports_api;
pub mod sensor_download_api;
pub mod sensor_update_policies_api;
pub mod sensor_visibility_exclusions_api;
pub mod spotlight_evaluation_logic_api;
pub mod spotlight_vulnerabilities_api;
pub mod tailored_intelligence_api;
pub mod user_management_api;
pub mod zero_trust_assessment_api;

pub mod configuration;
