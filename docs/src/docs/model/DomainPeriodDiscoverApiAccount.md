# DomainPeriodDiscoverApiAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_name** | Option<**String**> | The account's name (Domain\\Username or Hostname\\Username). | [optional]
**account_type** | Option<**String**> | The type of the account (Local or Domain). | [optional]
**admin_privileges** | Option<**String**> | Whether the account has administrator privileges (Yes, No, or Unknown). | [optional]
**cid** | **String** | The account's customer ID. |
**first_seen_timestamp** | Option<**String**> | The first time the account was seen successfully logging in to your environment. | [optional]
**id** | **String** | The unique ID of the account. |
**last_failed_login_hostname** | Option<**String**> | The hostname of the asset on which the account last made a failed login. | [optional]
**last_failed_login_timestamp** | Option<**String**> | The date and time of the account's most recent failed login. | [optional]
**last_failed_login_type** | Option<**String**> | The type of the account's most recent failed login. <ul><li>Interactive</li><li>Network</li><li>Batch</li><li>Service</li><li>Unlock</li><li>Network cleartext</li><li>New credentials</li><li>Terminal server</li><li>Cached credentials</li><li>Auditing</li></ul> | [optional]
**last_successful_login_host_city** | Option<**String**> | The name of the city where the asset is located on which the account last successfully logged in. | [optional]
**last_successful_login_host_country** | Option<**String**> | The name of the country where the asset is located on which the account last successfully logged in. | [optional]
**last_successful_login_hostname** | Option<**String**> | The hostname of the asset on which the account last successfully logged in. | [optional]
**last_successful_login_remote_ip** | Option<**String**> | The remote IP address of the asset on which the account last successfully logged in. | [optional]
**last_successful_login_timestamp** | Option<**String**> | The date and time of the account's most recent successful login. | [optional]
**last_successful_login_type** | Option<**String**> | The type of the account's most recent successful login. <ul><li>Interactive</li><li>Service</li><li>Terminal server</li><li>Cached credentials</li><li>Auditing</li></ul> | [optional]
**local_admin_privileges** | Option<**String**> | Whether the account has local administrator privileges (Yes, No). | [optional]
**login_domain** | Option<**String**> | The domain of the asset the account successfully logged in to. | [optional]
**password_last_set_timestamp** | Option<**String**> | The most recent date and time the account's password was changed. | [optional]
**user_sid** | Option<**String**> | The account's security identifier on Windows assets. | [optional]
**username** | Option<**String**> | The account's username. | [optional]

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
