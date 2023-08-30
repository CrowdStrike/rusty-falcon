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
pub struct ApiPeriodNotificationExposedDataRecordV1 {
    /// The individual or group who exposed the data
    #[serde(rename = "author")]
    pub author: String,
    /// The ID of the author within Recon
    #[serde(rename = "author_id", skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    /// The customer ID
    #[serde(rename = "cid")]
    pub cid: String,
    /// The company of the user
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// The date when this entity was created in Recon
    #[serde(rename = "created_date")]
    pub created_date: String,
    /// The domain where the credentials are valid
    #[serde(rename = "credentials_domain", skip_serializing_if = "Option::is_none")]
    pub credentials_domain: Option<String>,
    /// The IP where the credentials are valid
    #[serde(rename = "credentials_ip", skip_serializing_if = "Option::is_none")]
    pub credentials_ip: Option<String>,
    /// The URL where the credentials are valid
    #[serde(rename = "credentials_url", skip_serializing_if = "Option::is_none")]
    pub credentials_url: Option<String>,
    /// The nickname of the user on the impacted site
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The domain of the email linked to the impacted site
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The email linked to the impacted site
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The approximate date when the event occurred
    #[serde(rename = "event_date")]
    pub event_date: String,
    /// The date when the exposed data was posted online
    #[serde(rename = "exposure_date")]
    pub exposure_date: String,
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<Box<crate::models::ApiPeriodExposedDataFileDetailsV1>>,
    #[serde(rename = "financial", skip_serializing_if = "Option::is_none")]
    pub financial: Option<Box<crate::models::ApiPeriodExposedDataRecordFinancialV1>>,
    /// The full name of the user on the impacted site
    #[serde(rename = "full_name", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// The algorithm used to hash the password
    #[serde(rename = "hash_type", skip_serializing_if = "Option::is_none")]
    pub hash_type: Option<String>,
    /// The ID of this entity
    #[serde(rename = "id")]
    pub id: String,
    /// The users job at the company
    #[serde(rename = "job_position", skip_serializing_if = "Option::is_none")]
    pub job_position: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<crate::models::ApiPeriodExposedDataRecordLocationV1>>,
    #[serde(rename = "login_id", skip_serializing_if = "Option::is_none")]
    pub login_id: Option<String>,
    /// The ID of the parent notification associated with this entity
    #[serde(rename = "notification_id")]
    pub notification_id: String,
    /// The password used for login
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// The password hash
    #[serde(rename = "password_hash", skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
    /// The password salt
    #[serde(rename = "password_salt", skip_serializing_if = "Option::is_none")]
    pub password_salt: Option<String>,
    /// The phone number of the user on the impacted site
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "raw_intel_id")]
    pub raw_intel_id: String,
    #[serde(rename = "rule")]
    pub rule: Box<crate::models::ApiPeriodRuleDetailsV1>,
    /// The source where this entity was found
    #[serde(rename = "site")]
    pub site: String,
    /// The ID of the site within Recon
    #[serde(rename = "site_id", skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(rename = "social", skip_serializing_if = "Option::is_none")]
    pub social: Option<Box<crate::models::ApiPeriodExposedDataRecordSocialV1>>,
    /// The category of the source where this entity was found
    #[serde(rename = "source_category")]
    pub source_category: String,
    /// The ID of the user on the impacted site
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The IP of the user on the impacted site
    #[serde(rename = "user_ip", skip_serializing_if = "Option::is_none")]
    pub user_ip: Option<String>,
    #[serde(rename = "user_uuid")]
    pub user_uuid: String,
}

impl ApiPeriodNotificationExposedDataRecordV1 {
    pub fn new(
        author: String,
        cid: String,
        created_date: String,
        event_date: String,
        exposure_date: String,
        id: String,
        notification_id: String,
        raw_intel_id: String,
        rule: crate::models::ApiPeriodRuleDetailsV1,
        site: String,
        source_category: String,
        user_uuid: String,
    ) -> ApiPeriodNotificationExposedDataRecordV1 {
        ApiPeriodNotificationExposedDataRecordV1 {
            author,
            author_id: None,
            cid,
            company: None,
            created_date,
            credentials_domain: None,
            credentials_ip: None,
            credentials_url: None,
            display_name: None,
            domain: None,
            email: None,
            event_date,
            exposure_date,
            file: None,
            financial: None,
            full_name: None,
            hash_type: None,
            id,
            job_position: None,
            location: None,
            login_id: None,
            notification_id,
            password: None,
            password_hash: None,
            password_salt: None,
            phone_number: None,
            raw_intel_id,
            rule: Box::new(rule),
            site,
            site_id: None,
            social: None,
            source_category,
            user_id: None,
            user_ip: None,
            user_uuid,
        }
    }
}
