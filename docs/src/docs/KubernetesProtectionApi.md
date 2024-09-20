# \KubernetesProtectionApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_aws_account**](KubernetesProtectionApi.md#create_aws_account) | **POST** /kubernetes-protection/entities/accounts/aws/v1 | Creates a new AWS account in our system for a customer and generates the installation script
[**create_azure_subscription**](KubernetesProtectionApi.md#create_azure_subscription) | **POST** /kubernetes-protection/entities/accounts/azure/v1 | Creates a new Azure Subscription in our system
[**delete_aws_accounts_mixin0**](KubernetesProtectionApi.md#delete_aws_accounts_mixin0) | **DELETE** /kubernetes-protection/entities/accounts/aws/v1 | Delete AWS accounts.
[**delete_azure_subscription**](KubernetesProtectionApi.md#delete_azure_subscription) | **DELETE** /kubernetes-protection/entities/accounts/azure/v1 | Deletes a new Azure Subscription in our system
[**find_containers_by_container_run_time_version**](KubernetesProtectionApi.md#find_containers_by_container_run_time_version) | **GET** /container-security/aggregates/containers/find-by-runtimeversion/v1 | Retrieve containers by container_runtime_version
[**find_containers_count_affected_by_zero_day_vulnerabilities**](KubernetesProtectionApi.md#find_containers_count_affected_by_zero_day_vulnerabilities) | **GET** /container-security/aggregates/containers/count-by-zero-day/v1 | Retrieve containers count affected by zero day vulnerabilities
[**get_aws_accounts_mixin0**](KubernetesProtectionApi.md#get_aws_accounts_mixin0) | **GET** /kubernetes-protection/entities/accounts/aws/v1 | Provides a list of AWS accounts.
[**get_azure_install_script**](KubernetesProtectionApi.md#get_azure_install_script) | **GET** /kubernetes-protection/entities/user-script/azure/v1 | Provides the script to run for a given tenant id and subscription IDs
[**get_azure_tenant_config**](KubernetesProtectionApi.md#get_azure_tenant_config) | **GET** /kubernetes-protection/entities/config/azure/v1 | Gets the Azure tenant Config
[**get_azure_tenant_ids**](KubernetesProtectionApi.md#get_azure_tenant_ids) | **GET** /kubernetes-protection/entities/tenants/azure/v1 | Provides all the azure subscriptions and tenants
[**get_clusters**](KubernetesProtectionApi.md#get_clusters) | **GET** /kubernetes-protection/entities/kubernetes/clusters/v1 | Provides the clusters acknowledged by the Kubernetes Protection service
[**get_combined_cloud_clusters**](KubernetesProtectionApi.md#get_combined_cloud_clusters) | **GET** /kubernetes-protection/entities/cloud_cluster/v1 | Returns a combined list of provisioned cloud accounts and known kubernetes clusters
[**get_helm_values_yaml**](KubernetesProtectionApi.md#get_helm_values_yaml) | **GET** /kubernetes-protection/entities/integration/agent/v1 | Provides a sample Helm values.yaml file for a customer to install alongside the agent Helm chart
[**get_locations**](KubernetesProtectionApi.md#get_locations) | **GET** /kubernetes-protection/entities/cloud-locations/v1 | Provides the cloud locations acknowledged by the Kubernetes Protection service
[**get_static_scripts**](KubernetesProtectionApi.md#get_static_scripts) | **GET** /kubernetes-protection/entities/gen/scripts/v1 | Gets static bash scripts that are used during registration
[**group_containers_by_managed**](KubernetesProtectionApi.md#group_containers_by_managed) | **GET** /container-security/aggregates/containers/group-by-managed/v1 | Group the containers by Managed
[**list_azure_accounts**](KubernetesProtectionApi.md#list_azure_accounts) | **GET** /kubernetes-protection/entities/accounts/azure/v1 | Provides the azure subscriptions registered to Kubernetes Protection
[**patch_azure_service_principal**](KubernetesProtectionApi.md#patch_azure_service_principal) | **PATCH** /kubernetes-protection/entities/service-principal/azure/v1 | Adds the client ID for the given tenant ID to our system
[**read_cluster_combined**](KubernetesProtectionApi.md#read_cluster_combined) | **GET** /container-security/combined/clusters/v1 | Retrieve kubernetes clusters identified by the provided filter criteria
[**read_cluster_count**](KubernetesProtectionApi.md#read_cluster_count) | **GET** /container-security/aggregates/clusters/count/v1 | Retrieve cluster counts
[**read_cluster_enrichment**](KubernetesProtectionApi.md#read_cluster_enrichment) | **GET** /container-security/aggregates/enrichment/clusters/entities/v1 | Retrieve cluster enrichment data
[**read_clusters_by_date_range_count**](KubernetesProtectionApi.md#read_clusters_by_date_range_count) | **GET** /container-security/aggregates/clusters/count-by-date/v1 | Retrieve clusters by date range counts
[**read_clusters_by_kubernetes_version_count**](KubernetesProtectionApi.md#read_clusters_by_kubernetes_version_count) | **GET** /container-security/aggregates/clusters/count-by-kubernetes-version/v1 | Bucket clusters by kubernetes version
[**read_clusters_by_status_count**](KubernetesProtectionApi.md#read_clusters_by_status_count) | **GET** /container-security/aggregates/clusters/count-by-status/v1 | Bucket clusters by status
[**read_container_combined**](KubernetesProtectionApi.md#read_container_combined) | **GET** /container-security/combined/containers/v1 | Retrieve containers identified by the provided filter criteria
[**read_container_count**](KubernetesProtectionApi.md#read_container_count) | **GET** /container-security/aggregates/containers/count/v1 | Retrieve container counts
[**read_container_count_by_registry**](KubernetesProtectionApi.md#read_container_count_by_registry) | **GET** /container-security/aggregates/containers/count-by-registry/v1 | Retrieve top container image registries
[**read_container_enrichment**](KubernetesProtectionApi.md#read_container_enrichment) | **GET** /container-security/aggregates/enrichment/containers/entities/v1 | Retrieve container enrichment data
[**read_container_image_detections_count_by_date**](KubernetesProtectionApi.md#read_container_image_detections_count_by_date) | **GET** /container-security/aggregates/containers/image-detections-count-by-date/v1 | Retrieve count of image assessment detections on running containers over a period of time
[**read_container_images_by_most_used**](KubernetesProtectionApi.md#read_container_images_by_most_used) | **GET** /container-security/aggregates/images/most-used/v1 | Bucket container by image-digest
[**read_container_images_by_state**](KubernetesProtectionApi.md#read_container_images_by_state) | **GET** /container-security/aggregates/containers/images-by-state/v1 | Retrieve count of image states running on containers
[**read_container_vulnerabilities_by_severity_count**](KubernetesProtectionApi.md#read_container_vulnerabilities_by_severity_count) | **GET** /container-security/aggregates/containers/vulnerability-count-by-severity/v1 | Retrieve container vulnerabilities by severity counts
[**read_containers_by_date_range_count**](KubernetesProtectionApi.md#read_containers_by_date_range_count) | **GET** /container-security/aggregates/containers/count-by-date/v1 | Retrieve containers by date range counts
[**read_containers_sensor_coverage**](KubernetesProtectionApi.md#read_containers_sensor_coverage) | **GET** /container-security/aggregates/containers/sensor-coverage/v1 | Bucket containers by agent type and calculate sensor coverage
[**read_deployment_combined**](KubernetesProtectionApi.md#read_deployment_combined) | **GET** /container-security/combined/deployments/v1 | Retrieve kubernetes deployments identified by the provided filter criteria
[**read_deployment_count**](KubernetesProtectionApi.md#read_deployment_count) | **GET** /container-security/aggregates/deployments/count/v1 | Retrieve deployment counts
[**read_deployment_enrichment**](KubernetesProtectionApi.md#read_deployment_enrichment) | **GET** /container-security/aggregates/enrichment/deployments/entities/v1 | Retrieve deployment enrichment data
[**read_deployments_by_date_range_count**](KubernetesProtectionApi.md#read_deployments_by_date_range_count) | **GET** /container-security/aggregates/deployments/count-by-date/v1 | Retrieve deployments by date range counts
[**read_distinct_container_image_count**](KubernetesProtectionApi.md#read_distinct_container_image_count) | **GET** /container-security/aggregates/images/count-by-distinct/v1 | Retrieve count of distinct images running on containers
[**read_kubernetes_iom_by_date_range**](KubernetesProtectionApi.md#read_kubernetes_iom_by_date_range) | **GET** /container-security/aggregates/kubernetes-ioms/count-by-date/v1 | Returns the count of Kubernetes IOMs by the date. by default it's for 7 days.
[**read_kubernetes_iom_count**](KubernetesProtectionApi.md#read_kubernetes_iom_count) | **GET** /container-security/aggregates/kubernetes-ioms/count/v1 | Returns the total count of Kubernetes IOMs over the past seven days
[**read_kubernetes_iom_entities**](KubernetesProtectionApi.md#read_kubernetes_iom_entities) | **GET** /container-security/entities/kubernetes-ioms/v1 | Retrieve Kubernetes IOM entities identified by the provided IDs
[**read_namespace_count**](KubernetesProtectionApi.md#read_namespace_count) | **GET** /container-security/aggregates/namespaces/count/v1 | Retrieve namespace counts
[**read_namespaces_by_date_range_count**](KubernetesProtectionApi.md#read_namespaces_by_date_range_count) | **GET** /container-security/aggregates/namespaces/count-by-date/v1 | Retrieve namespaces by date range counts
[**read_node_combined**](KubernetesProtectionApi.md#read_node_combined) | **GET** /container-security/combined/nodes/v1 | Retrieve kubernetes nodes identified by the provided filter criteria
[**read_node_count**](KubernetesProtectionApi.md#read_node_count) | **GET** /container-security/aggregates/nodes/count/v1 | Retrieve node counts
[**read_node_enrichment**](KubernetesProtectionApi.md#read_node_enrichment) | **GET** /container-security/aggregates/enrichment/nodes/entities/v1 | Retrieve node enrichment data
[**read_nodes_by_cloud_count**](KubernetesProtectionApi.md#read_nodes_by_cloud_count) | **GET** /container-security/aggregates/nodes/count-by-cloud/v1 | Bucket nodes by cloud providers
[**read_nodes_by_container_engine_version_count**](KubernetesProtectionApi.md#read_nodes_by_container_engine_version_count) | **GET** /container-security/aggregates/nodes/count-by-container-engine-version/v1 | Bucket nodes by their container engine version
[**read_nodes_by_date_range_count**](KubernetesProtectionApi.md#read_nodes_by_date_range_count) | **GET** /container-security/aggregates/nodes/count-by-date/v1 | Retrieve nodes by date range counts
[**read_pod_combined**](KubernetesProtectionApi.md#read_pod_combined) | **GET** /container-security/combined/pods/v1 | Retrieve kubernetes pods identified by the provided filter criteria
[**read_pod_count**](KubernetesProtectionApi.md#read_pod_count) | **GET** /container-security/aggregates/pods/count/v1 | Retrieve pod counts
[**read_pod_enrichment**](KubernetesProtectionApi.md#read_pod_enrichment) | **GET** /container-security/aggregates/enrichment/pods/entities/v1 | Retrieve pod enrichment data
[**read_pods_by_date_range_count**](KubernetesProtectionApi.md#read_pods_by_date_range_count) | **GET** /container-security/aggregates/pods/count-by-date/v1 | Retrieve pods by date range counts
[**read_running_container_images**](KubernetesProtectionApi.md#read_running_container_images) | **GET** /container-security/combined/container-images/v1 | Retrieve images on running containers
[**read_vulnerable_container_image_count**](KubernetesProtectionApi.md#read_vulnerable_container_image_count) | **GET** /container-security/aggregates/containers/count-vulnerable-images/v1 | Retrieve count of vulnerable images running on containers
[**regenerate_api_key**](KubernetesProtectionApi.md#regenerate_api_key) | **POST** /kubernetes-protection/entities/integration/api-key/v1 | Regenerate API key for docker registry integrations
[**search_and_read_kubernetes_iom_entities**](KubernetesProtectionApi.md#search_and_read_kubernetes_iom_entities) | **GET** /container-security/combined/kubernetes-ioms/v1 | Search Kubernetes IOM by the provided search criteria
[**search_kubernetes_ioms**](KubernetesProtectionApi.md#search_kubernetes_ioms) | **GET** /container-security/queries/kubernetes-ioms/v1 | Search Kubernetes IOMs by the provided search criteria. this endpoint returns a list of Kubernetes IOM UUIDs matching the query
[**trigger_scan**](KubernetesProtectionApi.md#trigger_scan) | **POST** /kubernetes-protection/entities/scan/trigger/v1 | Triggers a dry run or a full scan of a customer's kubernetes footprint
[**update_aws_account**](KubernetesProtectionApi.md#update_aws_account) | **PATCH** /kubernetes-protection/entities/accounts/aws/v1 | Updates the AWS account per the query parameters provided

## create_aws_account

> models::K8sregPeriodCreateAwsAccResp create_aws_account(body)
Creates a new AWS account in our system for a customer and generates the installation script

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**K8sregPeriodCreateAwsAccReq**](K8sregPeriodCreateAwsAccReq.md) |  | [required] |

### Return type

[**models::K8sregPeriodCreateAwsAccResp**](k8sreg.CreateAWSAccResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_azure_subscription

> models::MsaPeriodBaseEntitiesResponse create_azure_subscription(body)
Creates a new Azure Subscription in our system

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**K8sregPeriodCreateAzureSubReq**](K8sregPeriodCreateAzureSubReq.md) |  | [required] |

### Return type

[**models::MsaPeriodBaseEntitiesResponse**](msa.BaseEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_aws_accounts_mixin0

> models::MsaspecPeriodMetaInfo delete_aws_accounts_mixin0(ids)
Delete AWS accounts.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | AWS Account IDs | [required] |

### Return type

[**models::MsaspecPeriodMetaInfo**](msaspec.MetaInfo.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_azure_subscription

> models::MsaPeriodBaseEntitiesResponse delete_azure_subscription(ids)
Deletes a new Azure Subscription in our system

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Azure Subscription IDs |  |

### Return type

[**models::MsaPeriodBaseEntitiesResponse**](msa.BaseEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## find_containers_by_container_run_time_version

> models::ModelsPeriodContainerRuntimePivotResponse find_containers_by_container_run_time_version(limit, offset, sort, filter)
Retrieve containers by container_runtime_version

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The upper-bound on the number of container records to retrieve. |  |
**offset** | Option<**i32**> | It is used to get the offset |  |
**sort** | Option<**String**> | Field to sort results by |  |
**filter** | Option<**String**> | Retrieve count of Kubernetes containers that match a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,allow_privilege_escalation,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_id,container_image_id,container_name,cve_id,detection_name,first_seen,image_detection_count,image_digest,image_has_been_assessed,image_id,image_registry,image_repository,image_tag,image_vulnerability_count,insecure_mount_source,insecure_mount_type,insecure_propagation_mode,interactive_mode,ipv4,ipv6,kac_agent_id,labels,last_seen,namespace,node_name,node_uid,package_name_version,pod_id,pod_name,port,privileged,root_write_access,run_as_root_group,run_as_root_user,running_status |  |

### Return type

[**models::ModelsPeriodContainerRuntimePivotResponse**](models.ContainerRuntimePivotResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## find_containers_count_affected_by_zero_day_vulnerabilities

> models::CommonPeriodCountResponse find_containers_count_affected_by_zero_day_vulnerabilities()
Retrieve containers count affected by zero day vulnerabilities

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CommonPeriodCountResponse**](common.CountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_aws_accounts_mixin0

> models::K8sregPeriodGetAwsAccountsResp get_aws_accounts_mixin0(ids, is_horizon_acct, status, limit, offset)
Provides a list of AWS accounts.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | AWS Account IDs |  |
**is_horizon_acct** | Option<**String**> | Filter by whether an account originates from Horizon or not |  |
**status** | Option<**String**> | Filter by account status |  |
**limit** | Option<**i32**> | Limit returned accounts |  |
**offset** | Option<**i32**> | Offset returned accounts |  |

### Return type

[**models::K8sregPeriodGetAwsAccountsResp**](k8sreg.GetAWSAccountsResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_azure_install_script

> models::K8sregPeriodGetAzureBashScriptResp get_azure_install_script(id, subscription_id)
Provides the script to run for a given tenant id and subscription IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | Azure Tenant ID |  |
**subscription_id** | Option<[**Vec<String>**](String.md)> | Azure Subscription IDs |  |

### Return type

[**models::K8sregPeriodGetAzureBashScriptResp**](k8sreg.GetAzureBashScriptResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_azure_tenant_config

> models::K8sregPeriodGetAzureTenantConfigResp get_azure_tenant_config(ids, limit, offset)
Gets the Azure tenant Config

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Azure Tenant IDs |  |
**limit** | Option<**i32**> | Limit returned accounts |  |
**offset** | Option<**i32**> | Offset returned accounts |  |

### Return type

[**models::K8sregPeriodGetAzureTenantConfigResp**](k8sreg.GetAzureTenantConfigResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_azure_tenant_ids

> models::K8sregPeriodGetAzureTenantInfoResp get_azure_tenant_ids(ids, status, limit, offset)
Provides all the azure subscriptions and tenants

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Azure Tenant IDs |  |
**status** | Option<**String**> | Cluster Status |  |
**limit** | Option<**i32**> | Limit returned accounts |  |
**offset** | Option<**i32**> | Offset returned accounts |  |

### Return type

[**models::K8sregPeriodGetAzureTenantInfoResp**](k8sreg.GetAzureTenantInfoResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_clusters

> models::K8sregPeriodGetClustersResp get_clusters(cluster_names, status, account_ids, locations, cluster_service, limit, offset)
Provides the clusters acknowledged by the Kubernetes Protection service

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_names** | Option<[**Vec<String>**](String.md)> | Cluster name. For EKS it will be cluster ARN. |  |
**status** | Option<[**Vec<String>**](String.md)> | Cluster Status |  |
**account_ids** | Option<[**Vec<String>**](String.md)> | Cluster Account id. For EKS it will be AWS account ID. |  |
**locations** | Option<[**Vec<String>**](String.md)> | Cloud location |  |
**cluster_service** | Option<**String**> | Cluster Service |  |
**limit** | Option<**i32**> | Limit returned accounts |  |
**offset** | Option<**i32**> | Offset returned accounts |  |

### Return type

[**models::K8sregPeriodGetClustersResp**](k8sreg.GetClustersResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_combined_cloud_clusters

> models::K8sregPeriodListClusterCloudResp get_combined_cloud_clusters(locations, ids, cluster_service, cluster_status, limit, offset)
Returns a combined list of provisioned cloud accounts and known kubernetes clusters

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locations** | Option<[**Vec<String>**](String.md)> | Cloud location |  |
**ids** | Option<[**Vec<String>**](String.md)> | Cloud Account IDs |  |
**cluster_service** | Option<[**Vec<String>**](String.md)> | Cluster Service |  |
**cluster_status** | Option<[**Vec<String>**](String.md)> | Cluster Status |  |
**limit** | Option<**i32**> | Limit returned accounts |  |
**offset** | Option<**i32**> | Offset returned accounts |  |

### Return type

[**models::K8sregPeriodListClusterCloudResp**](k8sreg.ListClusterCloudResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_helm_values_yaml

> serde_json::Value get_helm_values_yaml(cluster_name, is_self_managed_cluster)
Provides a sample Helm values.yaml file for a customer to install alongside the agent Helm chart

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** | Cluster name. For EKS it will be cluster ARN. | [required] |
**is_self_managed_cluster** | Option<**bool**> | Set to true if the cluster is not managed by a cloud provider, false if it is. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_locations

> models::K8sregPeriodGetLocationsResp get_locations(clouds)
Provides the cloud locations acknowledged by the Kubernetes Protection service

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**clouds** | Option<[**Vec<String>**](String.md)> | Cloud Provider |  |

### Return type

[**models::K8sregPeriodGetLocationsResp**](k8sreg.GetLocationsResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_static_scripts

> models::K8sregPeriodGetScriptsResp get_static_scripts()
Gets static bash scripts that are used during registration

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::K8sregPeriodGetScriptsResp**](k8sreg.GetScriptsResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## group_containers_by_managed

> models::ModelsPeriodContainerCoverageResponseEntity group_containers_by_managed(filter)
Group the containers by Managed

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve count of Kubernetes containers that match a query in Falcon Query Language (FQL). Supported filters:  agent_id,allow_privilege_escalation,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_id,container_image_id,container_name,cve_id,detection_name,first_seen,image_detection_count,image_digest,image_has_been_assessed,image_id,image_registry,image_repository,image_tag,image_vulnerability_count,insecure_mount_source,insecure_mount_type,insecure_propagation_mode,interactive_mode,ipv4,ipv6,kac_agent_id,labels,last_seen,namespace,node_name,node_uid,pod_id,pod_name,port,privileged,root_write_access,run_as_root_group,run_as_root_user,running_status |  |

### Return type

[**models::ModelsPeriodContainerCoverageResponseEntity**](models.ContainerCoverageResponseEntity.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## list_azure_accounts

> models::K8sregPeriodGetAzureSubscriptionsResp list_azure_accounts(ids, subscription_id, status, is_horizon_acct, limit, offset)
Provides the azure subscriptions registered to Kubernetes Protection

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Azure Tenant IDs |  |
**subscription_id** | Option<[**Vec<String>**](String.md)> | Azure Subscription IDs |  |
**status** | Option<**String**> | Filter by account status |  |
**is_horizon_acct** | Option<**String**> | Filter by whether an account originates from Horizon or not |  |
**limit** | Option<**i32**> | Limit returned accounts |  |
**offset** | Option<**i32**> | Offset returned accounts |  |

### Return type

[**models::K8sregPeriodGetAzureSubscriptionsResp**](k8sreg.GetAzureSubscriptionsResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## patch_azure_service_principal

> models::K8sregPeriodGetAzureTenantConfigResp patch_azure_service_principal(id, client_id)
Adds the client ID for the given tenant ID to our system

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Azure Tenant ID | [required] |
**client_id** | **String** | Azure Client ID | [required] |

### Return type

[**models::K8sregPeriodGetAzureTenantConfigResp**](k8sreg.GetAzureTenantConfigResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_cluster_combined

> models::ModelsPeriodClusterEntityResponse read_cluster_combined(filter, limit, offset, sort)
Retrieve kubernetes clusters identified by the provided filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Kubernetes clusters using a query in Falcon Query Language (FQL). Supported filters:  access,agent_id,agent_status,agent_type,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,cluster_status,container_count,iar_coverage,kac_agent_id,kubernetes_version,last_seen,management_status,node_count,pod_count,tags |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | Field to sort results by |  |

### Return type

[**models::ModelsPeriodClusterEntityResponse**](models.ClusterEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_cluster_count

> models::CommonPeriodCountResponse read_cluster_count(filter)
Retrieve cluster counts

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve count of Kubernetes clusters that match a query in Falcon Query Language (FQL). Supported filters:  access,agent_id,agent_status,agent_type,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,cluster_status,container_count,iar_coverage,kac_agent_id,kubernetes_version,last_seen,management_status,node_count,pod_count,tags |  |

### Return type

[**models::CommonPeriodCountResponse**](common.CountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_cluster_enrichment

> models::K8sassetsPeriodClusterEnrichmentResponse read_cluster_enrichment(cluster_id, filter)
Retrieve cluster enrichment data

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | [**Vec<String>**](String.md) | One or more cluster ids for which to retrieve enrichment info | [required] |
**filter** | Option<**String**> | Supported filters:  last_seen |  |

### Return type

[**models::K8sassetsPeriodClusterEnrichmentResponse**](k8sassets.ClusterEnrichmentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_clusters_by_date_range_count

> models::ModelsPeriodAggregateValuesByFieldResponse read_clusters_by_date_range_count()
Retrieve clusters by date range counts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_clusters_by_kubernetes_version_count

> models::ModelsPeriodAggregateValuesByFieldResponse read_clusters_by_kubernetes_version_count(filter)
Bucket clusters by kubernetes version

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve count of Kubernetes clusters that match a query in Falcon Query Language (FQL). Supported filters:  access,agent_id,agent_status,agent_type,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,cluster_status,container_count,iar_coverage,kac_agent_id,kubernetes_version,last_seen,management_status,node_count,pod_count,tags |  |

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_clusters_by_status_count

> models::ModelsPeriodAggregateValuesByFieldResponse read_clusters_by_status_count(filter)
Bucket clusters by status

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve count of Kubernetes clusters that match a query in Falcon Query Language (FQL). Supported filters:  access,agent_id,agent_status,agent_type,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,cluster_status,container_count,iar_coverage,kac_agent_id,kubernetes_version,last_seen,management_status,node_count,pod_count,tags |  |

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_container_combined

> models::ModelsPeriodContainerEntityResponse read_container_combined(filter, limit, offset, sort)
Retrieve containers identified by the provided filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Kubernetes containers using a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,allow_privilege_escalation,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_id,container_image_id,container_name,cve_id,detection_name,first_seen,image_detection_count,image_digest,image_has_been_assessed,image_id,image_registry,image_repository,image_tag,image_vulnerability_count,insecure_mount_source,insecure_mount_type,insecure_propagation_mode,interactive_mode,ipv4,ipv6,kac_agent_id,labels,last_seen,namespace,node_name,node_uid,package_name_version,pod_id,pod_name,port,privileged,root_write_access,run_as_root_group,run_as_root_user,running_status |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | Field to sort results by |  |

### Return type

[**models::ModelsPeriodContainerEntityResponse**](models.ContainerEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_container_count

> models::CommonPeriodCountResponse read_container_count(filter)
Retrieve container counts

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve count of Kubernetes containers that match a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,allow_privilege_escalation,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_id,container_image_id,container_name,cve_id,detection_name,first_seen,image_detection_count,image_digest,image_has_been_assessed,image_id,image_registry,image_repository,image_tag,image_vulnerability_count,insecure_mount_source,insecure_mount_type,insecure_propagation_mode,interactive_mode,ipv4,ipv6,kac_agent_id,labels,last_seen,namespace,node_name,node_uid,package_name_version,pod_id,pod_name,port,privileged,root_write_access,run_as_root_group,run_as_root_user,running_status |  |

### Return type

[**models::CommonPeriodCountResponse**](common.CountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_container_count_by_registry

> models::ModelsPeriodApiFilterResponse read_container_count_by_registry(under_assessment, limit)
Retrieve top container image registries

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**under_assessment** | Option<**bool**> | (true/false) whether to return registries under assessment or not under assessment. If not  provided all registries are considered |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |

### Return type

[**models::ModelsPeriodApiFilterResponse**](models.APIFilterResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_container_enrichment

> models::K8sassetsPeriodContainerEnrichmentResponse read_container_enrichment(container_id, filter)
Retrieve container enrichment data

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | [**Vec<String>**](String.md) | One or more container ids for which to retrieve enrichment info | [required] |
**filter** | Option<**String**> | Supported filters:  last_seen |  |

### Return type

[**models::K8sassetsPeriodContainerEnrichmentResponse**](k8sassets.ContainerEnrichmentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_container_image_detections_count_by_date

> models::ModelsPeriodApiFilterResponse read_container_image_detections_count_by_date(filter)
Retrieve count of image assessment detections on running containers over a period of time

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve count of Kubernetes containers that match a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,allow_privilege_escalation,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_id,container_image_id,container_name,cve_id,detection_name,first_seen,image_detection_count,image_digest,image_has_been_assessed,image_id,image_registry,image_repository,image_tag,image_vulnerability_count,insecure_mount_source,insecure_mount_type,insecure_propagation_mode,interactive_mode,ipv4,ipv6,kac_agent_id,labels,last_seen,namespace,node_name,node_uid,package_name_version,pod_id,pod_name,port,privileged,root_write_access,run_as_root_group,run_as_root_user,running_status |  |

### Return type

[**models::ModelsPeriodApiFilterResponse**](models.APIFilterResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_container_images_by_most_used

> models::ModelsPeriodAggregateValuesByFieldResponse read_container_images_by_most_used(filter)
Bucket container by image-digest

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve count of Kubernetes containers that match a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,allow_privilege_escalation,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_id,container_image_id,container_name,cve_id,detection_name,first_seen,image_detection_count,image_digest,image_has_been_assessed,image_id,image_registry,image_repository,image_tag,image_vulnerability_count,insecure_mount_source,insecure_mount_type,insecure_propagation_mode,interactive_mode,ipv4,ipv6,kac_agent_id,labels,last_seen,namespace,node_name,node_uid,package_name_version,pod_id,pod_name,port,privileged,root_write_access,run_as_root_group,run_as_root_user,running_status |  |

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_container_images_by_state

> models::ModelsPeriodApiFilterResponse read_container_images_by_state(filter)
Retrieve count of image states running on containers

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter using a query in Falcon Query Language (FQL). Supported filters: cid |  |

### Return type

[**models::ModelsPeriodApiFilterResponse**](models.APIFilterResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_container_vulnerabilities_by_severity_count

> models::ModelsPeriodAggregateValuesByFieldResponse read_container_vulnerabilities_by_severity_count(filter)
Retrieve container vulnerabilities by severity counts

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Get vulnerabilities count by severity for container using a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,allow_privilege_escalation,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_id,container_image_id,container_name,cve_id,detection_name,first_seen,image_detection_count,image_digest,image_has_been_assessed,image_id,image_registry,image_repository,image_tag,image_vulnerability_count,insecure_mount_source,insecure_mount_type,insecure_propagation_mode,interactive_mode,ipv4,ipv6,kac_agent_id,labels,last_seen,namespace,node_name,node_uid,package_name_version,pod_id,pod_name,port,privileged,root_write_access,run_as_root_group,run_as_root_user,running_status |  |

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_containers_by_date_range_count

> models::ModelsPeriodAggregateValuesByFieldResponse read_containers_by_date_range_count(filter)
Retrieve containers by date range counts

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Get container counts using a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,allow_privilege_escalation,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_id,container_image_id,container_name,cve_id,detection_name,first_seen,image_detection_count,image_digest,image_has_been_assessed,image_id,image_registry,image_repository,image_tag,image_vulnerability_count,insecure_mount_source,insecure_mount_type,insecure_propagation_mode,interactive_mode,ipv4,ipv6,kac_agent_id,labels,last_seen,namespace,node_name,node_uid,package_name_version,pod_id,pod_name,port,privileged,root_write_access,run_as_root_group,run_as_root_user,running_status |  |

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_containers_sensor_coverage

> models::ModelsPeriodAggregateValuesByFieldResponse read_containers_sensor_coverage(filter)
Bucket containers by agent type and calculate sensor coverage

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve count of Kubernetes containers that match a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,allow_privilege_escalation,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_id,container_image_id,container_name,cve_id,detection_name,first_seen,image_detection_count,image_digest,image_has_been_assessed,image_id,image_registry,image_repository,image_tag,image_vulnerability_count,insecure_mount_source,insecure_mount_type,insecure_propagation_mode,interactive_mode,ipv4,ipv6,kac_agent_id,labels,last_seen,namespace,node_name,node_uid,package_name_version,pod_id,pod_name,port,privileged,root_write_access,run_as_root_group,run_as_root_user,running_status |  |

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_deployment_combined

> models::ModelsPeriodDeploymentEntityResponse read_deployment_combined(filter, limit, offset, sort)
Retrieve kubernetes deployments identified by the provided filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Kubernetes deployments using a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,annotations_list,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,deployment_id,deployment_name,deployment_status,first_seen,kac_agent_id,last_seen,namespace,pod_count,resource_status |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | Field to sort results by |  |

### Return type

[**models::ModelsPeriodDeploymentEntityResponse**](models.DeploymentEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_deployment_count

> models::CommonPeriodCountResponse read_deployment_count(filter)
Retrieve deployment counts

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve count of Kubernetes deployments that match a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,annotations_list,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,deployment_id,deployment_name,deployment_status,first_seen,kac_agent_id,last_seen,namespace,pod_count,resource_status |  |

### Return type

[**models::CommonPeriodCountResponse**](common.CountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_deployment_enrichment

> models::K8sassetsPeriodDeploymentEnrichmentResponse read_deployment_enrichment(deployment_id, filter)
Retrieve deployment enrichment data

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | [**Vec<String>**](String.md) | One or more deployment ids for which to retrieve enrichment info | [required] |
**filter** | Option<**String**> | Supported filters:  last_seen |  |

### Return type

[**models::K8sassetsPeriodDeploymentEnrichmentResponse**](k8sassets.DeploymentEnrichmentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_deployments_by_date_range_count

> models::ModelsPeriodAggregateValuesByFieldResponse read_deployments_by_date_range_count()
Retrieve deployments by date range counts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_distinct_container_image_count

> models::ModelsPeriodApiFilterResponse read_distinct_container_image_count(filter)
Retrieve count of distinct images running on containers

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Kubernetes containers using a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,allow_privilege_escalation,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_id,container_image_id,container_name,cve_id,detection_name,first_seen,image_detection_count,image_digest,image_has_been_assessed,image_id,image_registry,image_repository,image_tag,image_vulnerability_count,insecure_mount_source,insecure_mount_type,insecure_propagation_mode,interactive_mode,ipv4,ipv6,kac_agent_id,labels,last_seen,namespace,node_name,node_uid,package_name_version,pod_id,pod_name,port,privileged,root_write_access,run_as_root_group,run_as_root_user,running_status |  |

### Return type

[**models::ModelsPeriodApiFilterResponse**](models.APIFilterResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_kubernetes_iom_by_date_range

> models::K8siomsPeriodKubernetesIomFieldValue read_kubernetes_iom_by_date_range(filter)
Returns the count of Kubernetes IOMs by the date. by default it's for 7 days.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters: cid,created_timestamp,detect_timestamp,prevented,severity |  |

### Return type

[**models::K8siomsPeriodKubernetesIomFieldValue**](k8sioms.kubernetesIOMFieldValue.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_kubernetes_iom_count

> models::K8siomsPeriodKubernetesIomCountValue read_kubernetes_iom_count(filter)
Returns the total count of Kubernetes IOMs over the past seven days

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters: cid,created_timestamp,detect_timestamp,prevented,severity |  |

### Return type

[**models::K8siomsPeriodKubernetesIomCountValue**](k8sioms.kubernetesIOMCountValue.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_kubernetes_iom_entities

> models::K8siomsPeriodKubernetesIomEntityResponse read_kubernetes_iom_entities(ids)
Retrieve Kubernetes IOM entities identified by the provided IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Search Kubernetes IOMs by ids - The maximum amount is 100 IDs |  |

### Return type

[**models::K8siomsPeriodKubernetesIomEntityResponse**](k8sioms.KubernetesIOMEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_namespace_count

> models::CommonPeriodCountResponse read_namespace_count(filter)
Retrieve namespace counts

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve count of Kubernetes namespaces that match a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,annotations_list,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,first_seen,kac_agent_id,last_seen,namespace_id,namespace_name,resource_status |  |

### Return type

[**models::CommonPeriodCountResponse**](common.CountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_namespaces_by_date_range_count

> models::ModelsPeriodAggregateValuesByFieldResponse read_namespaces_by_date_range_count()
Retrieve namespaces by date range counts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_node_combined

> models::ModelsPeriodNodeEntityResponse read_node_combined(filter, limit, offset, sort)
Retrieve kubernetes nodes identified by the provided filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Kubernetes nodes using a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,annotations_list,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_count,container_runtime_version,first_seen,image_digest,ipv4,kac_agent_id,last_seen,linux_sensor_coverage,node_name,pod_count,resource_status |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | Field to sort results by |  |

### Return type

[**models::ModelsPeriodNodeEntityResponse**](models.NodeEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_node_count

> models::CommonPeriodCountResponse read_node_count(filter)
Retrieve node counts

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve count of Kubernetes nodes that match a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,annotations_list,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_count,container_runtime_version,first_seen,image_digest,ipv4,kac_agent_id,last_seen,linux_sensor_coverage,node_name,pod_count,resource_status |  |

### Return type

[**models::CommonPeriodCountResponse**](common.CountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_node_enrichment

> models::K8sassetsPeriodNodeEnrichmentResponse read_node_enrichment(node_name, filter)
Retrieve node enrichment data

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_name** | [**Vec<String>**](String.md) | One or more node names for which to retrieve enrichment info | [required] |
**filter** | Option<**String**> | Supported filters:  last_seen |  |

### Return type

[**models::K8sassetsPeriodNodeEnrichmentResponse**](k8sassets.NodeEnrichmentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_nodes_by_cloud_count

> models::ModelsPeriodAggregateValuesByFieldResponse read_nodes_by_cloud_count(filter)
Bucket nodes by cloud providers

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Kubernetes nodes using a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,annotations_list,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_count,container_runtime_version,first_seen,image_digest,ipv4,kac_agent_id,last_seen,linux_sensor_coverage,node_name,pod_count,resource_status |  |

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_nodes_by_container_engine_version_count

> models::ModelsPeriodAggregateValuesByFieldResponse read_nodes_by_container_engine_version_count(filter)
Bucket nodes by their container engine version

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Kubernetes nodes using a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,annotations_list,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_count,container_runtime_version,first_seen,image_digest,ipv4,kac_agent_id,last_seen,linux_sensor_coverage,node_name,pod_count,resource_status |  |

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_nodes_by_date_range_count

> models::ModelsPeriodAggregateValuesByFieldResponse read_nodes_by_date_range_count(filter)
Retrieve nodes by date range counts

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Kubernetes nodes using a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,annotations_list,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_count,container_runtime_version,first_seen,image_digest,ipv4,kac_agent_id,last_seen,linux_sensor_coverage,node_name,pod_count,resource_status |  |

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_pod_combined

> models::ModelsPeriodPodEntityResponse read_pod_combined(filter, limit, offset, sort)
Retrieve kubernetes pods identified by the provided filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Kubernetes pods using a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,allow_privilege_escalation,annotations_list,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_count,first_seen,ipv4,ipv6,kac_agent_id,labels,last_seen,namespace,node_name,node_uid,owner_id,owner_type,pod_external_id,pod_id,pod_name,port,privileged,resource_status,root_write_access,run_as_root_group,run_as_root_user |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | Field to sort results by |  |

### Return type

[**models::ModelsPeriodPodEntityResponse**](models.PodEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_pod_count

> models::CommonPeriodCountResponse read_pod_count(filter)
Retrieve pod counts

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve count of Kubernetes pods that match a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,allow_privilege_escalation,annotations_list,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_count,first_seen,ipv4,ipv6,kac_agent_id,labels,last_seen,namespace,node_name,node_uid,owner_id,owner_type,pod_external_id,pod_id,pod_name,port,privileged,resource_status,root_write_access,run_as_root_group,run_as_root_user |  |

### Return type

[**models::CommonPeriodCountResponse**](common.CountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_pod_enrichment

> models::K8sassetsPeriodPodEnrichmentResponse read_pod_enrichment(pod_id, filter)
Retrieve pod enrichment data

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | [**Vec<String>**](String.md) | One or more pod ids for which to retrieve enrichment info | [required] |
**filter** | Option<**String**> | Supported filters:  last_seen |  |

### Return type

[**models::K8sassetsPeriodPodEnrichmentResponse**](k8sassets.PodEnrichmentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_pods_by_date_range_count

> models::ModelsPeriodAggregateValuesByFieldResponse read_pods_by_date_range_count()
Retrieve pods by date range counts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_running_container_images

> models::ModelsPeriodContainerImage read_running_container_images(filter, limit, offset, sort)
Retrieve images on running containers

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve list of images on running containers using a query in Falcon Query Language (FQL). Supported filters:  cid,cluster_id,cluster_name,hosts,image_digest,image_has_been_assessed,image_id,image_name,image_registry,image_repository,image_tag,last_seen,running_status |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | Field to sort results by |  |

### Return type

[**models::ModelsPeriodContainerImage**](models.ContainerImage.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_vulnerable_container_image_count

> models::ModelsPeriodApiFilterResponse read_vulnerable_container_image_count(filter)
Retrieve count of vulnerable images running on containers

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Retrieve count of Kubernetes containers that match a query in Falcon Query Language (FQL). Supported filters:  agent_id,agent_type,allow_privilege_escalation,cid,cloud_account_id,cloud_name,cloud_region,cloud_service,cluster_id,cluster_name,container_id,container_image_id,container_name,cve_id,detection_name,first_seen,image_detection_count,image_digest,image_has_been_assessed,image_id,image_registry,image_repository,image_tag,image_vulnerability_count,insecure_mount_source,insecure_mount_type,insecure_propagation_mode,interactive_mode,ipv4,ipv6,kac_agent_id,labels,last_seen,namespace,node_name,node_uid,package_name_version,pod_id,pod_name,port,privileged,root_write_access,run_as_root_group,run_as_root_user,running_status |  |

### Return type

[**models::ModelsPeriodApiFilterResponse**](models.APIFilterResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## regenerate_api_key

> models::K8sregPeriodRegenApiKeyResp regenerate_api_key()
Regenerate API key for docker registry integrations

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::K8sregPeriodRegenApiKeyResp**](k8sreg.RegenAPIKeyResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## search_and_read_kubernetes_iom_entities

> models::K8siomsPeriodKubernetesIomEntityResponse search_and_read_kubernetes_iom_entities(filter, limit, offset, sort)
Search Kubernetes IOM by the provided search criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Kubernetes IOMs using a query in Falcon Query Language (FQL). Supported filters:  cid,cis_id,cluster_id,cluster_name,containers_impacted_count,containers_impacted_ids,detection_type,name,namespace,prevented,resource_id,resource_name,resource_type,severity |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The fields to sort the records on. |  |

### Return type

[**models::K8siomsPeriodKubernetesIomEntityResponse**](k8sioms.KubernetesIOMEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## search_kubernetes_ioms

> models::CommonPeriodGenericEntityResponseLeftSquareBracketStringRightSquareBracket search_kubernetes_ioms(filter, limit, offset, sort)
Search Kubernetes IOMs by the provided search criteria. this endpoint returns a list of Kubernetes IOM UUIDs matching the query

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Kubernetes IOMs using a query in Falcon Query Language (FQL). Supported filters:  cid,cis_id,cluster_id,cluster_name,containers_impacted_count,containers_impacted_ids,detection_type,name,namespace,prevented,resource_id,resource_name,resource_type,severity |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The fields to sort the records on. |  |

### Return type

[**models::CommonPeriodGenericEntityResponseLeftSquareBracketStringRightSquareBracket**](common.GenericEntityResponse[string].md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## trigger_scan

> models::MsaPeriodBaseEntitiesResponse trigger_scan(scan_type)
Triggers a dry run or a full scan of a customer's kubernetes footprint

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_type** | **String** | Scan Type to do | [required] |[default to dry-run]

### Return type

[**models::MsaPeriodBaseEntitiesResponse**](msa.BaseEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_aws_account

> models::MsaPeriodBaseEntitiesResponse update_aws_account(ids, region)
Updates the AWS account per the query parameters provided

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | AWS Account ID | [required] |
**region** | Option<**String**> | Default Region for Account Automation |  |

### Return type

[**models::MsaPeriodBaseEntitiesResponse**](msa.BaseEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
