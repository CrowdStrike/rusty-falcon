# \MessageCenterApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**aggregate_cases**](MessageCenterApi.md#aggregate_cases) | **POST** /message-center/aggregates/cases/GET/v1 | Retrieve aggregate case values based on the matched filter
[**case_add_activity**](MessageCenterApi.md#case_add_activity) | **POST** /message-center/entities/case-activity/v1 | Add an activity to case. Only activities of type comment are allowed via API
[**case_add_attachment**](MessageCenterApi.md#case_add_attachment) | **POST** /message-center/entities/case-attachment/v1 | Upload an attachment for the case.
[**case_download_attachment**](MessageCenterApi.md#case_download_attachment) | **GET** /message-center/entities/case-attachment/v1 | retrieves an attachment for the case, given the attachment id
[**create_case**](MessageCenterApi.md#create_case) | **POST** /message-center/entities/case/v1 | create a new case
[**create_case_v2**](MessageCenterApi.md#create_case_v2) | **POST** /message-center/entities/case/v2 | create a new case
[**get_case_activity_by_ids**](MessageCenterApi.md#get_case_activity_by_ids) | **POST** /message-center/entities/case-activities/GET/v1 | Retrieve activities for given id's
[**get_case_entities_by_ids**](MessageCenterApi.md#get_case_entities_by_ids) | **POST** /message-center/entities/cases/GET/v1 | Retrieve message center cases
[**query_activity_by_case_id**](MessageCenterApi.md#query_activity_by_case_id) | **GET** /message-center/queries/case-activities/v1 | Retrieve activities id's for a case
[**query_cases_ids_by_filter**](MessageCenterApi.md#query_cases_ids_by_filter) | **GET** /message-center/queries/cases/v1 | Retrieve case id's that match the provided filter criteria

## aggregate_cases

> models::MsaPeriodAggregatesResponse aggregate_cases(body)
Retrieve aggregate case values based on the matched filter

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## case_add_activity

> models::MsaspecPeriodResponseFields case_add_activity(body)
Add an activity to case. Only activities of type comment are allowed via API

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodActivityCreationRequest**](DomainPeriodActivityCreationRequest.md) |  | [required] |

### Return type

[**models::MsaspecPeriodResponseFields**](msaspec.ResponseFields.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## case_add_attachment

> models::ApiPeriodMessageCenterAttachmentUploadResponse case_add_attachment(case_id, user_uuid, file)
Upload an attachment for the case.

Upload an attachment for the case. Maximum upload size allowed is *15 MB*.   Filename must start with *[a-zA-Z0-9_-]*. Allowed characters in file name are *[a-zA-Z0-9-_.\\s]*.    Maximum file name is *255* characters      Following attachment types are allowed:   - png   - bmp   - jpg   - jpeg   - gif   - pdf   - doc   - docx   - xls   - xlsx   - pptx   - txt   - csv

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**case_id** | **String** | Case ID | [required] |
**user_uuid** | **String** | User UUID | [required] |
**file** | **std::path::PathBuf** | File Body | [required] |

### Return type

[**models::ApiPeriodMessageCenterAttachmentUploadResponse**](api.MessageCenterAttachmentUploadResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## case_download_attachment

> String case_download_attachment(id)
retrieves an attachment for the case, given the attachment id

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | attachment ID | [required] |

### Return type

**String**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, image/png, image/bmp, image/jpeg, image/jpg, image/gif, application/zip, application/pdf, application/msword, application/vnd.openxmlformats-officedocument.wordprocessingml.document, application/vnd.ms-excel, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet, application/vnd.openxmlformats-officedocument.presentationml.presentation, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_case

> models::MsaPeriodReplyAffectedEntities create_case(body)
create a new case

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodCaseCreationRequest**](DomainPeriodCaseCreationRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodReplyAffectedEntities**](msa.ReplyAffectedEntities.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_case_v2

> models::MsaPeriodReplyAffectedEntities create_case_v2(body)
create a new case

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodCaseCreationRequestV2**](DomainPeriodCaseCreationRequestV2.md) |  | [required] |

### Return type

[**models::MsaPeriodReplyAffectedEntities**](msa.ReplyAffectedEntities.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_case_activity_by_ids

> models::ApiPeriodMessageCenterActivityResponse get_case_activity_by_ids(body)
Retrieve activities for given id's

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::ApiPeriodMessageCenterActivityResponse**](api.MessageCenterActivityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_case_entities_by_ids

> models::ApiPeriodMessageCenterCasesResponse get_case_entities_by_ids(body)
Retrieve message center cases

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::ApiPeriodMessageCenterCasesResponse**](api.MessageCenterCasesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_activity_by_case_id

> models::MsaspecPeriodQueryResponse query_activity_by_case_id(case_id, limit, sort, filter, offset)
Retrieve activities id's for a case

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**case_id** | **String** | Case ID | [required] |
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |
**sort** | Option<**String**> | The property to sort on, followed by a dot (.), followed by the sort direction, either \"asc\" or \"desc\". |  |
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. Allowed filters are:   activity.created_time activity.type |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_cases_ids_by_filter

> models::MsaspecPeriodQueryResponse query_cases_ids_by_filter(limit, sort, filter, offset)
Retrieve case id's that match the provided filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |
**sort** | Option<**String**> | The property to sort on, followed by a dot (.), followed by the sort direction, either \"asc\" or \"desc\". |  |
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. Allowed filters are:   _all activity.body case.aids case.assigner.display_name case.assigner.first_name case.assigner.last_name case.assigner.uid case.assigner.uuid case.body case.created_time case.detections.id case.hosts case.id case.incidents.id case.ip_addresses case.key case.last_modified_time case.status case.title case.type |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
