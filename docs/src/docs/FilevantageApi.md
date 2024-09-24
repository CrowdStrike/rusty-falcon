# \FilevantageApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_policies**](FilevantageApi.md#create_policies) | **POST** /filevantage/entities/policies/v1 | Creates a new policy of the specified type. New policies are always added at the end of the precedence list for the provided policy type.
[**create_rule_groups**](FilevantageApi.md#create_rule_groups) | **POST** /filevantage/entities/rule-groups/v1 | Creates a new rule group of the specified type.
[**create_rules**](FilevantageApi.md#create_rules) | **POST** /filevantage/entities/rule-groups-rules/v1 | Creates a new rule configuration within the specified rule group.
[**create_scheduled_exclusions**](FilevantageApi.md#create_scheduled_exclusions) | **POST** /filevantage/entities/policy-scheduled-exclusions/v1 | Creates a new scheduled exclusion configuration for the provided policy id.
[**delete_policies**](FilevantageApi.md#delete_policies) | **DELETE** /filevantage/entities/policies/v1 | Deletes 1 or more policies.
[**delete_rule_groups**](FilevantageApi.md#delete_rule_groups) | **DELETE** /filevantage/entities/rule-groups/v1 | Deletes 1 or more rule groups
[**delete_rules**](FilevantageApi.md#delete_rules) | **DELETE** /filevantage/entities/rule-groups-rules/v1 | Deletes 1 or more rules from the specified rule group.
[**delete_scheduled_exclusions**](FilevantageApi.md#delete_scheduled_exclusions) | **DELETE** /filevantage/entities/policy-scheduled-exclusions/v1 | Deletes 1 or more scheduled exclusions from the provided policy id.
[**get_actions_mixin0**](FilevantageApi.md#get_actions_mixin0) | **GET** /filevantage/entities/actions/v1 | Retrieves the processing results for 1 or more actions.
[**get_changes**](FilevantageApi.md#get_changes) | **GET** /filevantage/entities/changes/v2 | Retrieve information on changes
[**get_contents**](FilevantageApi.md#get_contents) | **GET** /filevantage/entities/change-content/v1 | Retrieves the content captured for the provided change id
[**get_policies**](FilevantageApi.md#get_policies) | **GET** /filevantage/entities/policies/v1 | Retrieves the configuration for 1 or more policies.
[**get_rule_groups**](FilevantageApi.md#get_rule_groups) | **GET** /filevantage/entities/rule-groups/v1 | Retrieves the rule group details for 1 or more rule groups.
[**get_rules**](FilevantageApi.md#get_rules) | **GET** /filevantage/entities/rule-groups-rules/v1 | Retrieves the configuration for 1 or more rules.
[**get_scheduled_exclusions**](FilevantageApi.md#get_scheduled_exclusions) | **GET** /filevantage/entities/policy-scheduled-exclusions/v1 | Retrieves the configuration of 1 or more scheduled exclusions from the provided policy id.
[**high_volume_query_changes**](FilevantageApi.md#high_volume_query_changes) | **GET** /filevantage/queries/changes/v3 | Returns 1 or more change ids
[**query_actions_mixin0**](FilevantageApi.md#query_actions_mixin0) | **GET** /filevantage/queries/actions/v1 | Returns one or more action ids
[**query_changes**](FilevantageApi.md#query_changes) | **GET** /filevantage/queries/changes/v2 | Returns 1 or more change ids
[**query_policies**](FilevantageApi.md#query_policies) | **GET** /filevantage/queries/policies/v1 | Retrieve the ids of all policies that are assigned the provided policy type.
[**query_rule_groups**](FilevantageApi.md#query_rule_groups) | **GET** /filevantage/queries/rule-groups/v1 | Retrieve the ids of all rule groups that are of the provided rule group type.
[**query_scheduled_exclusions**](FilevantageApi.md#query_scheduled_exclusions) | **GET** /filevantage/queries/policy-scheduled-exclusions/v1 | Retrieve the ids of all scheduled exclusions contained within the provided policy id.
[**signal_changes_external**](FilevantageApi.md#signal_changes_external) | **POST** /filevantage/entities/workflow/v1 | Initiates workflows for the provided change ids
[**start_actions**](FilevantageApi.md#start_actions) | **POST** /filevantage/entities/actions/v1 | Initiates the specified action on the provided change ids
[**update_policies**](FilevantageApi.md#update_policies) | **PATCH** /filevantage/entities/policies/v1 | Updates the general information of the provided policy.
[**update_policy_host_groups**](FilevantageApi.md#update_policy_host_groups) | **PATCH** /filevantage/entities/policies-host-groups/v1 | Manage host groups assigned to a policy.
[**update_policy_precedence**](FilevantageApi.md#update_policy_precedence) | **PATCH** /filevantage/entities/policies-precedence/v1 | Updates the policy precedence for all policies of a specific type.
[**update_policy_rule_groups**](FilevantageApi.md#update_policy_rule_groups) | **PATCH** /filevantage/entities/policies-rule-groups/v1 | Manage the rule groups assigned to the policy or set the rule group precedence for all rule groups within the policy.
[**update_rule_group_precedence**](FilevantageApi.md#update_rule_group_precedence) | **PATCH** /filevantage/entities/rule-groups-rule-precedence/v1 | Updates the rule precedence for all rules in the identified rule group.
[**update_rule_groups**](FilevantageApi.md#update_rule_groups) | **PATCH** /filevantage/entities/rule-groups/v1 | Updates the provided rule group.
[**update_rules**](FilevantageApi.md#update_rules) | **PATCH** /filevantage/entities/rule-groups-rules/v1 | Updates the provided rule configuration within the specified rule group.
[**update_scheduled_exclusions**](FilevantageApi.md#update_scheduled_exclusions) | **PATCH** /filevantage/entities/policy-scheduled-exclusions/v1 | Updates the provided scheduled exclusion configuration within the provided policy.

## create_policies

> models::PoliciesPeriodResponse create_policies(body)
Creates a new policy of the specified type. New policies are always added at the end of the precedence list for the provided policy type.

After they are created, host and rule groups can be assigned, scheduled exclusions can be defined, and policy precedence can be set.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PoliciesPeriodCreateRequest**](PoliciesPeriodCreateRequest.md) | Create a new policy.   *`name` must be between 1 and 100 characters.* `description` can be between 0 and 500 characters.   * `platform` must be one of `Windows`, `Linux`, or `Mac`   Rule and host group assignment and policy precedence setting is performed via their respective patch end-points. | [required] |

### Return type

[**models::PoliciesPeriodResponse**](policies.Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_rule_groups

> models::RulegroupsPeriodResponse create_rule_groups(body)
Creates a new rule group of the specified type.

Individual rules can be assigned to a rule group after it has been created.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RulegroupsPeriodCreateRequest**](RulegroupsPeriodCreateRequest.md) | Create a new rule group of a specific type.   *`name` must be between 1 and 100 characters.* `type` must be one of `WindowsFiles`, `WindowsRegistry`, `LinuxFiles` or `MacFiles`.   * `description` can be between 0 and 500 characters.   Note: rules are added/removed from rule groups using their dedicated end-points. | [required] |

### Return type

[**models::RulegroupsPeriodResponse**](rulegroups.Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_rules

> models::RulegroupsPeriodRulesResponse create_rules(body)
Creates a new rule configuration within the specified rule group.

Creates a new rule configuration within the specified rule group.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RulegroupsPeriodRule**](RulegroupsPeriodRule.md) | Create a new rule configuration for the specified rule group.   *`id` is not supported for creation of a rule, the new id of the created rule will be included in the response.* `rule_group_id` to add the new rule configuration.   *`description` can be between 0 and 500 characters.* `path` representing the file system or registry path to monitor.     *must be between 1 and 250 characters.* All paths must end with the path separator, e.g. c:\\windows\\ /usr/bin/    *`severity` to categorize change events produced by this rule; must be one of: `Low`, `Medium`, `High` or `Critical`* `depth` below the base path to monitor; must be one of: `1`, `2`, `3`, `4`, `5` or `ANY`   *`precedence` - is not supported for creation of a rule, new rules will be added last in precedence order.  Falcon GLOB syntax is supported for the following 6 properties. Allowed rule group configuration is based on the type of rule group the rule group is added to.* `include` represents the files, directories, registry keys, or registry values that will be monitored.    *`exclude` represents the files, directories, registry keys, or registry values that will `NOT` be monitored.* `include_users` represents the changes performed by specific users that will be monitored.   *`exclude_users` represents the changes performed by specific users that will `NOT` be monitored.* `include_processes` represents the changes performed by specific processes that will be monitored.   *`exclude_processes` represents the changes performed by specific processes that will be `NOT` monitored.* `content_files` represents the files whose content will be monitored. Listed files must match the file include pattern and not match the file exclude pattern   *`content_registry_values` represents the registry values whose content will be monitored. Listed registry values must match the registry include pattern and not match the registry exclude pattern* `enable_content_capture`   *`enable_hash_capture`  File system directory monitoring:* `watch_delete_directory_changes`   *`watch_create_directory_changes`* `watch_rename_directory_changes`   *`watch_attributes_directory_changes` (`macOS` is not supported at this time)* `watch_permissions_directory_changes` (`macOS` is not supported at this time)  File system file monitoring:   *`watch_rename_file_changes`* `watch_write_file_changes`   *`watch_create_file_changes`* `watch_delete_file_changes`   *`watch_attributes_file_changes` (`macOS` is not supported at this time)* `watch_permissions_file_changes` (`macOS` is not supported at this time)  Windows registry key and value monitoring:    *`watch_create_key_changes`* `watch_delete_key_changes`   *`watch_rename_key_changes`* `watch_set_value_changes`   *`watch_permissions_key_changes`* `watch_delete_value_changes`   * `watch_create_file_changes` | [required] |

### Return type

[**models::RulegroupsPeriodRulesResponse**](rulegroups.RulesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_scheduled_exclusions

> models::ScheduledexclusionsPeriodResponse create_scheduled_exclusions(body)
Creates a new scheduled exclusion configuration for the provided policy id.

Creates a new scheduled exclusion configuration for the provided policy id.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ScheduledexclusionsPeriodCreateRequest**](ScheduledexclusionsPeriodCreateRequest.md) | Create a new scheduled exclusion configuration for the specified policy.      *`policy_id` to add the scheduled exclusion to.* `name` must be between 1 and 100 characters.   *`description` can be between 0 and 500 characters.* `users` can be between 0 and 500 characters representing a comma separated list of user to exclude their changes.      *admin* excludes changes made by all usernames that begin with admin. Falcon GLOB syntax is supported.   *`processes` can be between 0 and 500 characters representing a comma separated list of processes to exclude their changes.* **\\RunMe.exe or**_/RunMe.sh excludes changes made by RunMe.exe or RunMe.sh in any location.   *`schedule_start` must be provided to indicate the start of the schedule. This date/time must be an rfc3339 formatted string  <https://datatracker.ietf.org/doc/html/rfc3339>.* `schedule_end` optionally provided to indicate the end of the schedule. This date/time must be an rfc3339 formatted string  <https://datatracker.ietf.org/doc/html/rfc3339>.   *`timezone`  must be provided to indicate the TimeZone Name set for the provided `scheduled_start` and `scheduled_end` values. See <https://en.wikipedia.org/wiki/List_of_tz_database_time_zones>.* `repeated` optionally provided to indicate that the exclusion is applied repeatedly within the `scheduled_start` and `scheduled_end` time.      *`start_time` must be the hour(00-23) and minute(00-59) of the day formatted as `HH:MM`. Required if `all_day` is not set to `true`* `end_time` must be the hour(00-23) and minute(00-59) of the day formatted as `HH:MM`. Required if `all_day` is not set to `true`      *`all_day` must be `true` or `false` to indicate the exclusion is applied all day.* `frequency` must be one of `daily`, `weekly` or `monthly`.       *`occurrence` must be one of the following when `frequency` is set to `monthly`:* `1st`, `2nd`, `3rd`, `4th` or `Last` represents the week.        *`Days` represents specific calendar days.* `weekly_days` must be one or more of `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday` or `Sunday` when `frequency` is set to `weekly` or `frequency` is set to `monthly` and `occurrence` is NOT set to `Days`.       * `monthly_days` must be set to one or more calendar days, between 1 and 31  when `frequency` is set to `monthly` and `occurrence` is set to `Days`.  | [required] |

### Return type

[**models::ScheduledexclusionsPeriodResponse**](scheduledexclusions.Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_policies

> models::PoliciesPeriodDeleteResponse delete_policies(ids)
Deletes 1 or more policies.

Only disabled policies are allowed to be deleted.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more (up to 500) policy ids in the form of `ids=ID1&ids=ID2` | [required] |

### Return type

[**models::PoliciesPeriodDeleteResponse**](policies.DeleteResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_rule_groups

> models::RulegroupsPeriodDeleteResponse delete_rule_groups(ids)
Deletes 1 or more rule groups

The rule groups represented by the provided ids and all rules that they contain will be deleted.   Rule groups can only be deleted if they are not assigned to a policy.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more (up to 500) rule group ids in the form of `ids=ID1&ids=ID2` | [required] |

### Return type

[**models::RulegroupsPeriodDeleteResponse**](rulegroups.DeleteResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_rules

> models::MsaspecPeriodQueryResponse delete_rules(rule_group_id, ids)
Deletes 1 or more rules from the specified rule group.

Rules that match a provided id will be deleted from the provided rule group id.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_group_id** | **String** | The id of the rule group from which the rules will be deleted. | [required] |
**ids** | [**Vec<String>**](String.md) | One or more (up to 500) rule ids in the form of `ids=ID1&ids=ID2` | [required] |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_scheduled_exclusions

> models::MsaspecPeriodQueryResponse delete_scheduled_exclusions(policy_id, ids)
Deletes 1 or more scheduled exclusions from the provided policy id.

Scheduled exclusions that match a provided id will be deleted from the provided policy id.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | ID of the policy to delete the scheduled exclusions from. | [required] |
**ids** | [**Vec<String>**](String.md) | One or more (up to 500) scheduled exclusion ids in the form of `ids=ID1&ids=ID2`. | [required] |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_actions_mixin0

> models::ActionsPeriodGetActionResponse get_actions_mixin0(ids)
Retrieves the processing results for 1 or more actions.

The processing results of each action that match the provided ids will be returned.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more actions ids in the form of `ids=ID1&ids=ID2` | [required] |

### Return type

[**models::ActionsPeriodGetActionResponse**](actions.GetActionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_changes

> models::ChangesPeriodGetChangesResponse get_changes(ids)
Retrieve information on changes

Retrieve key attributes of Falcon FileVantage changes for the specified ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more change ids in the form of `ids=ID1&ids=ID2`. The maximum number of ids that can be requested at once is `500`. | [required] |

### Return type

[**models::ChangesPeriodGetChangesResponse**](changes.GetChangesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_contents

> models::ContentchangesPeriodChangeContentsResponse get_contents(id, accept_encoding)
Retrieves the content captured for the provided change id

Retrieves the before and after change content for the provided change id.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the change in the form of id=ID1 | [required] |
**accept_encoding** | Option<**String**> | Providing the value of `gzip` compresses the response, otherwise the content is returned uncompressed. |  |

### Return type

[**models::ContentchangesPeriodChangeContentsResponse**](contentchanges.ChangeContentsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_policies

> models::PoliciesPeriodResponse get_policies(ids)
Retrieves the configuration for 1 or more policies.

The configuration of each policy that match the provided id will be returned.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more (up to 500) policy ids in the form of `ids=ID1&ids=ID2` | [required] |

### Return type

[**models::PoliciesPeriodResponse**](policies.Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_rule_groups

> models::RulegroupsPeriodResponse get_rule_groups(ids)
Retrieves the rule group details for 1 or more rule groups.

Full details of each rule group that matches a provided id will be returned in the response

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more (up to 500) rule group ids in the form of `ids=ID1&ids=ID2` | [required] |

### Return type

[**models::RulegroupsPeriodResponse**](rulegroups.Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_rules

> models::RulegroupsPeriodRulesResponse get_rules(rule_group_id, ids)
Retrieves the configuration for 1 or more rules.

Rules within the provided rule group id that match a provided id will be returned within the response.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_group_id** | **String** | Rule group from which to retrieve the rule configuration. | [required] |
**ids** | [**Vec<String>**](String.md) | One or more (up to 500) rule ids in the form of `ids=ID1&ids=ID2`. | [required] |

### Return type

[**models::RulegroupsPeriodRulesResponse**](rulegroups.RulesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_scheduled_exclusions

> models::ScheduledexclusionsPeriodResponse get_scheduled_exclusions(policy_id, ids)
Retrieves the configuration of 1 or more scheduled exclusions from the provided policy id.

Full details of each each scheduled exclusion that match a provided id will be returned in the response.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The id of the policy to retrieve the scheduled exclusion configurations. | [required] |
**ids** | [**Vec<String>**](String.md) | One or more (up to 500) scheduled exclusion ids in the form of `ids=ID1&ids=ID2`. | [required] |

### Return type

[**models::ScheduledexclusionsPeriodResponse**](scheduledexclusions.Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## high_volume_query_changes

> models::ChangesPeriodHighVolumeQueryResponse high_volume_query_changes(after, limit, sort, filter)
Returns 1 or more change ids

Returns a list of Falcon FileVantage change ids filtered, sorted and limited by the query parameters provided. It can retrieve an unlimited number of results using multiple requests.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request don't provide a value for the `after` token. On subsequent requests provide the `after` token value from the previous response to continue pagination from where you left. If the response returns an empty `after` token it means there are no more results to return. |  |
**limit** | Option<**i32**> | The maximum number of ids to return. Defaults to `100` if not specified. The maximum number of results that can be returned in a single call is `5000`. |  |[default to 100]
**sort** | Option<**String**> | Sort results using options like:  - `action_timestamp` (timestamp of the change occurrence)   Sort either `asc` (ascending) or `desc` (descending). For example: `action_timestamp|asc`. Defaults to`action_timestamp|desc` no value is specified. The full list of allowed sorting options can be reviewed in our API documentation. |  |[default to action_timestamp|desc]
**filter** | Option<**String**> | Filter changes using a query in Falcon Query Language (FQL).   Common filter options include:   - `host.name`  - `action_timestamp`   The full list of allowed filter parameters can be reviewed in our API documentation. |  |

### Return type

[**models::ChangesPeriodHighVolumeQueryResponse**](changes.HighVolumeQueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_actions_mixin0

> models::MsaspecPeriodQueryResponse query_actions_mixin0(offset, limit, sort, filter)
Returns one or more action ids

Returns the list of action ids filtered, sorted, and limited to the query parameters provided.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The first action index to return in the response. If not provided it will default to '0'. Use with the `limit` parameter to manage pagination of results. |  |
**limit** | Option<**i32**> | The maximum number of actions to return in the response (default: 100; max: 500). Use with the `offset` parameter to manage pagination of results |  |
**sort** | Option<**String**> | The sort expression that should be used to sort the results (e.g. created_date|desc) |  |
**filter** | Option<**String**> | Filter changes using a query in Falcon Query Language (FQL).   Common filter options include:   - `status`  - `operation_type`   The full list of allowed filter parameters can be reviewed in our API documentation. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_changes

> models::MsaspecPeriodQueryResponse query_changes(offset, limit, sort, filter)
Returns 1 or more change ids

Returns a list of Falcon FileVantage change ids filtered, sorted and limited by the query parameters provided. Using this endpoint you can retrieve up to `10000` results by using pagination with multiple requests. If you need to retrieve more than `10000` results consider using the `/queries/changes/v3` endpoint

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The offset to start retrieving records from. Defaults to `0` if not specified. |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of ids to return. Defaults to `100` if not specified. The maximum number of results that can be returned in a single call is `500`. |  |[default to 100]
**sort** | Option<**String**> | Sort results using options like:  - `action_timestamp` (timestamp of the change occurrence)   Sort either `asc` (ascending) or `desc` (descending). For example: `action_timestamp|asc`. The full list of allowed sorting options can be reviewed in our API documentation. |  |
**filter** | Option<**String**> | Filter changes using a query in Falcon Query Language (FQL).   Common filter options include:   - `host.name`  - `action_timestamp`   The full list of allowed filter parameters can be reviewed in our API documentation. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_policies

> models::MsaspecPeriodQueryResponse query_policies(r#type, offset, limit, sort)
Retrieve the ids of all policies that are assigned the provided policy type.

Policy ids will be returned sorted by a `precedence` order of ascending when a `sort` parameter is not provided.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The types of policies to retrieve.   Allowed values are: `Windows`, `Linux` or `Mac`. | [required] |
**offset** | Option<**i32**> | The offset to start retrieving records from. Defaults to 0 if not specified. |  |
**limit** | Option<**i32**> | The maximum number of ids to return. Defaults to 100 if not specified. The maximum number of results that can be returned in a single call is 500. |  |
**sort** | Option<**String**> | Sort the returned ids based on one of the following properties:  `precedence`, `created_timestamp` or `modified_timestamp`   Sort either `asc` (ascending) or `desc` (descending);  for example: `precedence|asc`. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_rule_groups

> models::MsaspecPeriodQueryResponse query_rule_groups(r#type, offset, limit, sort)
Retrieve the ids of all rule groups that are of the provided rule group type.

Rule group ids will be returned sorted by `created_timestamp` order if a `sort` parameter is not provided

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The rule group type to retrieve the ids of.   Allowed values are: `WindowsFiles`, `WindowsRegistry`, `LinuxFiles` or `MacFiles`. | [required] |
**offset** | Option<**i32**> | The offset to start retrieving records from. Defaults to 0 if not specified. |  |
**limit** | Option<**i32**> | The maximum number of ids to return. Defaults to 100 if not specified. The maximum number of results that can be returned in a single call is 500. |  |
**sort** | Option<**String**> | Sort the returned ids based on one of the following properties:   `created_timestamp` or `modified_timestamp`   Sort either `asc` (ascending) or `desc` (descending);  for example: `created_timestamp|asc`. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_scheduled_exclusions

> models::MsaspecPeriodQueryResponse query_scheduled_exclusions(policy_id)
Retrieve the ids of all scheduled exclusions contained within the provided policy id.

Retrieve the ids of all scheduled exclusions contained within the provided policy id

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The id of the policy from which to retrieve the scheduled exclusion ids. | [required] |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## signal_changes_external

> models::WorkflowPeriodResponse signal_changes_external(body)
Initiates workflows for the provided change ids

Provides the ability to initiate workflows for the specified change ids. Only 100 change ids can be provided per workflow request.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WorkflowPeriodRequest**](WorkflowPeriodRequest.md) | Change ids to initiate the workflows; limited to 100 per request. | [required] |

### Return type

[**models::WorkflowPeriodResponse**](workflow.Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## start_actions

> models::ActionsPeriodActionResponse start_actions(body)
Initiates the specified action on the provided change ids

Initiates the suppression, unsuppression, or purging of the provided change ids. Note that only 1 action may be initiated and active at a time.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ActionsPeriodCreateActionRequest**](ActionsPeriodCreateActionRequest.md) | Create a new action.   *`operation` must be one of the `suppress`, `unsuppress`, or `purge`* `change_ids` represent the ids of the changes the operation will perform; limited to 100 ids per action   * `comment` optional comment to describe the reason for the action | [required] |

### Return type

[**models::ActionsPeriodActionResponse**](actions.ActionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_policies

> models::PoliciesPeriodResponse update_policies(body)
Updates the general information of the provided policy.

Only name, description, and enabled status of the policy is allowed to be update. Rule and host group assignment is performed via their respective patch end points.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PoliciesPeriodUpdateRequest**](PoliciesPeriodUpdateRequest.md) | Enables updates to the following fields for an existing policy.    *`id` of the policy to update.* `name` must be between 1 and 100 characters.   *`description` can be between 0 and 500 characters.* `platform` may not be modified after the policy is created.   * `enabled` must be one of `true` or `false`.   Rule and host group assignment and policy precedence setting is performed via their respective patch end-points. | [required] |

### Return type

[**models::PoliciesPeriodResponse**](policies.Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_policy_host_groups

> models::PoliciesPeriodResponse update_policy_host_groups(policy_id, action, ids)
Manage host groups assigned to a policy.

Manage host groups assigned to a policy.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The id of the policy for which to perform the action. | [required] |
**action** | **String** | The action to perform with the provided ids, must be one of: `assign` or `unassign`. | [required] |
**ids** | [**Vec<String>**](String.md) | One or more host group ids in the form of `ids=ID1&ids=ID2` | [required] |

### Return type

[**models::PoliciesPeriodResponse**](policies.Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_policy_precedence

> models::PoliciesPeriodPrecedenceResponse update_policy_precedence(ids, r#type)
Updates the policy precedence for all policies of a specific type.

Requests that do not represent all ids of the provided policy type will not be processed.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Precedence of the policies for the provided type in the form of `ids=ID1&ids=ID2` | [required] |
**r#type** | **String** | The policy type for which to set the precedence order, must be one of `Windows`, `Linux` or `Mac`. | [required] |

### Return type

[**models::PoliciesPeriodPrecedenceResponse**](policies.PrecedenceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_policy_rule_groups

> models::PoliciesPeriodResponse update_policy_rule_groups(policy_id, action, ids)
Manage the rule groups assigned to the policy or set the rule group precedence for all rule groups within the policy.

Manage the rule groups assigned to the policy.   Rule groups must be of the same type as the policy they are being added:   *`WindowsRegistry` and `WindowsFiles` groups can only be added to a `Windows` policy.* `LinuxFiles` groups can only be added to a `Linux` policy.   * `MacFiles` groups can only be added to a `Mac` policy.  When setting rule group precedence, the precedence for `all` rule group ids within the policy must be provided.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The id of the policy for which to perform the action. | [required] |
**action** | **String** | The action to perform with the provided ids, must be one of: `assign`, `unassign`, or `precedence`. | [required] |
**ids** | [**Vec<String>**](String.md) | One or more rule group ids in the form of ids=ID1&ids=ID2. Note, for the precedence action, precedence is controlled by the order of the ids as they are specified in the request. | [required] |

### Return type

[**models::PoliciesPeriodResponse**](policies.Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_rule_group_precedence

> models::RulegroupsPeriodResponse update_rule_group_precedence(rule_group_id, ids)
Updates the rule precedence for all rules in the identified rule group.

The ids for `all` rules contained within the rule group must be specified in the desired precedence order. Requests that do not represent all ids will not be processed.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_group_id** | **String** | Rule group from which to set the precedence. | [required] |
**ids** | [**Vec<String>**](String.md) | One or more (up to 500) rule group ids in the form of `ids=ID1&ids=ID2`. | [required] |

### Return type

[**models::RulegroupsPeriodResponse**](rulegroups.Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_rule_groups

> models::RulegroupsPeriodResponse update_rule_groups(body)
Updates the provided rule group.

Provides the ability to update the name and description of the rule group.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RulegroupsPeriodUpdateRequest**](RulegroupsPeriodUpdateRequest.md) | Enables updates to the following fields for an existing rule group.    *`id` of the rule group to update.* `name` must be between 1 and 100 characters.   *`description` can be between 0 and 500 characters.* `type` may not be modified after the rule group is created.   Note: rules are added/removed from rule groups using their dedicated end-points. | [required] |

### Return type

[**models::RulegroupsPeriodResponse**](rulegroups.Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_rules

> models::RulegroupsPeriodRulesResponse update_rules(body)
Updates the provided rule configuration within the specified rule group.

The rule must currently exist within the specified rule group.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RulegroupsPeriodRule**](RulegroupsPeriodRule.md) | Update the rule configuration for the specified rule ID and group.   *`id` of the rule to update.* `rule_group_id` that contains the rule configuration.   *`description` can be between 0 and 500 characters.* `path` representing the file system or registry path to monitor.     *must be between 1 and 250 characters.* All paths must end with the path separator, e.g. c:\\windows\\ /usr/bin/    *`severity` to categorize change events produced by this rule; must be one of: `Low`, `Medium`, `High` or `Critical`* `depth` below the base path to monitor; must be one of: `1`, `2`, `3`, `4`, `5` or `ANY`   *`precedence` is the order in which rules will be evaluated starting with 1. Specifying a precedence value that is already set for another rule in the group will result this rule being placed before that existing rule.  Falcon GLOB syntax is supported for the following 6 properties. Allowed rule group configuration is based on the type of rule group the rule group is added to.* `include` represents the files, directories, registry keys, or registry values that will be monitored.    *`exclude` represents the files, directories, registry keys, or registry values that will `NOT` be monitored.* `include_users` represents the changes performed by specific users that will be monitored.   *`exclude_users` represents the changes performed by specific users that will `NOT` be monitored.* `include_processes` represents the changes performed by specific processes that will be monitored.   *`exclude_processes` represents the changes performed by specific processes that will be `NOT` monitored.* `content_files` represents the files that will be monitored. Listed files must match the file include pattern and not match the file exclude pattern   *`content_registry_values` represents the registry values whose content will be monitored. Listed registry values must match the registry include pattern and not match the registry exclude pattern* `enable_content_capture`   *`enable_hash_capture`  File system directory monitoring:* `watch_delete_directory_changes`   *`watch_create_directory_changes`* `watch_rename_directory_changes`   *`watch_attributes_directory_changes` (`macOS` is not supported at this time)* `watch_permissions_directory_changes` (`macOS` is not supported at this time)  File system file monitoring:   *`watch_rename_file_changes`* `watch_write_file_changes`   *`watch_create_file_changes`* `watch_delete_file_changes`   *`watch_attributes_file_changes` (`macOS` is not supported at this time)* `watch_permissions_file_changes` (`macOS` is not supported at this time)  Windows registry key and value monitoring:    *`watch_create_key_changes`* `watch_delete_key_changes`   *`watch_rename_key_changes`* `watch_set_value_changes`   *`watch_delete_value_changes`* `watch_create_file_changes` | [required] |

### Return type

[**models::RulegroupsPeriodRulesResponse**](rulegroups.RulesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_scheduled_exclusions

> models::ScheduledexclusionsPeriodResponse update_scheduled_exclusions(body)
Updates the provided scheduled exclusion configuration within the provided policy.

Updates the provided scheduled exclusion configuration within the provided policy.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ScheduledexclusionsPeriodUpdateRequest**](ScheduledexclusionsPeriodUpdateRequest.md) | Update an existing scheduled exclusion for the specified policy.      *`policy_id` to add the scheduled exclusion to.* `name` must be between 1 and 100 characters.   *`description` can be between 0 and 500 characters.* `users` can be between 0 and 500 characters representing a comma separated list of user to exclude their changes.      *admin* excludes changes made by all usernames that begin with admin. Falcon GLOB syntax is supported.   *`processes` can be between 0 and 500 characters representing a comma separated list of processes to exclude their changes.* **\\RunMe.exe or**_/RunMe.sh excludes changes made by RunMe.exe or RunMe.sh in any location.   *`schedule_start` must be provided to indicate the start of the schedule. This date/time must be an rfc3339 formatted string  <https://datatracker.ietf.org/doc/html/rfc3339>.* `schedule_end` optionally provided to indicate the end of the schedule. This date/time must be an rfc3339 formatted string  <https://datatracker.ietf.org/doc/html/rfc3339>.   *`timezone`  must be provided to indicate the TimeZone Name set for the provided `scheduled_start` and `scheduled_end` values. See <https://en.wikipedia.org/wiki/List_of_tz_database_time_zones>.* `repeated` optionally provided to indicate that the exclusion is applied repeatedly within the `scheduled_start` and `scheduled_end` time.      *`start_time` must be the hour(00-23) and minute(00-59) of the day formatted as `HH:MM`. Required if `all_day` is not set to `true`* `end_time` must be the hour(00-23) and minute(00-59) of the day formatted as `HH:MM`. Required if `all_day` is not set to `true`      *`all_day` must be `true` or `false` to indicate the exclusion is applied all day.* `frequency` must be one of `daily`, `weekly` or `monthly`.       *`occurrence` must be one of the following when `frequency` is set to `monthly`:* `1st`, `2nd`, `3rd`, `4th` or `Last` represents the week.        *`Days` represents specific calendar days.* `weekly_days` must be one or more of `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday` or `Sunday` when `frequency` is set to `weekly` or `frequency` is set to `monthly` and `occurrence` is NOT set to `Days`.       * `monthly_days` must be set to one or more calendar days, between 1 and 31  when `frequency` is set to `monthly` and `occurrence` is set to `Days`.  | [required] |

### Return type

[**models::ScheduledexclusionsPeriodResponse**](scheduledexclusions.Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
