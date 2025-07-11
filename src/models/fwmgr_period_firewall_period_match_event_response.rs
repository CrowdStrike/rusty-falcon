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
pub struct FwmgrPeriodFirewallPeriodMatchEventResponse {
    #[serde(rename = "aid")]
    pub aid: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "command_line")]
    pub command_line: String,
    #[serde(rename = "connection_direction")]
    pub connection_direction: String,
    #[serde(rename = "domain_name_list")]
    pub domain_name_list: String,
    #[serde(rename = "event_type")]
    pub event_type: String,
    #[serde(rename = "flags")]
    pub flags: Box<models::FwmgrPeriodFirewallPeriodFlags>,
    #[serde(rename = "hidden")]
    pub hidden: bool,
    #[serde(rename = "host_name")]
    pub host_name: String,
    #[serde(rename = "icmp_code")]
    pub icmp_code: String,
    #[serde(rename = "icmp_type")]
    pub icmp_type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "image_file_name")]
    pub image_file_name: String,
    #[serde(rename = "ipv")]
    pub ipv: String,
    #[serde(rename = "local_address")]
    pub local_address: String,
    #[serde(rename = "local_port")]
    pub local_port: String,
    #[serde(rename = "match_count")]
    pub match_count: String,
    #[serde(rename = "match_count_since_last_event")]
    pub match_count_since_last_event: String,
    #[serde(rename = "network_profile")]
    pub network_profile: String,
    #[serde(rename = "pid")]
    pub pid: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "policy_id")]
    pub policy_id: String,
    #[serde(rename = "policy_name")]
    pub policy_name: String,
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "remote_address")]
    pub remote_address: String,
    #[serde(rename = "remote_port")]
    pub remote_port: String,
    #[serde(rename = "rule_action")]
    pub rule_action: String,
    #[serde(rename = "rule_description")]
    pub rule_description: String,
    #[serde(rename = "rule_family_id")]
    pub rule_family_id: String,
    #[serde(rename = "rule_group_name")]
    pub rule_group_name: String,
    #[serde(rename = "rule_id")]
    pub rule_id: String,
    #[serde(rename = "rule_name")]
    pub rule_name: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "tree_id")]
    pub tree_id: String,
}

impl FwmgrPeriodFirewallPeriodMatchEventResponse {
    pub fn new(
        aid: String,
        cid: String,
        command_line: String,
        connection_direction: String,
        domain_name_list: String,
        event_type: String,
        flags: models::FwmgrPeriodFirewallPeriodFlags,
        hidden: bool,
        host_name: String,
        icmp_code: String,
        icmp_type: String,
        id: String,
        image_file_name: String,
        ipv: String,
        local_address: String,
        local_port: String,
        match_count: String,
        match_count_since_last_event: String,
        network_profile: String,
        pid: String,
        platform: String,
        policy_id: String,
        policy_name: String,
        protocol: String,
        remote_address: String,
        remote_port: String,
        rule_action: String,
        rule_description: String,
        rule_family_id: String,
        rule_group_name: String,
        rule_id: String,
        rule_name: String,
        status: String,
        timestamp: String,
        tree_id: String,
    ) -> FwmgrPeriodFirewallPeriodMatchEventResponse {
        FwmgrPeriodFirewallPeriodMatchEventResponse {
            aid,
            cid,
            command_line,
            connection_direction,
            domain_name_list,
            event_type,
            flags: Box::new(flags),
            hidden,
            host_name,
            icmp_code,
            icmp_type,
            id,
            image_file_name,
            ipv,
            local_address,
            local_port,
            match_count,
            match_count_since_last_event,
            network_profile,
            pid,
            platform,
            policy_id,
            policy_name,
            protocol,
            remote_address,
            remote_port,
            rule_action,
            rule_description,
            rule_family_id,
            rule_group_name,
            rule_id,
            rule_name,
            status,
            timestamp,
            tree_id,
        }
    }
}
