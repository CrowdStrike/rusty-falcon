# \WorkflowsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**workflow_activities_combined**](WorkflowsApi.md#workflow_activities_combined) | **GET** /workflows/combined/activities/v1 | Search for activities by name. Returns all supported activities if no filter specified
[**workflow_definitions_combined**](WorkflowsApi.md#workflow_definitions_combined) | **GET** /workflows/combined/definitions/v1 | Search workflow definitions based on the provided filter
[**workflow_definitions_export**](WorkflowsApi.md#workflow_definitions_export) | **GET** /workflows/entities/definitions/export/v1 | Exports a workflow definition for the given definition ID
[**workflow_definitions_import**](WorkflowsApi.md#workflow_definitions_import) | **POST** /workflows/entities/definitions/import/v1 | Imports a workflow definition based on the provided model
[**workflow_definitions_update**](WorkflowsApi.md#workflow_definitions_update) | **PUT** /workflows/entities/definitions/v1 | Updates a workflow definition based on the provided model
[**workflow_execute**](WorkflowsApi.md#workflow_execute) | **POST** /workflows/entities/execute/v1 | Executes an on-demand Workflow, the body is JSON used to trigger the execution, the response the execution ID(s)
[**workflow_execute_internal**](WorkflowsApi.md#workflow_execute_internal) | **POST** /workflows/entities/execute/internal/v1 | Executes an on-demand Workflow - internal workflows permitted, the body is JSON used to trigger the execution, the response the execution ID(s)
[**workflow_execution_results**](WorkflowsApi.md#workflow_execution_results) | **GET** /workflows/entities/execution-results/v1 | Get execution result of a given execution
[**workflow_executions_action**](WorkflowsApi.md#workflow_executions_action) | **POST** /workflows/entities/execution-actions/v1 | Allows a user to resume/retry a failed workflow execution.
[**workflow_executions_combined**](WorkflowsApi.md#workflow_executions_combined) | **GET** /workflows/combined/executions/v1 | Search workflow executions based on the provided filter
[**workflow_get_human_input_v1**](WorkflowsApi.md#workflow_get_human_input_v1) | **GET** /workflows/entities/human-inputs/v1 | Gets one or more specific human inputs by their IDs.
[**workflow_mock_execute**](WorkflowsApi.md#workflow_mock_execute) | **POST** /workflows/entities/mock-executions/v1 | Executes a workflow definition with mocks
[**workflow_system_definitions_de_provision**](WorkflowsApi.md#workflow_system_definitions_de_provision) | **POST** /workflows/system-definitions/deprovision/v1 | Deprovisions a system definition that was previously provisioned on the target CID
[**workflow_system_definitions_promote**](WorkflowsApi.md#workflow_system_definitions_promote) | **POST** /workflows/system-definitions/promote/v1 | Promotes a version of a system definition for a customer. The customer must already have been provisioned. This allows the caller to apply an updated template version to a specific cid and expects all parameters to be supplied. If the template supports multi-instance the customer scope definition ID must be supplied to determine which customer workflow should be updated.
[**workflow_system_definitions_provision**](WorkflowsApi.md#workflow_system_definitions_provision) | **POST** /workflows/system-definitions/provision/v1 | Provisions a system definition onto the target CID by using the template and provided parameters
[**workflow_triggers_combined**](WorkflowsApi.md#workflow_triggers_combined) | **GET** /workflows/combined/triggers/v1 | Search for triggers by namespaced identifier, i.e. FalconAudit, Detection, or FalconAudit/Detection/Status. Returns all triggers if no filter specified
[**workflow_update_human_input_v1**](WorkflowsApi.md#workflow_update_human_input_v1) | **PATCH** /workflows/entities/human-inputs/v1 | Provides an input in response to a human input action. Depending on action configuration, one or more of Approve, Decline, and/or Escalate are permitted.

## workflow_activities_combined

> models::ActivitiesPeriodActivityExternalResponse workflow_activities_combined(filter, offset, limit, sort)
Search for activities by name. Returns all supported activities if no filter specified

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | FQL query specifying filter parameters. |  |
**offset** | Option<**String**> | Starting pagination offset of records to return. |  |
**limit** | Option<**i32**> | Maximum number of records to return. |  |
**sort** | Option<**String**> | Sort items by providing a comma separated list of property and direction (eg name.desc,time.asc). If direction is omitted, defaults to descending. |  |

### Return type

[**models::ActivitiesPeriodActivityExternalResponse**](activities.ActivityExternalResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_definitions_combined

> models::DefinitionsPeriodDefinitionExternalResponse workflow_definitions_combined(filter, offset, limit, sort)
Search workflow definitions based on the provided filter

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | FQL query specifying filter parameters. |  |
**offset** | Option<**String**> | Starting pagination offset of records to return. |  |
**limit** | Option<**i32**> | Maximum number of records to return. |  |
**sort** | Option<**String**> | Sort items by providing a comma separated list of property and direction (eg name.desc,time.asc). If direction is omitted, defaults to descending. |  |

### Return type

[**models::DefinitionsPeriodDefinitionExternalResponse**](definitions.DefinitionExternalResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_definitions_export

> Vec<i32> workflow_definitions_export(id, sanitize)
Exports a workflow definition for the given definition ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of workflow definitions to return details for | [required] |
**sanitize** | Option<**bool**> | whether or not to sanitize PII from workflow before it's exported |  |[default to true]

### Return type

**Vec<i32>**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_definitions_import

> models::DefinitionsPeriodDefinitionImportResponse workflow_definitions_import(data_file, name, validate_only)
Imports a workflow definition based on the provided model

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_file** | **std::path::PathBuf** | A workflow definition in YAML format to import | [required] |
**name** | Option<**String**> | Workflow name to override |  |
**validate_only** | Option<**bool**> | When enabled, prevents saving workflow after validating |  |[default to false]

### Return type

[**models::DefinitionsPeriodDefinitionImportResponse**](definitions.DefinitionImportResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_definitions_update

> models::ApiPeriodResourceIdsResponse workflow_definitions_update(body, validate_only)
Updates a workflow definition based on the provided model

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodDefinitionUpdateRequestV2**](ModelsPeriodDefinitionUpdateRequestV2.md) |  | [required] |
**validate_only** | Option<**bool**> | When enabled, prevents saving workflow after validating |  |[default to false]

### Return type

[**models::ApiPeriodResourceIdsResponse**](api.ResourceIDsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/yaml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_execute

> models::ApiPeriodResourceIdsResponse workflow_execute(body, execution_cid, definition_id, name, key, depth, source_event_url)
Executes an on-demand Workflow, the body is JSON used to trigger the execution, the response the execution ID(s)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**execution_cid** | Option<[**Vec<String>**](String.md)> | CID(s) to execute on. This can be a child if this is a flight control enabled definition. If unset the definition CID is used. |  |
**definition_id** | Option<[**Vec<String>**](String.md)> | Definition ID to execute, either a name or an ID can be specified. |  |
**name** | Option<**String**> | Workflow name to execute, either a name or an ID can be specified. |  |
**key** | Option<**String**> | Key used to help deduplicate executions, if unset a new UUID is used |  |
**depth** | Option<**i32**> | Used to record the execution depth to help limit execution loops when a workflow triggers another. The maximum depth is 4. |  |
**source_event_url** | Option<**String**> | Used to record a URL to the source that led to triggering this workflow |  |

### Return type

[**models::ApiPeriodResourceIdsResponse**](api.ResourceIDsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_execute_internal

> models::ApiPeriodResourceIdsResponse workflow_execute_internal(body, execution_cid, definition_id, name, key, depth, batch_size, source_event_url)
Executes an on-demand Workflow - internal workflows permitted, the body is JSON used to trigger the execution, the response the execution ID(s)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**execution_cid** | Option<[**Vec<String>**](String.md)> | CID(s) to execute on. This can be a child if this is a flight control enabled definition. If unset the definition CID is used. |  |
**definition_id** | Option<[**Vec<String>**](String.md)> | Definition ID to execute, either a name or an ID can be specified. |  |
**name** | Option<**String**> | Workflow name to execute, either a name or an ID can be specified. |  |
**key** | Option<**String**> | Key used to help deduplicate executions, if unset a new UUID is used |  |
**depth** | Option<**i32**> | Used to record the execution depth to help limit execution loops when a workflow triggers another. The maximum depth is 4. |  |
**batch_size** | Option<**i32**> | Used to set the batchSize, if unset the default batchSize is used |  |
**source_event_url** | Option<**String**> | Used to record a URL to the source that led to triggering this workflow |  |

### Return type

[**models::ApiPeriodResourceIdsResponse**](api.ResourceIDsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_execution_results

> models::ApiPeriodExecutionResultsResponse workflow_execution_results(ids)
Get execution result of a given execution

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | workflow execution id to return results for. | [required] |

### Return type

[**models::ApiPeriodExecutionResultsResponse**](api.ExecutionResultsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_executions_action

> models::DefinitionsPeriodDefinitionEntitiesResponse workflow_executions_action(action_name, body)
Allows a user to resume/retry a failed workflow execution.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_name** | **String** | Specify one of these actions:  - `resume`: resume/retry the workflow execution(s) specified in ids | [required] |
**body** | [**ClientPeriodActionRequest**](ClientPeriodActionRequest.md) |  | [required] |

### Return type

[**models::DefinitionsPeriodDefinitionEntitiesResponse**](definitions.DefinitionEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_executions_combined

> models::ApiPeriodExecutionResultsResponse workflow_executions_combined(filter, offset, limit, sort)
Search workflow executions based on the provided filter

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | FQL query specifying filter parameters. |  |
**offset** | Option<**String**> | Starting pagination offset of records to return. |  |
**limit** | Option<**i32**> | Maximum number of records to return. |  |
**sort** | Option<**String**> | Sort items by providing a comma separated list of property and direction (eg name.desc,time.asc). If direction is omitted, defaults to descending. |  |

### Return type

[**models::ApiPeriodExecutionResultsResponse**](api.ExecutionResultsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_get_human_input_v1

> models::ModelPeriodUserInputReadResponse workflow_get_human_input_v1(ids)
Gets one or more specific human inputs by their IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | IDs of human inputs to read | [required] |

### Return type

[**models::ModelPeriodUserInputReadResponse**](model.UserInputReadResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_mock_execute

> models::ApiPeriodResourceIdsResponse workflow_mock_execute(body, execution_cid, definition_id, name, key, depth, source_event_url, validate_only)
Executes a workflow definition with mocks

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodMockExecutionCreateRequestV1**](ModelsPeriodMockExecutionCreateRequestV1.md) |  | [required] |
**execution_cid** | Option<[**Vec<String>**](String.md)> | CID(s) to execute on. This can be a child if this is a flight control enabled definition. If unset the definition CID is used. |  |
**definition_id** | Option<**String**> | Definition ID to execute, either a name or an ID, or the definition itself in the request body, can be specified. |  |
**name** | Option<**String**> | Workflow name to execute, either a name or an ID, or the definition itself in the request body, can be specified. |  |
**key** | Option<**String**> | Key used to help deduplicate executions, if unset a new UUID is used |  |
**depth** | Option<**i32**> | Used to record the execution depth to help limit execution loops when a workflow triggers another. The maximum depth is 4. |  |
**source_event_url** | Option<**String**> | Used to record a URL to the source that led to triggering this workflow |  |
**validate_only** | Option<**bool**> | When enabled, prevents execution after validating mocks against definition |  |[default to false]

### Return type

[**models::ApiPeriodResourceIdsResponse**](api.ResourceIDsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_system_definitions_de_provision

> models::ClientPeriodSystemDefinitionCreateResponse workflow_system_definitions_de_provision(body)
Deprovisions a system definition that was previously provisioned on the target CID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ClientPeriodSystemDefinitionDeProvisionRequest**](ClientPeriodSystemDefinitionDeProvisionRequest.md) |  | [required] |

### Return type

[**models::ClientPeriodSystemDefinitionCreateResponse**](client.SystemDefinitionCreateResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_system_definitions_promote

> models::ClientPeriodSystemDefinitionCreateResponse workflow_system_definitions_promote(body)
Promotes a version of a system definition for a customer. The customer must already have been provisioned. This allows the caller to apply an updated template version to a specific cid and expects all parameters to be supplied. If the template supports multi-instance the customer scope definition ID must be supplied to determine which customer workflow should be updated.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ClientPeriodSystemDefinitionPromoteRequest**](ClientPeriodSystemDefinitionPromoteRequest.md) |  | [required] |

### Return type

[**models::ClientPeriodSystemDefinitionCreateResponse**](client.SystemDefinitionCreateResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_system_definitions_provision

> models::ClientPeriodSystemDefinitionCreateResponse workflow_system_definitions_provision(body)
Provisions a system definition onto the target CID by using the template and provided parameters

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ClientPeriodSystemDefinitionProvisionRequest**](ClientPeriodSystemDefinitionProvisionRequest.md) |  | [required] |

### Return type

[**models::ClientPeriodSystemDefinitionCreateResponse**](client.SystemDefinitionCreateResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_triggers_combined

> models::TriggersPeriodTriggerExternalResponse workflow_triggers_combined(filter)
Search for triggers by namespaced identifier, i.e. FalconAudit, Detection, or FalconAudit/Detection/Status. Returns all triggers if no filter specified

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | FQL query specifying filter parameters. |  |

### Return type

[**models::TriggersPeriodTriggerExternalResponse**](triggers.TriggerExternalResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## workflow_update_human_input_v1

> models::ApiPeriodResourceIdsResponse workflow_update_human_input_v1(id, body)
Provides an input in response to a human input action. Depending on action configuration, one or more of Approve, Decline, and/or Escalate are permitted.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of human input to provide an input to | [required] |
**body** | [**ModelPeriodUserInputUpdateRequest**](ModelPeriodUserInputUpdateRequest.md) |  | [required] |

### Return type

[**models::ApiPeriodResourceIdsResponse**](api.ResourceIDsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
