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
pub struct DetectsPeriodDeviceDetailIndexed {
    #[serde(rename = "agent_load_flags", skip_serializing_if = "Option::is_none")]
    pub agent_load_flags: Option<String>,
    #[serde(rename = "agent_local_time", skip_serializing_if = "Option::is_none")]
    pub agent_local_time: Option<String>,
    #[serde(rename = "agent_version", skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "bios_manufacturer", skip_serializing_if = "Option::is_none")]
    pub bios_manufacturer: Option<String>,
    #[serde(rename = "bios_version", skip_serializing_if = "Option::is_none")]
    pub bios_version: Option<String>,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "config_id_base", skip_serializing_if = "Option::is_none")]
    pub config_id_base: Option<String>,
    #[serde(rename = "config_id_build", skip_serializing_if = "Option::is_none")]
    pub config_id_build: Option<String>,
    #[serde(rename = "config_id_platform", skip_serializing_if = "Option::is_none")]
    pub config_id_platform: Option<String>,
    #[serde(rename = "device_id")]
    pub device_id: String,
    #[serde(rename = "external_ip", skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    #[serde(
        rename = "first_login_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub first_login_timestamp: Option<String>,
    #[serde(rename = "first_login_user", skip_serializing_if = "Option::is_none")]
    pub first_login_user: Option<String>,
    #[serde(rename = "first_seen", skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<String>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "instance_id", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(
        rename = "last_login_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_login_timestamp: Option<String>,
    #[serde(rename = "last_login_user", skip_serializing_if = "Option::is_none")]
    pub last_login_user: Option<String>,
    #[serde(rename = "last_seen", skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    #[serde(rename = "local_ip", skip_serializing_if = "Option::is_none")]
    pub local_ip: Option<String>,
    #[serde(rename = "mac_address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "machine_domain", skip_serializing_if = "Option::is_none")]
    pub machine_domain: Option<String>,
    #[serde(rename = "major_version", skip_serializing_if = "Option::is_none")]
    pub major_version: Option<String>,
    #[serde(rename = "minor_version", skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<String>,
    #[serde(rename = "modified_timestamp", skip_serializing_if = "Option::is_none")]
    pub modified_timestamp: Option<String>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    #[serde(rename = "os_version", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "ou", skip_serializing_if = "Option::is_none")]
    pub ou: Option<Vec<String>>,
    #[serde(rename = "platform_id", skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "platform_name", skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(rename = "pod_id", skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,
    #[serde(rename = "pod_labels", skip_serializing_if = "Option::is_none")]
    pub pod_labels: Option<Vec<String>>,
    #[serde(rename = "pod_name", skip_serializing_if = "Option::is_none")]
    pub pod_name: Option<String>,
    #[serde(rename = "pod_namespace", skip_serializing_if = "Option::is_none")]
    pub pod_namespace: Option<String>,
    #[serde(
        rename = "pod_service_account_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_service_account_name: Option<String>,
    #[serde(rename = "product_type", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(rename = "product_type_desc", skip_serializing_if = "Option::is_none")]
    pub product_type_desc: Option<String>,
    #[serde(rename = "release_group", skip_serializing_if = "Option::is_none")]
    pub release_group: Option<String>,
    #[serde(rename = "service_provider", skip_serializing_if = "Option::is_none")]
    pub service_provider: Option<String>,
    #[serde(
        rename = "service_provider_account_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_provider_account_id: Option<String>,
    #[serde(rename = "site_name", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(
        rename = "system_manufacturer",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_manufacturer: Option<String>,
    #[serde(
        rename = "system_product_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_product_name: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl DetectsPeriodDeviceDetailIndexed {
    pub fn new(cid: String, device_id: String) -> DetectsPeriodDeviceDetailIndexed {
        DetectsPeriodDeviceDetailIndexed {
            agent_load_flags: None,
            agent_local_time: None,
            agent_version: None,
            bios_manufacturer: None,
            bios_version: None,
            cid,
            config_id_base: None,
            config_id_build: None,
            config_id_platform: None,
            device_id,
            external_ip: None,
            first_login_timestamp: None,
            first_login_user: None,
            first_seen: None,
            groups: None,
            hostname: None,
            instance_id: None,
            last_login_timestamp: None,
            last_login_user: None,
            last_seen: None,
            local_ip: None,
            mac_address: None,
            machine_domain: None,
            major_version: None,
            minor_version: None,
            modified_timestamp: None,
            notes: None,
            os_version: None,
            ou: None,
            platform_id: None,
            platform_name: None,
            pod_id: None,
            pod_labels: None,
            pod_name: None,
            pod_namespace: None,
            pod_service_account_name: None,
            product_type: None,
            product_type_desc: None,
            release_group: None,
            service_provider: None,
            service_provider_account_id: None,
            site_name: None,
            status: None,
            system_manufacturer: None,
            system_product_name: None,
            tags: None,
        }
    }
}
