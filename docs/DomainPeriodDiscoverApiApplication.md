# DomainPeriodDiscoverApiApplication

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**architectures** | Option<**Vec<String>**> | Represents the application architectures (x86 / x64). | [optional]
**category** | Option<**String**> | The category of the application. | [optional]
**cid** | **String** | The customer ID of this application. | 
**first_seen_timestamp** | Option<**String**> | Timestamp when this application was first seen by the cloud. | [optional]
**groups** | Option<**Vec<String>**> | The user defined groups this application is part of. | [optional]
**host** | Option<[**crate::models::DomainPeriodDiscoverApiApplicationHost**](domain.DiscoverAPIApplicationHost.md)> |  | [optional]
**id** | **String** | The unique ID for the application. | 
**installation_paths** | Option<**Vec<String>**> | The file paths where the application is installed on the host. Or the locations of the executables. | [optional]
**installation_timestamp** | Option<**String**> | Timestamp when the application was installed on the host. We might not have this data. | [optional]
**is_normalized** | Option<**bool**> | Whether or not the application is normalized | [optional]
**is_suspicious** | Option<**bool**> | Whether or not the application is suspicious | [optional]
**last_updated_timestamp** | Option<**String**> | Timestamp when this application was last updated (something changed in the application or in the host data). | [optional]
**last_used_file_hash** | Option<**String**> | The file hash that was last used for this application. | [optional]
**last_used_file_name** | Option<**String**> | The file name that was last used for this application. | [optional]
**last_used_timestamp** | Option<**String**> | Timestamp when this application was last used. | [optional]
**last_used_user_name** | Option<**String**> | The username of the user that last used this application. | [optional]
**last_used_user_sid** | Option<**String**> | The user SID of the last user that used this application. | [optional]
**name** | Option<**String**> | The name of the application. | [optional]
**name_vendor** | Option<**String**> | The combined field on which we will be able to group by app. | [optional]
**name_vendor_version** | Option<**String**> | The combined field on which we will be able to group by app + version. | [optional]
**vendor** | Option<**String**> | The name the application's vendor. | [optional]
**version** | Option<**String**> | The version of the application. | [optional]
**versioning_scheme** | Option<**String**> | The version scheme of the application. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


