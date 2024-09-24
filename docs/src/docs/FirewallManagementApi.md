# \FirewallManagementApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**aggregate_events**](FirewallManagementApi.md#aggregate_events) | **POST** /fwmgr/aggregates/events/GET/v1 | Aggregate events for customer
[**aggregate_policy_rules**](FirewallManagementApi.md#aggregate_policy_rules) | **POST** /fwmgr/aggregates/policy-rules/GET/v1 | Aggregate rules within a policy for customer
[**aggregate_rule_groups**](FirewallManagementApi.md#aggregate_rule_groups) | **POST** /fwmgr/aggregates/rule-groups/GET/v1 | Aggregate rule groups for customer
[**aggregate_rules**](FirewallManagementApi.md#aggregate_rules) | **POST** /fwmgr/aggregates/rules/GET/v1 | Aggregate rules for customer
[**create_network_locations**](FirewallManagementApi.md#create_network_locations) | **POST** /fwmgr/entities/network-locations/v1 | Create new network locations provided, and return the ID.
[**create_rule_group**](FirewallManagementApi.md#create_rule_group) | **POST** /fwmgr/entities/rule-groups/v1 | Create new rule group on a platform for a customer with a name and description, and return the ID
[**create_rule_group_validation**](FirewallManagementApi.md#create_rule_group_validation) | **POST** /fwmgr/entities/rule-groups/validation/v1 | Validates the request of creating a new rule group on a platform for a customer with a name and description
[**delete_network_locations**](FirewallManagementApi.md#delete_network_locations) | **DELETE** /fwmgr/entities/network-locations/v1 | Delete network location entities by ID.
[**delete_rule_groups**](FirewallManagementApi.md#delete_rule_groups) | **DELETE** /fwmgr/entities/rule-groups/v1 | Delete rule group entities by ID
[**get_events**](FirewallManagementApi.md#get_events) | **GET** /fwmgr/entities/events/v1 | Get events entities by ID and optionally version
[**get_firewall_fields**](FirewallManagementApi.md#get_firewall_fields) | **GET** /fwmgr/entities/firewall-fields/v1 | Get the firewall field specifications by ID
[**get_network_locations**](FirewallManagementApi.md#get_network_locations) | **GET** /fwmgr/entities/network-locations/v1 | Get a summary of network locations entities by ID
[**get_network_locations_details**](FirewallManagementApi.md#get_network_locations_details) | **GET** /fwmgr/entities/network-locations-details/v1 | Get network locations entities by ID
[**get_platforms**](FirewallManagementApi.md#get_platforms) | **GET** /fwmgr/entities/platforms/v1 | Get platforms by ID, e.g., windows or mac or droid
[**get_policy_containers**](FirewallManagementApi.md#get_policy_containers) | **GET** /fwmgr/entities/policies/v1 | Get policy container entities by policy ID
[**get_rule_groups**](FirewallManagementApi.md#get_rule_groups) | **GET** /fwmgr/entities/rule-groups/v1 | Get rule group entities by ID. These groups do not contain their rule entities, just the rule IDs in precedence order.
[**get_rules**](FirewallManagementApi.md#get_rules) | **GET** /fwmgr/entities/rules/v1 | Get rule entities by ID (64-bit unsigned int as decimal string) or Family ID (32-character hexadecimal string)
[**query_events**](FirewallManagementApi.md#query_events) | **GET** /fwmgr/queries/events/v1 | Find all event IDs matching the query with filter
[**query_firewall_fields**](FirewallManagementApi.md#query_firewall_fields) | **GET** /fwmgr/queries/firewall-fields/v1 | Get the firewall field specification IDs for the provided platform
[**query_network_locations**](FirewallManagementApi.md#query_network_locations) | **GET** /fwmgr/queries/network-locations/v1 | Get a list of network location IDs
[**query_platforms**](FirewallManagementApi.md#query_platforms) | **GET** /fwmgr/queries/platforms/v1 | Get the list of platform names
[**query_policy_rules**](FirewallManagementApi.md#query_policy_rules) | **GET** /fwmgr/queries/policy-rules/v1 | Find all firewall rule IDs matching the query with filter, and return them in precedence order
[**query_rule_groups**](FirewallManagementApi.md#query_rule_groups) | **GET** /fwmgr/queries/rule-groups/v1 | Find all rule group IDs matching the query with filter
[**query_rules**](FirewallManagementApi.md#query_rules) | **GET** /fwmgr/queries/rules/v1 | Find all rule IDs matching the query with filter
[**update_network_locations**](FirewallManagementApi.md#update_network_locations) | **PATCH** /fwmgr/entities/network-locations/v1 | Updates the network locations provided, and return the ID.
[**update_network_locations_metadata**](FirewallManagementApi.md#update_network_locations_metadata) | **POST** /fwmgr/entities/network-locations-metadata/v1 | Updates the network locations metadata such as polling_intervals for the cid
[**update_network_locations_precedence**](FirewallManagementApi.md#update_network_locations_precedence) | **POST** /fwmgr/entities/network-locations-precedence/v1 | Updates the network locations precedence according to the list of ids provided.
[**update_policy_container**](FirewallManagementApi.md#update_policy_container) | **PUT** /fwmgr/entities/policies/v2 | Update an identified policy container, including local logging functionality.
[**update_policy_container_v1**](FirewallManagementApi.md#update_policy_container_v1) | **PUT** /fwmgr/entities/policies/v1 | Update an identified policy container. WARNING: This endpoint is deprecated in favor of v2, using this endpoint could disable your local logging setting.
[**update_rule_group**](FirewallManagementApi.md#update_rule_group) | **PATCH** /fwmgr/entities/rule-groups/v1 | Update name, description, or enabled status of a rule group, or create, edit, delete, or reorder rules
[**update_rule_group_validation**](FirewallManagementApi.md#update_rule_group_validation) | **PATCH** /fwmgr/entities/rule-groups/validation/v1 | Validates the request of updating name, description, or enabled status of a rule group, or create, edit, delete, or reorder rules
[**upsert_network_locations**](FirewallManagementApi.md#upsert_network_locations) | **PUT** /fwmgr/entities/network-locations/v1 | Updates the network locations provided, and return the ID.
[**validate_filepath_pattern**](FirewallManagementApi.md#validate_filepath_pattern) | **POST** /fwmgr/entities/rules/validate-filepath/v1 | Validates that the test pattern matches the executable filepath glob pattern.

## aggregate_events

> models::FwmgrPeriodApiPeriodAggregatesResponse aggregate_events(body)
Aggregate events for customer

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::FwmgrPeriodMsaPeriodAggregateQueryRequest>**](fwmgr.msa.AggregateQueryRequest.md) | Query criteria and settings | [required] |

### Return type

[**models::FwmgrPeriodApiPeriodAggregatesResponse**](fwmgr.api.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## aggregate_policy_rules

> models::FwmgrPeriodApiPeriodAggregatesResponse aggregate_policy_rules(body)
Aggregate rules within a policy for customer

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::FwmgrPeriodMsaPeriodAggregateQueryRequest>**](fwmgr.msa.AggregateQueryRequest.md) | Query criteria and settings | [required] |

### Return type

[**models::FwmgrPeriodApiPeriodAggregatesResponse**](fwmgr.api.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## aggregate_rule_groups

> models::FwmgrPeriodApiPeriodAggregatesResponse aggregate_rule_groups(body)
Aggregate rule groups for customer

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::FwmgrPeriodMsaPeriodAggregateQueryRequest>**](fwmgr.msa.AggregateQueryRequest.md) | Query criteria and settings | [required] |

### Return type

[**models::FwmgrPeriodApiPeriodAggregatesResponse**](fwmgr.api.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## aggregate_rules

> models::FwmgrPeriodApiPeriodAggregatesResponse aggregate_rules(body)
Aggregate rules for customer

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::FwmgrPeriodMsaPeriodAggregateQueryRequest>**](fwmgr.msa.AggregateQueryRequest.md) | Query criteria and settings | [required] |

### Return type

[**models::FwmgrPeriodApiPeriodAggregatesResponse**](fwmgr.api.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_network_locations

> models::FwmgrPeriodApiPeriodNetworkLocationsResponse create_network_locations(body, clone_id, add_fw_rules, comment)
Create new network locations provided, and return the ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FwmgrPeriodApiPeriodNetworkLocationCreateRequestV1**](FwmgrPeriodApiPeriodNetworkLocationCreateRequestV1.md) |  | [required] |
**clone_id** | Option<**String**> | A network location ID from which to copy location. If this is provided then the body of the request is ignored. |  |
**add_fw_rules** | Option<**bool**> | A boolean to determine whether the cloned location needs to be added to the same firewall rules that original location is added to. |  |
**comment** | Option<**String**> | Audit log comment for this action |  |

### Return type

[**models::FwmgrPeriodApiPeriodNetworkLocationsResponse**](fwmgr.api.NetworkLocationsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_rule_group

> models::FwmgrPeriodApiPeriodQueryResponse create_rule_group(body, clone_id, library, comment)
Create new rule group on a platform for a customer with a name and description, and return the ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FwmgrPeriodApiPeriodRuleGroupCreateRequestV1**](FwmgrPeriodApiPeriodRuleGroupCreateRequestV1.md) |  | [required] |
**clone_id** | Option<**String**> | A rule group ID from which to copy rules. If this is provided then the 'rules' property of the body is ignored. |  |
**library** | Option<**String**> | If this flag is set to true then the rules will be cloned from the clone_id from the CrowdStrike Firewall Rule Groups Library. |  |
**comment** | Option<**String**> | Audit log comment for this action |  |

### Return type

[**models::FwmgrPeriodApiPeriodQueryResponse**](fwmgr.api.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_rule_group_validation

> models::FwmgrPeriodMsaspecPeriodQueryResponse create_rule_group_validation(body, clone_id, library, comment)
Validates the request of creating a new rule group on a platform for a customer with a name and description

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FwmgrPeriodApiPeriodRuleGroupCreateRequestV1**](FwmgrPeriodApiPeriodRuleGroupCreateRequestV1.md) |  | [required] |
**clone_id** | Option<**String**> | A rule group ID from which to copy rules. If this is provided then the 'rules' property of the body is ignored. |  |
**library** | Option<**String**> | If this flag is set to true then the rules will be cloned from the clone_id from the CrowdStrike Firewall Rule Groups Library. |  |
**comment** | Option<**String**> | Audit log comment for this action |  |

### Return type

[**models::FwmgrPeriodMsaspecPeriodQueryResponse**](fwmgr.msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_network_locations

> models::FwmgrPeriodMsaspecPeriodQueryResponse delete_network_locations(ids)
Delete network location entities by ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the network locations to be deleted | [required] |

### Return type

[**models::FwmgrPeriodMsaspecPeriodQueryResponse**](fwmgr.msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_rule_groups

> models::FwmgrPeriodApiPeriodQueryResponse delete_rule_groups(ids, comment)
Delete rule group entities by ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the rule groups to be deleted | [required] |
**comment** | Option<**String**> | Audit log comment for this action |  |

### Return type

[**models::FwmgrPeriodApiPeriodQueryResponse**](fwmgr.api.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_events

> models::FwmgrPeriodApiPeriodEventsResponse get_events(ids)
Get events entities by ID and optionally version

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The events to retrieve, identified by ID | [required] |

### Return type

[**models::FwmgrPeriodApiPeriodEventsResponse**](fwmgr.api.EventsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_firewall_fields

> models::FwmgrPeriodApiPeriodFirewallFieldsResponse get_firewall_fields(ids)
Get the firewall field specifications by ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the rule types to retrieve | [required] |

### Return type

[**models::FwmgrPeriodApiPeriodFirewallFieldsResponse**](fwmgr.api.FirewallFieldsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_network_locations

> models::FwmgrPeriodApiPeriodNetworkLocationSummariesResponse get_network_locations(ids)
Get a summary of network locations entities by ID

This endpoint returns a summary of network locations that includes name, description, enabled/disabled status, a count of associated rules etc

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The events to retrieve, identified by ID | [required] |

### Return type

[**models::FwmgrPeriodApiPeriodNetworkLocationSummariesResponse**](fwmgr.api.NetworkLocationSummariesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_network_locations_details

> models::FwmgrPeriodApiPeriodNetworkLocationsResponse get_network_locations_details(ids)
Get network locations entities by ID

This endpoint returns the complete network locations objects that includes all the network location conditions.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The events to retrieve, identified by ID | [required] |

### Return type

[**models::FwmgrPeriodApiPeriodNetworkLocationsResponse**](fwmgr.api.NetworkLocationsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_platforms

> models::FwmgrPeriodApiPeriodPlatformsResponse get_platforms(ids)
Get platforms by ID, e.g., windows or mac or droid

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the platforms to retrieve | [required] |

### Return type

[**models::FwmgrPeriodApiPeriodPlatformsResponse**](fwmgr.api.PlatformsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_policy_containers

> models::FwmgrPeriodApiPeriodPolicyContainersResponse get_policy_containers(ids)
Get policy container entities by policy ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The policy container(s) to retrieve, identified by policy ID | [required] |

### Return type

[**models::FwmgrPeriodApiPeriodPolicyContainersResponse**](fwmgr.api.PolicyContainersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_rule_groups

> models::FwmgrPeriodApiPeriodRuleGroupsResponse get_rule_groups(ids)
Get rule group entities by ID. These groups do not contain their rule entities, just the rule IDs in precedence order.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the rule groups to retrieve | [required] |

### Return type

[**models::FwmgrPeriodApiPeriodRuleGroupsResponse**](fwmgr.api.RuleGroupsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_rules

> models::FwmgrPeriodApiPeriodRulesResponse get_rules(ids)
Get rule entities by ID (64-bit unsigned int as decimal string) or Family ID (32-character hexadecimal string)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The rules to retrieve, identified by ID | [required] |

### Return type

[**models::FwmgrPeriodApiPeriodRulesResponse**](fwmgr.api.RulesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_events

> models::FwmgrPeriodApiPeriodQueryResponse query_events(sort, filter, q, offset, after, limit)
Find all event IDs matching the query with filter

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Possible order by fields:  |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. Filter term criteria: enabled, platform, name, description, etc TODO. Filter range criteria: created_on, modified_on; use any common date format, such as '2010-05-15T14:55:21.892315096Z'. |  |
**q** | Option<**String**> | Match query criteria, which includes all the filter string fields, plus TODO |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an `after` token. On subsequent requests, provide the `after` token from the previous response to continue from that place in the results. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |

### Return type

[**models::FwmgrPeriodApiPeriodQueryResponse**](fwmgr.api.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_firewall_fields

> models::FwmgrPeriodMsaspecPeriodQueryResponse query_firewall_fields(platform_id, offset, limit)
Get the firewall field specification IDs for the provided platform

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform_id** | Option<**String**> | Get fields configuration for this platform |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |

### Return type

[**models::FwmgrPeriodMsaspecPeriodQueryResponse**](fwmgr.msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_network_locations

> models::FwmgrPeriodApiPeriodQueryResponse query_network_locations(sort, filter, q, offset, after, limit)
Get a list of network location IDs

This endpoint returns a list of network location IDs based of query parameter.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Possible order by fields:  |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. Filter term criteria: name |  |
**q** | Option<**String**> | Match query criteria, which includes all the filter string fields |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an `after` token. On subsequent requests, provide the `after` token from the previous response to continue from that place in the results. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |

### Return type

[**models::FwmgrPeriodApiPeriodQueryResponse**](fwmgr.api.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_platforms

> models::FwmgrPeriodMsaspecPeriodQueryResponse query_platforms(offset, limit)
Get the list of platform names

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |

### Return type

[**models::FwmgrPeriodMsaspecPeriodQueryResponse**](fwmgr.msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_policy_rules

> models::FwmgrPeriodApiPeriodQueryResponse query_policy_rules(id, sort, filter, q, offset, limit)
Find all firewall rule IDs matching the query with filter, and return them in precedence order

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the policy container within which to query |  |
**sort** | Option<**String**> | Possible order by fields:  |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. Filter term criteria: enabled, platform, name, description, etc TODO. Filter range criteria: created_on, modified_on; use any common date format, such as '2010-05-15T14:55:21.892315096Z'. |  |
**q** | Option<**String**> | Match query criteria, which includes all the filter string fields, plus TODO |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |

### Return type

[**models::FwmgrPeriodApiPeriodQueryResponse**](fwmgr.api.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_rule_groups

> models::FwmgrPeriodApiPeriodQueryResponse query_rule_groups(sort, filter, q, offset, after, limit)
Find all rule group IDs matching the query with filter

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Possible order by fields:  |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. Filter term criteria: enabled, platform, name, description, etc TODO. Filter range criteria: created_on, modified_on; use any common date format, such as '2010-05-15T14:55:21.892315096Z'. |  |
**q** | Option<**String**> | Match query criteria, which includes all the filter string fields, plus TODO |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an `after` token. On subsequent requests, provide the `after` token from the previous response to continue from that place in the results. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |

### Return type

[**models::FwmgrPeriodApiPeriodQueryResponse**](fwmgr.api.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_rules

> models::FwmgrPeriodApiPeriodQueryResponse query_rules(sort, filter, q, offset, after, limit)
Find all rule IDs matching the query with filter

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Possible order by fields:  |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. Filter term criteria: enabled, platform, name, description, etc TODO. Filter range criteria: created_on, modified_on; use any common date format, such as '2010-05-15T14:55:21.892315096Z'. |  |
**q** | Option<**String**> | Match query criteria, which includes all the filter string fields, plus TODO |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an `after` token. On subsequent requests, provide the `after` token from the previous response to continue from that place in the results. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |

### Return type

[**models::FwmgrPeriodApiPeriodQueryResponse**](fwmgr.api.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_network_locations

> models::FwmgrPeriodMsaspecPeriodQueryResponse update_network_locations(body, comment)
Updates the network locations provided, and return the ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FwmgrPeriodApiPeriodNetworkLocationModifyRequestV1**](FwmgrPeriodApiPeriodNetworkLocationModifyRequestV1.md) |  | [required] |
**comment** | Option<**String**> | Audit log comment for this action |  |

### Return type

[**models::FwmgrPeriodMsaspecPeriodQueryResponse**](fwmgr.msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_network_locations_metadata

> models::FwmgrPeriodMsaspecPeriodQueryResponse update_network_locations_metadata(body, comment)
Updates the network locations metadata such as polling_intervals for the cid

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FwmgrPeriodApiPeriodNetworkLocationModifyMetadataRequestV1**](FwmgrPeriodApiPeriodNetworkLocationModifyMetadataRequestV1.md) |  | [required] |
**comment** | Option<**String**> | Audit log comment for this action |  |

### Return type

[**models::FwmgrPeriodMsaspecPeriodQueryResponse**](fwmgr.msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_network_locations_precedence

> models::FwmgrPeriodMsaspecPeriodQueryResponse update_network_locations_precedence(body, comment)
Updates the network locations precedence according to the list of ids provided.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FwmgrPeriodApiPeriodNetworkLocationModifyPrecedenceRequestV1**](FwmgrPeriodApiPeriodNetworkLocationModifyPrecedenceRequestV1.md) |  | [required] |
**comment** | Option<**String**> | Audit log comment for this action |  |

### Return type

[**models::FwmgrPeriodMsaspecPeriodQueryResponse**](fwmgr.msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_policy_container

> models::FwmgrPeriodMsaspecPeriodResponseFields update_policy_container(body)
Update an identified policy container, including local logging functionality.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FwmgrPeriodApiPeriodPolicyContainerUpsertRequestV1**](FwmgrPeriodApiPeriodPolicyContainerUpsertRequestV1.md) |  | [required] |

### Return type

[**models::FwmgrPeriodMsaspecPeriodResponseFields**](fwmgr.msaspec.ResponseFields.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_policy_container_v1

> models::FwmgrPeriodMsaspecPeriodResponseFields update_policy_container_v1(body)
Update an identified policy container. WARNING: This endpoint is deprecated in favor of v2, using this endpoint could disable your local logging setting.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FwmgrPeriodApiPeriodPolicyContainerUpsertRequestV1**](FwmgrPeriodApiPeriodPolicyContainerUpsertRequestV1.md) |  | [required] |

### Return type

[**models::FwmgrPeriodMsaspecPeriodResponseFields**](fwmgr.msaspec.ResponseFields.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_rule_group

> models::FwmgrPeriodApiPeriodQueryResponse update_rule_group(body, comment)
Update name, description, or enabled status of a rule group, or create, edit, delete, or reorder rules

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FwmgrPeriodApiPeriodRuleGroupModifyRequestV1**](FwmgrPeriodApiPeriodRuleGroupModifyRequestV1.md) |  | [required] |
**comment** | Option<**String**> | Audit log comment for this action |  |

### Return type

[**models::FwmgrPeriodApiPeriodQueryResponse**](fwmgr.api.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_rule_group_validation

> models::FwmgrPeriodMsaspecPeriodQueryResponse update_rule_group_validation(body, comment)
Validates the request of updating name, description, or enabled status of a rule group, or create, edit, delete, or reorder rules

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FwmgrPeriodApiPeriodRuleGroupModifyRequestV1**](FwmgrPeriodApiPeriodRuleGroupModifyRequestV1.md) |  | [required] |
**comment** | Option<**String**> | Audit log comment for this action |  |

### Return type

[**models::FwmgrPeriodMsaspecPeriodQueryResponse**](fwmgr.msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## upsert_network_locations

> models::FwmgrPeriodMsaspecPeriodQueryResponse upsert_network_locations(body, comment)
Updates the network locations provided, and return the ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FwmgrPeriodApiPeriodNetworkLocationModifyRequestV1**](FwmgrPeriodApiPeriodNetworkLocationModifyRequestV1.md) |  | [required] |
**comment** | Option<**String**> | Audit log comment for this action |  |

### Return type

[**models::FwmgrPeriodMsaspecPeriodQueryResponse**](fwmgr.msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## validate_filepath_pattern

> models::FwmgrPeriodApiPeriodValidateFilepathResponse validate_filepath_pattern(body)
Validates that the test pattern matches the executable filepath glob pattern.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FwmgrPeriodApiPeriodFilepathTestRequest**](FwmgrPeriodApiPeriodFilepathTestRequest.md) |  | [required] |

### Return type

[**models::FwmgrPeriodApiPeriodValidateFilepathResponse**](fwmgr.api.ValidateFilepathResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
