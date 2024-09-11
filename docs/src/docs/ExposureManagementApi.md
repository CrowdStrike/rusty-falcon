# \ExposureManagementApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**aggregate_external_assets**](ExposureManagementApi.md#aggregate_external_assets) | **POST** /fem/aggregates/external-assets/v1 | Returns external assets aggregates.
[**blob_download_external_assets**](ExposureManagementApi.md#blob_download_external_assets) | **GET** /fem/entities/blobs-download/v1 | Download the entire contents of the blob. The relative link to this endpoint is returned in the GET /entities/external-assets/v1 request.
[**blob_preview_external_assets**](ExposureManagementApi.md#blob_preview_external_assets) | **GET** /fem/entities/blobs-preview/v1 | Download a preview of the blob. The relative link to this endpoint is returned in the GET /entities/external-assets/v1 request.
[**get_external_assets**](ExposureManagementApi.md#get_external_assets) | **GET** /fem/entities/external-assets/v1 | Get details on external assets by providing one or more IDs.
[**patch_external_assets**](ExposureManagementApi.md#patch_external_assets) | **PATCH** /fem/entities/external-assets/v1 | Update the details of external assets.
[**query_external_assets**](ExposureManagementApi.md#query_external_assets) | **GET** /fem/queries/external-assets/v1 | Get a list of external asset IDs that match the provided filter conditions. Use these IDs with the /entities/external-assets/v1 endpoints

## aggregate_external_assets

> models::MsaPeriodAggregatesResponse aggregate_external_assets(body)
Returns external assets aggregates.

Returns external assets aggregates as specified via JSON in request body.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) | Aggregation specification. | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## blob_download_external_assets

> Vec<i32> blob_download_external_assets(asset_id, hash)
Download the entire contents of the blob. The relative link to this endpoint is returned in the GET /entities/external-assets/v1 request.

Download the entire contents of the blob.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The Asset ID | [required] |
**hash** | **String** | The File Hash | [required] |

### Return type

**Vec<i32>**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## blob_preview_external_assets

> models::DomainPeriodExternalAssetsBlobApiTypeV1 blob_preview_external_assets(asset_id, hash)
Download a preview of the blob. The relative link to this endpoint is returned in the GET /entities/external-assets/v1 request.

Download a preview of the blob.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The Asset ID | [required] |
**hash** | **String** | The File Hash | [required] |

### Return type

[**models::DomainPeriodExternalAssetsBlobApiTypeV1**](domain.ExternalAssetsBlobAPITypeV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_external_assets

> models::DomainPeriodExternalAssetsApiTypeV1 get_external_assets(ids)
Get details on external assets by providing one or more IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more asset IDs (max: 100). Find asset IDs with GET `/fem/queries/external-assets/v1` | [required] |

### Return type

[**models::DomainPeriodExternalAssetsApiTypeV1**](domain.ExternalAssetsAPITypeV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## patch_external_assets

> models::DomainPeriodExternalAssetsApiTypeV1 patch_external_assets(body)
Update the details of external assets.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodExternalAssetApiPatchRequestV1**](DomainPeriodExternalAssetApiPatchRequestV1.md) |  | [required] |

### Return type

[**models::DomainPeriodExternalAssetsApiTypeV1**](domain.ExternalAssetsAPITypeV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_external_assets

> models::MsaspecPeriodQueryResponse query_external_assets(offset, limit, sort, filter)
Get a list of external asset IDs that match the provided filter conditions. Use these IDs with the /entities/external-assets/v1 endpoints

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of result set from which to return IDs. |  |
**limit** | Option<**i32**> | Number of IDs to return. |  |
**sort** | Option<**String**> | Order by fields. |  |
**filter** | Option<**String**> | Filter assets using an FQL query. Common filter options include:<ul><li>asset_type:'ip'</li><li>last_seen_timestamp:>'now-7d'</li></ul>    </br>Available filter fields that support exact match: asset_id, asset_type, confidence, connectivity_status, criticality, criticality_description, criticality_timestamp, criticality_username, data_providers, discovered_by, dns_domain.fqdn, dns_domain.isps, dns_domain.parent_domain, dns_domain.resolved_ips, dns_domain.services.applications.category, dns_domain.services.applications.cpe, dns_domain.services.applications.name, dns_domain.services.applications.vendor, dns_domain.services.applications.version, dns_domain.services.cloud_provider, dns_domain.services.cpes, dns_domain.services.hosting_provider, dns_domain.services.last_seen, dns_domain.services.platform_name, dns_domain.services.port, dns_domain.services.protocol, dns_domain.services.protocol_port, dns_domain.services.status, dns_domain.services.status_code, dns_domain.services.transport, dns_domain.type, first_seen, id, internet_exposure, ip.asn, ip.cloud_provider, ip.cloud_vm.description, ip.cloud_vm.instance_id, ip.cloud_vm.lifecycle, ip.cloud_vm.mac_address, ip.cloud_vm.owner_id, ip.cloud_vm.platform, ip.cloud_vm.private_ip, ip.cloud_vm.public_ip, ip.cloud_vm.region, ip.cloud_vm.security_groups, ip.cloud_vm.source, ip.cloud_vm.status, ip.fqdns, ip.ip_address, ip.isp, ip.location.area_code, ip.location.city, ip.location.country_code, ip.location.country_name, ip.location.postal_code, ip.location.region_code, ip.location.region_name, ip.location.timezone, ip.ptr, ip.aid, ip.services.applications.category, ip.services.applications.cpe, ip.services.applications.name, ip.services.applications.vendor, ip.services.applications.version, ip.services.cloud_provider, ip.services.cpes, ip.services.first_seen, ip.services.last_seen, ip.services.platform_name, ip.services.port, ip.services.protocol, ip.services.protocol_port, ip.services.status, ip.services.status_code, ip.services.transport, last_seen, manual, perimeter, subsidiaries.id, subsidiaries.name, triage.action, triage.assigned_to, triage.status, triage.updated_by, triage.updated_timestamp    </br>Available filter fields that supports wildcard (*): asset_id, asset_type, confidence, connectivity_status, criticality, criticality_username, data_providers, discovered_by, dns_domain.fqdn, dns_domain.isps, dns_domain.parent_domain, dns_domain.resolved_ips, dns_domain.services.applications.category, dns_domain.services.applications.cpe, dns_domain.services.applications.name, dns_domain.services.applications.vendor, dns_domain.services.applications.version, dns_domain.services.cloud_provider, dns_domain.services.cpes, dns_domain.services.hosting_provider, dns_domain.services.id, dns_domain.services.platform_name, dns_domain.services.port, dns_domain.services.protocol, dns_domain.services.protocol_port, dns_domain.services.status, dns_domain.services.status_code, dns_domain.services.transport, dns_domain.type, id, internet_exposure, ip.asn, ip.cloud_vm.instance_id, ip.cloud_vm.lifecycle, ip.cloud_vm.mac_address, ip.cloud_vm.owner_id, ip.cloud_vm.platform, ip.cloud_vm.private_ip, ip.cloud_vm.public_ip, ip.cloud_vm.region, ip.cloud_vm.security_groups, ip.cloud_vm.source, ip.cloud_vm.status, ip.fqdns, ip.ip_address, ip.isp, ip.location.area_code, ip.location.city, ip.location.country_code, ip.location.country_name, ip.location.postal_code, ip.location.region_code, ip.location.region_name, ip.location.timezone, ip.ptr, ip.aid, ip.services.applications.category, ip.services.applications.cpe, ip.services.applications.name, ip.services.applications.vendor, ip.services.applications.version, ip.services.cloud_provider, ip.services.cpes, ip.services.platform_name, ip.services.port, ip.services.protocol, ip.services.protocol_port, ip.services.status, ip.services.status_code, ip.services.transport, manual, perimeter, subsidiaries.id, subsidiaries.name, triage.action, triage.assigned_to, triage.status, triage.updated_by    </br>Available filter fields that supports in ([v1, v2]): asset_id, asset_type, confidence, connectivity_status, criticality, criticality_username, data_providers, discovered_by, dns_domain.fqdn, dns_domain.isps, dns_domain.parent_domain, dns_domain.services.applications.category, dns_domain.services.applications.cpe, dns_domain.services.applications.name, dns_domain.services.applications.vendor, dns_domain.services.applications.version, dns_domain.services.cloud_provider, dns_domain.services.cpes, dns_domain.services.id, dns_domain.services.platform_name, dns_domain.services.port, dns_domain.services.protocol, dns_domain.services.protocol_port, dns_domain.services.status, dns_domain.services.status_code, dns_domain.services.transport, dns_domain.type, id, internet_exposure, ip.asn, ip.cloud_vm.instance_id, ip.cloud_vm.lifecycle, ip.cloud_vm.mac_address, ip.cloud_vm.owner_id, ip.cloud_vm.platform, ip.cloud_vm.region, ip.cloud_vm.security_groups, ip.cloud_vm.source, ip.cloud_vm.status, ip.fqdns, ip.isp, ip.location.area_code, ip.location.city, ip.location.country_code, ip.location.country_name, ip.location.postal_code, ip.location.region_code, ip.location.region_name, ip.location.timezone, ip.ptr, ip.aid, ip.services.applications.category, ip.services.applications.cpe, ip.services.applications.name, ip.services.applications.vendor, ip.services.applications.version, ip.services.cloud_provider, ip.services.cpes, ip.services.platform_name, ip.services.port, ip.services.protocol, ip.services.protocol_port, ip.services.status, ip.services.status_code, ip.services.transport, manual, perimeter, subsidiaries.id, subsidiaries.name, triage.action, triage.assigned_to, triage.status, triage.updated_by    </br>Available filter fields that supports range comparisons (>, <, >=, <=): criticality_timestamp, dns_domain.resolved_ips, dns_domain.services.first_seen, dns_domain.services.last_seen, dns_domain.services.port, dns_domain.services.status_code, first_seen, ip.cloud_vm.private_ip, ip.cloud_vm.public_ip, ip.ip_address, ip.services.first_seen, ip.services.last_seen, ip.services.port, ip.services.status_code, last_seen, triage.updated_timestamp    </br>All filter fields and operations supports negation (!). |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
