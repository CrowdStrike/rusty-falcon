# \ComplianceAssessmentsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ext_aggregate_cluster_assessments**](ComplianceAssessmentsApi.md#ext_aggregate_cluster_assessments) | **GET** /container-compliance/aggregates/compliance-by-clusters/v2 | get the assessments for each cluster
[**ext_aggregate_failed_containers_by_rules_path**](ComplianceAssessmentsApi.md#ext_aggregate_failed_containers_by_rules_path) | **GET** /container-compliance/aggregates/failed-containers-by-rules/v2 | get the containers grouped into rules on which they failed
[**ext_aggregate_failed_containers_count_by_severity**](ComplianceAssessmentsApi.md#ext_aggregate_failed_containers_count_by_severity) | **GET** /container-compliance/aggregates/failed-containers-count-by-severity/v2 | get the failed containers count grouped into severity levels
[**ext_aggregate_failed_images_by_rules_path**](ComplianceAssessmentsApi.md#ext_aggregate_failed_images_by_rules_path) | **GET** /container-compliance/aggregates/failed-images-by-rules/v2 | get the images grouped into rules on which they failed
[**ext_aggregate_failed_images_count_by_severity**](ComplianceAssessmentsApi.md#ext_aggregate_failed_images_count_by_severity) | **GET** /container-compliance/aggregates/failed-images-count-by-severity/v2 | get the failed images count grouped into severity levels
[**ext_aggregate_failed_rules_by_clusters**](ComplianceAssessmentsApi.md#ext_aggregate_failed_rules_by_clusters) | **GET** /container-compliance/aggregates/failed-rules-by-clusters/v2 | get the failed rules for each cluster grouped into severity levels
[**ext_aggregate_failed_rules_by_images**](ComplianceAssessmentsApi.md#ext_aggregate_failed_rules_by_images) | **GET** /container-compliance/aggregates/failed-rules-by-images/v2 | get images with failed rules, rule count grouped by severity for each image
[**ext_aggregate_failed_rules_count_by_severity**](ComplianceAssessmentsApi.md#ext_aggregate_failed_rules_count_by_severity) | **GET** /container-compliance/aggregates/failed-rules-count-by-severity/v2 | get the failed rules count grouped into severity levels
[**ext_aggregate_image_assessments**](ComplianceAssessmentsApi.md#ext_aggregate_image_assessments) | **GET** /container-compliance/aggregates/compliance-by-images/v2 | get the assessments for each image
[**ext_aggregate_rules_assessments**](ComplianceAssessmentsApi.md#ext_aggregate_rules_assessments) | **GET** /container-compliance/aggregates/compliance-by-rules/v2 | get the assessments for each rule
[**ext_aggregate_rules_by_status**](ComplianceAssessmentsApi.md#ext_aggregate_rules_by_status) | **GET** /container-compliance/aggregates/rules-by-status/v2 | get the rules grouped by their statuses

## ext_aggregate_cluster_assessments

> models::DomainPeriodAggregateClusterAssessmentsResponse ext_aggregate_cluster_assessments(filter)
get the assessments for each cluster

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter results using a query in Falcon Query Language (FQL). Supported Filters: cloud_info.cloud_account_id: Cloud account ID cloud_info.cluster_name: Kubernetes cluster name cloud_info.namespace: Kubernetes namespace cloud_info.cloud_region: Cloud region cloud_info.cloud_provider: Cloud provider cid: Customer ID compliance_finding.framework: Compliance finding framework (available values: CIS)  |  |

### Return type

[**models::DomainPeriodAggregateClusterAssessmentsResponse**](domain.AggregateClusterAssessmentsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ext_aggregate_failed_containers_by_rules_path

> models::DomainPeriodAggregateFailedAssetsByRulesResponse ext_aggregate_failed_containers_by_rules_path(filter)
get the containers grouped into rules on which they failed

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter results using a query in Falcon Query Language (FQL). Supported Filters: compliance_finding.id: Compliance finding ID compliance_finding.name: Compliance finding Name compliance_finding.framework: Compliance finding framework (available values: CIS) image_tag: Image tag cloud_info.cloud_account_id: Cloud account ID cloud_info.namespace: Kubernetes namespace image_digest: Image digest (sha256 digest) image_repository: Image repository cloud_info.cloud_region: Cloud region cloud_info.cluster_name: Kubernetes cluster name compliance_finding.severity: Compliance finding severity; available values: 4, 3, 2, 1 (4: critical, 3: high, 2: medium, 1:low) image_id: Image ID cid: Customer ID cloud_info.cloud_provider: Cloud provider image_registry: Image registry  |  |

### Return type

[**models::DomainPeriodAggregateFailedAssetsByRulesResponse**](domain.AggregateFailedAssetsByRulesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ext_aggregate_failed_containers_count_by_severity

> models::DomainPeriodAggregateFailedAssetCountBySeverityResponse ext_aggregate_failed_containers_count_by_severity(filter)
get the failed containers count grouped into severity levels

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter results using a query in Falcon Query Language (FQL). Supported Filters: cloud_info.cluster_name: Kubernetes cluster name cloud_info.namespace: Kubernetes namespace cid: Customer ID cloud_info.cloud_provider: Cloud provider compliance_finding.id: Compliance finding ID compliance_finding.severity: Compliance finding severity; available values: 4, 3, 2, 1 (4: critical, 3: high, 2: medium, 1:low) compliance_finding.framework: Compliance finding framework (available values: CIS) cloud_info.cloud_region: Cloud region cloud_info.cloud_account_id: Cloud account ID image_tag: Image tag compliance_finding.name: Compliance finding Name image_repository: Image repository image_id: Image ID image_registry: Image registry image_digest: Image digest (sha256 digest)  |  |

### Return type

[**models::DomainPeriodAggregateFailedAssetCountBySeverityResponse**](domain.AggregateFailedAssetCountBySeverityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ext_aggregate_failed_images_by_rules_path

> models::DomainPeriodAggregateFailedAssetsByRulesResponse ext_aggregate_failed_images_by_rules_path(filter)
get the images grouped into rules on which they failed

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter results using a query in Falcon Query Language (FQL). Supported Filters: cloud_info.namespace: Kubernetes namespace cloud_info.cluster_name: Kubernetes cluster name cloud_info.cloud_account_id: Cloud account ID image_id: Image ID compliance_finding.framework: Compliance finding framework (available values: CIS) image_digest: Image digest (sha256 digest) image_repository: Image repository cloud_info.cloud_region: Cloud region compliance_finding.id: Compliance finding ID image_tag: Image tag cloud_info.cloud_provider: Cloud provider compliance_finding.name: Compliance finding Name compliance_finding.severity: Compliance finding severity; available values: 4, 3, 2, 1 (4: critical, 3: high, 2: medium, 1:low) image_registry: Image registry cid: Customer ID  |  |

### Return type

[**models::DomainPeriodAggregateFailedAssetsByRulesResponse**](domain.AggregateFailedAssetsByRulesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ext_aggregate_failed_images_count_by_severity

> models::DomainPeriodAggregateFailedAssetCountBySeverityResponse ext_aggregate_failed_images_count_by_severity(filter)
get the failed images count grouped into severity levels

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter results using a query in Falcon Query Language (FQL). Supported Filters: cloud_info.cloud_region: Cloud region image_repository: Image repository cloud_info.cloud_account_id: Cloud account ID image_id: Image ID cloud_info.namespace: Kubernetes namespace compliance_finding.name: Compliance finding Name compliance_finding.framework: Compliance finding framework (available values: CIS) cloud_info.cluster_name: Kubernetes cluster name compliance_finding.id: Compliance finding ID compliance_finding.severity: Compliance finding severity; available values: 4, 3, 2, 1 (4: critical, 3: high, 2: medium, 1:low) image_tag: Image tag image_digest: Image digest (sha256 digest) cloud_info.cloud_provider: Cloud provider image_registry: Image registry cid: Customer ID  |  |

### Return type

[**models::DomainPeriodAggregateFailedAssetCountBySeverityResponse**](domain.AggregateFailedAssetCountBySeverityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ext_aggregate_failed_rules_by_clusters

> models::DomainPeriodAggregateFailedRulesByClustersResponse ext_aggregate_failed_rules_by_clusters(filter)
get the failed rules for each cluster grouped into severity levels

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter results using a query in Falcon Query Language (FQL). Supported Filters: cloud_info.cluster_name: Kubernetes cluster name compliance_finding.name: Compliance finding Name compliance_finding.framework: Compliance finding framework (available values: CIS) cloud_info.cloud_provider: Cloud provider image_tag: Image tag asset_type: asset type (container, image) image_id: Image ID cid: Customer ID image_digest: Image digest (sha256 digest) image_repository: Image repository cloud_info.cloud_region: Cloud region compliance_finding.id: Compliance finding ID image_registry: Image registry cloud_info.cloud_account_id: Cloud account ID compliance_finding.severity: Compliance finding severity; available values: 4, 3, 2, 1 (4: critical, 3: high, 2: medium, 1:low)  |  |

### Return type

[**models::DomainPeriodAggregateFailedRulesByClustersResponse**](domain.AggregateFailedRulesByClustersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ext_aggregate_failed_rules_by_images

> models::DomainPeriodAggregateFailedRulesByImagesResponse ext_aggregate_failed_rules_by_images(filter)
get images with failed rules, rule count grouped by severity for each image

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter results using a query in Falcon Query Language (FQL). Supported Filters: cloud_info.cloud_account_id: Cloud account ID cloud_info.cloud_provider: Cloud provider cloud_info.cluster_name: Kubernetes cluster name image_id: Image ID image_registry: Image registry image_digest: Image digest (sha256 digest) compliance_finding.id: Compliance finding ID asset_type: asset type (container, image) compliance_finding.name: Compliance finding Name cloud_info.namespace: Kubernetes namespace compliance_finding.framework: Compliance finding framework (available values: CIS) image_repository: Image repository image_tag: Image tag compliance_finding.severity: Compliance finding severity; available values: 4, 3, 2, 1 (4: critical, 3: high, 2: medium, 1:low) cid: Customer ID cloud_info.cloud_region: Cloud region  |  |

### Return type

[**models::DomainPeriodAggregateFailedRulesByImagesResponse**](domain.AggregateFailedRulesByImagesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ext_aggregate_failed_rules_count_by_severity

> models::DomainPeriodAggregateFailedRulesCountBySeverityResponse ext_aggregate_failed_rules_count_by_severity(filter)
get the failed rules count grouped into severity levels

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter results using a query in Falcon Query Language (FQL). Supported Filters: cloud_info.cluster_name: Kubernetes cluster name cloud_info.cloud_provider: Cloud provider image_id: Image ID compliance_finding.severity: Compliance finding severity; available values: 4, 3, 2, 1 (4: critical, 3: high, 2: medium, 1:low) compliance_finding.framework: Compliance finding framework (available values: CIS) image_digest: Image digest (sha256 digest) image_registry: Image registry asset_type: asset type (container, image) cid: Customer ID compliance_finding.name: Compliance finding Name image_repository: Image repository cloud_info.cloud_region: Cloud region compliance_finding.id: Compliance finding ID image_tag: Image tag cloud_info.cloud_account_id: Cloud account ID  |  |

### Return type

[**models::DomainPeriodAggregateFailedRulesCountBySeverityResponse**](domain.AggregateFailedRulesCountBySeverityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ext_aggregate_image_assessments

> models::DomainPeriodAggregateImageAssessmentsResponse ext_aggregate_image_assessments(filter, after, limit)
get the assessments for each image

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter results using a query in Falcon Query Language (FQL). Supported Filters: image_tag: Image tag cloud_info.cloud_provider: Cloud provider image_id: Image ID cid: Customer ID compliance_finding.severity: Compliance finding severity; available values: 4, 3, 2, 1 (4: critical, 3: high, 2: medium, 1:low) cloud_info.namespace: Kubernetes namespace compliance_finding.id: Compliance finding ID asset_type: asset type (container, image) compliance_finding.framework: Compliance finding framework (available values: CIS) image_repository: Image repository cloud_info.cloud_account_id: Cloud account ID cloud_info.cluster_name: Kubernetes cluster name compliance_finding.name: Compliance finding Name image_digest: Image digest (sha256 digest) cloud_info.cloud_region: Cloud region image_registry: Image registry  |  |
**after** | Option<**String**> | 'after' value from the last response. Keep it empty for the first request. |  |
**limit** | Option<**String**> | number of images to return in the response after 'after' key. Keep it empty for the default number of 10000 |  |

### Return type

[**models::DomainPeriodAggregateImageAssessmentsResponse**](domain.AggregateImageAssessmentsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ext_aggregate_rules_assessments

> models::DomainPeriodAggregateRulesAssessmentsResponse ext_aggregate_rules_assessments(filter)
get the assessments for each rule

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter results using a query in Falcon Query Language (FQL). Supported Filters: cloud_info.cluster_name: Kubernetes cluster name compliance_finding.severity: Compliance finding severity; available values: 4, 3, 2, 1 (4: critical, 3: high, 2: medium, 1:low) image_id: Image ID compliance_finding.name: Compliance finding Name image_digest: Image digest (sha256 digest) cloud_info.cloud_provider: Cloud provider cid: Customer ID image_repository: Image repository compliance_finding.id: Compliance finding ID image_tag: Image tag cloud_info.cloud_account_id: Cloud account ID image_registry: Image registry compliance_finding.framework: Compliance finding framework (available values: CIS) cloud_info.cloud_region: Cloud region  |  |

### Return type

[**models::DomainPeriodAggregateRulesAssessmentsResponse**](domain.AggregateRulesAssessmentsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ext_aggregate_rules_by_status

> models::DomainPeriodAggregateRulesByStatusResponse ext_aggregate_rules_by_status(filter)
get the rules grouped by their statuses

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter results using a query in Falcon Query Language (FQL). Supported Filters: image_repository: Image repository cloud_info.cloud_region: Cloud region asset_type: asset type (container, image) compliance_finding.name: Compliance finding Name container_name: Container name cloud_info.cloud_account_id: Cloud account ID compliance_finding.id: Compliance finding ID compliance_finding.framework: Compliance finding framework (available values: CIS) image_digest: Image digest (sha256 digest) image_tag: Image tag container_id: Container ID cloud_info.cloud_provider: Cloud provider cloud_info.cluster_name: Kubernetes cluster name compliance_finding.severity: Compliance finding severity; available values: 4, 3, 2, 1 (4: critical, 3: high, 2: medium, 1:low) image_id: Image ID cid: Customer ID image_registry: Image registry  |  |

### Return type

[**models::DomainPeriodAggregateRulesByStatusResponse**](domain.AggregateRulesByStatusResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
