# \KubernetesProtectionApi

All URIs are relative to *https://api.crowdstrike.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_aws_account**](KubernetesProtectionApi.md#create_aws_account) | **POST** /kubernetes-protection/entities/accounts/aws/v1 | Creates a new AWS account in our system for a customer and generates the installation script
[**create_azure_subscription**](KubernetesProtectionApi.md#create_azure_subscription) | **POST** /kubernetes-protection/entities/accounts/azure/v1 | Creates a new Azure Subscription in our system
[**delete_aws_accounts_mixin0**](KubernetesProtectionApi.md#delete_aws_accounts_mixin0) | **DELETE** /kubernetes-protection/entities/accounts/aws/v1 | Delete AWS accounts.
[**delete_azure_subscription**](KubernetesProtectionApi.md#delete_azure_subscription) | **DELETE** /kubernetes-protection/entities/accounts/azure/v1 | Deletes a new Azure Subscription in our system
[**get_aws_accounts_mixin0**](KubernetesProtectionApi.md#get_aws_accounts_mixin0) | **GET** /kubernetes-protection/entities/accounts/aws/v1 | Provides a list of AWS accounts.
[**get_azure_install_script**](KubernetesProtectionApi.md#get_azure_install_script) | **GET** /kubernetes-protection/entities/user-script/azure/v1 | Provides the script to run for a given tenant id and subscription IDs
[**get_azure_tenant_config**](KubernetesProtectionApi.md#get_azure_tenant_config) | **GET** /kubernetes-protection/entities/config/azure/v1 | Gets the Azure tenant Config
[**get_azure_tenant_ids**](KubernetesProtectionApi.md#get_azure_tenant_ids) | **GET** /kubernetes-protection/entities/tenants/azure/v1 | Provides all the azure subscriptions and tenants
[**get_clusters**](KubernetesProtectionApi.md#get_clusters) | **GET** /kubernetes-protection/entities/kubernetes/clusters/v1 | Provides the clusters acknowledged by the Kubernetes Protection service
[**get_combined_cloud_clusters**](KubernetesProtectionApi.md#get_combined_cloud_clusters) | **GET** /kubernetes-protection/entities/cloud_cluster/v1 | Returns a combined list of provisioned cloud accounts and known kubernetes clusters
[**get_helm_values_yaml**](KubernetesProtectionApi.md#get_helm_values_yaml) | **GET** /kubernetes-protection/entities/integration/agent/v1 | Provides a sample Helm values.yaml file for a customer to install alongside the agent Helm chart
[**get_locations**](KubernetesProtectionApi.md#get_locations) | **GET** /kubernetes-protection/entities/cloud-locations/v1 | Provides the cloud locations acknowledged by the Kubernetes Protection service
[**get_static_scripts**](KubernetesProtectionApi.md#get_static_scripts) | **GET** /kubernetes-protection/entities/gen/scripts/v1 | Gets static bash scripts that are used during registration
[**list_azure_accounts**](KubernetesProtectionApi.md#list_azure_accounts) | **GET** /kubernetes-protection/entities/accounts/azure/v1 | Provides the azure subscriptions registered to Kubernetes Protection
[**patch_azure_service_principal**](KubernetesProtectionApi.md#patch_azure_service_principal) | **PATCH** /kubernetes-protection/entities/service-principal/azure/v1 | Adds the client ID for the given tenant ID to our system
[**regenerate_api_key**](KubernetesProtectionApi.md#regenerate_api_key) | **POST** /kubernetes-protection/entities/integration/api-key/v1 | Regenerate API key for docker registry integrations
[**trigger_scan**](KubernetesProtectionApi.md#trigger_scan) | **POST** /kubernetes-protection/entities/scan/trigger/v1 | Triggers a dry run or a full scan of a customer's kubernetes footprint
[**update_aws_account**](KubernetesProtectionApi.md#update_aws_account) | **PATCH** /kubernetes-protection/entities/accounts/aws/v1 | Updates the AWS account per the query parameters provided



## create_aws_account

> crate::models::K8sregPeriodCreateAwsAccResp create_aws_account(body)
Creates a new AWS account in our system for a customer and generates the installation script

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**K8sregPeriodCreateAwsAccReq**](K8sregPeriodCreateAwsAccReq.md) |  | [required] |

### Return type

[**crate::models::K8sregPeriodCreateAwsAccResp**](k8sreg.CreateAWSAccResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_azure_subscription

> crate::models::MsaPeriodBaseEntitiesResponse create_azure_subscription(body)
Creates a new Azure Subscription in our system

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**K8sregPeriodCreateAzureSubReq**](K8sregPeriodCreateAzureSubReq.md) |  | [required] |

### Return type

[**crate::models::MsaPeriodBaseEntitiesResponse**](msa.BaseEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_aws_accounts_mixin0

> crate::models::MsaspecPeriodMetaInfo delete_aws_accounts_mixin0(ids)
Delete AWS accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | AWS Account IDs | [required] |

### Return type

[**crate::models::MsaspecPeriodMetaInfo**](msaspec.MetaInfo.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_azure_subscription

> crate::models::MsaPeriodBaseEntitiesResponse delete_azure_subscription(ids)
Deletes a new Azure Subscription in our system

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Azure Subscription IDs |  |

### Return type

[**crate::models::MsaPeriodBaseEntitiesResponse**](msa.BaseEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aws_accounts_mixin0

> crate::models::K8sregPeriodGetAwsAccountsResp get_aws_accounts_mixin0(ids, is_horizon_acct, status, limit, offset)
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

[**crate::models::K8sregPeriodGetAwsAccountsResp**](k8sreg.GetAWSAccountsResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_azure_install_script

> crate::models::K8sregPeriodGetAzureBashScriptResp get_azure_install_script(id, subscription_id)
Provides the script to run for a given tenant id and subscription IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | Azure Tenant ID |  |
**subscription_id** | Option<[**Vec<String>**](String.md)> | Azure Subscription IDs |  |

### Return type

[**crate::models::K8sregPeriodGetAzureBashScriptResp**](k8sreg.GetAzureBashScriptResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_azure_tenant_config

> crate::models::K8sregPeriodGetAzureTenantConfigResp get_azure_tenant_config(ids, limit, offset)
Gets the Azure tenant Config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Azure Tenant IDs |  |
**limit** | Option<**i32**> | Limit returned accounts |  |
**offset** | Option<**i32**> | Offset returned accounts |  |

### Return type

[**crate::models::K8sregPeriodGetAzureTenantConfigResp**](k8sreg.GetAzureTenantConfigResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_azure_tenant_ids

> crate::models::K8sregPeriodGetAzureTenantInfoResp get_azure_tenant_ids(ids, status, limit, offset)
Provides all the azure subscriptions and tenants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Azure Tenant IDs |  |
**status** | Option<**String**> | Cluster Status |  |
**limit** | Option<**i32**> | Limit returned accounts |  |
**offset** | Option<**i32**> | Offset returned accounts |  |

### Return type

[**crate::models::K8sregPeriodGetAzureTenantInfoResp**](k8sreg.GetAzureTenantInfoResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_clusters

> crate::models::K8sregPeriodGetClustersResp get_clusters(cluster_names, status, account_ids, locations, cluster_service, limit, offset)
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

[**crate::models::K8sregPeriodGetClustersResp**](k8sreg.GetClustersResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_combined_cloud_clusters

> crate::models::K8sregPeriodListClusterCloudResp get_combined_cloud_clusters(locations, ids, cluster_service, cluster_status, limit, offset)
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

[**crate::models::K8sregPeriodListClusterCloudResp**](k8sreg.ListClusterCloudResp.md)

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

> crate::models::K8sregPeriodGetLocationsResp get_locations(clouds)
Provides the cloud locations acknowledged by the Kubernetes Protection service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**clouds** | Option<[**Vec<String>**](String.md)> | Cloud Provider |  |

### Return type

[**crate::models::K8sregPeriodGetLocationsResp**](k8sreg.GetLocationsResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_static_scripts

> crate::models::K8sregPeriodGetScriptsResp get_static_scripts()
Gets static bash scripts that are used during registration

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::K8sregPeriodGetScriptsResp**](k8sreg.GetScriptsResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_accounts

> crate::models::K8sregPeriodGetAzureSubscriptionsResp list_azure_accounts(ids, subscription_id, status, is_horizon_acct, limit, offset)
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

[**crate::models::K8sregPeriodGetAzureSubscriptionsResp**](k8sreg.GetAzureSubscriptionsResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_azure_service_principal

> crate::models::K8sregPeriodGetAzureTenantConfigResp patch_azure_service_principal(id, client_id)
Adds the client ID for the given tenant ID to our system

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Azure Tenant ID | [required] |
**client_id** | **String** | Azure Client ID | [required] |

### Return type

[**crate::models::K8sregPeriodGetAzureTenantConfigResp**](k8sreg.GetAzureTenantConfigResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## regenerate_api_key

> crate::models::K8sregPeriodRegenApiKeyResp regenerate_api_key()
Regenerate API key for docker registry integrations

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::K8sregPeriodRegenApiKeyResp**](k8sreg.RegenAPIKeyResp.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_scan

> crate::models::MsaPeriodBaseEntitiesResponse trigger_scan(scan_type)
Triggers a dry run or a full scan of a customer's kubernetes footprint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_type** | **String** | Scan Type to do | [required] |[default to dry-run]

### Return type

[**crate::models::MsaPeriodBaseEntitiesResponse**](msa.BaseEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_aws_account

> crate::models::MsaPeriodBaseEntitiesResponse update_aws_account(ids, region)
Updates the AWS account per the query parameters provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | AWS Account ID | [required] |
**region** | Option<**String**> | Default Region for Account Automation |  |

### Return type

[**crate::models::MsaPeriodBaseEntitiesResponse**](msa.BaseEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

