/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: 2023-08-08T23:00:01Z
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct K8sregPeriodListClusterCloudItemResp {
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "aws_meta", skip_serializing_if = "Option::is_none")]
    pub aws_meta: Option<Box<crate::models::K8sregPeriodAwsClusterItemResp>>,
    #[serde(rename = "azure_meta", skip_serializing_if = "Option::is_none")]
    pub azure_meta: Option<Box<crate::models::K8sregPeriodAzureAcctClusterItemResp>>,
    #[serde(rename = "cloud_status")]
    pub cloud_status: String,
    #[serde(rename = "cluster_name", skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "cluster_service")]
    pub cluster_service: String,
    #[serde(rename = "cluster_status", skip_serializing_if = "Option::is_none")]
    pub cluster_status: Option<String>,
    #[serde(rename = "from_cspm", skip_serializing_if = "Option::is_none")]
    pub from_cspm: Option<bool>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl K8sregPeriodListClusterCloudItemResp {
    pub fn new(
        account_id: String,
        cloud_status: String,
        cluster_service: String,
    ) -> K8sregPeriodListClusterCloudItemResp {
        K8sregPeriodListClusterCloudItemResp {
            account_id,
            aws_meta: None,
            azure_meta: None,
            cloud_status,
            cluster_name: None,
            cluster_service,
            cluster_status: None,
            from_cspm: None,
            location: None,
        }
    }
}