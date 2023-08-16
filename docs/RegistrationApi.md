# \RegistrationApi

All URIs are relative to *https://api.crowdstrike.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**register_cspm_snapshot_account**](RegistrationApi.md#register_cspm_snapshot_account) | **POST** /snapshots/entities/accounts/v1 | Register customer cloud account for snapshot scanning



## register_cspm_snapshot_account

> crate::models::ModelsPeriodAccountStatusResponse register_cspm_snapshot_account(body)
Register customer cloud account for snapshot scanning

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodAccountEntitiesInput**](ModelsPeriodAccountEntitiesInput.md) |  | [required] |

### Return type

[**crate::models::ModelsPeriodAccountStatusResponse**](models.AccountStatusResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

