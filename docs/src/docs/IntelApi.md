# \IntelApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_intel_actor_entities**](IntelApi.md#get_intel_actor_entities) | **GET** /intel/entities/actors/v1 | Retrieve specific actors using their actor IDs.
[**get_intel_indicator_entities**](IntelApi.md#get_intel_indicator_entities) | **POST** /intel/entities/indicators/GET/v1 | Retrieve specific indicators using their indicator IDs.
[**get_intel_report_entities**](IntelApi.md#get_intel_report_entities) | **GET** /intel/entities/reports/v1 | Retrieve specific reports using their report IDs.
[**get_intel_report_pdf**](IntelApi.md#get_intel_report_pdf) | **GET** /intel/entities/report-files/v1 | Return a Report PDF attachment
[**get_intel_rule_entities**](IntelApi.md#get_intel_rule_entities) | **GET** /intel/entities/rules/v1 | Retrieve details for rule sets for the specified ids.
[**get_intel_rule_file**](IntelApi.md#get_intel_rule_file) | **GET** /intel/entities/rules-files/v1 | Download earlier rule sets.
[**get_latest_intel_rule_file**](IntelApi.md#get_latest_intel_rule_file) | **GET** /intel/entities/rules-latest-files/v1 | Download the latest rule set.
[**get_malware_entities**](IntelApi.md#get_malware_entities) | **GET** /intel/entities/malware/v1 | Get malware entities for specified ids.
[**get_mitre_report**](IntelApi.md#get_mitre_report) | **GET** /intel/entities/mitre-reports/v1 | Export Mitre ATT&CK information for a given actor.
[**get_vulnerabilities**](IntelApi.md#get_vulnerabilities) | **POST** /intel/entities/vulnerabilities/GET/v1 | Get vulnerabilities
[**post_mitre_attacks**](IntelApi.md#post_mitre_attacks) | **POST** /intel/entities/mitre/v1 | Retrieves report and observable IDs associated with the given actor and attacks
[**query_intel_actor_entities**](IntelApi.md#query_intel_actor_entities) | **GET** /intel/combined/actors/v1 | Get info about actors that match provided FQL filters.
[**query_intel_actor_ids**](IntelApi.md#query_intel_actor_ids) | **GET** /intel/queries/actors/v1 | Get actor IDs that match provided FQL filters.
[**query_intel_indicator_entities**](IntelApi.md#query_intel_indicator_entities) | **GET** /intel/combined/indicators/v1 | Get info about indicators that match provided FQL filters.
[**query_intel_indicator_ids**](IntelApi.md#query_intel_indicator_ids) | **GET** /intel/queries/indicators/v1 | Get indicators IDs that match provided FQL filters.
[**query_intel_report_entities**](IntelApi.md#query_intel_report_entities) | **GET** /intel/combined/reports/v1 | Get info about reports that match provided FQL filters.
[**query_intel_report_ids**](IntelApi.md#query_intel_report_ids) | **GET** /intel/queries/reports/v1 | Get report IDs that match provided FQL filters.
[**query_intel_rule_ids**](IntelApi.md#query_intel_rule_ids) | **GET** /intel/queries/rules/v1 | Search for rule IDs that match provided filter criteria.
[**query_malware**](IntelApi.md#query_malware) | **GET** /intel/queries/malware/v1 | Get malware family names that match provided FQL filters.
[**query_mitre_attacks**](IntelApi.md#query_mitre_attacks) | **GET** /intel/queries/mitre/v1 | Gets MITRE tactics and techniques for the given actor, returning concatenation of id and tactic and technique ids, example: fancy-bear_TA0011_T1071
[**query_mitre_attacks_for_malware**](IntelApi.md#query_mitre_attacks_for_malware) | **GET** /intel/queries/mitre-malware/v1 | Gets MITRE tactics and techniques for the given malware
[**query_vulnerabilities**](IntelApi.md#query_vulnerabilities) | **GET** /intel/queries/vulnerabilities/v1 | Get vulnerabilities IDs

## get_intel_actor_entities

> models::DomainPeriodActorsResponse get_intel_actor_entities(ids, fields)
Retrieve specific actors using their actor IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the actors you want to retrieve. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | The fields to return, or a predefined set of fields in the form of the collection name surrounded by two underscores like:  \\*\\*\\<collection\\>\\*\\*.  Ex: slug \\*\\*full\\*\\*.  Defaults to \\*\\*basic\\*\\*. |  |

### Return type

[**models::DomainPeriodActorsResponse**](domain.ActorsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_intel_indicator_entities

> models::DomainPeriodPublicIndicatorsV3Response get_intel_indicator_entities(body)
Retrieve specific indicators using their indicator IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::DomainPeriodPublicIndicatorsV3Response**](domain.PublicIndicatorsV3Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_intel_report_entities

> models::DomainPeriodNewsResponse get_intel_report_entities(ids, fields)
Retrieve specific reports using their report IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the reports you want to retrieve. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | The fields to return, or a predefined set of fields in the form of the collection name surrounded by two underscores like:  \\*\\*\\<collection\\>\\*\\*.  Ex: slug \\*\\*full\\*\\*.  Defaults to \\*\\*basic\\*\\*. |  |

### Return type

[**models::DomainPeriodNewsResponse**](domain.NewsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_intel_report_pdf

> get_intel_report_pdf(id, ids)
Return a Report PDF attachment

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the report you want to download as a PDF. |  |
**ids** | Option<**String**> | The ID of the report you want to download as a PDF. This parameter is used only if no id parameter given. |  |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/pdf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_intel_rule_entities

> models::DomainPeriodRulesResponse get_intel_rule_entities(ids)
Retrieve details for rule sets for the specified ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The ids of rules to return. | [required] |

### Return type

[**models::DomainPeriodRulesResponse**](domain.RulesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_intel_rule_file

> get_intel_rule_file(id, accept, format)
Download earlier rule sets.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the rule set. | [required] |
**accept** | Option<**String**> | Choose the format you want the rule set in. |  |
**format** | Option<**String**> | Choose the format you want the rule set in. Valid formats are zip and gzip. Defaults to zip. |  |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/zip, application/gzip, application/octet-stream, application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_latest_intel_rule_file

> get_latest_intel_rule_file(r#type, accept, if_none_match, if_modified_since, format)
Download the latest rule set.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The rule news report type. Accepted values:  snort-suricata-master  snort-suricata-update  snort-suricata-changelog  yara-master  yara-update  yara-changelog  common-event-format  netwitness  cql-master  cql-update  cql-changelog | [required] |
**accept** | Option<**String**> | Choose the format you want the rule set in. |  |
**if_none_match** | Option<**String**> | Download the latest rule set only if it doesn't have an ETag matching the given ones. |  |
**if_modified_since** | Option<**String**> | Download the latest rule set only if the rule was modified after this date. http, ANSIC and RFC850 formats accepted |  |
**format** | Option<**String**> | Choose the format you want the rule set in. Valid formats are zip and gzip. Defaults to zip. |  |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/zip, application/gzip, application/octet-stream, application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_malware_entities

> models::DomainPeriodMalwareResponse get_malware_entities(ids)
Get malware entities for specified ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Malware family name in lower case with spaces, dots and slashes replaced with dashes | [required] |

### Return type

[**models::DomainPeriodMalwareResponse**](domain.MalwareResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_mitre_report

> get_mitre_report(actor_id, format)
Export Mitre ATT&CK information for a given actor.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Actor ID(derived from the actor's name) | [required] |
**format** | **String** | Supported report formats: CSV or JSON | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_vulnerabilities

> models::DomainPeriodVulnerabilityResponse get_vulnerabilities(body)
Get vulnerabilities

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::DomainPeriodVulnerabilityResponse**](domain.VulnerabilityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## post_mitre_attacks

> post_mitre_attacks(body)
Retrieves report and observable IDs associated with the given actor and attacks

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_intel_actor_entities

> models::DomainPeriodActorsResponse query_intel_actor_entities(offset, limit, sort, filter, q, fields)
Get info about actors that match provided FQL filters.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Set the starting row number to return actors from. Defaults to 0. |  |
**limit** | Option<**i32**> | Set the number of actors to return. The value must be between 1 and 5000. |  |
**sort** | Option<**String**> | Order fields in ascending or descending order.  Ex: created_date|asc. |  |
**filter** | Option<**String**> | Filter your query by specifying FQL filter parameters. Filter parameters include:  actor_type, capabilities, capability, capability.id, capability.slug, capability.value, created_date, description, ecrime_kill_chain.attribution, ecrime_kill_chain.crimes, ecrime_kill_chain.customers, ecrime_kill_chain.marketing, ecrime_kill_chain.monetization, ecrime_kill_chain.services_offered, ecrime_kill_chain.services_used, ecrime_kill_chain.technical_tradecraft, ecrime_kill_chain.victims, first_activity_date, group, group.id, group.slug, group.value, id, kill_chain.actions_and_objectives, kill_chain.actions_on_objectives, kill_chain.command_and_control, kill_chain.delivery, kill_chain.exploitation, kill_chain.installation, kill_chain.objectives, kill_chain.reconnaissance, kill_chain.weaponization, known_as, last_activity_date, last_modified_date, motivations, motivations.id, motivations.slug, motivations.value, name, objectives, origins, origins.id, origins.slug, origins.value, region, region.id, region.slug, region.value, short_description, slug, status, target_countries, target_countries.id, target_countries.slug, target_countries.value, target_industries, target_industries.id, target_industries.slug, target_industries.value, target_regions, target_regions.id, target_regions.slug, target_regions.value. |  |
**q** | Option<**String**> | Perform a generic substring search across all fields. |  |
**fields** | Option<[**Vec<String>**](String.md)> | The fields to return, or a predefined set of fields in the form of the collection name surrounded by two underscores like:  \\*\\*\\<collection\\>\\*\\*.  Ex: slug \\*\\*full\\*\\*.  Defaults to \\*\\*basic\\*\\*. |  |

### Return type

[**models::DomainPeriodActorsResponse**](domain.ActorsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_intel_actor_ids

> models::MsaPeriodQueryResponse query_intel_actor_ids(offset, limit, sort, filter, q)
Get actor IDs that match provided FQL filters.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Set the starting row number to return actors IDs from. Defaults to 0. |  |
**limit** | Option<**i32**> | Set the number of actor IDs to return. The value must be between 1 and 5000. |  |
**sort** | Option<**String**> | Order fields in ascending or descending order.  Ex: created_date|asc. |  |
**filter** | Option<**String**> | Filter your query by specifying FQL filter parameters. Filter parameters include:  actor_type, capabilities, capability, capability.id, capability.slug, capability.value, created_date, description, ecrime_kill_chain.attribution, ecrime_kill_chain.crimes, ecrime_kill_chain.customers, ecrime_kill_chain.marketing, ecrime_kill_chain.monetization, ecrime_kill_chain.services_offered, ecrime_kill_chain.services_used, ecrime_kill_chain.technical_tradecraft, ecrime_kill_chain.victims, first_activity_date, group, group.id, group.slug, group.value, id, kill_chain.actions_and_objectives, kill_chain.actions_on_objectives, kill_chain.command_and_control, kill_chain.delivery, kill_chain.exploitation, kill_chain.installation, kill_chain.objectives, kill_chain.reconnaissance, kill_chain.weaponization, known_as, last_activity_date, last_modified_date, motivations, motivations.id, motivations.slug, motivations.value, name, objectives, origins, origins.id, origins.slug, origins.value, region, region.id, region.slug, region.value, short_description, slug, status, target_countries, target_countries.id, target_countries.slug, target_countries.value, target_industries, target_industries.id, target_industries.slug, target_industries.value, target_regions, target_regions.id, target_regions.slug, target_regions.value. |  |
**q** | Option<**String**> | Perform a generic substring search across all fields. |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_intel_indicator_entities

> models::DomainPeriodPublicIndicatorsV3Response query_intel_indicator_entities(offset, limit, sort, filter, q, include_deleted, include_relations)
Get info about indicators that match provided FQL filters.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Set the starting row number to return indicators from. Defaults to 0. |  |
**limit** | Option<**i32**> | Set the number of indicators to return. The number must be between 1 and 10000 |  |
**sort** | Option<**String**> | Order fields in ascending or descending order.  Ex: published_date|asc. |  |
**filter** | Option<**String**> | Filter your query by specifying FQL filter parameters. Filter parameters include:  _marker, actors, deleted, domain_types, id, indicator, ip_address_types, kill_chains, labels, labels.created_on, labels.last_valid_on, labels.name, last_updated, malicious_confidence, malware_families, published_date, reports, scope, targets, threat_types, type, vulnerabilities. |  |
**q** | Option<**String**> | Perform a generic substring search across all fields. |  |
**include_deleted** | Option<**bool**> | If true, include both published and deleted indicators in the response. Defaults to false. |  |
**include_relations** | Option<**bool**> | If true, include related indicators in the response. Defaults to true. |  |

### Return type

[**models::DomainPeriodPublicIndicatorsV3Response**](domain.PublicIndicatorsV3Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_intel_indicator_ids

> models::MsaPeriodQueryResponse query_intel_indicator_ids(offset, limit, sort, filter, q, include_deleted, include_relations)
Get indicators IDs that match provided FQL filters.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Set the starting row number to return indicator IDs from. Defaults to 0. |  |
**limit** | Option<**i32**> | Set the number of indicator IDs to return. The number must be between 1 and 10000 |  |
**sort** | Option<**String**> | Order fields in ascending or descending order.  Ex: published_date|asc. |  |
**filter** | Option<**String**> | Filter your query by specifying FQL filter parameters. Filter parameters include:  _marker, actors, deleted, domain_types, id, indicator, ip_address_types, kill_chains, labels, labels.created_on, labels.last_valid_on, labels.name, last_updated, malicious_confidence, malware_families, published_date, reports, scope, targets, threat_types, type, vulnerabilities. |  |
**q** | Option<**String**> | Perform a generic substring search across all fields. |  |
**include_deleted** | Option<**bool**> | If true, include both published and deleted indicators in the response. Defaults to false. |  |
**include_relations** | Option<**bool**> | If true, include related indicators in the response. Defaults to true. |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_intel_report_entities

> models::DomainPeriodNewsResponse query_intel_report_entities(offset, limit, sort, filter, q, fields)
Get info about reports that match provided FQL filters.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Set the starting row number to return reports from. Defaults to 0. |  |
**limit** | Option<**i32**> | Set the number of reports to return. The value must be between 1 and 5000. |  |
**sort** | Option<**String**> | Order fields in ascending or descending order. Ex: created_date|asc. |  |
**filter** | Option<**String**> | Filter your query by specifying FQL filter parameters. Filter parameters include:  actors, actors.id, actors.name, actors.slug, actors.url, created_date, description, id, last_modified_date, malware, malware.community_identifiers, malware.family_name, malware.slug, motivations, motivations.id, motivations.slug, motivations.value, name, name.raw, short_description, slug, sub_type, sub_type.id, sub_type.name, sub_type.slug, tags, tags.id, tags.slug, tags.value, target_countries, target_countries.id, target_countries.slug, target_countries.value, target_industries, target_industries.id, target_industries.slug, target_industries.value, type, type.id, type.name, type.slug, url. |  |
**q** | Option<**String**> | Perform a generic substring search across all fields. |  |
**fields** | Option<[**Vec<String>**](String.md)> | The fields to return, or a predefined set of fields in the form of the collection name surrounded by two underscores like:  \\*\\*\\<collection\\>\\*\\*.  Ex: slug \\*\\*full\\*\\*.  Defaults to \\*\\*basic\\*\\*. |  |

### Return type

[**models::DomainPeriodNewsResponse**](domain.NewsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_intel_report_ids

> models::MsaPeriodQueryResponse query_intel_report_ids(offset, limit, sort, filter, q)
Get report IDs that match provided FQL filters.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Set the starting row number to return report IDs from. Defaults to 0. |  |
**limit** | Option<**i32**> | Set the number of report IDs to return. The value must be between 1 and 5000. |  |
**sort** | Option<**String**> | Order fields in ascending or descending order.  Ex: created_date|asc. |  |
**filter** | Option<**String**> | Filter your query by specifying FQL filter parameters. Filter parameters include:  actors, actors.id, actors.name, actors.slug, actors.url, created_date, description, id, last_modified_date, malware, malware.community_identifiers, malware.family_name, malware.slug, motivations, motivations.id, motivations.slug, motivations.value, name, name.raw, short_description, slug, sub_type, sub_type.id, sub_type.name, sub_type.slug, tags, tags.id, tags.slug, tags.value, target_countries, target_countries.id, target_countries.slug, target_countries.value, target_industries, target_industries.id, target_industries.slug, target_industries.value, type, type.id, type.name, type.slug, url. |  |
**q** | Option<**String**> | Perform a generic substring search across all fields. |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_intel_rule_ids

> models::MsaPeriodQueryResponse query_intel_rule_ids(r#type, offset, limit, sort, name, description, tags, min_created_date, max_created_date, q)
Search for rule IDs that match provided filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The rule news report type. Accepted values:  snort-suricata-master  snort-suricata-update  snort-suricata-changelog  yara-master  yara-update  yara-changelog  common-event-format  netwitness  cql-master  cql-update  cql-changelog | [required] |
**offset** | Option<**i32**> | Set the starting row number to return reports from. Defaults to 0. |  |
**limit** | Option<**i32**> | The number of rule IDs to return. Defaults to 10. |  |
**sort** | Option<**String**> | Order fields in ascending or descending order.  Ex: created_date|asc. |  |
**name** | Option<[**Vec<String>**](String.md)> | Search by rule title. |  |
**description** | Option<[**Vec<String>**](String.md)> | Substring match on description field. |  |
**tags** | Option<[**Vec<String>**](String.md)> | Search for rule tags. |  |
**min_created_date** | Option<**i32**> | Filter results to those created on or after a certain date. |  |
**max_created_date** | Option<**String**> | Filter results to those created on or before a certain date. |  |
**q** | Option<**String**> | Perform a generic substring search across all fields. |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_malware

> models::DomainPeriodQueryResponse query_malware(offset, limit, sort, filter, q)
Get malware family names that match provided FQL filters.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | Set the starting row number to return malware IDs from. Defaults to 0. |  |
**limit** | Option<**i32**> | Set the number of malware IDs to return. The value must be between 1 and 5000. |  |
**sort** | Option<**String**> | Order fields in ascending or descending order.  Ex: created_date|asc. |  |
**filter** | Option<**String**> | Filter your query by specifying FQL filter parameters. |  |
**q** | Option<**String**> | Perform a generic substring search across all fields. |  |

### Return type

[**models::DomainPeriodQueryResponse**](domain.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_mitre_attacks

> models::DomainPeriodQueryMitreAttacksResponse query_mitre_attacks(id, ids)
Gets MITRE tactics and techniques for the given actor, returning concatenation of id and tactic and technique ids, example: fancy-bear_TA0011_T1071

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The actor ID(derived from the actor's name) for which to retrieve a list of attacks, for example: fancy-bear. Only one value is allowed |  |
**ids** | Option<[**Vec<String>**](String.md)> | The actor ID(derived from the actor's name) for which to retrieve a list of attacks, for example: fancy-bear. Multiple values are allowed |  |

### Return type

[**models::DomainPeriodQueryMitreAttacksResponse**](domain.QueryMitreAttacksResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_mitre_attacks_for_malware

> models::DomainPeriodQueryResponse query_mitre_attacks_for_malware(ids)
Gets MITRE tactics and techniques for the given malware

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Malware family name in lower case with spaces replaced with dashes | [required] |

### Return type

[**models::DomainPeriodQueryResponse**](domain.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_vulnerabilities

> models::MsaPeriodQueryResponse query_vulnerabilities(offset, limit, sort, filter, q)
Get vulnerabilities IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of result set from which to return IDs. |  |
**limit** | Option<**i32**> | Number of IDs to return. |  |
**sort** | Option<**String**> | Order by fields. |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. |  |
**q** | Option<**String**> | Match phrase_prefix query criteria; included fields:_all (all filter string fields indexed). |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
