# \IncidentsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**crowd_score**](IncidentsApi.md#crowd_score) | **GET** /incidents/combined/crowdscores/v1 | Query environment wide CrowdScore and return the entity data
[**get_behaviors**](IncidentsApi.md#get_behaviors) | **POST** /incidents/entities/behaviors/GET/v1 | Get details on behaviors by providing behavior IDs
[**get_incidents**](IncidentsApi.md#get_incidents) | **POST** /incidents/entities/incidents/GET/v1 | Get details on incidents by providing incident IDs
[**perform_incident_action**](IncidentsApi.md#perform_incident_action) | **POST** /incidents/entities/incident-actions/v1 | Perform a set of actions on one or more incidents, such as adding tags or comments or updating the incident name or description
[**query_behaviors**](IncidentsApi.md#query_behaviors) | **GET** /incidents/queries/behaviors/v1 | Search for behaviors by providing an FQL filter, sorting, and paging details
[**query_incidents**](IncidentsApi.md#query_incidents) | **GET** /incidents/queries/incidents/v1 | Search for incidents by providing an FQL filter, sorting, and paging details

## crowd_score

> models::DomainPeriodMsaEnvironmentScoreResponse crowd_score(filter, offset, limit, sort)
Query environment wide CrowdScore and return the entity data

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**i32**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | The maximum records to return. [1-2500] |  |
**sort** | Option<**String**> | The property to sort on, followed by a dot (.), followed by the sort direction, either \"asc\" or \"desc\". |  |

### Return type

[**models::DomainPeriodMsaEnvironmentScoreResponse**](domain.MsaEnvironmentScoreResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_behaviors

> models::DomainPeriodMsaExternalBehaviorResponse get_behaviors(body)
Get details on behaviors by providing behavior IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::DomainPeriodMsaExternalBehaviorResponse**](domain.MsaExternalBehaviorResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_incidents

> models::DomainPeriodMsaExternalIncidentResponse get_incidents(body)
Get details on incidents by providing incident IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::DomainPeriodMsaExternalIncidentResponse**](domain.MsaExternalIncidentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## perform_incident_action

> models::DomainPeriodMsaIncidentPerformActionResponse perform_incident_action(body, update_detects, overwrite_detects)
Perform a set of actions on one or more incidents, such as adding tags or comments or updating the incident name or description

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodEntityActionRequest**](DomainPeriodEntityActionRequest.md) | Incident Update request body containing minimum 1 and maximum 5000 Incident ID(s) and action param(s) to be performed action against. | [required] |
**update_detects** | Option<**bool**> | If true, update assigned-to-uuid and or status of detections associated with the incident(s). Defaults to false |  |[default to false]
**overwrite_detects** | Option<**bool**> | If true and update-detects is true, the assigned-to-uuid or status for ALL detections associated with the incident(s) will be overwritten. If false, only detects that have default values for assigned-to-uuid and/or status will be updated. Defaults to false. Ignored if 'update-detects' is missing or false. |  |[default to false]

### Return type

[**models::DomainPeriodMsaIncidentPerformActionResponse**](domain.MsaIncidentPerformActionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_behaviors

> models::MsaPeriodQueryResponse query_behaviors(filter, offset, limit, sort)
Search for behaviors by providing an FQL filter, sorting, and paging details

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**i32**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |
**sort** | Option<**String**> | The property to sort on, followed by a dot (.), followed by the sort direction, either \"asc\" or \"desc\". |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_incidents

> models::DomainPeriodMsaIncidentQueryResponse query_incidents(sort, filter, offset, limit)
Search for incidents by providing an FQL filter, sorting, and paging details

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | The property to sort on, followed by a dot (.), followed by the sort direction, either \"asc\" or \"desc\". |  |
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**i32**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |

### Return type

[**models::DomainPeriodMsaIncidentQueryResponse**](domain.MsaIncidentQueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
