# \InstallationTokensApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**audit_events_query**](InstallationTokensApi.md#audit_events_query) | **GET** /installation-tokens/queries/audit-events/v1 | Search for audit events by providing an FQL filter and paging details.
[**audit_events_read**](InstallationTokensApi.md#audit_events_read) | **GET** /installation-tokens/entities/audit-events/v1 | Gets the details of one or more audit events by id.
[**customer_settings_read**](InstallationTokensApi.md#customer_settings_read) | **GET** /installation-tokens/entities/customer-settings/v1 | Check current installation token settings.
[**tokens_create**](InstallationTokensApi.md#tokens_create) | **POST** /installation-tokens/entities/tokens/v1 | Creates a token.
[**tokens_delete**](InstallationTokensApi.md#tokens_delete) | **DELETE** /installation-tokens/entities/tokens/v1 | Deletes a token immediately. To revoke a token, use PATCH /installation-tokens/entities/tokens/v1 instead.
[**tokens_query**](InstallationTokensApi.md#tokens_query) | **GET** /installation-tokens/queries/tokens/v1 | Search for tokens by providing an FQL filter and paging details.
[**tokens_read**](InstallationTokensApi.md#tokens_read) | **GET** /installation-tokens/entities/tokens/v1 | Gets the details of one or more tokens by id.
[**tokens_update**](InstallationTokensApi.md#tokens_update) | **PATCH** /installation-tokens/entities/tokens/v1 | Updates one or more tokens. Use this endpoint to edit labels, change expiration, revoke, or restore.

## audit_events_query

> models::MsaspecPeriodQueryResponse audit_events_query(offset, limit, sort, filter)
Search for audit events by providing an FQL filter and paging details.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The offset to start retrieving records from. |  |
**limit** | Option<**i32**> | The maximum records to return. [1-1000]. Defaults to 50. |  |
**sort** | Option<**String**> | The property to sort by (e.g. timestamp.desc). |  |
**filter** | Option<**String**> | The filter expression that should be used to limit the results (e.g., `action:'token_create'`). |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## audit_events_read

> models::ApiPeriodAuditEventDetailsResponseV1 audit_events_read(ids)
Gets the details of one or more audit events by id.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | IDs of audit events to retrieve details for |  |

### Return type

[**models::ApiPeriodAuditEventDetailsResponseV1**](api.auditEventDetailsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## customer_settings_read

> models::ApiPeriodCustomerSettingsResponseV1 customer_settings_read()
Check current installation token settings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiPeriodCustomerSettingsResponseV1**](api.customerSettingsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## tokens_create

> models::ApiPeriodTokenDetailsResponseV1 tokens_create(body)
Creates a token.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodTokenCreateRequestV1**](ApiPeriodTokenCreateRequestV1.md) |  | [required] |

### Return type

[**models::ApiPeriodTokenDetailsResponseV1**](api.TokenDetailsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## tokens_delete

> models::MsaspecPeriodResponseFields tokens_delete(ids)
Deletes a token immediately. To revoke a token, use PATCH /installation-tokens/entities/tokens/v1 instead.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The token ids to delete. | [required] |

### Return type

[**models::MsaspecPeriodResponseFields**](msaspec.ResponseFields.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## tokens_query

> models::MsaspecPeriodQueryResponse tokens_query(offset, limit, sort, filter)
Search for tokens by providing an FQL filter and paging details.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The offset to start retrieving records from. |  |
**limit** | Option<**i32**> | The maximum records to return. [1-1000]. Defaults to 50. |  |
**sort** | Option<**String**> | The property to sort by (e.g. created_timestamp.desc). |  |
**filter** | Option<**String**> | The filter expression that should be used to limit the results (e.g., `status:'valid'`). |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## tokens_read

> models::ApiPeriodTokenDetailsResponseV1 tokens_read(ids)
Gets the details of one or more tokens by id.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | IDs of tokens to retrieve details for |  |

### Return type

[**models::ApiPeriodTokenDetailsResponseV1**](api.TokenDetailsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## tokens_update

> models::MsaspecPeriodQueryResponse tokens_update(ids, body)
Updates one or more tokens. Use this endpoint to edit labels, change expiration, revoke, or restore.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The token ids to update. | [required] |
**body** | [**ApiPeriodTokenPatchRequestV1**](ApiPeriodTokenPatchRequestV1.md) |  | [required] |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
