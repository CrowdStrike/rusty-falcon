# \AspmApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_executor_node**](AspmApi.md#create_executor_node) | **POST** /aspm-api-gateway/api/v1/executor_nodes | Create a new relay node
[**create_integration**](AspmApi.md#create_integration) | **POST** /aspm-api-gateway/api/v1/integrations | Create a new integration
[**create_integration_task**](AspmApi.md#create_integration_task) | **POST** /aspm-api-gateway/api/v1/integration_tasks | Create new integration task.
[**delete_executor_node**](AspmApi.md#delete_executor_node) | **DELETE** /aspm-api-gateway/api/v1/executor_nodes/{ID} | Delete a relay node
[**delete_integration**](AspmApi.md#delete_integration) | **DELETE** /aspm-api-gateway/api/v1/integrations/{ID} | Delete an existing integration by its ID
[**delete_integration_task**](AspmApi.md#delete_integration_task) | **DELETE** /aspm-api-gateway/api/v1/integration_tasks/{ID} | Delete an existing integration task by its ID
[**delete_tags**](AspmApi.md#delete_tags) | **POST** /aspm-api-gateway/api/v1/tags | Remove existing tags
[**execute_query**](AspmApi.md#execute_query) | **POST** /aspm-api-gateway/api/v1/query | Execute a query. The syntax used is identical to that of the query page.
[**get_executor_nodes**](AspmApi.md#get_executor_nodes) | **GET** /aspm-api-gateway/api/v1/executor_nodes | Get all the relay nodes
[**get_integration_tasks**](AspmApi.md#get_integration_tasks) | **GET** /aspm-api-gateway/api/v1/integration_tasks | Get all the integration tasks
[**get_integration_types**](AspmApi.md#get_integration_types) | **GET** /aspm-api-gateway/api/v1/integration_types | Get all the integration types
[**get_integrations**](AspmApi.md#get_integrations) | **GET** /aspm-api-gateway/api/v1/integrations | Get a list of all the integrations
[**get_service_violation_types**](AspmApi.md#get_service_violation_types) | **GET** /aspm-api-gateway/api/v1/services/violations/types | Get the different types of violation
[**get_services_count**](AspmApi.md#get_services_count) | **POST** /aspm-api-gateway/api/v1/services/count | Get the total amount of existing services
[**get_tags**](AspmApi.md#get_tags) | **GET** /aspm-api-gateway/api/v1/tags | Get all the tags
[**run_integration_task**](AspmApi.md#run_integration_task) | **POST** /aspm-api-gateway/api/v1/integration_tasks/{ID}/run | Run an integration task by its ID
[**service_now_get_deployments**](AspmApi.md#service_now_get_deployments) | **GET** /aspm-api-gateway/api/v1/servicenow/deployments |
[**service_now_get_services**](AspmApi.md#service_now_get_services) | **GET** /aspm-api-gateway/api/v1/servicenow/services |
[**update_executor_node**](AspmApi.md#update_executor_node) | **PUT** /aspm-api-gateway/api/v1/executor_nodes | Update an existing relay node
[**update_integration**](AspmApi.md#update_integration) | **PUT** /aspm-api-gateway/api/v1/integrations/{ID} | Update an existing integration by its ID
[**update_integration_task**](AspmApi.md#update_integration_task) | **PUT** /aspm-api-gateway/api/v1/integration_tasks/{ID} | Update an existing integration task by its ID
[**upsert_business_applications**](AspmApi.md#upsert_business_applications) | **PUT** /aspm-api-gateway/api/v1/business_applications | Create or Update Business Applications
[**upsert_tags**](AspmApi.md#upsert_tags) | **PUT** /aspm-api-gateway/api/v1/tags | Create new or update existing tag. You can update unique tags table or regular tags table

## create_executor_node

> models::TypesPeriodExecutorNode create_executor_node(body)
Create a new relay node

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TypesPeriodExecutorNode**](TypesPeriodExecutorNode.md) |  | [required] |

### Return type

[**models::TypesPeriodExecutorNode**](types.ExecutorNode.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_integration

> models::TypesPeriodIntegrationResponse create_integration(body)
Create a new integration

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TypesPeriodCreateIntegrationRequest**](TypesPeriodCreateIntegrationRequest.md) |  | [required] |

### Return type

[**models::TypesPeriodIntegrationResponse**](types.IntegrationResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_integration_task

> models::TypesPeriodIntegrationTaskResponse create_integration_task(body)
Create new integration task.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TypesPeriodCreateIntegrationTaskRequest**](TypesPeriodCreateIntegrationTaskRequest.md) |  | [required] |

### Return type

[**models::TypesPeriodIntegrationTaskResponse**](types.IntegrationTaskResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_executor_node

> delete_executor_node(id)
Delete a relay node

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_integration

> delete_integration(id)
Delete an existing integration by its ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_integration_task

> delete_integration_task(id)
Delete an existing integration task by its ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_tags

> delete_tags(body)
Remove existing tags

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TypesPeriodEditTagRequest**](TypesPeriodEditTagRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## execute_query

> models::TypesPeriodQueryResult execute_query(body)
Execute a query. The syntax used is identical to that of the query page.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TypesPeriodQueryRequest**](TypesPeriodQueryRequest.md) |  **params details:** - selectFields: - **fields** - For filtering relevant fields only. - **withoutServices** - Default is set to **true**, you will not receive information about the services. If you want to get the relevant service, set to **false**. - **serviceFields**-  For filtering relevant fields of the service (if you chose to get it) | [required] |

### Return type

[**models::TypesPeriodQueryResult**](types.QueryResult.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_executor_nodes

> models::TypesPeriodListExecutorNodesResponse get_executor_nodes(node_type, integration_type)
Get all the relay nodes

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_type** | **String** |  | [required] |
**integration_type** | Option<**i32**> |  |  |

### Return type

[**models::TypesPeriodListExecutorNodesResponse**](types.ListExecutorNodesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_integration_tasks

> models::TypesPeriodListIntegrationTasksResponse get_integration_tasks(integration_task_type, category)
Get all the integration tasks

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_task_type** | Option<**i32**> |  |  |
**category** | Option<**String**> |  |  |

### Return type

[**models::TypesPeriodListIntegrationTasksResponse**](types.ListIntegrationTasksResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_integration_types

> models::TypesPeriodListIntegrationTypesResponse get_integration_types()
Get all the integration types

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TypesPeriodListIntegrationTypesResponse**](types.ListIntegrationTypesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_integrations

> models::TypesPeriodListIntegrationsResponse get_integrations(integration_type, category)
Get a list of all the integrations

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_type** | Option<**i32**> |  |  |
**category** | Option<**String**> |  |  |

### Return type

[**models::TypesPeriodListIntegrationsResponse**](types.ListIntegrationsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_service_violation_types

> models::TypesPeriodGetViolationTypesResponse get_service_violation_types(body)
Get the different types of violation

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TypesPeriodGenericUserFacingRequest**](TypesPeriodGenericUserFacingRequest.md) |  | [required] |

### Return type

[**models::TypesPeriodGetViolationTypesResponse**](types.GetViolationTypesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_services_count

> get_services_count(body)
Get the total amount of existing services

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TypesPeriodGetServicesRequest**](TypesPeriodGetServicesRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_tags

> models::TypesPeriodTagsDataResponse get_tags(is_unique, tag_name, limit, offset, name)
Get all the tags

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**is_unique** | Option<**bool**> |  |  |
**tag_name** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**name** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::TypesPeriodTagsDataResponse**](types.TagsDataResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## run_integration_task

> models::TypesPeriodIntegrationTaskTestConnectionResponse run_integration_task(id, body, category)
Run an integration task by its ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**body** | [**TypesPeriodRunIntegrationTaskRequest**](TypesPeriodRunIntegrationTaskRequest.md) |  | [required] |
**category** | Option<**String**> |  |  |

### Return type

[**models::TypesPeriodIntegrationTaskTestConnectionResponse**](types.IntegrationTaskTestConnectionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## service_now_get_deployments

> models::TypesPeriodServiceNowDeploymentsResponse service_now_get_deployments(ql_filters, limit, offset, order_by, direction)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ql_filters** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**order_by** | Option<**String**> |  |  |
**direction** | Option<**String**> |  |  |

### Return type

[**models::TypesPeriodServiceNowDeploymentsResponse**](types.ServiceNowDeploymentsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## service_now_get_services

> models::TypesPeriodServiceNowServicesResponse service_now_get_services(ql_filters, limit, offset, order_by, direction)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ql_filters** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**order_by** | Option<**String**> |  |  |
**direction** | Option<**String**> |  |  |

### Return type

[**models::TypesPeriodServiceNowServicesResponse**](types.ServiceNowServicesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_executor_node

> models::TypesPeriodExecutorNode update_executor_node(body)
Update an existing relay node

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TypesPeriodExecutorNode**](TypesPeriodExecutorNode.md) |  | [required] |

### Return type

[**models::TypesPeriodExecutorNode**](types.ExecutorNode.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_integration

> models::TypesPeriodIntegrationResponse update_integration(id, body)
Update an existing integration by its ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**body** | [**TypesPeriodUpdateIntegrationRequest**](TypesPeriodUpdateIntegrationRequest.md) |  | [required] |

### Return type

[**models::TypesPeriodIntegrationResponse**](types.IntegrationResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_integration_task

> models::TypesPeriodIntegrationTaskResponse update_integration_task(id, body)
Update an existing integration task by its ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**body** | [**TypesPeriodUpdateIntegrationTaskRequest**](TypesPeriodUpdateIntegrationTaskRequest.md) |  | [required] |

### Return type

[**models::TypesPeriodIntegrationTaskResponse**](types.IntegrationTaskResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## upsert_business_applications

> upsert_business_applications(body)
Create or Update Business Applications

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TypesPeriodUpsertBusinessApplicationRequest**](TypesPeriodUpsertBusinessApplicationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## upsert_tags

> upsert_tags(body)
Create new or update existing tag. You can update unique tags table or regular tags table

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TypesPeriodEditUniqueTagRequest**](TypesPeriodEditUniqueTagRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
