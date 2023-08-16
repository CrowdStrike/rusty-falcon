# \InventoriesApi

All URIs are relative to *https://api.crowdstrike.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_inventory**](InventoriesApi.md#create_inventory) | **POST** /snapshots/entities/inventories/v1 | Create inventory from data received from snapshot



## create_inventory

> crate::models::CommonPeriodEntitiesResponse create_inventory(body)
Create inventory from data received from snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodSnapshotInventoryPayload**](ModelsPeriodSnapshotInventoryPayload.md) |  | [required] |

### Return type

[**crate::models::CommonPeriodEntitiesResponse**](common.EntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

