# DomainPeriodDiscoverApiLogin

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The ID of the account that made the login. | [optional]
**account_name** | Option<**String**> | The name of the account that made the login (Domain\\Username or Hostname\\Username). | [optional]
**account_type** | Option<**String**> | The type of the account that made the login. | [optional]
**admin_privileges** | Option<**String**> | Whether the account that made the login has administrator privileges (Yes, No, or Unknown). | [optional]
**aggregation_time_interval** | Option<**String**> | A login represents an aggregation of login attempts made in a window of time (1-hour window for logins in the past day, or 24-hour window for logins older than a day). | [optional]
**aid** | Option<**String**> | The agent ID of the Falcon sensor installed on the asset where the login took place. | [optional]
**cid** | Option<**String**> | The customer ID where login took place. | [optional]
**failure_description** | Option<**String**> | The description of the reason why the login failed, if login_status=\"Failed\".<ul><li>There are currently no login servers available to service the login request.</li><li>User login with misspelled or bad user account</li><li>User login with misspelled or bad password</li><li>This is either due to a bad username or authentication information</li><li>Unknown user name or bad password.</li><li>User login outside authorized hours</li><li>User login from unauthorized workstation</li><li>User login with expired password</li><li>User login to account disabled by administrator</li><li>Indicates the Sam Server was in the wrong state to perform the desired operation.</li><li>Clocks between DC and other computer too far out of sync</li><li>The user has not been granted the requested login type (aka login right) at this machine</li><li>The login request failed because the trust relationship between the primary domain and the trusted domain failed.</li><li>An attempt was made to login, but the Netlogon service was not started.</li><li>User login with expired account</li><li>User is required to change password at next login</li><li>Evidently a bug in Windows and not a risk</li><li>User login with account locked</li><li>Failure Reason: An Error occurred during Login</li><li>Login Failure: The machine you are logging in to is protected by an authentication firewall. The specified account is not allowed to authenticate to the machine.</li><li>Status OK.</li><li>Invalid password entered</li></ul> | [optional]
**host_city** | Option<**String**> | The name of the city where the asset is located where the login took place. | [optional]
**host_country** | Option<**String**> | The name of the country where the asset is located on which the login took place. | [optional]
**host_id** | Option<**String**> | The unique ID of the asset where the login took place. | [optional]
**hostname** | Option<**String**> | The hostname of the host where the login took place. | [optional]
**id** | Option<**String**> | The unique ID of the login. | [optional]
**is_suspicious** | Option<**bool**> | Whether the failed login is considered suspicious based on criteria including login failures and locations (true or false). | [optional]
**local_admin_privileges** | Option<**String**> | Whether the account has local administrator privileges (Yes, No). | [optional]
**local_ip** | Option<**String**> | The external IP address of the asset where the login took place. | [optional]
**login_domain** | Option<**String**> | The domain of the asset where the login took place. | [optional]
**login_event_count** | Option<**i64**> | Number of times a login attempt happened in the specified aggregation time interval for this login (1-hour window for logins in the past day, or 24-hour window for logins older than a day). | [optional]
**login_status** | Option<**String**> | The status of the login (Successful or Failed). | [optional]
**login_timestamp** | Option<**String**> | The date and time of the most recent attempt in the login. | [optional]
**login_type** | Option<**String**> | The type of the login.  For successful logins: <ul><li>Interactive</li><li>Service</li><li>Terminal server</li><li>Cached credentials</li><li>Auditing</li></ul> For failed logins: <ul><li>Interactive</li><li>Network</li><li>Batch</li><li>Service</li><li>Unlock</li><li>Network cleartext</li><li>New credentials</li><li>Terminal server</li><li>Cached credentials</li><li>Auditing</li></ul> | [optional]
**remote_ip** | Option<**String**> | The remote IP address where the login was initiated. | [optional]
**user_sid** | Option<**String**> | The security identifier of the account on a Windows asset that made the login. | [optional]
**username** | Option<**String**> | The username of the account that made the login. | [optional]

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
