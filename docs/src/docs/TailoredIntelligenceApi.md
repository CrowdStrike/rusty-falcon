# \TailoredIntelligenceApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_events_body**](TailoredIntelligenceApi.md#get_events_body) | **GET** /ti/events/entities/events-full-body/v2 | Get event body for the provided event ID
[**get_events_entities**](TailoredIntelligenceApi.md#get_events_entities) | **POST** /ti/events/entities/events/GET/v2 | Get events entities for specified ids.
[**get_rules_entities**](TailoredIntelligenceApi.md#get_rules_entities) | **POST** /ti/rules/entities/rules/GET/v2 | Get rules entities for specified ids.
[**query_events**](TailoredIntelligenceApi.md#query_events) | **GET** /ti/events/queries/events/v2 | Get events ids that match the provided filter criteria.
[**query_rules**](TailoredIntelligenceApi.md#query_rules) | **GET** /ti/rules/queries/rules/v2 | Get rules ids that match the provided filter criteria.

## get_events_body

> Vec<i32> get_events_body(id)
Get event body for the provided event ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Return the event body for event id. | [required] |

### Return type

**Vec<i32>**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_events_entities

> models::DomainPeriodEventEntitiesResponse get_events_entities(body)
Get events entities for specified ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::DomainPeriodEventEntitiesResponse**](domain.EventEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_rules_entities

> models::DomainPeriodRuleEntitiesResponse get_rules_entities(body)
Get rules entities for specified ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::DomainPeriodRuleEntitiesResponse**](domain.RuleEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_events

> models::DomainPeriodQueryResponse query_events(offset, limit, sort, filter, q)
Get events ids that match the provided filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |
**sort** | Option<**String**> | Possible order by fields: source_type, created_date, updated_date. Ex: 'updated_date|desc'. |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. Special value '*' means to not filter on anything. |  |
**q** | Option<**String**> | Match phrase_prefix query criteria; included fields:_all (all filter string fields indexed). |  |

### Return type

[**models::DomainPeriodQueryResponse**](domain.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_rules

> models::DomainPeriodQueryResponse query_rules(offset, limit, sort, filter, q)
Get rules ids that match the provided filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |
**sort** | Option<**String**> | Possible order by fields: name, value, rule_type, customer_id, created_date, updated_date. Ex: 'updated_date|asc'. |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. Special value '*' means to not filter on anything. |  |
**q** | Option<**String**> | Match phrase_prefix query criteria; included fields:_all (all filter string fields indexed). |  |

### Return type

[**models::DomainPeriodQueryResponse**](domain.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
