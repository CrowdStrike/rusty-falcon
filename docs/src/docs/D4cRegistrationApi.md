# \D4cRegistrationApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**connect_d4_cgcp_account**](D4cRegistrationApi.md#connect_d4_cgcp_account) | **POST** /cloud-connect-gcp/entities/account/v2 | Creates a new GCP account with newly-uploaded service account or connects with existing service account with only the following fields: parent_id, parent_type and service_account_id
[**create_d4_c_aws_account**](D4cRegistrationApi.md#create_d4_c_aws_account) | **POST** /cloud-connect-aws/entities/account/v2 | Creates a new account in our system for a customer and generates a script for them to run in their AWS cloud environment to grant us access.
[**create_d4_cgcp_account**](D4cRegistrationApi.md#create_d4_cgcp_account) | **POST** /cloud-connect-gcp/entities/account/v1 | Creates a new account in our system for a customer and generates a new service account for them to add access to in their GCP environment to grant us access.
[**create_discover_cloud_azure_account**](D4cRegistrationApi.md#create_discover_cloud_azure_account) | **POST** /cloud-connect-azure/entities/account/v1 | Creates a new account in our system for a customer and generates a script for them to run in their cloud environment to grant us access.
[**delete_d4_c_aws_account**](D4cRegistrationApi.md#delete_d4_c_aws_account) | **DELETE** /cloud-connect-aws/entities/account/v2 | Deletes an existing AWS account or organization in our system.
[**delete_d4_cgcp_account**](D4cRegistrationApi.md#delete_d4_cgcp_account) | **DELETE** /cloud-connect-gcp/entities/account/v1 | Deletes a GCP account from the system.
[**discover_cloud_azure_download_certificate**](D4cRegistrationApi.md#discover_cloud_azure_download_certificate) | **GET** /cloud-connect-azure/entities/download-certificate/v1 | Returns JSON object(s) that contain the base64 encoded certificate for a service principal.
[**get_d4_c_aws_account**](D4cRegistrationApi.md#get_d4_c_aws_account) | **GET** /cloud-connect-aws/entities/account/v2 | Returns information about the current status of an AWS account.
[**get_d4_c_aws_console_setup_urls**](D4cRegistrationApi.md#get_d4_c_aws_console_setup_urls) | **GET** /cloud-connect-aws/entities/console-setup-urls/v1 | Return a URL for customer to visit in their cloud environment to grant us access to their AWS environment.
[**get_d4_caws_account_scripts_attachment**](D4cRegistrationApi.md#get_d4_caws_account_scripts_attachment) | **GET** /cloud-connect-aws/entities/user-scripts-download/v1 | Return a script for customer to run in their cloud environment to grant us access to their AWS environment as a downloadable attachment.
[**get_d4_ccgp_account**](D4cRegistrationApi.md#get_d4_ccgp_account) | **GET** /cloud-connect-gcp/entities/account/v1 | Returns information about the current status of an GCP account.
[**get_d4_cgcp_service_accounts_ext**](D4cRegistrationApi.md#get_d4_cgcp_service_accounts_ext) | **GET** /cloud-connect-gcp/entities/service-accounts/v1 | Returns the service account id and client email for external clients.
[**get_d4_cgcp_user_scripts**](D4cRegistrationApi.md#get_d4_cgcp_user_scripts) | **GET** /cloud-connect-gcp/entities/user-scripts/v1 | Return a script for customer to run in their cloud environment to grant us access to their GCP environment
[**get_d4_cgcp_user_scripts_attachment**](D4cRegistrationApi.md#get_d4_cgcp_user_scripts_attachment) | **GET** /cloud-connect-gcp/entities/user-scripts-download/v1 | Return a script for customer to run in their cloud environment to grant us access to their GCP environment as a downloadable attachment
[**get_discover_cloud_azure_account**](D4cRegistrationApi.md#get_discover_cloud_azure_account) | **GET** /cloud-connect-azure/entities/account/v1 | Return information about Azure account registration
[**get_discover_cloud_azure_tenant_ids**](D4cRegistrationApi.md#get_discover_cloud_azure_tenant_ids) | **GET** /cloud-connect-azure/entities/tenant-id/v1 | Return available tenant ids for discover for cloud
[**get_discover_cloud_azure_user_scripts**](D4cRegistrationApi.md#get_discover_cloud_azure_user_scripts) | **GET** /cloud-connect-azure/entities/user-scripts/v1 | Return a script for customer to run in their cloud environment to grant us access to their Azure environment
[**get_discover_cloud_azure_user_scripts_attachment**](D4cRegistrationApi.md#get_discover_cloud_azure_user_scripts_attachment) | **GET** /cloud-connect-azure/entities/user-scripts-download/v1 | Return a script for customer to run in their cloud environment to grant us access to their Azure environment as a downloadable attachment
[**get_horizon_d4_c_scripts**](D4cRegistrationApi.md#get_horizon_d4_c_scripts) | **GET** /settings-discover/entities/gen/scripts/v1 | Returns static install scripts for Horizon.
[**update_d4_cgcp_service_accounts_ext**](D4cRegistrationApi.md#update_d4_cgcp_service_accounts_ext) | **PATCH** /cloud-connect-gcp/entities/service-accounts/v1 | Patches the service account key for external clients.
[**update_discover_cloud_azure_account_client_id**](D4cRegistrationApi.md#update_discover_cloud_azure_account_client_id) | **PATCH** /cloud-connect-azure/entities/client-id/v1 | Update an Azure service account in our system by with the user-created client_id created with the public key we've provided

## connect_d4_cgcp_account

> models::RegistrationPeriodGcpAccountResponseExtV2 connect_d4_cgcp_account(body)
Creates a new GCP account with newly-uploaded service account or connects with existing service account with only the following fields: parent_id, parent_type and service_account_id

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RegistrationPeriodGcpAccountExtRequestV2**](RegistrationPeriodGcpAccountExtRequestV2.md) |  | [required] |

### Return type

[**models::RegistrationPeriodGcpAccountResponseExtV2**](registration.GCPAccountResponseExtV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_d4_c_aws_account

> models::RegistrationPeriodAwsAccountResponseV2 create_d4_c_aws_account(body)
Creates a new account in our system for a customer and generates a script for them to run in their AWS cloud environment to grant us access.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RegistrationPeriodAwsAccountCreateRequestD4CExtV2**](RegistrationPeriodAwsAccountCreateRequestD4CExtV2.md) |  | [required] |

### Return type

[**models::RegistrationPeriodAwsAccountResponseV2**](registration.AWSAccountResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_d4_cgcp_account

> models::RegistrationPeriodGcpAccountResponseV1 create_d4_cgcp_account(body)
Creates a new account in our system for a customer and generates a new service account for them to add access to in their GCP environment to grant us access.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RegistrationPeriodGcpAccountCreateRequestExtV1**](RegistrationPeriodGcpAccountCreateRequestExtV1.md) |  | [required] |

### Return type

[**models::RegistrationPeriodGcpAccountResponseV1**](registration.GCPAccountResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_discover_cloud_azure_account

> models::RegistrationPeriodAzureAccountResponseV1 create_discover_cloud_azure_account(body)
Creates a new account in our system for a customer and generates a script for them to run in their cloud environment to grant us access.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RegistrationPeriodAzureAccountCreateRequestExternalV1**](RegistrationPeriodAzureAccountCreateRequestExternalV1.md) |  | [required] |

### Return type

[**models::RegistrationPeriodAzureAccountResponseV1**](registration.AzureAccountResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_d4_c_aws_account

> models::MsaPeriodBaseEntitiesResponse delete_d4_c_aws_account(ids, organization_ids)
Deletes an existing AWS account or organization in our system.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | AWS account IDs to remove |  |
**organization_ids** | Option<[**Vec<String>**](String.md)> | AWS organization IDs to remove |  |

### Return type

[**models::MsaPeriodBaseEntitiesResponse**](msa.BaseEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_d4_cgcp_account

> models::MsaPeriodBaseEntitiesResponse delete_d4_cgcp_account(ids)
Deletes a GCP account from the system.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Hierarchical Resource IDs of accounts |  |

### Return type

[**models::MsaPeriodBaseEntitiesResponse**](msa.BaseEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## discover_cloud_azure_download_certificate

> models::RegistrationPeriodAzureDownloadCertificateResponseV1 discover_cloud_azure_download_certificate(tenant_id, refresh, years_valid)
Returns JSON object(s) that contain the base64 encoded certificate for a service principal.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | [**Vec<String>**](String.md) | Azure Tenant ID | [required] |
**refresh** | Option<**bool**> | Setting to true will invalidate the current certificate and generate a new certificate |  |[default to false]
**years_valid** | Option<**String**> | Years the certificate should be valid (only used when refresh=true) |  |

### Return type

[**models::RegistrationPeriodAzureDownloadCertificateResponseV1**](registration.AzureDownloadCertificateResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_d4_c_aws_account

> models::RegistrationPeriodAwsAccountResponseV2 get_d4_c_aws_account(scan_type, ids, organization_ids, status, limit, offset, migrated)
Returns information about the current status of an AWS account.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_type** | Option<**String**> | Type of scan, dry or full, to perform on selected accounts |  |
**ids** | Option<[**Vec<String>**](String.md)> | AWS account IDs |  |
**organization_ids** | Option<[**Vec<String>**](String.md)> | AWS organization IDs |  |
**status** | Option<**String**> | Account status to filter results by. |  |
**limit** | Option<**i32**> | The maximum records to return. Defaults to 100. |  |[default to 100]
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**migrated** | Option<**String**> | Only return migrated d4c accounts |  |

### Return type

[**models::RegistrationPeriodAwsAccountResponseV2**](registration.AWSAccountResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_d4_c_aws_console_setup_urls

> models::RegistrationPeriodAwsConsoleUrlResponseV2 get_d4_c_aws_console_setup_urls(region)
Return a URL for customer to visit in their cloud environment to grant us access to their AWS environment.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | Option<**String**> | Region |  |

### Return type

[**models::RegistrationPeriodAwsConsoleUrlResponseV2**](registration.AWSConsoleURLResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_d4_caws_account_scripts_attachment

> models::RegistrationPeriodAwsProvisionGetAccountScriptResponseV2 get_d4_caws_account_scripts_attachment(ids, template, accounts, behavior_assessment_enabled, sensor_management_enabled, use_existing_cloudtrail, organization_id, aws_profile, custom_role_name)
Return a script for customer to run in their cloud environment to grant us access to their AWS environment as a downloadable attachment.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | AWS account IDs |  |
**template** | Option<**String**> | Template to be rendered |  |[default to aws-bash]
**accounts** | Option<[**Vec<String>**](String.md)> | The list of accounts to register |  |
**behavior_assessment_enabled** | Option<**String**> |  |  |
**sensor_management_enabled** | Option<**String**> |  |  |
**use_existing_cloudtrail** | Option<**String**> |  |  |
**organization_id** | Option<**String**> | The AWS organization ID to be registered |  |
**aws_profile** | Option<**String**> | The AWS profile to be used during registration |  |
**custom_role_name** | Option<**String**> | The custom IAM role to be used during registration |  |

### Return type

[**models::RegistrationPeriodAwsProvisionGetAccountScriptResponseV2**](registration.AWSProvisionGetAccountScriptResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_d4_ccgp_account

> models::RegistrationPeriodGcpAccountResponseV1 get_d4_ccgp_account(parent_type, ids, scan_type, status, limit, offset, sort)
Returns information about the current status of an GCP account.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_type** | Option<**String**> | GCP Hierarchy Parent Type, organization/folder/project |  |
**ids** | Option<[**Vec<String>**](String.md)> | Hierarchical Resource IDs of accounts |  |
**scan_type** | Option<**String**> | Type of scan, dry or full, to perform on selected accounts |  |
**status** | Option<**String**> | Account status to filter results by. |  |
**limit** | Option<**i32**> | The maximum records to return. Defaults to 100. |  |[default to 100]
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**sort** | Option<**String**> | Order fields in ascending or descending order. Ex: parent_type|asc. |  |

### Return type

[**models::RegistrationPeriodGcpAccountResponseV1**](registration.GCPAccountResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_d4_cgcp_service_accounts_ext

> models::RegistrationPeriodGcpServiceAccountResponseExtV1 get_d4_cgcp_service_accounts_ext(id)
Returns the service account id and client email for external clients.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | Service Account ID |  |

### Return type

[**models::RegistrationPeriodGcpServiceAccountResponseExtV1**](registration.GCPServiceAccountResponseExtV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_d4_cgcp_user_scripts

> models::RegistrationPeriodGcpProvisionGetUserScriptResponseV1 get_d4_cgcp_user_scripts(parent_type)
Return a script for customer to run in their cloud environment to grant us access to their GCP environment

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_type** | Option<**String**> | GCP Hierarchy Parent Type, organization/folder/project |  |

### Return type

[**models::RegistrationPeriodGcpProvisionGetUserScriptResponseV1**](registration.GCPProvisionGetUserScriptResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_d4_cgcp_user_scripts_attachment

> models::RegistrationPeriodGcpProvisionGetUserScriptResponseV1 get_d4_cgcp_user_scripts_attachment(parent_type, ids, status)
Return a script for customer to run in their cloud environment to grant us access to their GCP environment as a downloadable attachment

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_type** | Option<**String**> | GCP Hierarchy Parent Type, organization/folder/project |  |
**ids** | Option<[**Vec<String>**](String.md)> | Hierarchical Resource IDs of accounts |  |
**status** | Option<**String**> | Account status to filter results by. |  |

### Return type

[**models::RegistrationPeriodGcpProvisionGetUserScriptResponseV1**](registration.GCPProvisionGetUserScriptResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_discover_cloud_azure_account

> models::RegistrationPeriodAzureAccountResponseV1 get_discover_cloud_azure_account(ids, tenant_ids, scan_type, status, limit, offset)
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

[**models::RegistrationPeriodAzureAccountResponseV1**](registration.AzureAccountResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_discover_cloud_azure_tenant_ids

> models::RegistrationPeriodAzureTenantIdsResponseV1 get_discover_cloud_azure_tenant_ids()
Return available tenant ids for discover for cloud

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RegistrationPeriodAzureTenantIdsResponseV1**](registration.AzureTenantIDsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_discover_cloud_azure_user_scripts

> models::RegistrationPeriodAzureProvisionGetUserScriptResponseV1 get_discover_cloud_azure_user_scripts()
Return a script for customer to run in their cloud environment to grant us access to their Azure environment

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RegistrationPeriodAzureProvisionGetUserScriptResponseV1**](registration.AzureProvisionGetUserScriptResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_discover_cloud_azure_user_scripts_attachment

> models::RegistrationPeriodAzureProvisionGetUserScriptResponseV1 get_discover_cloud_azure_user_scripts_attachment(tenant_id, subscription_ids, template, azure_management_group)
Return a script for customer to run in their cloud environment to grant us access to their Azure environment as a downloadable attachment

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | [**Vec<String>**](String.md) | Azure Tenant ID | [required] |
**subscription_ids** | Option<[**Vec<String>**](String.md)> | Azure Subscription ID |  |
**template** | Option<**String**> | Template to be rendered |  |
**azure_management_group** | Option<**bool**> | Use Azure Management Group |  |

### Return type

[**models::RegistrationPeriodAzureProvisionGetUserScriptResponseV1**](registration.AzureProvisionGetUserScriptResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_horizon_d4_c_scripts

> models::RegistrationPeriodStaticScriptsResponse get_horizon_d4_c_scripts(single_account, organization_id, delete, account_type)
Returns static install scripts for Horizon.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**single_account** | Option<**String**> | Get static script for single account |  |
**organization_id** | Option<**String**> | AWS organization ID |  |
**delete** | Option<**String**> |  |  |
**account_type** | Option<**String**> | Account type (e.g.: commercial,gov) Only applicable when registering AWS commercial account in a Gov environment |  |

### Return type

[**models::RegistrationPeriodStaticScriptsResponse**](registration.StaticScriptsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_d4_cgcp_service_accounts_ext

> models::RegistrationPeriodGcpServiceAccountResponseExtV1 update_d4_cgcp_service_accounts_ext(body)
Patches the service account key for external clients.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RegistrationPeriodGcpServiceAccountPatchRequestV1**](RegistrationPeriodGcpServiceAccountPatchRequestV1.md) |  | [required] |

### Return type

[**models::RegistrationPeriodGcpServiceAccountResponseExtV1**](registration.GCPServiceAccountResponseExtV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_discover_cloud_azure_account_client_id

> models::RegistrationPeriodAzureTenantConfigurationResponseV1 update_discover_cloud_azure_account_client_id(id, object_id, tenant_id)
Update an Azure service account in our system by with the user-created client_id created with the public key we've provided

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ClientID to use for the Service Principal associated with the customer's Azure account | [required] |
**object_id** | Option<**String**> | Object ID to use for the Service Principal associated with the customer's Azure account |  |
**tenant_id** | Option<**String**> | Tenant ID to update client ID for. Required if multiple tenants are registered. |  |

### Return type

[**models::RegistrationPeriodAzureTenantConfigurationResponseV1**](registration.AzureTenantConfigurationResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
