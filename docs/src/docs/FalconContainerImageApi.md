# \FalconContainerImageApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_registry_entities**](FalconContainerImageApi.md#create_registry_entities) | **POST** /container-security/entities/registries/v1 | Create a registry entity using the provided details
[**delete_registry_entities**](FalconContainerImageApi.md#delete_registry_entities) | **DELETE** /container-security/entities/registries/v1 | Delete the registry entity identified by the entity UUID
[**read_registry_entities**](FalconContainerImageApi.md#read_registry_entities) | **GET** /container-security/queries/registries/v1 | Retrieve registry entities identified by the customer id
[**read_registry_entities_by_uuid**](FalconContainerImageApi.md#read_registry_entities_by_uuid) | **GET** /container-security/entities/registries/v1 | Retrieve the registry entity identified by the entity UUID
[**update_registry_entities**](FalconContainerImageApi.md#update_registry_entities) | **PATCH** /container-security/entities/registries/v1 | Update the registry entity, as identified by the entity UUID, using the provided details

## create_registry_entities

> models::DomainPeriodExternalRegistryResponse create_registry_entities(body)
Create a registry entity using the provided details

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RegistryassessmentPeriodExternalRegistryPayload**](RegistryassessmentPeriodExternalRegistryPayload.md) |  | [required] |

### Return type

[**models::DomainPeriodExternalRegistryResponse**](domain.ExternalRegistryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_registry_entities

> models::DomainPeriodExternalRegistryListResponse delete_registry_entities(ids)
Delete the registry entity identified by the entity UUID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Registry entity UUID | [required] |

### Return type

[**models::DomainPeriodExternalRegistryListResponse**](domain.ExternalRegistryListResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_registry_entities

> models::DomainPeriodExternalQueryResponse read_registry_entities(limit, offset, sort)
Retrieve registry entities identified by the customer id

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The field to sort on, e.g. id.desc or id.asc. |  |

### Return type

[**models::DomainPeriodExternalQueryResponse**](domain.ExternalQueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_registry_entities_by_uuid

> models::DomainPeriodExternalRegistryListResponse read_registry_entities_by_uuid(ids)
Retrieve the registry entity identified by the entity UUID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Registry entity UUID | [required] |

### Return type

[**models::DomainPeriodExternalRegistryListResponse**](domain.ExternalRegistryListResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_registry_entities

> models::DomainPeriodExternalRegistryResponse update_registry_entities(id, body)
Update the registry entity, as identified by the entity UUID, using the provided details

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Registry entity UUID | [required] |
**body** | [**RegistryassessmentPeriodExternalRegistryPatchPayload**](RegistryassessmentPeriodExternalRegistryPatchPayload.md) |  | [required] |

### Return type

[**models::DomainPeriodExternalRegistryResponse**](domain.ExternalRegistryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
