# \CloudSnapshotsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_deployment_entity**](CloudSnapshotsApi.md#create_deployment_entity) | **POST** /snapshots/entities/deployments/v1 | Launch a snapshot scan for a given cloud asset
[**get_credentials_mixin0_mixin60**](CloudSnapshotsApi.md#get_credentials_mixin0_mixin60) | **GET** /snapshots/entities/image-registry-credentials/v1 | Gets the registry credentials
[**get_scan_report**](CloudSnapshotsApi.md#get_scan_report) | **GET** /snapshots/entities/scanreports/v1 | retrieve the scan report for an instance
[**read_deployments_combined**](CloudSnapshotsApi.md#read_deployments_combined) | **GET** /snapshots/combined/deployments/v1 | Retrieve snapshot jobs identified by the provided IDs
[**read_deployments_entities**](CloudSnapshotsApi.md#read_deployments_entities) | **GET** /snapshots/entities/deployments/v1 | Retrieve snapshot jobs identified by the provided IDs
[**register_cspm_snapshot_account**](CloudSnapshotsApi.md#register_cspm_snapshot_account) | **POST** /snapshots/entities/accounts/v1 | Register customer cloud account for snapshot scanning

## create_deployment_entity

> models::DeploymentsPeriodEntityResponse create_deployment_entity(body)
Launch a snapshot scan for a given cloud asset

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodCreateDeploymentInput**](ModelsPeriodCreateDeploymentInput.md) |  | [required] |

### Return type

[**models::DeploymentsPeriodEntityResponse**](deployments.EntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_credentials_mixin0_mixin60

> models::ModelsPeriodRegistryCredentialsResponse get_credentials_mixin0_mixin60()
Gets the registry credentials

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModelsPeriodRegistryCredentialsResponse**](models.RegistryCredentialsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_scan_report

> models::ScanreportsPeriodEntitiesResponse get_scan_report(ids)
retrieve the scan report for an instance

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | the instance identifiers to fetch the report for | [required] |

### Return type

[**models::ScanreportsPeriodEntitiesResponse**](scanreports.EntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_deployments_combined

> models::DeploymentsPeriodEntityResponse read_deployments_combined(filter, limit, offset, sort)
Retrieve snapshot jobs identified by the provided IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search snapshot jobs using a query in Falcon Query Language (FQL). Supported filters:  account_id,asset_identifier,cloud_provider,region,status |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The fields to sort the records on. Supported columns:  [account_id asset_identifier cloud_provider instance_type last_updated_timestamp region status] |  |

### Return type

[**models::DeploymentsPeriodEntityResponse**](deployments.EntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_deployments_entities

> models::DeploymentsPeriodEntityResponse read_deployments_entities(ids)
Retrieve snapshot jobs identified by the provided IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Search snapshot jobs by ids - The maximum amount is 100 IDs |  |

### Return type

[**models::DeploymentsPeriodEntityResponse**](deployments.EntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## register_cspm_snapshot_account

> models::ModelsPeriodAccountStatusResponse register_cspm_snapshot_account(body)
Register customer cloud account for snapshot scanning

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodAccountEntitiesInput**](ModelsPeriodAccountEntitiesInput.md) |  | [required] |

### Return type

[**models::ModelsPeriodAccountStatusResponse**](models.AccountStatusResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
