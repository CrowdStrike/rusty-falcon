# \RealTimeResponseAuditApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**r_tr_audit_sessions**](RealTimeResponseAuditApi.md#r_tr_audit_sessions) | **GET** /real-time-response-audit/combined/sessions/v1 | Get all the RTR sessions created for a customer in a specified duration

## r_tr_audit_sessions

> models::DomainPeriodSessionResponseWrapper r_tr_audit_sessions(filter, sort, limit, offset, with_command_info)
Get all the RTR sessions created for a customer in a specified duration

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Optional filter criteria in the form of an FQL query. For more information about FQL queries, see our [FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**sort** | Option<**String**> | how to sort the session IDs. e.g. sort=created_at|desc will sort the results based on createdAt in descending order |  |
**limit** | Option<**String**> | number of sessions to be returned |  |
**offset** | Option<**String**> | offset value to be used for paginated results |  |
**with_command_info** | Option<**bool**> | get sessions with command info included; by default sessions are returned without command info which include cloud_request_ids and logs fields |  |[default to false]

### Return type

[**models::DomainPeriodSessionResponseWrapper**](domain.SessionResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
