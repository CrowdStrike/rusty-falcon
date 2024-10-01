use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct ResponseContent<T: Debug> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

impl<T: Debug> Display for ResponseContent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?} {:?}", self.status, self.content, self.entity)
    }
}

impl<T: Debug> std::error::Error for ResponseContent<T> {}

#[derive(Debug, thiserror::Error)]
pub enum Error<T: Debug> {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Response error: {0}")]
    ResponseError(#[from] ResponseContent<T>),
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod alerts_api;
pub mod api_integrations_api;
pub mod aspm_api;
pub mod certificate_based_exclusions_api;
pub mod cloud_connect_aws_api;
pub mod cloud_snapshots_api;
pub mod compliance_assessments_api;
pub mod configuration_assessment_api;
pub mod configuration_assessment_evaluation_logic_api;
pub mod container_alerts_api;
pub mod container_detections_api;
pub mod container_images_api;
pub mod container_packages_api;
pub mod container_vulnerabilities_api;
pub mod cspg_iacapi_api;
pub mod cspm_registration_api;
pub mod custom_ioa_api;
pub mod custom_storage_api;
pub mod d4c_registration_api;
pub mod datascanner_api;
pub mod delivery_settings_api;
pub mod detects_api;
pub mod device_control_policies_api;
pub mod discover_api;
pub mod discover_iot_api;
pub mod drift_indicators_api;
pub mod event_schema_api;
pub mod event_streams_api;
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
pub mod handle_api;
pub mod host_group_api;
pub mod host_migration_api;
pub mod hosts_api;
pub mod humio_auth_proxy_api;
pub mod identity_entities_api;
pub mod identity_protection_api;
pub mod image_assessment_policies_api;
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
pub mod real_time_response_audit_api;
pub mod recon_api;
pub mod report_executions_api;
pub mod response_policies_api;
pub mod runtime_detections_api;
pub mod sample_uploads_api;
pub mod scheduled_reports_api;
pub mod sensor_download_api;
pub mod sensor_update_policies_api;
pub mod sensor_visibility_exclusions_api;
pub mod spotlight_evaluation_logic_api;
pub mod spotlight_vulnerabilities_api;
pub mod tailored_intelligence_api;
pub mod threatgraph_api;
pub mod unidentified_containers_api;
pub mod user_management_api;
pub mod workflows_api;
pub mod zero_trust_assessment_api;

pub mod configuration;
