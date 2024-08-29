# \InstallationTokensSettingsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_settings_update**](InstallationTokensSettingsApi.md#customer_settings_update) | **PATCH** /installation-tokens/entities/customer-settings/v1 | Update installation token settings.

## customer_settings_update

> models::MsaspecPeriodQueryResponse customer_settings_update(body)
Update installation token settings.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodCustomerSettingsPatchRequestV1**](ApiPeriodCustomerSettingsPatchRequestV1.md) |  | [required] |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
