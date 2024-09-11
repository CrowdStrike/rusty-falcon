# \CustomIoaApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_rule**](CustomIoaApi.md#create_rule) | **POST** /ioarules/entities/rules/v1 | Create a rule within a rule group. Returns the rule.
[**create_rule_group_mixin0**](CustomIoaApi.md#create_rule_group_mixin0) | **POST** /ioarules/entities/rule-groups/v1 | Create a rule group for a platform with a name and an optional description. Returns the rule group.
[**delete_rule_groups_mixin0**](CustomIoaApi.md#delete_rule_groups_mixin0) | **DELETE** /ioarules/entities/rule-groups/v1 | Delete rule groups by ID.
[**delete_rules**](CustomIoaApi.md#delete_rules) | **DELETE** /ioarules/entities/rules/v1 | Delete rules from a rule group by ID.
[**get_patterns**](CustomIoaApi.md#get_patterns) | **GET** /ioarules/entities/pattern-severities/v1 | Get pattern severities by ID.
[**get_platforms_mixin0**](CustomIoaApi.md#get_platforms_mixin0) | **GET** /ioarules/entities/platforms/v1 | Get platforms by ID.
[**get_rule_groups_mixin0**](CustomIoaApi.md#get_rule_groups_mixin0) | **GET** /ioarules/entities/rule-groups/v1 | Get rule groups by ID.
[**get_rule_types**](CustomIoaApi.md#get_rule_types) | **GET** /ioarules/entities/rule-types/v1 | Get rule types by ID.
[**get_rules_get**](CustomIoaApi.md#get_rules_get) | **POST** /ioarules/entities/rules/GET/v1 | Get rules by ID and optionally with cid and/or version in the following format: `[cid:]ID[:version]`.
[**get_rules_mixin0**](CustomIoaApi.md#get_rules_mixin0) | **GET** /ioarules/entities/rules/v1 | Get rules by ID and optionally with cid and/or version in the following format: `[cid:]ID[:version]`. The max number of IDs is constrained by URL size.
[**query_patterns**](CustomIoaApi.md#query_patterns) | **GET** /ioarules/queries/pattern-severities/v1 | Get all pattern severity IDs.
[**query_platforms_mixin0**](CustomIoaApi.md#query_platforms_mixin0) | **GET** /ioarules/queries/platforms/v1 | Get all platform IDs.
[**query_rule_groups_full**](CustomIoaApi.md#query_rule_groups_full) | **GET** /ioarules/queries/rule-groups-full/v1 | Find all rule groups matching the query with optional filter.
[**query_rule_groups_mixin0**](CustomIoaApi.md#query_rule_groups_mixin0) | **GET** /ioarules/queries/rule-groups/v1 | Finds all rule group IDs matching the query with optional filter.
[**query_rule_types**](CustomIoaApi.md#query_rule_types) | **GET** /ioarules/queries/rule-types/v1 | Get all rule type IDs.
[**query_rules_mixin0**](CustomIoaApi.md#query_rules_mixin0) | **GET** /ioarules/queries/rules/v1 | Finds all rule IDs matching the query with optional filter.
[**update_rule_group_mixin0**](CustomIoaApi.md#update_rule_group_mixin0) | **PATCH** /ioarules/entities/rule-groups/v1 | Update a rule group. The following properties can be modified: name, description, enabled.
[**update_rules**](CustomIoaApi.md#update_rules) | **PATCH** /ioarules/entities/rules/v1 | Update rules within a rule group. Return the updated rules.
[**update_rules_v2**](CustomIoaApi.md#update_rules_v2) | **PATCH** /ioarules/entities/rules/v2 | Update name, description, enabled or field_values for individual rules within a rule group. The v1 flavor of this call requires the caller to specify the complete state for all the rules in the rule group, instead the v2 flavor will accept the subset of rules in the rule group and apply the attribute updates to the subset of rules in the rule group.Return the updated rules.
[**validate**](CustomIoaApi.md#validate) | **POST** /ioarules/entities/rules/validate/v1 | Validates field values and checks for matches if a test string is provided.

## create_rule

> models::ApiPeriodRulesResponse create_rule(body)
Create a rule within a rule group. Returns the rule.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodRuleCreateV1**](ApiPeriodRuleCreateV1.md) |  | [required] |

### Return type

[**models::ApiPeriodRulesResponse**](api.RulesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_rule_group_mixin0

> models::ApiPeriodRuleGroupsResponse create_rule_group_mixin0(body)
Create a rule group for a platform with a name and an optional description. Returns the rule group.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodRuleGroupCreateRequestV1**](ApiPeriodRuleGroupCreateRequestV1.md) |  | [required] |

### Return type

[**models::ApiPeriodRuleGroupsResponse**](api.RuleGroupsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_rule_groups_mixin0

> models::MsaPeriodReplyMetaOnly delete_rule_groups_mixin0(ids, comment)
Delete rule groups by ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the entities | [required] |
**comment** | Option<**String**> | Explains why the entity is being deleted |  |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_rules

> models::MsaPeriodReplyMetaOnly delete_rules(rule_group_id, ids, comment)
Delete rules from a rule group by ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_group_id** | **String** | The parent rule group | [required] |
**ids** | [**Vec<String>**](String.md) | The IDs of the entities | [required] |
**comment** | Option<**String**> | Explains why the entity is being deleted |  |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_patterns

> models::ApiPeriodPatternsResponse get_patterns(ids)
Get pattern severities by ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the entities | [required] |

### Return type

[**models::ApiPeriodPatternsResponse**](api.PatternsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_platforms_mixin0

> models::ApiPeriodPlatformsResponse get_platforms_mixin0(ids)
Get platforms by ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the entities | [required] |

### Return type

[**models::ApiPeriodPlatformsResponse**](api.PlatformsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_rule_groups_mixin0

> models::ApiPeriodRuleGroupsResponse get_rule_groups_mixin0(ids)
Get rule groups by ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the entities | [required] |

### Return type

[**models::ApiPeriodRuleGroupsResponse**](api.RuleGroupsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_rule_types

> models::ApiPeriodRuleTypesResponse get_rule_types(ids)
Get rule types by ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the entities | [required] |

### Return type

[**models::ApiPeriodRuleTypesResponse**](api.RuleTypesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_rules_get

> models::ApiPeriodRulesResponse get_rules_get(body)
Get rules by ID and optionally with cid and/or version in the following format: `[cid:]ID[:version]`.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodRulesGetRequestV1**](ApiPeriodRulesGetRequestV1.md) | The \"ids\" field contains a list of the rules to retrieve. | [required] |

### Return type

[**models::ApiPeriodRulesResponse**](api.RulesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_rules_mixin0

> models::ApiPeriodRulesResponse get_rules_mixin0(ids)
Get rules by ID and optionally with cid and/or version in the following format: `[cid:]ID[:version]`. The max number of IDs is constrained by URL size.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the entities | [required] |

### Return type

[**models::ApiPeriodRulesResponse**](api.RulesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_patterns

> models::MsaPeriodQueryResponse query_patterns(offset, limit)
Get all pattern severity IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of overall result set from which to return IDs |  |
**limit** | Option<**i32**> | Number of IDs to return |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_platforms_mixin0

> models::MsaPeriodQueryResponse query_platforms_mixin0(offset, limit)
Get all platform IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of overall result set from which to return IDs |  |
**limit** | Option<**i32**> | Number of IDs to return |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_rule_groups_full

> models::MsaPeriodQueryResponse query_rule_groups_full(sort, filter, q, offset, limit)
Find all rule groups matching the query with optional filter.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Possible order by fields: {created_by, created_on, enabled, modified_by, modified_on, name} |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. Filter term criteria: [enabled platform name description rules.action_label rules.name rules.description rules.pattern_severity rules.ruletype_name rules.enabled]. Filter range criteria: created_on, modified_on; use any common date format, such as '2010-05-15T14:55:21.892315096Z'. |  |
**q** | Option<**String**> | Match query criteria, which includes all the filter string fields |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return IDs |  |
**limit** | Option<**i32**> | Number of IDs to return |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_rule_groups_mixin0

> models::MsaPeriodQueryResponse query_rule_groups_mixin0(sort, filter, q, offset, limit)
Finds all rule group IDs matching the query with optional filter.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Possible order by fields: {created_by, created_on, enabled, modified_by, modified_on, name} |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. Filter term criteria: [enabled platform name description rules.action_label rules.name rules.description rules.pattern_severity rules.ruletype_name rules.enabled]. Filter range criteria: created_on, modified_on; use any common date format, such as '2010-05-15T14:55:21.892315096Z'. |  |
**q** | Option<**String**> | Match query criteria, which includes all the filter string fields |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return IDs |  |
**limit** | Option<**i32**> | Number of IDs to return |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_rule_types

> models::MsaPeriodQueryResponse query_rule_types(offset, limit)
Get all rule type IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of overall result set from which to return IDs |  |
**limit** | Option<**i32**> | Number of IDs to return |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_rules_mixin0

> models::MsaPeriodQueryResponse query_rules_mixin0(sort, filter, q, offset, limit)
Finds all rule IDs matching the query with optional filter.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Possible order by fields: {rules.created_by, rules.created_on, rules.current_version.action_label, rules.current_version.description, rules.current_version.modified_by, rules.current_version.modified_on, rules.current_version.name, rules.current_version.pattern_severity, rules.enabled, rules.ruletype_name} |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. Filter term criteria: [enabled platform name description rules.action_label rules.name rules.description rules.pattern_severity rules.ruletype_name rules.enabled]. Filter range criteria: created_on, modified_on; use any common date format, such as '2010-05-15T14:55:21.892315096Z'. |  |
**q** | Option<**String**> | Match query criteria, which includes all the filter string fields |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return IDs |  |
**limit** | Option<**i32**> | Number of IDs to return |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_rule_group_mixin0

> models::ApiPeriodRuleGroupsResponse update_rule_group_mixin0(body)
Update a rule group. The following properties can be modified: name, description, enabled.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodRuleGroupModifyRequestV1**](ApiPeriodRuleGroupModifyRequestV1.md) |  | [required] |

### Return type

[**models::ApiPeriodRuleGroupsResponse**](api.RuleGroupsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_rules

> models::ApiPeriodRulesResponse update_rules(body)
Update rules within a rule group. Return the updated rules.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodRuleUpdatesRequestV1**](ApiPeriodRuleUpdatesRequestV1.md) |  | [required] |

### Return type

[**models::ApiPeriodRulesResponse**](api.RulesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_rules_v2

> models::ApiPeriodRulesResponse update_rules_v2(body)
Update name, description, enabled or field_values for individual rules within a rule group. The v1 flavor of this call requires the caller to specify the complete state for all the rules in the rule group, instead the v2 flavor will accept the subset of rules in the rule group and apply the attribute updates to the subset of rules in the rule group.Return the updated rules.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodRuleUpdatesRequestV2**](ApiPeriodRuleUpdatesRequestV2.md) |  | [required] |

### Return type

[**models::ApiPeriodRulesResponse**](api.RulesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## validate

> models::ApiPeriodValidationResponseV1 validate(body)
Validates field values and checks for matches if a test string is provided.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodValidationRequestV1**](ApiPeriodValidationRequestV1.md) |  | [required] |

### Return type

[**models::ApiPeriodValidationResponseV1**](api.ValidationResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
