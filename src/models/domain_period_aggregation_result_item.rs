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
pub struct DomainPeriodAggregationResultItem {
    /// count of the documents in the bucket
    #[serde(rename = "count")]
    pub count: i64,
    /// numerical value of the date or number start of the range
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<f64>,
    /// string value of the key, usually not populated
    #[serde(rename = "key_as_string", skip_serializing_if = "Option::is_none")]
    pub key_as_string: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<serde_json::Value>,
    /// string value of the from property, usually representing a date or number
    #[serde(rename = "string_from", skip_serializing_if = "Option::is_none")]
    pub string_from: Option<String>,
    /// string value of the to property, usually representing a date or number
    #[serde(rename = "string_to", skip_serializing_if = "Option::is_none")]
    pub string_to: Option<String>,
    /// sub-aggregations of the bucket
    #[serde(rename = "sub_aggregates", skip_serializing_if = "Option::is_none")]
    pub sub_aggregates: Option<Vec<models::DomainPeriodAggregationResult>>,
    /// numerical value of the date or number end of the range
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<f64>,
    /// numerical value of the bucket
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// value as a string, usually not populated
    #[serde(rename = "value_as_string", skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}

impl DomainPeriodAggregationResultItem {
    pub fn new(count: i64) -> DomainPeriodAggregationResultItem {
        DomainPeriodAggregationResultItem {
            count,
            from: None,
            key_as_string: None,
            label: None,
            string_from: None,
            string_to: None,
            sub_aggregates: None,
            to: None,
            value: None,
            value_as_string: None,
        }
    }
}