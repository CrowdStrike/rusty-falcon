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

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod activity_monitor_api;
pub mod alerts_api;
pub mod api_integrations_api;
pub mod aspm_api;
pub mod cao_hunting_api;
pub mod certificate_based_exclusions_api;
pub mod cloud_aws_registration_api;
pub mod cloud_azure_registration_api;
pub mod cloud_connect_aws_api;
pub mod cloud_oci_registration_api;
pub mod cloud_security_assets_api;
pub mod cloud_snapshots_api;
pub mod configuration_assessment_api;
pub mod configuration_assessment_evaluation_logic_api;
pub mod container_alerts_api;
pub mod container_detections_api;
pub mod container_image_compliance_api;
pub mod container_images_api;
pub mod container_packages_api;
pub mod container_vulnerabilities_api;
pub mod content_update_policies_api;
pub mod correlation_rules_api;
pub mod cspg_iacapi_api;
pub mod cspm_registration_api;
pub mod custom_ioa_api;
pub mod custom_storage_api;
pub mod d4c_registration_api;
pub mod default_api;
pub mod delivery_settings_api;
pub mod deployments_api;
pub mod detects_api;
pub mod device_content_api;
pub mod device_control_policies_api;
pub mod device_control_with_bluetooth_api;
pub mod discover_api;
pub mod discover_iot_api;
pub mod downloads_api_api;
pub mod drift_indicators_api;
pub mod event_schema_api;
pub mod event_streams_api;
pub mod execution_api;
pub mod exposure_management_api;
pub mod falcon_complete_dashboard_api;
pub mod falcon_container_api;
pub mod falcon_container_cli_api;
pub mod falcon_container_image_api;
pub mod falconx_sandbox_api;
pub mod field_schema_api;
pub mod filevantage_api;
pub mod firewall_management_api;
pub mod firewall_policies_api;
pub mod foundry_logscale_api;
pub mod host_group_api;
pub mod host_migration_api;
pub mod hosts_api;
pub mod identity_entities_api;
pub mod identity_protection_api;
pub mod image_assessment_policies_api;
pub mod incidents_api;
pub mod installation_tokens_api;
pub mod installation_tokens_settings_api;
pub mod integration_builder_api;
pub mod intel_api;
pub mod intelligence_feeds_api;
pub mod intelligence_indicator_graph_api;
pub mod inventories_api;
pub mod ioa_exclusions_api;
pub mod ioc_api;
pub mod iocs_api;
pub mod kubernetes_container_compliance_api;
pub mod kubernetes_protection_api;
pub mod lookup_files_api;
pub mod malquery_api;
pub mod message_center_api;
pub mod ml_exclusions_api;
pub mod mobile_enrollment_api;
pub mod mssp_api;
pub mod ngsiem_api;
pub mod oauth2_api;
pub mod ods_api;
pub mod overwatch_dashboard_api;
pub mod prevention_policies_api;
pub mod quarantine_api;
pub mod quick_scan_api;
pub mod quick_scan_pro_api;
pub mod real_time_response_admin_api;
pub mod real_time_response_api;
pub mod real_time_response_audit_api;
pub mod recon_api;
pub mod release_notes_api;
pub mod releases_api;
pub mod report_executions_api;
pub mod response_policies_api;
pub mod runtime_detections_api;
pub mod saa_s_api;
pub mod sample_uploads_api;
pub mod scheduled_reports_api;
pub mod security_check_api;
pub mod sensor_download_api;
pub mod sensor_update_policies_api;
pub mod sensor_usage_api_api;
pub mod sensor_visibility_exclusions_api;
pub mod serverless_vulnerabilities_api;
pub mod spotlight_evaluation_logic_api;
pub mod spotlight_vulnerabilities_api;
pub mod system_api;
pub mod tailored_intelligence_api;
pub mod threatgraph_api;
pub mod unidentified_containers_api;
pub mod user_management_api;
pub mod workflows_api;
pub mod zero_trust_assessment_api;

pub mod configuration;
