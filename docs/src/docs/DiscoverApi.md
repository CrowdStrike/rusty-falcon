# \DiscoverApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**combined_applications**](DiscoverApi.md#combined_applications) | **GET** /discover/combined/applications/v1 | Search for applications in your environment by providing an FQL filter and paging details. Returns details on applications which match the filter criteria.
[**combined_hosts**](DiscoverApi.md#combined_hosts) | **GET** /discover/combined/hosts/v1 | Search for assets in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns details on assets which match the filter criteria.
[**get_accounts**](DiscoverApi.md#get_accounts) | **GET** /discover/entities/accounts/v1 | Get details on accounts by providing one or more IDs.
[**get_applications**](DiscoverApi.md#get_applications) | **GET** /discover/entities/applications/v1 | Get details on applications by providing one or more IDs.
[**get_hosts**](DiscoverApi.md#get_hosts) | **GET** /discover/entities/hosts/v1 | Get details on assets by providing one or more IDs.
[**get_logins**](DiscoverApi.md#get_logins) | **GET** /discover/entities/logins/v1 | Get details on logins by providing one or more IDs.
[**query_accounts**](DiscoverApi.md#query_accounts) | **GET** /discover/queries/accounts/v1 | Search for accounts in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of account IDs which match the filter criteria.
[**query_applications**](DiscoverApi.md#query_applications) | **GET** /discover/queries/applications/v1 | Search for applications in your environment by providing an FQL filter and paging details. returns a set of application IDs which match the filter criteria.
[**query_hosts**](DiscoverApi.md#query_hosts) | **GET** /discover/queries/hosts/v1 | Search for assets in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of asset IDs which match the filter criteria.
[**query_logins**](DiscoverApi.md#query_logins) | **GET** /discover/queries/logins/v1 | Search for logins in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of login IDs which match the filter criteria.

## combined_applications

> models::DomainPeriodDiscoverApiCombinedApplicationsResponse combined_applications(filter, after, limit, sort, facet)
Search for applications in your environment by providing an FQL filter and paging details. Returns details on applications which match the filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | Search for applications in your environment by providing an FQL filter.     Available filter fields that support exact match: name, version, vendor, name_vendor, name_vendor_version, first_seen_timestamp, installation_timestamp, architectures, installation_paths, versioning_scheme, groups, is_normalized, last_used_user_sid, last_used_user_name, last_used_file_name, last_used_file_hash, last_used_timestamp, last_updated_timestamp, is_suspicious, host.id, host.platform_name, host.hostname, cid, host.os_version, host.machine_domain, host.ou, host.site_name, host.country, host.current_mac_address, host.current_network_prefix, host.tags, host.groups, host.product_type_desc, host.kernel_version, host.system_manufacturer, host.internet_exposure, host.agent_version, host.external_ip, host.aid     Available filter fields that supports wildcard (*): name, version, vendor, name_vendor, name_vendor_version, architectures, installation_paths, groups, last_used_user_sid, last_used_user_name, last_used_file_name, last_used_file_hash, host.platform_name, host.hostname, cid, host.os_version, host.machine_domain, host.ou, host.site_name, host.country, host.current_mac_address, host.current_network_prefix, host.tags, host.groups, host.product_type_desc, host.kernel_version, host.system_manufacturer, host.internet_exposure, host.agent_version, host.external_ip, host.aid     Available filter fields that supports range comparisons (>, <, >=, <=): first_seen_timestamp, installation_timestamp, last_used_timestamp, last_updated_timestamp     All filter fields and operations supports negation (!). | [required] |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an `after` token. On subsequent requests, provide the `after` token from the previous response to continue from that place in the results. |  |
**limit** | Option<**i32**> | The number of application ids to return in this response (Min: 1, Max: 1000, Default: 100). Use with the `after` parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort applications by their properties. A single sort field is allowed. |  |
**facet** | Option<[**Vec<String>**](String.md)> | Select various details blocks to be returned for each application entity. Supported values:  <ul><li>browser_extension</li><li>host_info</li><li>install_usage</li></ul> |  |

### Return type

[**models::DomainPeriodDiscoverApiCombinedApplicationsResponse**](domain.DiscoverAPICombinedApplicationsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## combined_hosts

> models::DomainPeriodDiscoverApiCombinedHostsResponse combined_hosts(filter, after, limit, sort, facet)
Search for assets in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns details on assets which match the filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | Filter assets using an FQL query. Common filter options include:<ul><li>entity_type:'managed'</li><li>product_type_desc:'Workstation'</li><li>platform_name:'Windows'</li><li>last_seen_timestamp:>'now-7d'</li></ul>    Available filter fields that support exact match: id, aid, entity_type, country, city, platform_name, os_version, kernel_version, product_type_desc, tags, groups, agent_version, system_product_name, system_manufacturer, system_serial_number, bios_manufacturer, bios_version, ou, machine_domain, site_name, external_ip, hostname, local_ips_count, network_interfaces.local_ip, network_interfaces.mac_address, network_interfaces.interface_alias, network_interfaces.interface_description, network_interfaces.network_prefix, last_discoverer_aid, discoverer_count, discoverer_aids, discoverer_tags, discoverer_platform_names, discoverer_product_type_descs, confidence, internet_exposure,  os_is_eol, data_providers, data_providers_count, mac_addresses, local_ip_addresses, reduced_functionality_mode, number_of_disk_drives, processor_package_count, physical_core_count, logical_core_count, total_disk_space, disk_sizes.disk_name, disk_sizes.disk_space, cpu_processor_name, total_memory, encryption_status, encrypted_drives, encrypted_drives_count, unencrypted_drives, unencrypted_drives_count, os_security.secure_boot_requested_status, os_security.device_guard_status, os_security.device_guard_status, os_security.device_guard_status, os_security.system_guard_status, os_security.credential_guard_status, os_security.iommu_protection_status, os_security.secure_boot_enabled_status, os_security.uefi_memory_protection_status, os_security.virtualization_based_security_status, os_security.kernel_dma_protection_status, total_bios_files, bios_hashes_data.sha256_hash, bios_hashes_data.measurement_type, bios_id, average_processor_usage, average_memory_usage, average_memory_usage_pct, max_processor_usage, max_memory_usage, max_memory_usage_pct, used_disk_space, used_disk_space_pct, available_disk_space, available_disk_space_pct, mount_storage_info.mount_path, mount_storage_info.used_space, mount_storage_info.available_space, form_factor, servicenow_id, owned_by, managed_by, assigned_to, department, fqdn, used_for, object_guid, object_sid, ad_user_account_control, account_enabled, creation_timestamp, email, os_service_pack, location, state, cpu_manufacturer, discovering_by    Available filter fields that supports wildcard (*): id, aid, entity_type, country, city, platform_name, os_version, kernel_version, product_type_desc, tags, groups, agent_version, system_product_name, system_manufacturer, system_serial_number, bios_manufacturer, bios_version, ou, machine_domain, site_name, external_ip, hostname, network_interfaces.local_ip, network_interfaces.mac_address, network_interfaces.interface_alias, network_interfaces.interface_description, network_interfaces.network_prefix, last_discoverer_aid, discoverer_aids, discoverer_tags, discoverer_platform_names, discoverer_product_type_descs, confidence, internet_exposure,  os_is_eol, data_providers, mac_addresses, local_ip_addresses, reduced_functionality_mode, disk_sizes.disk_name, cpu_processor_name, encryption_status, encrypted_drives, unencrypted_drives, os_security.secure_boot_requested_status, os_security.device_guard_status, os_security.device_guard_status, os_security.device_guard_status, os_security.system_guard_status, os_security.credential_guard_status, os_security.iommu_protection_status, os_security.secure_boot_enabled_status, os_security.uefi_memory_protection_status, os_security.virtualization_based_security_status, os_security.kernel_dma_protection_status, bios_hashes_data.sha256_hash, bios_hashes_data.measurement_type, bios_id, mount_storage_info.mount_path, form_factor, servicenow_id, owned_by, managed_by, assigned_to, department, fqdn, used_for, object_guid, object_sid, account_enabled, email, os_service_pack, location, state, cpu_manufacturer, discovering_by    Available filter fields that supports range comparisons (>, <, >=, <=): first_seen_timestamp, last_seen_timestamp, local_ips_count, discoverer_count, confidence, number_of_disk_drives, processor_package_count, physical_core_count, data_providers_count, logical_core_count, total_disk_space, disk_sizes.disk_space, total_memory, encrypted_drives_count, unencrypted_drives_count, total_bios_files, average_processor_usage, average_memory_usage, average_memory_usage_pct, max_processor_usage, max_memory_usage, max_memory_usage_pct, used_disk_space, used_disk_space_pct, available_disk_space, available_disk_space_pct, mount_storage_info.used_space, mount_storage_info.available_space, ad_user_account_control, creation_timestamp    All filter fields and operations supports negation (!). | [required] |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an `after` token. On subsequent requests, provide the `after` token from the previous response to continue from that place in the results. |  |
**limit** | Option<**i32**> | The number of asset IDs to return in this response (min: 1, max: 1000, default: 100). Use with the `after` parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort assets by their properties. A single sort field is allowed. Common sort options include:  <ul><li>hostname|asc</li><li>product_type_desc|desc</li></ul> |  |
**facet** | Option<[**Vec<String>**](String.md)> | Select various details blocks to be returned for each host entity. Supported values:  <ul><li>system_insights</li><li>third_party</li><li>risk_factors</li></ul> |  |

### Return type

[**models::DomainPeriodDiscoverApiCombinedHostsResponse**](domain.DiscoverAPICombinedHostsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_accounts

> models::DomainPeriodDiscoverApiAccountEntitiesResponse get_accounts(ids)
Get details on accounts by providing one or more IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more account IDs (max: 100). Find account IDs with GET `/discover/queries/accounts/v1` | [required] |

### Return type

[**models::DomainPeriodDiscoverApiAccountEntitiesResponse**](domain.DiscoverAPIAccountEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_applications

> models::DomainPeriodDiscoverApiApplicationEntitiesResponse get_applications(ids)
Get details on applications by providing one or more IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of applications to retrieve. (Min: 1, Max: 100) | [required] |

### Return type

[**models::DomainPeriodDiscoverApiApplicationEntitiesResponse**](domain.DiscoverAPIApplicationEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_hosts

> models::DomainPeriodDiscoverApiHostEntitiesResponse get_hosts(ids)
Get details on assets by providing one or more IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more asset IDs (max: 100). Find asset IDs with GET `/discover/queries/hosts/v1` | [required] |

### Return type

[**models::DomainPeriodDiscoverApiHostEntitiesResponse**](domain.DiscoverAPIHostEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_logins

> models::DomainPeriodDiscoverApiLoginEntitiesResponse get_logins(ids)
Get details on logins by providing one or more IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more login IDs (max: 100). Find login IDs with GET `/discover/queries/logins/v1` | [required] |

### Return type

[**models::DomainPeriodDiscoverApiLoginEntitiesResponse**](domain.DiscoverAPILoginEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_accounts

> models::MsaPeriodQueryResponse query_accounts(offset, limit, sort, filter)
Search for accounts in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of account IDs which match the filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | An offset used with the `limit` parameter to manage pagination of results. On your first request, don’t provide an `offset`. On subsequent requests, add previous `offset` with the previous `limit` to continue from that place in the results. |  |
**limit** | Option<**i32**> | The number of account IDs to return in this response (min: 1, max: 100, default: 100). Use with the `offset` parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort accounts by their properties. A single sort field is allowed. Common sort options include:  <ul><li>username|asc</li><li>last_failed_login_timestamp|desc</li></ul> |  |
**filter** | Option<**String**> | Filter accounts using an FQL query. Common filter options include:<ul><li>account_type:'Local'</li><li>admin_privileges:'Yes'</li><li>first_seen_timestamp:<'now-7d'</li><li>last_successful_login_type:'Terminal server'</li></ul>    Available filter fields that support exact match: id, cid, user_sid, account_name, username, account_type, admin_privileges, first_seen_timestamp, last_successful_login_type, last_successful_login_timestamp, last_successful_login_hostname, last_successful_login_remote_ip, last_successful_login_host_country, last_successful_login_host_city, login_domain, last_failed_login_type, last_failed_login_timestamp, last_failed_login_hostname, password_last_set_timestamp, local_admin_privileges    Available filter fields that supports wildcard (*): id, cid, user_sid, account_name, username, account_type, admin_privileges, last_successful_login_type, last_successful_login_hostname, last_successful_login_remote_ip, last_successful_login_host_country, last_successful_login_host_city, login_domain, last_failed_login_type, last_failed_login_hostname, local_admin_privileges    Available filter fields that supports range comparisons (>, <, >=, <=): first_seen_timestamp, last_successful_login_timestamp,last_failed_login_timestamp, password_last_set_timestamp    All filter fields and operations supports negation (!). |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_applications

> models::MsaspecPeriodQueryResponse query_applications(offset, limit, sort, filter)
Search for applications in your environment by providing an FQL filter and paging details. returns a set of application IDs which match the filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | An offset used with the `limit` parameter to manage pagination of results. On your first request, don’t provide an `offset`. On subsequent requests, add previous `offset` with the previous `limit` to continue from that place in the results. |  |
**limit** | Option<**i32**> | The number of application ids to return in this response (Min: 1, Max: 100, Default: 100). |  |
**sort** | Option<**String**> | Sort applications by their properties. A single sort field is allowed. |  |
**filter** | Option<**String**> | Search for applications in your environment by providing an FQL filter.     Available filter fields that support exact match: name, version, vendor, name_vendor, name_vendor_version, first_seen_timestamp, installation_timestamp, architectures, installation_paths, versioning_scheme, groups, is_normalized, last_used_user_sid, last_used_user_name, last_used_file_name, last_used_file_hash, last_used_timestamp, last_updated_timestamp, is_suspicious, host.id, host.platform_name, host.hostname, cid, host.os_version, host.machine_domain, host.ou, host.site_name, host.country, host.current_mac_address, host.current_network_prefix, host.tags, host.groups, host.product_type_desc, host.kernel_version, host.system_manufacturer, host.internet_exposure, host.agent_version, host.external_ip, host.aid     Available filter fields that supports wildcard (*): name, version, vendor, name_vendor, name_vendor_version, architectures, installation_paths, groups, last_used_user_sid, last_used_user_name, last_used_file_name, last_used_file_hash, host.platform_name, host.hostname, cid, host.os_version, host.machine_domain, host.ou, host.site_name, host.country, host.current_mac_address, host.current_network_prefix, host.tags, host.groups, host.product_type_desc, host.kernel_version, host.system_manufacturer, host.internet_exposure, host.agent_version, host.external_ip, host.aid     Available filter fields that supports range comparisons (>, <, >=, <=): first_seen_timestamp, installation_timestamp, last_used_timestamp, last_updated_timestamp     All filter fields and operations supports negation (!). |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_hosts

> models::MsaspecPeriodQueryResponse query_hosts(offset, limit, sort, filter)
Search for assets in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of asset IDs which match the filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | An offset used with the `limit` parameter to manage pagination of results. On your first request, don’t provide an `offset`. On subsequent requests, add previous `offset` with the previous `limit` to continue from that place in the results. |  |
**limit** | Option<**i32**> | The number of asset IDs to return in this response (min: 1, max: 100, default: 100). Use with the `offset` parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort assets by their properties. A single sort field is allowed. Common sort options include:  <ul><li>hostname|asc</li><li>product_type_desc|desc</li></ul> |  |
**filter** | Option<**String**> | Filter assets using an FQL query. Common filter options include:<ul><li>entity_type:'managed'</li><li>product_type_desc:'Workstation'</li><li>platform_name:'Windows'</li><li>last_seen_timestamp:>'now-7d'</li></ul>    Available filter fields that support exact match: id, aid, entity_type, country, city, platform_name, os_version, kernel_version, product_type_desc, tags, groups, agent_version, system_product_name, system_manufacturer, system_serial_number, bios_manufacturer, bios_version, ou, machine_domain, site_name, external_ip, hostname, local_ips_count, network_interfaces.local_ip, network_interfaces.mac_address, network_interfaces.interface_alias, network_interfaces.interface_description, network_interfaces.network_prefix, last_discoverer_aid, discoverer_count, discoverer_aids, discoverer_tags, discoverer_platform_names, discoverer_product_type_descs, confidence, internet_exposure,  os_is_eol, data_providers, data_providers_count, mac_addresses, local_ip_addresses, reduced_functionality_mode, number_of_disk_drives, processor_package_count, physical_core_count, logical_core_count, total_disk_space, disk_sizes.disk_name, disk_sizes.disk_space, cpu_processor_name, total_memory, encryption_status, encrypted_drives, encrypted_drives_count, unencrypted_drives, unencrypted_drives_count, os_security.secure_boot_requested_status, os_security.device_guard_status, os_security.device_guard_status, os_security.device_guard_status, os_security.system_guard_status, os_security.credential_guard_status, os_security.iommu_protection_status, os_security.secure_boot_enabled_status, os_security.uefi_memory_protection_status, os_security.virtualization_based_security_status, os_security.kernel_dma_protection_status, total_bios_files, bios_hashes_data.sha256_hash, bios_hashes_data.measurement_type, bios_id, average_processor_usage, average_memory_usage, average_memory_usage_pct, max_processor_usage, max_memory_usage, max_memory_usage_pct, used_disk_space, used_disk_space_pct, available_disk_space, available_disk_space_pct, mount_storage_info.mount_path, mount_storage_info.used_space, mount_storage_info.available_space, form_factor, servicenow_id, owned_by, managed_by, assigned_to, department, fqdn, used_for, object_guid, object_sid, ad_user_account_control, account_enabled, creation_timestamp, email, os_service_pack, location, state, cpu_manufacturer, discovering_by    Available filter fields that supports wildcard (*): id, aid, entity_type, country, city, platform_name, os_version, kernel_version, product_type_desc, tags, groups, agent_version, system_product_name, system_manufacturer, system_serial_number, bios_manufacturer, bios_version, ou, machine_domain, site_name, external_ip, hostname, network_interfaces.local_ip, network_interfaces.mac_address, network_interfaces.interface_alias, network_interfaces.interface_description, network_interfaces.network_prefix, last_discoverer_aid, discoverer_aids, discoverer_tags, discoverer_platform_names, discoverer_product_type_descs, confidence, internet_exposure,  os_is_eol, data_providers, mac_addresses, local_ip_addresses, reduced_functionality_mode, disk_sizes.disk_name, cpu_processor_name, encryption_status, encrypted_drives, unencrypted_drives, os_security.secure_boot_requested_status, os_security.device_guard_status, os_security.device_guard_status, os_security.device_guard_status, os_security.system_guard_status, os_security.credential_guard_status, os_security.iommu_protection_status, os_security.secure_boot_enabled_status, os_security.uefi_memory_protection_status, os_security.virtualization_based_security_status, os_security.kernel_dma_protection_status, bios_hashes_data.sha256_hash, bios_hashes_data.measurement_type, bios_id, mount_storage_info.mount_path, form_factor, servicenow_id, owned_by, managed_by, assigned_to, department, fqdn, used_for, object_guid, object_sid, account_enabled, email, os_service_pack, location, state, cpu_manufacturer, discovering_by    Available filter fields that supports range comparisons (>, <, >=, <=): first_seen_timestamp, last_seen_timestamp, local_ips_count, discoverer_count, confidence, number_of_disk_drives, processor_package_count, physical_core_count, data_providers_count, logical_core_count, total_disk_space, disk_sizes.disk_space, total_memory, encrypted_drives_count, unencrypted_drives_count, total_bios_files, average_processor_usage, average_memory_usage, average_memory_usage_pct, max_processor_usage, max_memory_usage, max_memory_usage_pct, used_disk_space, used_disk_space_pct, available_disk_space, available_disk_space_pct, mount_storage_info.used_space, mount_storage_info.available_space, ad_user_account_control, creation_timestamp    All filter fields and operations supports negation (!). |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_logins

> models::MsaPeriodQueryResponse query_logins(offset, limit, sort, filter)
Search for logins in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of login IDs which match the filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | An offset used with the `limit` parameter to manage pagination of results. On your first request, don’t provide an `offset`. On subsequent requests, add previous `offset` with the previous `limit` to continue from that place in the results. |  |
**limit** | Option<**i32**> | The number of login IDs to return in this response (min: 1, max: 100, default: 100). Use with the `offset` parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort logins by their properties. A single sort field is allowed. Common sort options include:  <ul><li>account_name|asc</li><li>login_timestamp|desc</li></ul> |  |
**filter** | Option<**String**> | Filter logins using an FQL query. Common filter options include:<ul><li>account_type:'Local'</li><li>login_type:'Interactive'</li><li>first_seen_timestamp:<'now-7d'</li><li>admin_privileges:'No'</li></ul>    Available filter fields that support exact match: id, cid, login_status, account_id, host_id, user_sid, aid, account_name, username, hostname, account_type, login_type, login_timestamp, login_domain, admin_privileges, local_admin_privileges, local_ip, remote_ip, host_country, host_city, is_suspicious, failure_description, login_event_count, aggregation_time_interval    Available filter fields that supports wildcard (*): id, cid, login_status, account_id, host_id, user_sid, aid, account_name, username, hostname, account_type, login_type, login_domain, admin_privileges, local_admin_privileges, local_ip, remote_ip, host_country, host_city, failure_description, aggregation_time_interval    Available filter fields that supports range comparisons (>, <, >=, <=): login_timestamp, login_event_count    All filter fields and operations supports negation (!). |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
