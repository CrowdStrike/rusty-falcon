# \HostMigrationApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_migration_v1**](HostMigrationApi.md#create_migration_v1) | **POST** /host-migration/entities/migrations/v1 | Create a device migration job.
[**get_host_migration_ids_v1**](HostMigrationApi.md#get_host_migration_ids_v1) | **GET** /host-migration/queries/host-migrations/v1 | Query host migration IDs.
[**get_host_migrations_v1**](HostMigrationApi.md#get_host_migrations_v1) | **POST** /host-migration/entities/host-migrations/GET/v1 | Get host migration details.
[**get_migration_destinations_v1**](HostMigrationApi.md#get_migration_destinations_v1) | **POST** /host-migration/entities/migration-destinations/GET/v1 | Get destinations for a migration.
[**get_migration_ids_v1**](HostMigrationApi.md#get_migration_ids_v1) | **GET** /host-migration/queries/migrations/v1 | Query migration jobs.
[**get_migrations_v1**](HostMigrationApi.md#get_migrations_v1) | **GET** /host-migration/entities/migrations/v1 | Get migration job details.
[**host_migration_aggregates_v1**](HostMigrationApi.md#host_migration_aggregates_v1) | **POST** /host-migration/aggregates/host-migrations/v1 | Get host migration aggregates as specified via json in request body.
[**host_migrations_actions_v1**](HostMigrationApi.md#host_migrations_actions_v1) | **POST** /host-migration/entities/host-migrations-actions/v1 | Perform an action on host migrations.
[**migration_aggregates_v1**](HostMigrationApi.md#migration_aggregates_v1) | **POST** /host-migration/aggregates/migrations/v1 | Get migration aggregates as specified via json in request body.
[**migrations_actions_v1**](HostMigrationApi.md#migrations_actions_v1) | **POST** /host-migration/entities/migrations-actions/v1 | Perform an action on a migration job.

## create_migration_v1

> models::ApiPeriodCreateMigrationResponseV1 create_migration_v1(body)
Create a device migration job.

`device_ids` and `filter` are mutually exclusive. Filter takes precedence.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodCreateMigrationRequestV1**](ApiPeriodCreateMigrationRequestV1.md) |  | [required] |

### Return type

[**models::ApiPeriodCreateMigrationResponseV1**](api.CreateMigrationResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_host_migration_ids_v1

> models::MsaspecPeriodQueryResponse get_host_migration_ids_v1(id, offset, limit, sort, filter)
Query host migration IDs.

Query host migration IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The migration job to query | [required] |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-10000] |  |
**sort** | Option<**String**> | The property to sort by. |  |
**filter** | Option<**String**> | The filter expression that should be used to limit the results. Valid fields: hostgroups, hostname, status, host_migration_id, id, created_time, groups, static_host_groups, target_cid, source_cid, migration_id |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_host_migrations_v1

> models::ApiPeriodGetHostMigrationResponseV1 get_host_migrations_v1(body)
Get host migration details.

# Events   The `events` field describes actions that have occurred to the host migration entity. Each object is defined by the `action` field. When `user` is present, it is the user who performed the action. `time` is when the action occurred.  ## Event actions  ### added  This action is emitted when the host migration is created.  ``` { \"action\": \"added\", \"user\": \"example@example.com\", \"time\": \"2024-01-01T00:00:00Z\" } ```  ### assigned_static_host_groups  This action is emitted when a user assigns static host groups to a host migration. `ids` are the ids of the new host groups that have been assigned.  ``` { \"action\": \"assigned_static_host_groups\", \"ids\": [\"foo\", \"bar\"],  \"user\": \"example@example.com\", \"time\": \"2024-01-01T00:00:00Z\" } ```  ### removed_static_host_groups  This action is emitted when a user removes static host groups from a host migration. `ids` are the ids of the host groups that have been removed.  ``` { \"action\": \"removed_static_host_groups\", \"ids\": [\"foo\", \"bar\"],  \"user\": \"example@example.com\", \"time\": \"2024-01-01T00:00:00Z\" } ```  ### queued  This action is emitted when the migration is started.  ``` { \"action\": \"queued\", \"user\": \"example@example.com\", \"time\": \"2024-01-01T00:00:00Z\" } ```  ### failed  This action is emitted when the host migration fails. `reason` is the reason for failure. `reason` can be `unsupported_sensor_version`, `unsupported_sensor_platform`, `host_missing`, `migration_expired`, or `internal_error`.  ``` { \"action\": \"failed\", \"reason\": \"unsupported_sensor_version\", \"time\": \"2024-01-01T00:00:00Z\" } ```  ### cancelled  This action is emitted when the migration has been cancelled.  ``` { \"action\": \"cancelled\", \"user\": \"example@example.com\", \"time\": \"2024-01-01T00:00:00Z\" } ```  ### completed  This action is emitted when the host has successfully migrated.  ``` { \"action\": \"completed\", \"time\": \"2024-01-01T00:00:00Z\" } ```  # Status Details  The `status_details` field is an optional field that provides some more details about the status of a failed host migration. It may be omitted or empty from a response.  ### internal_error  This status detail is provided when an internal occurs during a host migration.  ### canceled_by_user  This status detail is provided when a migration has been canceled by a user.  ### host_missing  This status detail is provided when a host migration is canceled because the source host can no longer be found.  ### migration_expired  This status detail is provided when a host migration is expired because the migration is too old.  ### migration_already_in_progress  This status detail is provided when attempting to start a host migration on a host that is already in progress in another migration.  ### source_host_unsupported_version  This status detail is provided when attempting to create or start a host migration when the sensor is on an unsupported version.  ### source_host_unsupported_platform  This status detail is provided when attempting to create or start a host migration when the sensor is an unsupported platform

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::ApiPeriodGetHostMigrationResponseV1**](api.GetHostMigrationResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_migration_destinations_v1

> models::ApiPeriodGetMigrationDestinationsResponseV1 get_migration_destinations_v1(body)
Get destinations for a migration.

`device_ids` and `filter` are mutually exclusive.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodGetMigrationDestinationsRequestBodyV1**](ApiPeriodGetMigrationDestinationsRequestBodyV1.md) |  | [required] |

### Return type

[**models::ApiPeriodGetMigrationDestinationsResponseV1**](api.GetMigrationDestinationsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_migration_ids_v1

> models::MsaspecPeriodQueryResponse get_migration_ids_v1(offset, limit, sort, filter)
Query migration jobs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-10000] |  |
**sort** | Option<**String**> | The property to sort by. |  |
**filter** | Option<**String**> | The filter expression that should be used to limit the results. Valid fields: status, migration_status, created_by, created_time, name, id, migration_id, target_cid |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_migrations_v1

> models::ApiPeriodGetMigrationsResponseV1 get_migrations_v1(ids)
Get migration job details.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The migration jobs of interest. | [required] |

### Return type

[**models::ApiPeriodGetMigrationsResponseV1**](api.GetMigrationsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## host_migration_aggregates_v1

> models::MsaPeriodAggregatesResponse host_migration_aggregates_v1(body)
Get host migration aggregates as specified via json in request body.

Get host migration aggregates as specified via json in request body.  # Supported Types  Both types support the following FQL filter properties: `groups`, `hostgroups`, `static_host_groups`, `hostname`, `status`, `target_cid`, `source_cid`, `migration_id`, `id`, `host_migration_id`, `created_time`.  The values `groups` and `hostgroups` are aliases for `static_host_groups`.  The value `host_migration_id` is an alias for `id`  ## Terms `\"type\": \"terms\"`  Supported `field` values: `groups`, `hostgroups`, `static_host_groups`, `hostname`, `status`, `target_cid`, `source_cid`, `migration_id`, `id`, `host_migration_id`.  `sort` must be done on the same value as `field` and include a direction (`asc` or `desc`). Supports all FQL fields except for `groups`, `hostgroups`, or `static_host_groups`.  Examples sort value: `status|asc` or `created_by|desc`   ## Date Range `\"type\": \"date_range\"`  Supported `field` fields: `created_time`.  Does not support `sort`, `size`, or `from`.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## host_migrations_actions_v1

> models::MsaspecPeriodQueryResponse host_migrations_actions_v1(id, action_name, body)
Perform an action on host migrations.

The available actions are `add_host_groups`, `remove_host_groups`, and `remove_hosts`.  FQL filter supports the following fields: `groups`, `hostgroups`, `static_host_groups`, `hostname`, `status`, `target_cid`, `source_cid`, `migration_id`, `id`, `host_migration_id`, `created_time`.  These actions only works if the migration has not started.  `add_host_groups` adds static host groups to the selected hosts in a migration. This action accepts the following action parameter: `{ \"name\": \"host_group\": \"value\": \"$host_group_id\" }`. Action parameters can be repeated to add multiple static host groups in a single request.  `remove_host_groups` removes static host groups from the selected hosts in a migration. This action accepts the following action parameter: `{ \"name\": \"host_group\": \"value\": \"$host_group_id\" }`. Action parameters can be repeated to remove multiple static host groups in a single request.  `remove_hosts` removes the selected hosts from a migration. This action does not accept any action parameters.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The migration job to perform actions on | [required] |
**action_name** | **String** | The action to perform | [required] |
**body** | [**MsaPeriodEntityActionRequestV3**](MsaPeriodEntityActionRequestV3.md) |  | [required] |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## migration_aggregates_v1

> models::MsaPeriodAggregatesResponse migration_aggregates_v1(body)
Get migration aggregates as specified via json in request body.

Get migration aggregates as specified via json in request body.  # Supported Types  Both types support the following FQL filter props: `name`, `id`, `migration_id`, `target_cid`, `status`, `migration_status`, `created_by`, `created_time`.  The value `migration_status` is an alias for `status`.  The value `migration_id` is an alias for `id`.  ## Terms `\"type\": \"terms\"`  Supported `field` values: `name`, `id`, `migration_id,` `target_cid`, `status`, `migration_status`, `created_by`.  `sort` on `terms` type must be done on the same value as `field` and include a direction (`asc` or `desc`). Supports all supported FQL fields.  Examples sort value: `status|asc` or `created_by|desc`.   ## Date Range `\"type\": \"date_range\"`  Supported `field` fields: `created_time`.  Does not support `sort`, `size`, or `from`.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## migrations_actions_v1

> models::MsaspecPeriodQueryResponse migrations_actions_v1(action_name, body)
Perform an action on a migration job.

The available actions are `start_migration`, `cancel_migration`, `rename_migration`, and `delete_migration`.  `start_migration` starts the selected migrations. This action only works if the migration has not started. This action does not accept any action parameters. Only one migration may be started per request.  `cancel_migration` cancels the selected migrations. This actions only works if the migration has started and not completed. This action does not accept any action parameters.  `rename_migration` renames the selected migrations. This action can be called at any time. Only 1 action parameter may be supplied. Action parameters take the form of `{\"name\": \"migration_name\": \"value\": \"$new_migration_name\"}`.  `delete_migration` deletes the selected migrations. This action only works if the migration has not started. This action does not accept any action parameters.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_name** | **String** | The action to perform | [required] |
**body** | [**MsaPeriodEntityActionRequestV3**](MsaPeriodEntityActionRequestV3.md) |  | [required] |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
