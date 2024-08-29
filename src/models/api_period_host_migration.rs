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
pub struct ApiPeriodHostMigration {
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<models::ApiPeriodEvent>>,
    #[serde(rename = "host_migration_id")]
    pub host_migration_id: String,
    /// assigned static hostgroups, may need more details here
    #[serde(rename = "hostgroups")]
    pub hostgroups: Vec<String>,
    /// hostname at the time of migration
    #[serde(rename = "hostname")]
    pub hostname: String,
    #[serde(rename = "migration_id")]
    pub migration_id: String,
    /// platform at the time of migration
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "source_cid")]
    pub source_cid: String,
    /// device_id in the source cid
    #[serde(rename = "source_device_id")]
    pub source_device_id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "status_details")]
    pub status_details: String,
    /// not sure if this is necessary since it's common
    #[serde(rename = "target_cid")]
    pub target_cid: String,
    /// device_id in the target cid. This may change while the migration is incomplete.
    #[serde(rename = "target_device_id")]
    pub target_device_id: String,
    #[serde(rename = "updated_time")]
    pub updated_time: String,
}

impl ApiPeriodHostMigration {
    pub fn new(
        created_time: String,
        host_migration_id: String,
        hostgroups: Vec<String>,
        hostname: String,
        migration_id: String,
        platform: String,
        source_cid: String,
        source_device_id: String,
        status: String,
        status_details: String,
        target_cid: String,
        target_device_id: String,
        updated_time: String,
    ) -> ApiPeriodHostMigration {
        ApiPeriodHostMigration {
            created_time,
            events: None,
            host_migration_id,
            hostgroups,
            hostname,
            migration_id,
            platform,
            source_cid,
            source_device_id,
            status,
            status_details,
            target_cid,
            target_device_id,
            updated_time,
        }
    }
}