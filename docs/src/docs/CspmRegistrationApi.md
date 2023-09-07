# CspmRegistrationApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**azure_download_certificate**](CspmRegistrationApi.md#azure_download_certificate) | **GET** /cloud-connect-cspm-azure/entities/download-certificate/v1 | Returns JSON object(s) that contain the base64 encoded certificate for a service principal.
[**create_cspm_aws_account**](CspmRegistrationApi.md#create_cspm_aws_account) | **POST** /cloud-connect-cspm-aws/entities/account/v1 | Creates a new account in our system for a customer and generates a script for them to run in their AWS cloud environment to grant us access.
[**create_cspm_azure_account**](CspmRegistrationApi.md#create_cspm_azure_account) | **POST** /cloud-connect-cspm-azure/entities/account/v1 | Creates a new account in our system for a customer and generates a script for them to run in their cloud environment to grant us access.
[**delete_cspm_aws_account**](CspmRegistrationApi.md#delete_cspm_aws_account) | **DELETE** /cloud-connect-cspm-aws/entities/account/v1 | Deletes an existing AWS account or organization in our system.
[**delete_cspm_azure_account**](CspmRegistrationApi.md#delete_cspm_azure_account) | **DELETE** /cloud-connect-cspm-azure/entities/account/v1 | Deletes an Azure subscription from the system.
[**get_behavior_detections**](CspmRegistrationApi.md#get_behavior_detections) | **GET** /detects/entities/ioa/v1 | Get list of detected behaviors
[**get_configuration_detection_entities**](CspmRegistrationApi.md#get_configuration_detection_entities) | **GET** /detects/entities/iom/v2 | Get misconfigurations based on the ID - including custom policy detections in addition to default policy detections.
[**get_configuration_detection_ids_v2**](CspmRegistrationApi.md#get_configuration_detection_ids_v2) | **GET** /detects/queries/iom/v2 | Get list of active misconfiguration ids - including custom policy detections in addition to default policy detections.
[**get_configuration_detections**](CspmRegistrationApi.md#get_configuration_detections) | **GET** /detects/entities/iom/v1 | Get list of active misconfigurations
[**get_cspm_aws_account**](CspmRegistrationApi.md#get_cspm_aws_account) | **GET** /cloud-connect-cspm-aws/entities/account/v1 | Returns information about the current status of an AWS account.
[**get_cspm_aws_account_scripts_attachment**](CspmRegistrationApi.md#get_cspm_aws_account_scripts_attachment) | **GET** /cloud-connect-cspm-aws/entities/user-scripts-download/v1 | Return a script for customer to run in their cloud environment to grant us access to their AWS environment as a downloadable attachment.
[**get_cspm_aws_console_setup_urls**](CspmRegistrationApi.md#get_cspm_aws_console_setup_urls) | **GET** /cloud-connect-cspm-aws/entities/console-setup-urls/v1 | Return a URL for customer to visit in their cloud environment to grant us access to their AWS environment.
[**get_cspm_azure_account**](CspmRegistrationApi.md#get_cspm_azure_account) | **GET** /cloud-connect-cspm-azure/entities/account/v1 | Return information about Azure account registration
[**get_cspm_azure_user_scripts_attachment**](CspmRegistrationApi.md#get_cspm_azure_user_scripts_attachment) | **GET** /cloud-connect-cspm-azure/entities/user-scripts-download/v1 | Return a script for customer to run in their cloud environment to grant us access to their Azure environment as a downloadable attachment
[**get_cspm_policies_details**](CspmRegistrationApi.md#get_cspm_policies_details) | **GET** /settings/entities/policy-details/v2 | Given an array of policy IDs, returns detailed policies information.
[**get_cspm_policy**](CspmRegistrationApi.md#get_cspm_policy) | **GET** /settings/entities/policy-details/v1 | Given a policy ID, returns detailed policy information.
[**get_cspm_policy_settings**](CspmRegistrationApi.md#get_cspm_policy_settings) | **GET** /settings/entities/policy/v1 | Returns information about current policy settings.
[**get_cspm_scan_schedule**](CspmRegistrationApi.md#get_cspm_scan_schedule) | **GET** /settings/scan-schedule/v1 | Returns scan schedule configuration for one or more cloud platforms.
[**patch_cspm_aws_account**](CspmRegistrationApi.md#patch_cspm_aws_account) | **PATCH** /cloud-connect-cspm-aws/entities/account/v1 | Patches a existing account in our system for a customer.
[**update_cspm_azure_account_client_id**](CspmRegistrationApi.md#update_cspm_azure_account_client_id) | **PATCH** /cloud-connect-cspm-azure/entities/client-id/v1 | Update an Azure service account in our system by with the user-created client_id created with the public key we've provided
[**update_cspm_azure_tenant_default_subscription_id**](CspmRegistrationApi.md#update_cspm_azure_tenant_default_subscription_id) | **PATCH** /cloud-connect-cspm-azure/entities/default-subscription-id/v1 | Update an Azure default subscription_id in our system for given tenant_id
[**update_cspm_policy_settings**](CspmRegistrationApi.md#update_cspm_policy_settings) | **PATCH** /settings/entities/policy/v1 | Updates a policy setting - can be used to override policy severity or to disable a policy entirely.
[**update_cspm_scan_schedule**](CspmRegistrationApi.md#update_cspm_scan_schedule) | **POST** /settings/scan-schedule/v1 | Updates scan schedule configuration for one or more cloud platforms.

## azure_download_certificate

> crate::models::RegistrationPeriodAzureDownloadCertificateResponseV1 azure_download_certificate(tenant_id, refresh, years_valid)
Returns JSON object(s) that contain the base64 encoded certificate for a service principal.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | [**Vec<String>**](String.md) | Azure Tenant ID | [required] |
**refresh** | Option<**bool**> | Setting to true will invalidate the current certificate and generate a new certificate |  |[default to false]
**years_valid** | Option<**String**> | Years the certificate should be valid (only used when refresh=true) |  |

### Return type

[**crate::models::RegistrationPeriodAzureDownloadCertificateResponseV1**](registration.AzureDownloadCertificateResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## create_cspm_aws_account

> crate::models::RegistrationPeriodAwsAccountResponseV2 create_cspm_aws_account(body)
Creates a new account in our system for a customer and generates a script for them to run in their AWS cloud environment to grant us access.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RegistrationPeriodAwsAccountCreateRequestExtV2**](RegistrationPeriodAwsAccountCreateRequestExtV2.md) |  | [required] |

### Return type

[**crate::models::RegistrationPeriodAwsAccountResponseV2**](registration.AWSAccountResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## create_cspm_azure_account

> crate::models::RegistrationPeriodAzureAccountResponseV1 create_cspm_azure_account(body)
Creates a new account in our system for a customer and generates a script for them to run in their cloud environment to grant us access.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RegistrationPeriodAzureAccountCreateRequestExternalV1**](RegistrationPeriodAzureAccountCreateRequestExternalV1.md) |  | [required] |

### Return type

[**crate::models::RegistrationPeriodAzureAccountResponseV1**](registration.AzureAccountResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_cspm_aws_account

> crate::models::MsaPeriodBaseEntitiesResponse delete_cspm_aws_account(ids, organization_ids)
Deletes an existing AWS account or organization in our system.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | AWS account IDs to remove |  |
**organization_ids** | Option<[**Vec<String>**](String.md)> | AWS organization IDs to remove |  |

### Return type

[**crate::models::MsaPeriodBaseEntitiesResponse**](msa.BaseEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_cspm_azure_account

> crate::models::MsaPeriodBaseEntitiesResponse delete_cspm_azure_account(ids, tenant_ids, retain_tenant)
Deletes an Azure subscription from the system.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Azure subscription IDs to remove |  |
**tenant_ids** | Option<[**Vec<String>**](String.md)> | Tenant ids to remove |  |
**retain_tenant** | Option<**String**> |  |  |

### Return type

[**crate::models::MsaPeriodBaseEntitiesResponse**](msa.BaseEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## get_behavior_detections

> crate::models::RegistrationPeriodExternalIoaEventResponse get_behavior_detections(cloud_provider, service, account_id, aws_account_id, azure_subscription_id, azure_tenant_id, state, date_time_since, since, severity, next_token, limit, resource_id, resource_uuid)
Get list of detected behaviors

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_provider** | Option<**String**> | Cloud Provider (e.g.: aws|azure) |  |
**service** | Option<**String**> | Cloud Service (e.g. EC2 | EBS | S3) |  |
**account_id** | Option<**String**> | Cloud Account ID (e.g.: AWS accountID, Azure subscriptionID) |  |
**aws_account_id** | Option<**String**> | AWS Account ID |  |
**azure_subscription_id** | Option<**String**> | Azure Subscription ID |  |
**azure_tenant_id** | Option<**String**> | Azure Tenant ID |  |
**state** | Option<**String**> | State (e.g.: open | closed) |  |
**date_time_since** | Option<**String**> | Filter to get all events after this date, in format RFC3339 : e.g. 2006-01-02T15:04:05Z07:00 |  |
**since** | Option<**String**> | Filter events using a duration string (e.g. 24h) |  |[default to 24h]
**severity** | Option<**String**> | Policy Severity |  |
**next_token** | Option<**String**> | String to get next page of results, is associated with a previous execution of GetBehaviorDetections. Must include all filters from previous execution. |  |
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |
**resource_id** | Option<[**Vec<String>**](String.md)> | Resource ID |  |
**resource_uuid** | Option<[**Vec<String>**](String.md)> | Resource UUID |  |

### Return type

[**crate::models::RegistrationPeriodExternalIoaEventResponse**](registration.ExternalIOAEventResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## get_configuration_detection_entities

> crate::models::RegistrationPeriodExternalIomEventResponseV2 get_configuration_detection_entities(ids)
Get misconfigurations based on the ID - including custom policy detections in addition to default policy detections.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | detection ids | [required] |

### Return type

[**crate::models::RegistrationPeriodExternalIomEventResponseV2**](registration.ExternalIOMEventResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## get_configuration_detection_ids_v2

> crate::models::RegistrationPeriodIomEventIdsResponseV2 get_configuration_detection_ids_v2(filter, sort, limit, offset, next_token)
Get list of active misconfiguration ids - including custom policy detections in addition to default policy detections.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | use_current_scan_ids - *use this to get records for latest scans* account_name account_id agent_id attack_types azure_subscription_id cloud_provider cloud_service_keyword custom_policy_id is_managed policy_id policy_type resource_id region status scan_time severity severity_string  |  |
**sort** | Option<**String**> | account_name account_id attack_types azure_subscription_id cloud_provider cloud_service_keyword status is_managed policy_id policy_type resource_id region scan_time severity severity_string timestamp |  |[default to timestamp|desc]
**limit** | Option<**i32**> | The max number of detections to return |  |[default to 500]
**offset** | Option<**i32**> | Offset returned detections. Cannot be combined with next_token filter |  |
**next_token** | Option<**String**> | String to get next page of results. Cannot be combined with any filter except limit. |  |

### Return type

[**crate::models::RegistrationPeriodIomEventIdsResponseV2**](registration.IOMEventIDsResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## get_configuration_detections

> crate::models::RegistrationPeriodExternalIomEventResponse get_configuration_detections(cloud_provider, account_id, azure_subscription_id, azure_tenant_id, status, region, severity, service, next_token, limit)
Get list of active misconfigurations

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_provider** | Option<**String**> | Cloud Provider (e.g.: aws|azure|gcp) |  |
**account_id** | Option<**String**> | AWS account ID or GCP Project Number or Azure subscription ID |  |
**azure_subscription_id** | Option<**String**> | Azure Subscription ID |  |
**azure_tenant_id** | Option<**String**> | Azure Tenant ID |  |
**status** | Option<**String**> | Status (e.g.: new|reoccurring|all) |  |
**region** | Option<**String**> | Cloud Provider Region |  |
**severity** | Option<**String**> | Policy Severity |  |
**service** | Option<**String**> | Cloud Service (e.g.: EBS|EC2|S3 etc.) |  |
**next_token** | Option<**String**> | String to get next page of results, is associated with a previous execution of GetConfigurationDetections. Cannot be combined with any filter except limit. |  |
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |

### Return type

[**crate::models::RegistrationPeriodExternalIomEventResponse**](registration.ExternalIOMEventResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cspm_aws_account

> crate::models::RegistrationPeriodAwsAccountResponseV2 get_cspm_aws_account(scan_type, ids, iam_role_arns, organization_ids, status, limit, migrated, offset, group_by)
Returns information about the current status of an AWS account.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_type** | Option<**String**> | Type of scan, dry or full, to perform on selected accounts |  |
**ids** | Option<[**Vec<String>**](String.md)> | AWS account IDs |  |
**iam_role_arns** | Option<[**Vec<String>**](String.md)> | AWS IAM role ARNs |  |
**organization_ids** | Option<[**Vec<String>**](String.md)> | AWS organization IDs |  |
**status** | Option<**String**> | Account status to filter results by. |  |
**limit** | Option<**i32**> | The maximum records to return. Defaults to 100. |  |[default to 100]
**migrated** | Option<**String**> | Only return migrated d4c accounts |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**group_by** | Option<**String**> | Field to group by. |  |

### Return type

[**crate::models::RegistrationPeriodAwsAccountResponseV2**](registration.AWSAccountResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cspm_aws_account_scripts_attachment

> crate::models::RegistrationPeriodAwsProvisionGetAccountScriptResponseV2 get_cspm_aws_account_scripts_attachment(ids)
Return a script for customer to run in their cloud environment to grant us access to their AWS environment as a downloadable attachment.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | AWS account IDs |  |

### Return type

[**crate::models::RegistrationPeriodAwsProvisionGetAccountScriptResponseV2**](registration.AWSProvisionGetAccountScriptResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cspm_aws_console_setup_urls

> crate::models::RegistrationPeriodAwsAccountConsoleUrl get_cspm_aws_console_setup_urls(ids, use_existing_cloudtrail, region)
Return a URL for customer to visit in their cloud environment to grant us access to their AWS environment.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | AWS account IDs |  |
**use_existing_cloudtrail** | Option<**String**> |  |  |
**region** | Option<**String**> | Region |  |

### Return type

[**crate::models::RegistrationPeriodAwsAccountConsoleUrl**](registration.AWSAccountConsoleURL.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cspm_azure_account

> crate::models::RegistrationPeriodAzureAccountResponseV1 get_cspm_azure_account(ids, tenant_ids, scan_type, status, limit, offset)
Return information about Azure account registration

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | SubscriptionIDs of accounts to select for this status operation. If this is empty then all accounts are returned. |  |
**tenant_ids** | Option<[**Vec<String>**](String.md)> | Tenant ids to filter azure accounts |  |
**scan_type** | Option<**String**> | Type of scan, dry or full, to perform on selected accounts |  |
**status** | Option<**String**> | Account status to filter results by. |  |
**limit** | Option<**i32**> | The maximum records to return. Defaults to 100. |  |[default to 100]
**offset** | Option<**i32**> | The offset to start retrieving records from |  |

### Return type

[**crate::models::RegistrationPeriodAzureAccountResponseV1**](registration.AzureAccountResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cspm_azure_user_scripts_attachment

> crate::models::RegistrationPeriodAzureProvisionGetUserScriptResponseV1 get_cspm_azure_user_scripts_attachment(tenant_id, subscription_ids, account_type, template)
Return a script for customer to run in their cloud environment to grant us access to their Azure environment as a downloadable attachment

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | Option<**String**> | Tenant ID to generate script for. Defaults to most recently registered tenant. |  |
**subscription_ids** | Option<[**Vec<String>**](String.md)> | Subscription IDs to generate script for. Defaults to all. |  |
**account_type** | Option<**String**> |  |  |
**template** | Option<**String**> | Template to be rendered |  |

### Return type

[**crate::models::RegistrationPeriodAzureProvisionGetUserScriptResponseV1**](registration.AzureProvisionGetUserScriptResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cspm_policies_details

> crate::models::RegistrationPeriodPolicyResponseV1 get_cspm_policies_details(ids)
Given an array of policy IDs, returns detailed policies information.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<i32>**](i32.md) | Policy IDs | [required] |

### Return type

[**crate::models::RegistrationPeriodPolicyResponseV1**](registration.PolicyResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cspm_policy

> crate::models::RegistrationPeriodPolicyResponseV1 get_cspm_policy(ids)
Given a policy ID, returns detailed policy information.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **i32** | Policy ID | [required] |

### Return type

[**crate::models::RegistrationPeriodPolicyResponseV1**](registration.PolicyResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cspm_policy_settings

> crate::models::RegistrationPeriodPolicySettingsResponseV1 get_cspm_policy_settings(service, policy_id, cloud_platform)
Returns information about current policy settings.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | Option<**String**> | Service type to filter policy settings by. |  |
**policy_id** | Option<**String**> | Policy ID |  |
**cloud_platform** | Option<**String**> | Cloud Platform (e.g.: aws|azure|gcp) |  |

### Return type

[**crate::models::RegistrationPeriodPolicySettingsResponseV1**](registration.PolicySettingsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cspm_scan_schedule

> crate::models::RegistrationPeriodScanScheduleResponseV1 get_cspm_scan_schedule(cloud_platform)
Returns scan schedule configuration for one or more cloud platforms.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_platform** | Option<[**Vec<String>**](String.md)> | Cloud Platform |  |

### Return type

[**crate::models::RegistrationPeriodScanScheduleResponseV1**](registration.ScanScheduleResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## patch_cspm_aws_account

> crate::models::RegistrationPeriodAwsAccountResponseV2 patch_cspm_aws_account(body)
Patches a existing account in our system for a customer.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RegistrationPeriodAwsAccountPatchRequest**](RegistrationPeriodAwsAccountPatchRequest.md) |  | [required] |

### Return type

[**crate::models::RegistrationPeriodAwsAccountResponseV2**](registration.AWSAccountResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## update_cspm_azure_account_client_id

> crate::models::RegistrationPeriodAzureTenantConfigurationResponseV1 update_cspm_azure_account_client_id(id, tenant_id)
Update an Azure service account in our system by with the user-created client_id created with the public key we've provided

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ClientID to use for the Service Principal associated with the customer's Azure account | [required] |
**tenant_id** | Option<**String**> | Tenant ID to update client ID for. Required if multiple tenants are registered. |  |

### Return type

[**crate::models::RegistrationPeriodAzureTenantConfigurationResponseV1**](registration.AzureTenantConfigurationResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## update_cspm_azure_tenant_default_subscription_id

> crate::models::RegistrationPeriodAzureTenantDefaultSubscriptionIdResponseV1 update_cspm_azure_tenant_default_subscription_id(subscription_id, tenant_id)
Update an Azure default subscription_id in our system for given tenant_id

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | Default Subscription ID to patch for all subscriptions belonged to a tenant. | [required] |
**tenant_id** | Option<**String**> | Tenant ID to update client ID for. Required if multiple tenants are registered. |  |

### Return type

[**crate::models::RegistrationPeriodAzureTenantDefaultSubscriptionIdResponseV1**](registration.AzureTenantDefaultSubscriptionIDResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## update_cspm_policy_settings

> crate::models::RegistrationPeriodPolicySettingsResponseV1 update_cspm_policy_settings(body)
Updates a policy setting - can be used to override policy severity or to disable a policy entirely.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RegistrationPeriodPolicyRequestExtV1**](RegistrationPeriodPolicyRequestExtV1.md) |  | [required] |

### Return type

[**crate::models::RegistrationPeriodPolicySettingsResponseV1**](registration.PolicySettingsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)

## update_cspm_scan_schedule

> crate::models::RegistrationPeriodScanScheduleResponseV1 update_cspm_scan_schedule(body)
Updates scan schedule configuration for one or more cloud platforms.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RegistrationPeriodScanScheduleUpdateRequestV1**](RegistrationPeriodScanScheduleUpdateRequestV1.md) |  | [required] |

### Return type

[**crate::models::RegistrationPeriodScanScheduleResponseV1**](registration.ScanScheduleResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](../README.md)
