# \CertificateBasedExclusionsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cb_exclusions_period_create_period_v1**](CertificateBasedExclusionsApi.md#cb_exclusions_period_create_period_v1) | **POST** /exclusions/entities/cert-based-exclusions/v1 | Create new Certificate Based Exclusions.
[**cb_exclusions_period_delete_period_v1**](CertificateBasedExclusionsApi.md#cb_exclusions_period_delete_period_v1) | **DELETE** /exclusions/entities/cert-based-exclusions/v1 | Delete the exclusions by id
[**cb_exclusions_period_get_period_v1**](CertificateBasedExclusionsApi.md#cb_exclusions_period_get_period_v1) | **GET** /exclusions/entities/cert-based-exclusions/v1 | Find all exclusion IDs matching the query with filter
[**cb_exclusions_period_query_period_v1**](CertificateBasedExclusionsApi.md#cb_exclusions_period_query_period_v1) | **GET** /exclusions/queries/cert-based-exclusions/v1 | Search for cert-based exclusions.
[**cb_exclusions_period_update_period_v1**](CertificateBasedExclusionsApi.md#cb_exclusions_period_update_period_v1) | **PATCH** /exclusions/entities/cert-based-exclusions/v1 | Updates existing Certificate Based Exclusions
[**certificates_period_get_period_v1**](CertificateBasedExclusionsApi.md#certificates_period_get_period_v1) | **GET** /exclusions/entities/certificates/v1 | Retrieves certificate signing information for a file

## cb_exclusions_period_create_period_v1

> models::ApiPeriodCertBasedExclusionRespV1 cb_exclusions_period_create_period_v1(body)
Create new Certificate Based Exclusions.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodCertBasedExclusionsCreateReqV1**](ApiPeriodCertBasedExclusionsCreateReqV1.md) |  | [required] |

### Return type

[**models::ApiPeriodCertBasedExclusionRespV1**](api.CertBasedExclusionRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## cb_exclusions_period_delete_period_v1

> models::ApiPeriodCertBasedExclusionRespV1 cb_exclusions_period_delete_period_v1(ids, comment)
Delete the exclusions by id

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The ids of the exclusions to delete | [required] |
**comment** | Option<**String**> | The comment why these exclusions were deleted |  |

### Return type

[**models::ApiPeriodCertBasedExclusionRespV1**](api.CertBasedExclusionRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## cb_exclusions_period_get_period_v1

> models::ApiPeriodCertBasedExclusionRespV1 cb_exclusions_period_get_period_v1(ids)
Find all exclusion IDs matching the query with filter

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The ids of the exclusions to retrieve | [required] |

### Return type

[**models::ApiPeriodCertBasedExclusionRespV1**](api.CertBasedExclusionRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## cb_exclusions_period_query_period_v1

> models::MsaspecPeriodQueryResponse cb_exclusions_period_query_period_v1(filter, offset, limit, sort)
Search for cert-based exclusions.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to limit the results. |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-100] |  |
**sort** | Option<**String**> | The sort expression that should be used to sort the results. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## cb_exclusions_period_update_period_v1

> models::ApiPeriodCertBasedExclusionRespV1 cb_exclusions_period_update_period_v1(body)
Updates existing Certificate Based Exclusions

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodCertBasedExclusionsUpdateReqV1**](ApiPeriodCertBasedExclusionsUpdateReqV1.md) |  | [required] |

### Return type

[**models::ApiPeriodCertBasedExclusionRespV1**](api.CertBasedExclusionRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## certificates_period_get_period_v1

> models::ApiPeriodRespCertificatesV1 certificates_period_get_period_v1(ids)
Retrieves certificate signing information for a file

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The SHA256 Hash of the file to retrieve certificate signing info for | [required] |

### Return type

[**models::ApiPeriodRespCertificatesV1**](api.RespCertificatesV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
