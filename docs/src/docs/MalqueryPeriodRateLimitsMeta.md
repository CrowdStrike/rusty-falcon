# MalqueryPeriodRateLimitsMeta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**days_left** | **i32** | Days left until the limits are refreshed |
**download_count** | **i32** | How many downloads were executed in the last month |
**download_counts** | Option<[**Vec<crate::models::MalqueryPeriodUserRequestCount>**](malquery.UserRequestCount.md)> | Download counts per user | [optional]
**download_limit** | **i32** | Total download limit per month |
**hunt_count** | **i32** | How many hunts were executed in the last month |
**hunt_counts** | Option<[**Vec<crate::models::MalqueryPeriodUserRequestCount>**](malquery.UserRequestCount.md)> | Hunt counts per user | [optional]
**hunt_limit** | **i32** | Total hunt limit per month |
**monitor_count** | **i32** | How many monitors were created in the last month |
**monitor_limit** | **i32** | Total monitor limit per month |
**refresh_time** | **String** | Time when the limits are refreshed. ISO 8601 format |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
