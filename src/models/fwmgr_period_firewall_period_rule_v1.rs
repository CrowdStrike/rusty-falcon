/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */
use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FwmgrPeriodFirewallPeriodRuleV1 {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "address_family")]
    pub address_family: String,
    #[serde(rename = "created_by")]
    pub created_by: String,
    #[serde(rename = "created_on")]
    pub created_on: String,
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "direction")]
    pub direction: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "family")]
    pub family: String,
    #[serde(rename = "fields")]
    pub fields: Vec<models::FwmgrPeriodFirewallPeriodFieldValue>,
    #[serde(rename = "fqdn")]
    pub fqdn: String,
    #[serde(rename = "fqdn_enabled")]
    pub fqdn_enabled: bool,
    #[serde(rename = "icmp")]
    pub icmp: Box<models::FwmgrPeriodFirewallPeriodIcmp>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "local_address")]
    pub local_address: Vec<models::FwmgrPeriodFirewallPeriodAddressRange>,
    #[serde(rename = "local_port")]
    pub local_port: Vec<models::FwmgrPeriodFirewallPeriodPortRange>,
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(rename = "modified_on", skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,
    #[serde(rename = "monitor")]
    pub monitor: Box<models::FwmgrPeriodFirewallPeriodMonitoring>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "platform_ids")]
    pub platform_ids: Vec<String>,
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "remote_address")]
    pub remote_address: Vec<models::FwmgrPeriodFirewallPeriodAddressRange>,
    #[serde(rename = "remote_port")]
    pub remote_port: Vec<models::FwmgrPeriodFirewallPeriodPortRange>,
    #[serde(rename = "rule_group")]
    pub rule_group: Box<models::FwmgrPeriodFirewallPeriodRuleGroupSummaryV1>,
    #[serde(rename = "version")]
    pub version: i64,
}

impl FwmgrPeriodFirewallPeriodRuleV1 {
    pub fn new(
        action: String,
        address_family: String,
        created_by: String,
        created_on: String,
        deleted: bool,
        description: String,
        direction: String,
        enabled: bool,
        family: String,
        fields: Vec<models::FwmgrPeriodFirewallPeriodFieldValue>,
        fqdn: String,
        fqdn_enabled: bool,
        icmp: models::FwmgrPeriodFirewallPeriodIcmp,
        id: String,
        local_address: Vec<models::FwmgrPeriodFirewallPeriodAddressRange>,
        local_port: Vec<models::FwmgrPeriodFirewallPeriodPortRange>,
        monitor: models::FwmgrPeriodFirewallPeriodMonitoring,
        name: String,
        platform_ids: Vec<String>,
        protocol: String,
        remote_address: Vec<models::FwmgrPeriodFirewallPeriodAddressRange>,
        remote_port: Vec<models::FwmgrPeriodFirewallPeriodPortRange>,
        rule_group: models::FwmgrPeriodFirewallPeriodRuleGroupSummaryV1,
        version: i64,
    ) -> FwmgrPeriodFirewallPeriodRuleV1 {
        FwmgrPeriodFirewallPeriodRuleV1 {
            action,
            address_family,
            created_by,
            created_on,
            customer_id: None,
            deleted,
            description,
            direction,
            enabled,
            family,
            fields,
            fqdn,
            fqdn_enabled,
            icmp: Box::new(icmp),
            id,
            local_address,
            local_port,
            modified_by: None,
            modified_on: None,
            monitor: Box::new(monitor),
            name,
            platform_ids,
            protocol,
            remote_address,
            remote_port,
            rule_group: Box::new(rule_group),
            version,
        }
    }
}
