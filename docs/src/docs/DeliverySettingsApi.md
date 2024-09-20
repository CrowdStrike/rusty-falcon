# \DeliverySettingsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_delivery_settings**](DeliverySettingsApi.md#get_delivery_settings) | **GET** /delivery-settings/entities/delivery-settings/v1 | Get Delivery Settings
[**post_delivery_settings**](DeliverySettingsApi.md#post_delivery_settings) | **POST** /delivery-settings/entities/delivery-settings/v1 | Create Delivery Settings

## get_delivery_settings

> models::ModelsPeriodDeliverySettingsEntityResponse get_delivery_settings()
Get Delivery Settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModelsPeriodDeliverySettingsEntityResponse**](models.DeliverySettingsEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## post_delivery_settings

> models::ModelsPeriodDeliverySettingsEntityResponse post_delivery_settings(body)
Create Delivery Settings

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodDeliverySettingsRequest**](ModelsPeriodDeliverySettingsRequest.md) |  | [required] |

### Return type

[**models::ModelsPeriodDeliverySettingsEntityResponse**](models.DeliverySettingsEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
