# \ThreatgraphApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**combined_edges_get**](ThreatgraphApi.md#combined_edges_get) | **GET** /threatgraph/combined/edges/v1 | Retrieve edges for a given vertex id.  One edge type must be specified
[**combined_ran_on_get**](ThreatgraphApi.md#combined_ran_on_get) | **GET** /threatgraph/combined/ran-on/v1 | Look up instances of indicators such as hashes, domain names, and ip addresses that have been seen on devices in your environment.
[**combined_summary_get**](ThreatgraphApi.md#combined_summary_get) | **GET** /threatgraph/combined/{vertex-type}/summary/v1 | Retrieve summary for a given vertex ID
[**entities_vertices_get**](ThreatgraphApi.md#entities_vertices_get) | **GET** /threatgraph/entities/{vertex-type}/v1 | Retrieve metadata for a given vertex ID. Note: This is a legacy endpoint used by CrowdStrike Store partners prior to release of the ThreatGraph OAuth 2.0 APIs. If you’re not currently using this endpoint, use the /v2 endpoint instead.
[**entities_vertices_getv2**](ThreatgraphApi.md#entities_vertices_getv2) | **GET** /threatgraph/entities/{vertex-type}/v2 | Retrieve metadata for a given vertex ID
[**queries_edgetypes_get**](ThreatgraphApi.md#queries_edgetypes_get) | **GET** /threatgraph/queries/edge-types/v1 | Show all available edge types

## combined_edges_get

> combined_edges_get(ids, edge_type, limit, offset, direction, scope, nano)
Retrieve edges for a given vertex id.  One edge type must be specified

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Vertex ID to get details for.  Only one value is supported | [required] |
**edge_type** | **String** | The type of edges that you would like to retrieve | [required] |
**limit** | Option<**i32**> | How many edges to return in a single request [1-100] |  |[default to 100]
**offset** | Option<**String**> | The offset to use to retrieve the next page of results |  |
**direction** | Option<**String**> | The direction of edges that you would like to retrieve. |  |
**scope** | Option<**String**> | Scope of the request |  |[default to device]
**nano** | Option<**bool**> | Return nano-precision entity timestamps |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## combined_ran_on_get

> combined_ran_on_get(value, r#type, limit, offset, nano)
Look up instances of indicators such as hashes, domain names, and ip addresses that have been seen on devices in your environment.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**value** | **String** | The value of the indicator to search by. | [required] |
**r#type** | **String** | The type of indicator that you would like to retrieve | [required] |
**limit** | Option<**i32**> | How many edges to return in a single request [1-100] |  |[default to 100]
**offset** | Option<**String**> | The offset to use to retrieve the next page of results |  |
**nano** | Option<**bool**> | Return nano-precision entity timestamps |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## combined_summary_get

> combined_summary_get(vertex_type, ids, scope, nano)
Retrieve summary for a given vertex ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vertex_type** | **String** | Type of vertex to get properties for | [required] |
**ids** | [**Vec<String>**](String.md) | Vertex ID to get details for | [required] |
**scope** | Option<**String**> | Scope of the request |  |[default to device]
**nano** | Option<**bool**> | Return nano-precision entity timestamps |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## entities_vertices_get

> models::ThreatgraphPeriodVertexDetailsResponse entities_vertices_get(vertex_type, ids, scope, nano)
Retrieve metadata for a given vertex ID. Note: This is a legacy endpoint used by CrowdStrike Store partners prior to release of the ThreatGraph OAuth 2.0 APIs. If you’re not currently using this endpoint, use the /v2 endpoint instead.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vertex_type** | **String** | Type of vertex to get properties for | [required] |
**ids** | [**Vec<String>**](String.md) | Vertex ID to get details for | [required] |
**scope** | Option<**String**> | Scope of the request |  |[default to device]
**nano** | Option<**bool**> | Return nano-precision entity timestamps |  |[default to false]

### Return type

[**models::ThreatgraphPeriodVertexDetailsResponse**](threatgraph.VertexDetailsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## entities_vertices_getv2

> models::ThreatgraphPeriodVertexDetailsResponse entities_vertices_getv2(vertex_type, ids, scope, nano)
Retrieve metadata for a given vertex ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vertex_type** | **String** | Type of vertex to get properties for | [required] |
**ids** | [**Vec<String>**](String.md) | Vertex ID to get details for | [required] |
**scope** | Option<**String**> | Scope of the request |  |[default to device]
**nano** | Option<**bool**> | Return nano-precision entity timestamps |  |[default to false]

### Return type

[**models::ThreatgraphPeriodVertexDetailsResponse**](threatgraph.VertexDetailsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## queries_edgetypes_get

> queries_edgetypes_get()
Show all available edge types

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
