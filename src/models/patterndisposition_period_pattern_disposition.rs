/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PatterndispositionPeriodPatternDisposition {
    #[serde(rename = "blocking_unsupported_or_disabled")]
    pub blocking_unsupported_or_disabled: bool,
    #[serde(rename = "bootup_safeguard_enabled")]
    pub bootup_safeguard_enabled: bool,
    #[serde(rename = "critical_process_disabled")]
    pub critical_process_disabled: bool,
    #[serde(rename = "detect")]
    pub detect: bool,
    #[serde(rename = "fs_operation_blocked")]
    pub fs_operation_blocked: bool,
    #[serde(rename = "handle_operation_downgraded")]
    pub handle_operation_downgraded: bool,
    #[serde(rename = "inddet_mask")]
    pub inddet_mask: bool,
    #[serde(rename = "indicator")]
    pub indicator: bool,
    #[serde(rename = "kill_action_failed")]
    pub kill_action_failed: bool,
    #[serde(rename = "kill_parent")]
    pub kill_parent: bool,
    #[serde(rename = "kill_process")]
    pub kill_process: bool,
    #[serde(rename = "kill_subprocess")]
    pub kill_subprocess: bool,
    #[serde(rename = "operation_blocked")]
    pub operation_blocked: bool,
    #[serde(rename = "policy_disabled")]
    pub policy_disabled: bool,
    #[serde(rename = "process_blocked")]
    pub process_blocked: bool,
    #[serde(rename = "quarantine_file")]
    pub quarantine_file: bool,
    #[serde(rename = "quarantine_machine")]
    pub quarantine_machine: bool,
    #[serde(rename = "registry_operation_blocked")]
    pub registry_operation_blocked: bool,
    #[serde(rename = "rooting")]
    pub rooting: bool,
    #[serde(rename = "sensor_only")]
    pub sensor_only: bool,
    #[serde(rename = "suspend_parent")]
    pub suspend_parent: bool,
    #[serde(rename = "suspend_process")]
    pub suspend_process: bool,
}

impl PatterndispositionPeriodPatternDisposition {
    pub fn new(
        blocking_unsupported_or_disabled: bool,
        bootup_safeguard_enabled: bool,
        critical_process_disabled: bool,
        detect: bool,
        fs_operation_blocked: bool,
        handle_operation_downgraded: bool,
        inddet_mask: bool,
        indicator: bool,
        kill_action_failed: bool,
        kill_parent: bool,
        kill_process: bool,
        kill_subprocess: bool,
        operation_blocked: bool,
        policy_disabled: bool,
        process_blocked: bool,
        quarantine_file: bool,
        quarantine_machine: bool,
        registry_operation_blocked: bool,
        rooting: bool,
        sensor_only: bool,
        suspend_parent: bool,
        suspend_process: bool,
    ) -> PatterndispositionPeriodPatternDisposition {
        PatterndispositionPeriodPatternDisposition {
            blocking_unsupported_or_disabled,
            bootup_safeguard_enabled,
            critical_process_disabled,
            detect,
            fs_operation_blocked,
            handle_operation_downgraded,
            inddet_mask,
            indicator,
            kill_action_failed,
            kill_parent,
            kill_process,
            kill_subprocess,
            operation_blocked,
            policy_disabled,
            process_blocked,
            quarantine_file,
            quarantine_machine,
            registry_operation_blocked,
            rooting,
            sensor_only,
            suspend_parent,
            suspend_process,
        }
    }
}
