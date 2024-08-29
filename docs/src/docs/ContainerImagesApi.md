# \ContainerImagesApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**aggregate_image_assessment_history**](ContainerImagesApi.md#aggregate_image_assessment_history) | **GET** /container-security/aggregates/images/assessment-history/v1 | Image assessment history
[**aggregate_image_count**](ContainerImagesApi.md#aggregate_image_count) | **GET** /container-security/aggregates/images/count/v1 | Aggregate count of images
[**aggregate_image_count_by_base_os**](ContainerImagesApi.md#aggregate_image_count_by_base_os) | **GET** /container-security/aggregates/images/count-by-os-distribution/v1 | Aggregate count of images grouped by Base OS distribution
[**aggregate_image_count_by_state**](ContainerImagesApi.md#aggregate_image_count_by_state) | **GET** /container-security/aggregates/images/count-by-state/v1 | Aggregate count of images grouped by state
[**combined_base_images**](ContainerImagesApi.md#combined_base_images) | **GET** /container-security/combined/base-images/v1 | Retrieve base images for provided filter
[**combined_image_by_vulnerability_count**](ContainerImagesApi.md#combined_image_by_vulnerability_count) | **GET** /container-security/combined/images/by-vulnerability-count/v1 | Retrieve top x images with the most vulnerabilities
[**combined_image_detail**](ContainerImagesApi.md#combined_image_detail) | **GET** /container-security/combined/images/detail/v1 | Retrieve image entities identified by the provided filter criteria
[**combined_image_issues_summary**](ContainerImagesApi.md#combined_image_issues_summary) | **GET** /container-security/combined/images/issues-summary/v1 | Retrieve image issues summary such as Image detections, Runtime detections, Policies, vulnerabilities
[**combined_image_vulnerability_summary**](ContainerImagesApi.md#combined_image_vulnerability_summary) | **GET** /container-security/combined/images/vulnerabilities-summary/v1 | aggregates information about vulnerabilities for an image
[**create_base_images_entities**](ContainerImagesApi.md#create_base_images_entities) | **POST** /container-security/entities/base-images/v1 | Creates base images using the provided details
[**delete_base_images**](ContainerImagesApi.md#delete_base_images) | **DELETE** /container-security/entities/base-images/v1 | Delete base images by base image uuid
[**get_combined_images**](ContainerImagesApi.md#get_combined_images) | **GET** /container-security/combined/image-assessment/images/v1 | Get image assessment results by providing an FQL filter and paging details
[**read_combined_images_export**](ContainerImagesApi.md#read_combined_images_export) | **GET** /container-security/combined/images/export/v1 | Retrieve images with an option to expand aggregated vulnerabilities/detections

## aggregate_image_assessment_history

> models::ImagesPeriodApiImageAssessmentHistory aggregate_image_assessment_history(filter)
Image assessment history

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter using a query in Falcon Query Language (FQL). Supported filters:  cid,registry,repository |  |

### Return type

[**models::ImagesPeriodApiImageAssessmentHistory**](images.apiImageAssessmentHistory.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## aggregate_image_count

> models::ImagesPeriodApiImageCount aggregate_image_count(filter)
Aggregate count of images

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters:  arch,base_os,cid,container_id,container_running_status,cps_rating,crowdstrike_user,cve_id,detection_count,detection_name,detection_severity,first_seen,image_digest,image_id,layer_digest,package_name_version,registry,repository,tag,vulnerability_count,vulnerability_severity |  |

### Return type

[**models::ImagesPeriodApiImageCount**](images.apiImageCount.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## aggregate_image_count_by_base_os

> models::ImagesPeriodApiImageCountByBaseOs aggregate_image_count_by_base_os(filter)
Aggregate count of images grouped by Base OS distribution

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters:  arch,base_os,cid,registry,repository,tag |  |

### Return type

[**models::ImagesPeriodApiImageCountByBaseOs**](images.apiImageCountByBaseOS.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## aggregate_image_count_by_state

> models::ImagesPeriodApiImageCountByState aggregate_image_count_by_state(filter)
Aggregate count of images grouped by state

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters:  cid,last_seen,registry,repository |  |

### Return type

[**models::ImagesPeriodApiImageCountByState**](images.apiImageCountByState.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## combined_base_images

> models::CorePeriodEntitiesResponse combined_base_images(filter)
Retrieve base images for provided filter

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search base images using a query in Falcon Query Language (FQL). Supported filters:  image_digest,image_id,registry,repository,tag |  |

### Return type

[**models::CorePeriodEntitiesResponse**](core.EntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## combined_image_by_vulnerability_count

> models::ImagesPeriodApiImageByVulnerabilityCount combined_image_by_vulnerability_count(filter, limit, offset)
Retrieve top x images with the most vulnerabilities

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters:  arch,base_os,cid,registry,repository,tag |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | This is not used in the backend but is added here for compatibility purposes as some clients expects this i.e UI widgets. |  |

### Return type

[**models::ImagesPeriodApiImageByVulnerabilityCount**](images.apiImageByVulnerabilityCount.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## combined_image_detail

> models::ImagesPeriodApiCustomerAndImage combined_image_detail(filter, with_config, limit, offset, sort)
Retrieve image entities identified by the provided filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters:  registry,repository,tag |  |
**with_config** | Option<**bool**> | (true/false) include image config, default is false |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The fields to sort the records on. |  |

### Return type

[**models::ImagesPeriodApiCustomerAndImage**](images.apiCustomerAndImage.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## combined_image_issues_summary

> models::ImagesPeriodApiImageIssuesSummary combined_image_issues_summary(cid, registry, repository, tag)
Retrieve image issues summary such as Image detections, Runtime detections, Policies, vulnerabilities

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | CID | [required] |
**registry** | **String** | registry name | [required] |
**repository** | **String** | repository name | [required] |
**tag** | **String** | tag name | [required] |

### Return type

[**models::ImagesPeriodApiImageIssuesSummary**](images.apiImageIssuesSummary.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## combined_image_vulnerability_summary

> models::ImagesPeriodApiImageVulnerabilitiesSummary combined_image_vulnerability_summary(cid, registry, repository, tag)
aggregates information about vulnerabilities for an image

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | CID | [required] |
**registry** | **String** | registry name | [required] |
**repository** | **String** | repository name | [required] |
**tag** | **String** | tag name | [required] |

### Return type

[**models::ImagesPeriodApiImageVulnerabilitiesSummary**](images.apiImageVulnerabilitiesSummary.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_base_images_entities

> models::CorePeriodEntitiesResponse create_base_images_entities(body)
Creates base images using the provided details

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodCreateBaseImagesRequest**](ModelsPeriodCreateBaseImagesRequest.md) |  | [required] |

### Return type

[**models::CorePeriodEntitiesResponse**](core.EntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_base_images

> models::CorePeriodEntitiesResponse delete_base_images(ids)
Delete base images by base image uuid

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | BaseImageIDs | [required] |

### Return type

[**models::CorePeriodEntitiesResponse**](core.EntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_combined_images

> models::ImagesPeriodExtCombinedImagesResponse get_combined_images(filter, limit, offset, sort)
Get image assessment results by providing an FQL filter and paging details

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters:  container_id, container_running_status, cve_id, detection_name, detection_severity, first_seen, image_digest, image_id, registry, repository, tag, vulnerability_severity |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve [1-100] |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The fields to sort the records on. Supported columns:  [first_seen highest_detection_severity highest_vulnerability_severity image_digest image_id registry repository tag] |  |

### Return type

[**models::ImagesPeriodExtCombinedImagesResponse**](images.ExtCombinedImagesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_combined_images_export

> models::ImagesPeriodApiCombinedImageExport read_combined_images_export(filter, expand_vulnerabilities, expand_detections, limit, offset, sort)
Retrieve images with an option to expand aggregated vulnerabilities/detections

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters:  arch,base_os,cid,container_id,container_running_status,cps_rating,crowdstrike_user,cve_id,detection_count,detection_name,detection_severity,first_seen,image_digest,image_id,layer_digest,package_name_version,registry,repository,tag,vulnerability_count,vulnerability_severity |  |
**expand_vulnerabilities** | Option<**bool**> | expand vulnerabilities |  |
**expand_detections** | Option<**bool**> | expand detections |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The fields to sort the records on. Supported columns:  [base_os cid detections firstScanned first_seen highest_cps_current_rating highest_detection_severity highest_vulnerability_severity image_digest image_id last_seen layers_with_vulnerabilities packages registry repository tag vulnerabilities] |  |

### Return type

[**models::ImagesPeriodApiCombinedImageExport**](images.apiCombinedImageExport.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
