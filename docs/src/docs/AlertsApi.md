# \AlertsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_queries_alerts_v1**](AlertsApi.md#get_queries_alerts_v1) | **GET** /alerts/queries/alerts/v1 | retrieves all Alerts ids that match a given query
[**get_queries_alerts_v2**](AlertsApi.md#get_queries_alerts_v2) | **GET** /alerts/queries/alerts/v2 | retrieves all Alerts ids that match a given query
[**patch_entities_alerts_v2**](AlertsApi.md#patch_entities_alerts_v2) | **PATCH** /alerts/entities/alerts/v2 | Perform actions on Alerts identified by composite ID(s) in request. Each action has a name and a description which describes what the action does. If a request adds and removes tag in a single request, the order of processing would be to remove tags before adding new ones in.
[**patch_entities_alerts_v3**](AlertsApi.md#patch_entities_alerts_v3) | **PATCH** /alerts/entities/alerts/v3 | Perform actions on Alerts identified by composite ID(s) in request. Each action has a name and a description which describes what the action does. If a request adds and removes tag in a single request, the order of processing would be to remove tags before adding new ones in.
[**post_aggregates_alerts_v1**](AlertsApi.md#post_aggregates_alerts_v1) | **POST** /alerts/aggregates/alerts/v1 | retrieves aggregate values for Alerts across all CIDs
[**post_aggregates_alerts_v2**](AlertsApi.md#post_aggregates_alerts_v2) | **POST** /alerts/aggregates/alerts/v2 | retrieves aggregate values for Alerts across all CIDs
[**post_entities_alerts_v1**](AlertsApi.md#post_entities_alerts_v1) | **POST** /alerts/entities/alerts/v1 | retrieves all Alerts given their ids
[**post_entities_alerts_v2**](AlertsApi.md#post_entities_alerts_v2) | **POST** /alerts/entities/alerts/v2 | retrieves all Alerts given their composite ids

## get_queries_alerts_v1

> models::DetectsapiPeriodAlertQueryResponse get_queries_alerts_v1(offset, limit, sort, filter, q)
retrieves all Alerts ids that match a given query

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The first detection to return, where `0` is the latest detection. Use with the `offset` parameter to manage pagination of results. |  |
**limit** | Option<**i32**> | The maximum number of detections to return in this response (default: 100; max: 10000). Use with the `offset` parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort parameter takes the form <field|direction>. Direction can be either `asc` (ascending) or `desc` (descending) order. For example: `status|asc` or `status|desc`.  The sorting fields can be any keyword field that is part of #domain.Alert except for the text based fields. Most commonly used fields are status, cid, aggregate_id, timestamp, created_timestamp, updated_timestamp, assigned_to_name, assigned_to_uid, assigned_to_uuid, show_in_ui, tactic_id, tactic, technique, technique_id, pattern_id, product, comment, tags If the fields are missing from the Alerts, the service will fallback to its default ordering  |  |
**filter** | Option<**String**> | Filter Alerts using a query in Falcon Query Language (FQL).Filter fields can be any keyword field that is part of #domain.Alert  An asterisk wildcard `*` includes all results.   Empty value means to not filter on anything. Most commonly used filter fields that supports exact match: cid, id, aggregate_id, product, type, pattern_id, platform ... Most commonly used filter fields that supports wildcard (*): assigned_to_name, assigned_to_uuid, tactic_id, technique ... Most commonly filter fields that supports range comparisons (>, <, >=, <=): severity, created_timestamp, timestamp, updated_timestamp... All filter fields and operations support negation (!).   The full list of valid filter options is extensive. Review it in our [documentation inside the Falcon console](https://falcon.crowdstrike.com/documentation/45/falcon-query-language-fql). |  |
**q** | Option<**String**> | Search all alert metadata for the provided string |  |

### Return type

[**models::DetectsapiPeriodAlertQueryResponse**](detectsapi.AlertQueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_queries_alerts_v2

> models::DetectsapiPeriodAlertQueryResponse get_queries_alerts_v2(include_hidden, offset, limit, sort, filter, q)
retrieves all Alerts ids that match a given query

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_hidden** | Option<**bool**> | allows previously hidden alerts to be retrieved |  |[default to true]
**offset** | Option<**i32**> | The first detection to return, where `0` is the latest detection. Use with the `offset` parameter to manage pagination of results. |  |
**limit** | Option<**i32**> | The maximum number of detections to return in this response (default: 100; max: 10000). Use with the `offset` parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort parameter takes the form <field|direction>. Direction can be either `asc` (ascending) or `desc` (descending) order. For example: `status|asc` or `status|desc`.  The sorting fields can be any keyword field that is part of #domain.Alert except for the text based fields. Most commonly used fields are status, cid, aggregate_id, timestamp, created_timestamp, updated_timestamp, assigned_to_name, assigned_to_uid, assigned_to_uuid, show_in_ui, tactic_id, tactic, technique, technique_id, pattern_id, product, comment, tags If the fields are missing from the Alerts, the service will fallback to its default ordering  |  |
**filter** | Option<**String**> | Filter Alerts using a query in Falcon Query Language (FQL).Filter fields can be any keyword field that is part of #domain.Alert  An asterisk wildcard `*` includes all results.   Empty value means to not filter on anything. Most commonly used filter fields that supports exact match: cid, id, aggregate_id, product, type, pattern_id, platform ... Most commonly used filter fields that supports wildcard (*): assigned_to_name, assigned_to_uuid, tactic_id, technique ... Most commonly filter fields that supports range comparisons (>, <, >=, <=): severity, created_timestamp, timestamp, updated_timestamp... All filter fields and operations support negation (!).   The full list of valid filter options is extensive. Review it in our [documentation inside the Falcon console](https://falcon.crowdstrike.com/documentation/45/falcon-query-language-fql). |  |
**q** | Option<**String**> | Search all alert metadata for the provided string |  |

### Return type

[**models::DetectsapiPeriodAlertQueryResponse**](detectsapi.AlertQueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## patch_entities_alerts_v2

> models::DetectsapiPeriodResponseFields patch_entities_alerts_v2(body)
Perform actions on Alerts identified by composite ID(s) in request. Each action has a name and a description which describes what the action does. If a request adds and removes tag in a single request, the order of processing would be to remove tags before adding new ones in.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DetectsapiPeriodPatchEntitiesAlertsV2Request**](DetectsapiPeriodPatchEntitiesAlertsV2Request.md) | `ids` - IDs of Alerts to modify.  `action_parameters` values - `assign_to_uuid`  - Assign Alert to user UUID, such as `00000000-0000-0000-0000-000000000000` - `assign_to_user_id`  - Assign Alert to user ID, such as `user@example.com` - `assign_to_name`  - Assign Alert to username, such as `John Doe` - `unassign`  - Unassign Alert clears out the assigned user UUID, user ID, and username. - `add_tag`   - Add a tag to the Alert. - `remove_tag`  - Remove a tag from the Alert. - `remove_tags_by_prefix`  - Remove tags from the Alert based on the prefix. - `append_comment`  - Comments are displayed with the Alert in Falcon and are usually used to provide context or notes for other Falcon users. An Alert can have multiple comments over time. - `update_status` values  - `new`  - `in_progress`  - `reopened`  - `closed` - `show_in_ui` values  - `true`: This alert is displayed in Falcon  - `false`: This alert is not displayed in Falcon.  | [required] |

### Return type

[**models::DetectsapiPeriodResponseFields**](detectsapi.ResponseFields.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## patch_entities_alerts_v3

> models::DetectsapiPeriodResponseFields patch_entities_alerts_v3(body, include_hidden)
Perform actions on Alerts identified by composite ID(s) in request. Each action has a name and a description which describes what the action does. If a request adds and removes tag in a single request, the order of processing would be to remove tags before adding new ones in.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DetectsapiPeriodPatchEntitiesAlertsV3Request**](DetectsapiPeriodPatchEntitiesAlertsV3Request.md) | `composite_ids` - CompositeIDs of Alerts to modify.  `action_parameters` values - `assign_to_uuid`  - Assign Alert to user UUID, such as `00000000-0000-0000-0000-000000000000` - `assign_to_user_id`  - Assign Alert to user ID, such as `user@example.com` - `assign_to_name`  - Assign Alert to username, such as `John Doe` - `unassign`  - Unassign Alert clears out the assigned user UUID, user ID, and username. - `add_tag`   - Add a tag to the Alert. - `remove_tag`  - Remove a tag from the Alert. - `remove_tags_by_prefix`  - Remove tags from the Alert based on the prefix. - `append_comment`  - Comments are displayed with the Alert in Falcon and are usually used to provide context or notes for other Falcon users. An Alert can have multiple comments over time. - `update_status` values  - `new`  - `in_progress`  - `reopened`  - `closed` - `show_in_ui` values  - `true`: This alert is displayed in Falcon  - `false`: This alert is not displayed in Falcon.  | [required] |
**include_hidden** | Option<**bool**> | allows previously hidden alerts to be retrieved |  |[default to true]

### Return type

[**models::DetectsapiPeriodResponseFields**](detectsapi.ResponseFields.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## post_aggregates_alerts_v1

> models::DetectsapiPeriodAggregatesResponse post_aggregates_alerts_v1(body)
retrieves aggregate values for Alerts across all CIDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::DetectsapiPeriodAggregateAlertQueryRequest>**](detectsapi.AggregateAlertQueryRequest.md) | request body takes a list of aggregate-alert query requests | [required] |

### Return type

[**models::DetectsapiPeriodAggregatesResponse**](detectsapi.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## post_aggregates_alerts_v2

> models::DetectsapiPeriodAggregatesResponse post_aggregates_alerts_v2(body, include_hidden)
retrieves aggregate values for Alerts across all CIDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::DetectsapiPeriodAggregateAlertQueryRequest>**](detectsapi.AggregateAlertQueryRequest.md) | request body takes a list of aggregate-alert query requests | [required] |
**include_hidden** | Option<**bool**> | allows previously hidden alerts to be retrieved |  |[default to true]

### Return type

[**models::DetectsapiPeriodAggregatesResponse**](detectsapi.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## post_entities_alerts_v1

> models::DetectsapiPeriodPostEntitiesAlertsV1ResponseSwagger post_entities_alerts_v1(body)
retrieves all Alerts given their ids

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DetectsapiPeriodPostEntitiesAlertsV1Request**](DetectsapiPeriodPostEntitiesAlertsV1Request.md) |  | [required] |

### Return type

[**models::DetectsapiPeriodPostEntitiesAlertsV1ResponseSwagger**](detectsapi.PostEntitiesAlertsV1ResponseSwagger.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## post_entities_alerts_v2

> models::DetectsapiPeriodPostEntitiesAlertsV2ResponseSwagger post_entities_alerts_v2(body, include_hidden)
retrieves all Alerts given their composite ids

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DetectsapiPeriodPostEntitiesAlertsV2Request**](DetectsapiPeriodPostEntitiesAlertsV2Request.md) |  | [required] |
**include_hidden** | Option<**bool**> | allows previously hidden alerts to be retrieved |  |[default to true]

### Return type

[**models::DetectsapiPeriodPostEntitiesAlertsV2ResponseSwagger**](detectsapi.PostEntitiesAlertsV2ResponseSwagger.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
