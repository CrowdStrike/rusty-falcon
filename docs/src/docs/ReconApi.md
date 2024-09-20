# \ReconApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**aggregate_notifications_exposed_data_records_v1**](ReconApi.md#aggregate_notifications_exposed_data_records_v1) | **POST** /recon/aggregates/notifications-exposed-data-records/GET/v1 | Get notification exposed data record aggregates as specified via JSON in request body. The valid aggregation fields are: [cid notification_id created_date rule.id rule.name rule.topic source_category site author file.name credential_status bot.operating_system.hardware_id bot.bot_id]
[**aggregate_notifications_v1**](ReconApi.md#aggregate_notifications_v1) | **POST** /recon/aggregates/notifications/GET/v1 | Get notification aggregates as specified via JSON in request body.
[**create_actions_v1**](ReconApi.md#create_actions_v1) | **POST** /recon/entities/actions/v1 | Create actions for a monitoring rule. Accepts a list of actions that will be attached to the monitoring rule.
[**create_export_jobs_v1**](ReconApi.md#create_export_jobs_v1) | **POST** /recon/entities/exports/v1 | Launch asynchronous export job. Use the job ID to poll the status of the job using GET /entities/exports/v1.
[**create_rules_v1**](ReconApi.md#create_rules_v1) | **POST** /recon/entities/rules/v1 | Create monitoring rules.
[**delete_action_v1**](ReconApi.md#delete_action_v1) | **DELETE** /recon/entities/actions/v1 | Delete an action from a monitoring rule based on the action ID.
[**delete_export_jobs_v1**](ReconApi.md#delete_export_jobs_v1) | **DELETE** /recon/entities/exports/v1 | Delete export jobs (and their associated file(s)) based on their IDs.
[**delete_notifications_v1**](ReconApi.md#delete_notifications_v1) | **DELETE** /recon/entities/notifications/v1 | Delete notifications based on IDs. Notifications cannot be recovered after they are deleted.
[**delete_rules_v1**](ReconApi.md#delete_rules_v1) | **DELETE** /recon/entities/rules/v1 | Delete monitoring rules.
[**get_actions_v1**](ReconApi.md#get_actions_v1) | **GET** /recon/entities/actions/v1 | Get actions based on their IDs. IDs can be retrieved using the GET /queries/actions/v1 endpoint.
[**get_export_jobs_v1**](ReconApi.md#get_export_jobs_v1) | **GET** /recon/entities/exports/v1 | Get the status of export jobs based on their IDs. Export jobs can be launched by calling POST /entities/exports/v1. When a job is complete, use the job ID to download the file(s) associated with it using GET entities/export-files/v1.
[**get_file_content_for_export_jobs_v1**](ReconApi.md#get_file_content_for_export_jobs_v1) | **GET** /recon/entities/export-files/v1 | Download the file associated with a job ID.
[**get_notifications_detailed_translated_v1**](ReconApi.md#get_notifications_detailed_translated_v1) | **GET** /recon/entities/notifications-detailed-translated/v1 | Get detailed notifications based on their IDs. These include the translated raw intelligence content that generated the match or part of it.
[**get_notifications_detailed_v1**](ReconApi.md#get_notifications_detailed_v1) | **GET** /recon/entities/notifications-detailed/v1 | Get detailed notifications based on their IDs. These include the raw intelligence content that generated the match or part of it.
[**get_notifications_exposed_data_records_v1**](ReconApi.md#get_notifications_exposed_data_records_v1) | **GET** /recon/entities/notifications-exposed-data-records/v1 | Get notifications exposed data records based on their IDs. IDs can be retrieved using the GET /queries/notifications-exposed-data-records/v1 endpoint. The associate notification can be fetched using the /entities/notifications/v* endpoints
[**get_notifications_translated_v1**](ReconApi.md#get_notifications_translated_v1) | **GET** /recon/entities/notifications-translated/v1 | Get notifications based on their IDs. IDs can be retrieved using the GET /queries/notifications/v1 endpoint. This endpoint will return translated notification content. The only target language available is English.
[**get_notifications_v1**](ReconApi.md#get_notifications_v1) | **GET** /recon/entities/notifications/v1 | Get notifications based on their IDs. IDs can be retrieved using the GET /queries/notifications/v1 endpoint.
[**get_rules_v1**](ReconApi.md#get_rules_v1) | **GET** /recon/entities/rules/v1 | Get monitoring rules based on their IDs. IDs can be retrieved using the GET /queries/rules/v1 endpoint.
[**preview_rule_v1**](ReconApi.md#preview_rule_v1) | **POST** /recon/aggregates/rules-preview/GET/v1 | Preview rules notification count and distribution. This will return aggregations on: channel, count, site.
[**query_actions_v1**](ReconApi.md#query_actions_v1) | **GET** /recon/queries/actions/v1 | Query actions based on provided criteria. Use the IDs from this response to get the action entities on GET /entities/actions/v1.
[**query_notifications_exposed_data_records_v1**](ReconApi.md#query_notifications_exposed_data_records_v1) | **GET** /recon/queries/notifications-exposed-data-records/v1 | Query notifications exposed data records based on provided criteria. Use the IDs from this response to get the notification +entities on GET /entities/notifications-exposed-data-records/v1
[**query_notifications_v1**](ReconApi.md#query_notifications_v1) | **GET** /recon/queries/notifications/v1 | Query notifications based on provided criteria. Use the IDs from this response to get the notification +entities on GET /entities/notifications/v1, GET /entities/notifications-detailed/v1, +GET /entities/notifications-translated/v1 or GET /entities/notifications-detailed-translated/v1.
[**query_rules_v1**](ReconApi.md#query_rules_v1) | **GET** /recon/queries/rules/v1 | Query monitoring rules based on provided criteria. Use the IDs from this response to fetch the rules on /entities/rules/v1.
[**update_action_v1**](ReconApi.md#update_action_v1) | **PATCH** /recon/entities/actions/v1 | Update an action for a monitoring rule.
[**update_notifications_v1**](ReconApi.md#update_notifications_v1) | **PATCH** /recon/entities/notifications/v1 | Update notification status or assignee. Accepts bulk requests
[**update_rules_v1**](ReconApi.md#update_rules_v1) | **PATCH** /recon/entities/rules/v1 | Update monitoring rules.

## aggregate_notifications_exposed_data_records_v1

> models::DomainPeriodAggregatesResponse aggregate_notifications_exposed_data_records_v1(body)
Get notification exposed data record aggregates as specified via JSON in request body. The valid aggregation fields are: [cid notification_id created_date rule.id rule.name rule.topic source_category site author file.name credential_status bot.operating_system.hardware_id bot.bot_id]

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) |  | [required] |

### Return type

[**models::DomainPeriodAggregatesResponse**](domain.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## aggregate_notifications_v1

> models::DomainPeriodAggregatesResponse aggregate_notifications_v1(body)
Get notification aggregates as specified via JSON in request body.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) |  | [required] |

### Return type

[**models::DomainPeriodAggregatesResponse**](domain.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_actions_v1

> models::DomainPeriodActionEntitiesResponseV1 create_actions_v1(body)
Create actions for a monitoring rule. Accepts a list of actions that will be attached to the monitoring rule.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodRegisterActionsRequest**](DomainPeriodRegisterActionsRequest.md) |  | [required] |

### Return type

[**models::DomainPeriodActionEntitiesResponseV1**](domain.ActionEntitiesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_export_jobs_v1

> models::DomainPeriodLaunchExportJobResponseV1 create_export_jobs_v1(body)
Launch asynchronous export job. Use the job ID to poll the status of the job using GET /entities/exports/v1.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::DomainPeriodLaunchExportJobRequestV1>**](domain.LaunchExportJobRequestV1.md) |  | [required] |

### Return type

[**models::DomainPeriodLaunchExportJobResponseV1**](domain.LaunchExportJobResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_rules_v1

> models::DomainPeriodRulesEntitiesResponseV1 create_rules_v1(body)
Create monitoring rules.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::SadomainPeriodCreateRuleRequestV1>**](sadomain.CreateRuleRequestV1.md) |  | [required] |

### Return type

[**models::DomainPeriodRulesEntitiesResponseV1**](domain.RulesEntitiesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_action_v1

> models::DomainPeriodQueryResponse delete_action_v1(id)
Delete an action from a monitoring rule based on the action ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the action. | [required] |

### Return type

[**models::DomainPeriodQueryResponse**](domain.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_export_jobs_v1

> models::DomainPeriodExportJobIdResponseV1 delete_export_jobs_v1(ids)
Delete export jobs (and their associated file(s)) based on their IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Export Job IDs. | [required] |

### Return type

[**models::DomainPeriodExportJobIdResponseV1**](domain.ExportJobIDResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_notifications_v1

> models::DomainPeriodNotificationIdResponse delete_notifications_v1(ids)
Delete notifications based on IDs. Notifications cannot be recovered after they are deleted.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Notifications IDs. | [required] |

### Return type

[**models::DomainPeriodNotificationIdResponse**](domain.NotificationIDResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_rules_v1

> models::DomainPeriodRuleQueryResponseV1 delete_rules_v1(ids, notifications_deletion_requested)
Delete monitoring rules.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | IDs of rules. | [required] |
**notifications_deletion_requested** | Option<**bool**> | Whether we should delete the notifications generated by this rule or not |  |

### Return type

[**models::DomainPeriodRuleQueryResponseV1**](domain.RuleQueryResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_actions_v1

> models::DomainPeriodActionEntitiesResponseV1 get_actions_v1(ids)
Get actions based on their IDs. IDs can be retrieved using the GET /queries/actions/v1 endpoint.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Action IDs. | [required] |

### Return type

[**models::DomainPeriodActionEntitiesResponseV1**](domain.ActionEntitiesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_export_jobs_v1

> models::DomainPeriodExportJobEntitiesResponseV1 get_export_jobs_v1(ids)
Get the status of export jobs based on their IDs. Export jobs can be launched by calling POST /entities/exports/v1. When a job is complete, use the job ID to download the file(s) associated with it using GET entities/export-files/v1.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Export Job IDs. | [required] |

### Return type

[**models::DomainPeriodExportJobEntitiesResponseV1**](domain.ExportJobEntitiesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_file_content_for_export_jobs_v1

> Vec<i32> get_file_content_for_export_jobs_v1(id)
Download the file associated with a job ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Export Job ID. | [required] |

### Return type

**Vec<i32>**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_notifications_detailed_translated_v1

> models::DomainPeriodNotificationDetailsResponseV1 get_notifications_detailed_translated_v1(ids)
Get detailed notifications based on their IDs. These include the translated raw intelligence content that generated the match or part of it.

Get detailed notifications based on their IDs. These include the translated raw intelligence content that generated the match or part of it. This API endpoint will return translated notification content. The only target language available is English. A single notification can be translated per request. In case the item's content is only partial, a URL is provided under the resource's 'details.full_content_url' path, but the content available at this URL will be the original one.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Notification IDs. | [required] |

### Return type

[**models::DomainPeriodNotificationDetailsResponseV1**](domain.NotificationDetailsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_notifications_detailed_v1

> models::DomainPeriodNotificationDetailsResponseV1 get_notifications_detailed_v1(ids)
Get detailed notifications based on their IDs. These include the raw intelligence content that generated the match or part of it.

Get detailed notifications based on their IDs. These include the raw intelligence content that generated the match or part of it. In case the content is only partial, a URL is provided under the resource's 'details.full_content_url' path. When present, use this URL to retrieve the full raw text content of the item. Please note this URL has a limited TTL. To get a fresh valid one, perform a new call to this API endpoint.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Notification IDs. | [required] |

### Return type

[**models::DomainPeriodNotificationDetailsResponseV1**](domain.NotificationDetailsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_notifications_exposed_data_records_v1

> models::ApiPeriodNotificationExposedDataRecordEntitiesResponseV1 get_notifications_exposed_data_records_v1(ids)
Get notifications exposed data records based on their IDs. IDs can be retrieved using the GET /queries/notifications-exposed-data-records/v1 endpoint. The associate notification can be fetched using the /entities/notifications/v* endpoints

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Notification exposed records IDs. | [required] |

### Return type

[**models::ApiPeriodNotificationExposedDataRecordEntitiesResponseV1**](api.NotificationExposedDataRecordEntitiesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_notifications_translated_v1

> models::DomainPeriodNotificationEntitiesResponseV1 get_notifications_translated_v1(ids)
Get notifications based on their IDs. IDs can be retrieved using the GET /queries/notifications/v1 endpoint. This endpoint will return translated notification content. The only target language available is English.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Notification IDs. | [required] |

### Return type

[**models::DomainPeriodNotificationEntitiesResponseV1**](domain.NotificationEntitiesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_notifications_v1

> models::DomainPeriodNotificationEntitiesResponseV1 get_notifications_v1(ids)
Get notifications based on their IDs. IDs can be retrieved using the GET /queries/notifications/v1 endpoint.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Notification IDs. | [required] |

### Return type

[**models::DomainPeriodNotificationEntitiesResponseV1**](domain.NotificationEntitiesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_rules_v1

> models::DomainPeriodRulesEntitiesResponseV1 get_rules_v1(ids)
Get monitoring rules based on their IDs. IDs can be retrieved using the GET /queries/rules/v1 endpoint.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | IDs of rules. | [required] |

### Return type

[**models::DomainPeriodRulesEntitiesResponseV1**](domain.RulesEntitiesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## preview_rule_v1

> models::DomainPeriodAggregatesResponse preview_rule_v1(body)
Preview rules notification count and distribution. This will return aggregations on: channel, count, site.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodRulePreviewRequest**](DomainPeriodRulePreviewRequest.md) |  | [required] |

### Return type

[**models::DomainPeriodAggregatesResponse**](domain.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_actions_v1

> models::DomainPeriodQueryResponse query_actions_v1(offset, limit, sort, filter, q)
Query actions based on provided criteria. Use the IDs from this response to get the action entities on GET /entities/actions/v1.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Starting index of overall result set from which to return IDs. |  |
**limit** | Option<**i32**> | Number of IDs to return. Offset + limit should NOT be above 10K. |  |
**sort** | Option<**String**> | Possible order by fields: created_timestamp, updated_timestamp. Ex: 'updated_timestamp|desc'. |  |
**filter** | Option<**String**> | FQL query to filter actions by. Possible filter properties are: [id cid user_uuid rule_id type frequency content_format trigger_matchless recipients status created_timestamp updated_timestamp] |  |
**q** | Option<**String**> | Free text search across all indexed fields |  |

### Return type

[**models::DomainPeriodQueryResponse**](domain.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_notifications_exposed_data_records_v1

> models::DomainPeriodQueryResponse query_notifications_exposed_data_records_v1(offset, limit, sort, filter, q)
Query notifications exposed data records based on provided criteria. Use the IDs from this response to get the notification +entities on GET /entities/notifications-exposed-data-records/v1

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of IDs to return. Offset + limit should NOT be above 10K. |  |
**sort** | Option<**String**> | Possible order by fields: created_date, updated_date. Ex: 'updated_date|desc'. |  |
**filter** | Option<**String**> | FQL query to filter notifications by. Possible filter properties are: [id cid user_uuid created_date exposure_date rule.id rule.name rule.topic notification_id source_category site site_id author author_id user_id user_name credentials_url credentials_domain credentials_ip email domain hash_type display_name full_name user_ip phone_number company job_position file.name file.complete_data_set file.download_urls location.postal_code location.city location.state location.federal_district location.federal_admin_region location.country_code social.twitter_id social.facebook_id social.vk_id social.vk_token social.aim_id social.icq_id social.msn_id social.instagram_id social.skype_id financial.credit_card financial.bank_account financial.crypto_currency_addresses login_id credential_status _all bot.operating_system.hardware_id bot.bot_id] |  |
**q** | Option<**String**> | Free text search across all indexed fields. |  |

### Return type

[**models::DomainPeriodQueryResponse**](domain.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_notifications_v1

> models::DomainPeriodQueryResponse query_notifications_v1(offset, limit, sort, filter, q)
Query notifications based on provided criteria. Use the IDs from this response to get the notification +entities on GET /entities/notifications/v1, GET /entities/notifications-detailed/v1, +GET /entities/notifications-translated/v1 or GET /entities/notifications-detailed-translated/v1.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Starting index of overall result set from which to return IDs. |  |
**limit** | Option<**i32**> | Number of IDs to return. Offset + limit should NOT be above 10K. |  |
**sort** | Option<**String**> | Possible order by fields: `created_date`, `updated_date`. Ex: `updated_date|desc`. |  |
**filter** | Option<**String**> | FQL query to filter notifications by. Possible filter properties are: [id cid user_uuid status rule_id rule_name rule_topic rule_priority item_type item_site typosquatting.id typosquatting.unicode_format typosquatting.punycode_format typosquatting.parent_domain.id typosquatting.parent_domain.unicode_format typosquatting.parent_domain.punycode_format typosquatting.base_domain.id typosquatting.base_domain.unicode_format typosquatting.base_domain.punycode_format typosquatting.base_domain.is_registered typosquatting.base_domain.whois.registrar.name typosquatting.base_domain.whois.registrar.status typosquatting.base_domain.whois.registrant.email typosquatting.base_domain.whois.registrant.name typosquatting.base_domain.whois.registrant.org typosquatting.base_domain.whois.name_servers created_date updated_date assigned_to_uuid breach_summary.credential_statuses breach_summary.is_retroactively_deduped] |  |
**q** | Option<**String**> | Free text search across all indexed fields. |  |

### Return type

[**models::DomainPeriodQueryResponse**](domain.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_rules_v1

> models::DomainPeriodRuleQueryResponseV1 query_rules_v1(offset, limit, sort, filter, q, secondary_sort)
Query monitoring rules based on provided criteria. Use the IDs from this response to fetch the rules on /entities/rules/v1.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Starting index of overall result set from which to return IDs. |  |
**limit** | Option<**i32**> | Number of IDs to return. Offset + limit should NOT be above 10K. |  |
**sort** | Option<**String**> | Possible order by fields: created_timestamp, last_updated_timestamp. Ex: `last_updated_timestamp|desc`. |  |
**filter** | Option<**String**> | FQL query to filter rules by. Possible filter properties are: [id cid user_uuid topic priority permissions status filter breach_monitoring_enabled substring_matching_enabled created_timestamp last_updated_timestamp]. |  |
**q** | Option<**String**> | Free text search across all indexed fields. |  |
**secondary_sort** | Option<**String**> | Possible order by fields: created_timestamp, last_updated_timestamp. Ex: `last_updated_timestamp|desc`. |  |

### Return type

[**models::DomainPeriodRuleQueryResponseV1**](domain.RuleQueryResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_action_v1

> models::DomainPeriodActionEntitiesResponseV1 update_action_v1(body)
Update an action for a monitoring rule.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodUpdateActionRequest**](DomainPeriodUpdateActionRequest.md) |  | [required] |

### Return type

[**models::DomainPeriodActionEntitiesResponseV1**](domain.ActionEntitiesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_notifications_v1

> models::DomainPeriodNotificationEntitiesResponseV1 update_notifications_v1(body)
Update notification status or assignee. Accepts bulk requests

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::DomainPeriodUpdateNotificationRequestV1>**](domain.UpdateNotificationRequestV1.md) |  | [required] |

### Return type

[**models::DomainPeriodNotificationEntitiesResponseV1**](domain.NotificationEntitiesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_rules_v1

> models::DomainPeriodRulesEntitiesResponseV1 update_rules_v1(body)
Update monitoring rules.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::DomainPeriodUpdateRuleRequestV1>**](domain.UpdateRuleRequestV1.md) |  | [required] |

### Return type

[**models::DomainPeriodRulesEntitiesResponseV1**](domain.RulesEntitiesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
