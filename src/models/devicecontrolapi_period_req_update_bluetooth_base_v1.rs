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
pub struct DevicecontrolapiPeriodReqUpdateBluetoothBaseV1 {
    #[serde(rename = "custom_end_user_notifications")]
    pub custom_end_user_notifications:
        Box<models::DevicecontrolapiPeriodBluetoothCustomNotifications>,
    /// Determines if a notification will be shown to the end user (omit to keep current)
    #[serde(rename = "end_user_notification")]
    pub end_user_notification: EndUserNotification,
    /// Enforcement for the Bluetooth policy (omit to keep current)
    #[serde(rename = "enforcement_mode")]
    pub enforcement_mode: EnforcementMode,
}

impl DevicecontrolapiPeriodReqUpdateBluetoothBaseV1 {
    pub fn new(
        custom_end_user_notifications: models::DevicecontrolapiPeriodBluetoothCustomNotifications,
        end_user_notification: EndUserNotification,
        enforcement_mode: EnforcementMode,
    ) -> DevicecontrolapiPeriodReqUpdateBluetoothBaseV1 {
        DevicecontrolapiPeriodReqUpdateBluetoothBaseV1 {
            custom_end_user_notifications: Box::new(custom_end_user_notifications),
            end_user_notification,
            enforcement_mode,
        }
    }
}
/// Determines if a notification will be shown to the end user (omit to keep current)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EndUserNotification {
    #[serde(rename = "NOTIFY_USER,SILENT")]
    NotifyUserCommaSilent,
}

impl Default for EndUserNotification {
    fn default() -> EndUserNotification {
        Self::NotifyUserCommaSilent
    }
}
/// Enforcement for the Bluetooth policy (omit to keep current)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EnforcementMode {
    #[serde(rename = "MONITOR_ONLY,MONITOR_ENFORCE,OFF")]
    MonitorOnlyCommaMonitorEnforceCommaOff,
}

impl Default for EnforcementMode {
    fn default() -> EnforcementMode {
        Self::MonitorOnlyCommaMonitorEnforceCommaOff
    }
}
