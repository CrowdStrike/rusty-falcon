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
pub struct DomainPeriodEvent {
    /// The raw body of the event
    #[serde(rename = "body")]
    pub body: String,
    /// By default, event bodies are truncated to 64kb and bodyIsTruncated is set to True. For event bodies larger than 64kb, call the /events-full-body endpoint with the respective eventId
    #[serde(rename = "body_is_truncated")]
    pub body_is_truncated: bool,
    /// Link to the event, can be missing
    #[serde(rename = "body_link", skip_serializing_if = "Option::is_none")]
    pub body_link: Option<String>,
    #[serde(
        rename = "botnet_config_source",
        skip_serializing_if = "Option::is_none"
    )]
    pub botnet_config_source: Option<Box<crate::models::DomainPeriodBotnetConfigSource>>,
    /// The date the event was created (in UTC format)
    #[serde(rename = "created_date")]
    pub created_date: String,
    #[serde(rename = "ddos_attack_source", skip_serializing_if = "Option::is_none")]
    pub ddos_attack_source: Option<Box<crate::models::DomainPeriodDdosAttackSource>>,
    /// The type of event. One of `TweetEvent`, `CodePasteEvent`, `BotnetConfigEvent`, `DdosAttackEvent`
    #[serde(rename = "event_type")]
    pub event_type: String,
    /// The event's fingerprint
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    /// The unique event ID
    #[serde(rename = "id")]
    pub id: String,
    /// List of objects with rules that matched the event
    #[serde(rename = "matched_rules", skip_serializing_if = "Option::is_none")]
    pub matched_rules: Option<Vec<crate::models::DomainPeriodMatchedRule>>,
    #[serde(
        rename = "pastebin_text_source",
        skip_serializing_if = "Option::is_none"
    )]
    pub pastebin_text_source: Option<Box<crate::models::DomainPeriodPastebinTextSource>>,
    /// A list of tags summarizing event content
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "tweet_source", skip_serializing_if = "Option::is_none")]
    pub tweet_source: Option<Box<crate::models::DomainPeriodTweetSource>>,
    /// The date the event was last updated (in UTC format)
    #[serde(rename = "updated_date")]
    pub updated_date: String,
}

impl DomainPeriodEvent {
    pub fn new(
        body: String,
        body_is_truncated: bool,
        created_date: String,
        event_type: String,
        fingerprint: String,
        id: String,
        updated_date: String,
    ) -> DomainPeriodEvent {
        DomainPeriodEvent {
            body,
            body_is_truncated,
            body_link: None,
            botnet_config_source: None,
            created_date,
            ddos_attack_source: None,
            event_type,
            fingerprint,
            id,
            matched_rules: None,
            pastebin_text_source: None,
            tags: None,
            tweet_source: None,
            updated_date,
        }
    }
}
