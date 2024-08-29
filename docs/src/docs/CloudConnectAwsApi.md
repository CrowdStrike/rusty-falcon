# \CloudConnectAwsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_or_update_aws_settings**](CloudConnectAwsApi.md#create_or_update_aws_settings) | **POST** /cloud-connect-aws/entities/settings/v1 | Create or update Global Settings which are applicable to all provisioned AWS accounts
[**delete_aws_accounts**](CloudConnectAwsApi.md#delete_aws_accounts) | **DELETE** /cloud-connect-aws/entities/accounts/v1 | Delete a set of AWS Accounts by specifying their IDs
[**get_aws_accounts**](CloudConnectAwsApi.md#get_aws_accounts) | **GET** /cloud-connect-aws/entities/accounts/v1 | Retrieve a set of AWS Accounts by specifying their IDs
[**get_aws_settings**](CloudConnectAwsApi.md#get_aws_settings) | **GET** /cloud-connect-aws/combined/settings/v1 | Retrieve a set of Global Settings which are applicable to all provisioned AWS accounts
[**provision_aws_accounts**](CloudConnectAwsApi.md#provision_aws_accounts) | **POST** /cloud-connect-aws/entities/accounts/v1 | Provision AWS Accounts by specifying details about the accounts to provision
[**query_aws_accounts**](CloudConnectAwsApi.md#query_aws_accounts) | **GET** /cloud-connect-aws/combined/accounts/v1 | Search for provisioned AWS Accounts by providing an FQL filter and paging details. Returns a set of AWS accounts which match the filter criteria
[**query_aws_accounts_for_ids**](CloudConnectAwsApi.md#query_aws_accounts_for_ids) | **GET** /cloud-connect-aws/queries/accounts/v1 | Search for provisioned AWS Accounts by providing an FQL filter and paging details. Returns a set of AWS account IDs which match the filter criteria
[**update_aws_accounts**](CloudConnectAwsApi.md#update_aws_accounts) | **PATCH** /cloud-connect-aws/entities/accounts/v1 | Update AWS Accounts by specifying the ID of the account and details to update
[**verify_aws_account_access**](CloudConnectAwsApi.md#verify_aws_account_access) | **POST** /cloud-connect-aws/entities/verify-account-access/v1 | Performs an Access Verification check on the specified AWS Account IDs

## create_or_update_aws_settings

> models::ModelsPeriodCustomerConfigurationsV1 create_or_update_aws_settings(body)
Create or update Global Settings which are applicable to all provisioned AWS accounts

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodModifyAwsCustomerSettingsV1**](ModelsPeriodModifyAwsCustomerSettingsV1.md) |  | [required] |

### Return type

[**models::ModelsPeriodCustomerConfigurationsV1**](models.CustomerConfigurationsV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_aws_accounts

> models::ModelsPeriodBaseResponseV1 delete_aws_accounts(ids)
Delete a set of AWS Accounts by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | IDs of accounts to remove | [required] |

### Return type

[**models::ModelsPeriodBaseResponseV1**](models.BaseResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_aws_accounts

> models::ModelsPeriodAwsAccountsV1 get_aws_accounts(ids)
Retrieve a set of AWS Accounts by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | IDs of accounts to retrieve details | [required] |

### Return type

[**models::ModelsPeriodAwsAccountsV1**](models.AWSAccountsV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_aws_settings

> models::ModelsPeriodCustomerConfigurationsV1 get_aws_settings()
Retrieve a set of Global Settings which are applicable to all provisioned AWS accounts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModelsPeriodCustomerConfigurationsV1**](models.CustomerConfigurationsV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## provision_aws_accounts

> models::ModelsPeriodAwsAccountsV1 provision_aws_accounts(body, mode)
Provision AWS Accounts by specifying details about the accounts to provision

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodCreateAwsAccountsV1**](ModelsPeriodCreateAwsAccountsV1.md) |  | [required] |
**mode** | Option<**String**> | Mode for provisioning. Allowed values are `manual` or `cloudformation`. Defaults to manual if not defined. |  |[default to manual]

### Return type

[**models::ModelsPeriodAwsAccountsV1**](models.AWSAccountsV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_aws_accounts

> models::ModelsPeriodAwsAccountsV1 query_aws_accounts(limit, offset, sort, filter)
Search for provisioned AWS Accounts by providing an FQL filter and paging details. Returns a set of AWS accounts which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum records to return. [1-1000]. Defaults to 100. |  |[default to 100]
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**sort** | Option<**String**> | The property to sort by (e.g. alias.desc or state.asc) |  |
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |

### Return type

[**models::ModelsPeriodAwsAccountsV1**](models.AWSAccountsV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_aws_accounts_for_ids

> models::MsaPeriodQueryResponse query_aws_accounts_for_ids(limit, offset, sort, filter)
Search for provisioned AWS Accounts by providing an FQL filter and paging details. Returns a set of AWS account IDs which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum records to return. [1-1000]. Defaults to 100. |  |[default to 100]
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**sort** | Option<**String**> | The property to sort by (e.g. alias.desc or state.asc) |  |
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_aws_accounts

> models::ModelsPeriodAwsAccountsV1 update_aws_accounts(body)
Update AWS Accounts by specifying the ID of the account and details to update

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodUpdateAwsAccountsV1**](ModelsPeriodUpdateAwsAccountsV1.md) |  | [required] |

### Return type

[**models::ModelsPeriodAwsAccountsV1**](models.AWSAccountsV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## verify_aws_account_access

> models::ModelsPeriodVerifyAccessResponseV1 verify_aws_account_access(ids)
Performs an Access Verification check on the specified AWS Account IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | IDs of accounts to verify access on | [required] |

### Return type

[**models::ModelsPeriodVerifyAccessResponseV1**](models.VerifyAccessResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
