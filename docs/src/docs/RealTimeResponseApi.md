w# \RealTimeResponseApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_active_responder_cmd**](RealTimeResponseApi.md#batch_active_responder_cmd) | **POST** /real-time-response/combined/batch-active-responder-command/v1 | Batch executes a RTR active-responder command across the hosts mapped to the given batch ID.
[**batch_cmd**](RealTimeResponseApi.md#batch_cmd) | **POST** /real-time-response/combined/batch-command/v1 | Batch executes a RTR read-only command across the hosts mapped to the given batch ID.
[**batch_get_cmd**](RealTimeResponseApi.md#batch_get_cmd) | **POST** /real-time-response/combined/batch-get-command/v1 | Batch executes `get` command across hosts to retrieve files. After this call is made `GET /real-time-response/combined/batch-get-command/v1` is used to query for the results.
[**batch_get_cmd_status**](RealTimeResponseApi.md#batch_get_cmd_status) | **GET** /real-time-response/combined/batch-get-command/v1 | Retrieves the status of the specified batch get command.  Will return successful files when they are finished processing.
[**batch_init_sessions**](RealTimeResponseApi.md#batch_init_sessions) | **POST** /real-time-response/combined/batch-init-session/v1 | Batch initialize a RTR session on multiple hosts.  Before any RTR commands can be used, an active session is needed on the host.
[**batch_refresh_sessions**](RealTimeResponseApi.md#batch_refresh_sessions) | **POST** /real-time-response/combined/batch-refresh-session/v1 | Batch refresh a RTR session on multiple hosts. RTR sessions will expire after 10 minutes unless refreshed.
[**r_tr_aggregate_sessions**](RealTimeResponseApi.md#r_tr_aggregate_sessions) | **POST** /real-time-response/aggregates/sessions/GET/v1 | Get aggregates on session data.
[**r_tr_check_active_responder_command_status**](RealTimeResponseApi.md#r_tr_check_active_responder_command_status) | **GET** /real-time-response/entities/active-responder-command/v1 | Get status of an executed active-responder command on a single host.
[**r_tr_check_command_status**](RealTimeResponseApi.md#r_tr_check_command_status) | **GET** /real-time-response/entities/command/v1 | Get status of an executed command on a single host.
[**r_tr_delete_file**](RealTimeResponseApi.md#r_tr_delete_file) | **DELETE** /real-time-response/entities/file/v1 | Delete a RTR session file.
[**r_tr_delete_file_v2**](RealTimeResponseApi.md#r_tr_delete_file_v2) | **DELETE** /real-time-response/entities/file/v2 | Delete a RTR session file.
[**r_tr_delete_queued_session**](RealTimeResponseApi.md#r_tr_delete_queued_session) | **DELETE** /real-time-response/entities/queued-sessions/command/v1 | Delete a queued session command
[**r_tr_delete_session**](RealTimeResponseApi.md#r_tr_delete_session) | **DELETE** /real-time-response/entities/sessions/v1 | Delete a session.
[**r_tr_execute_active_responder_command**](RealTimeResponseApi.md#r_tr_execute_active_responder_command) | **POST** /real-time-response/entities/active-responder-command/v1 | Execute an active responder command on a single host.
[**r_tr_execute_command**](RealTimeResponseApi.md#r_tr_execute_command) | **POST** /real-time-response/entities/command/v1 | Execute a command on a single host.
[**r_tr_get_extracted_file_contents**](RealTimeResponseApi.md#r_tr_get_extracted_file_contents) | **GET** /real-time-response/entities/extracted-file-contents/v1 | Get RTR extracted file contents for specified session and sha256.
[**r_tr_init_session**](RealTimeResponseApi.md#r_tr_init_session) | **POST** /real-time-response/entities/sessions/v1 | Initialize a new session with the RTR cloud.
[**r_tr_list_all_sessions**](RealTimeResponseApi.md#r_tr_list_all_sessions) | **GET** /real-time-response/queries/sessions/v1 | Get a list of session_ids.
[**r_tr_list_files**](RealTimeResponseApi.md#r_tr_list_files) | **GET** /real-time-response/entities/file/v1 | Get a list of files for the specified RTR session.
[**r_tr_list_files_v2**](RealTimeResponseApi.md#r_tr_list_files_v2) | **GET** /real-time-response/entities/file/v2 | Get a list of files for the specified RTR session.
[**r_tr_list_queued_sessions**](RealTimeResponseApi.md#r_tr_list_queued_sessions) | **POST** /real-time-response/entities/queued-sessions/GET/v1 | Get queued session metadata by session ID.
[**r_tr_list_sessions**](RealTimeResponseApi.md#r_tr_list_sessions) | **POST** /real-time-response/entities/sessions/GET/v1 | Get session metadata by session id.
[**r_tr_pulse_session**](RealTimeResponseApi.md#r_tr_pulse_session) | **POST** /real-time-response/entities/refresh-session/v1 | Refresh a session timeout on a single host.

## batch_active_responder_cmd

> models::DomainPeriodMultiCommandExecuteResponseWrapper batch_active_responder_cmd(body, timeout, timeout_duration, host_timeout_duration)
Batch executes a RTR active-responder command across the hosts mapped to the given batch ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodBatchExecuteCommandRequest**](DomainPeriodBatchExecuteCommandRequest.md) | Use this endpoint to run these [real time response commands](https://falcon.crowdstrike.com/documentation/page/b8c1738c/real-time-response-and-network-containment#k893b7c0): - `cat` - `cd` - `clear` - `cp` - `encrypt` - `env` - `eventlog` - `filehash` - `get` - `getsid` - `help` - `history` - `ipconfig` - `kill` - `ls` - `map` - `memdump` - `mkdir` - `mount` - `mv` - `netstat` - `ps` - `reg query` - `reg set` - `reg delete` - `reg load` - `reg unload` - `restart` - `rm` - `runscript` - `shutdown` - `unmap` - `update history` - `update install` - `update list` - `update query` - `xmemdump` - `zip`  **`base_command`** Active-Responder command type we are going to execute, for example: `get` or `cp`.  Refer to the RTR documentation for the full list of commands. **`batch_id`** Batch ID to execute the command on.  Received from `/real-time-response/combined/batch-init-session/v1`. **`command_string`** Full command string for the command. For example  `get some_file.txt` **`optional_hosts`** List of a subset of hosts we want to run the command on.  If this list is supplied, only these hosts will receive the command. | [required] |
**timeout** | Option<**i32**> | Timeout for how long to wait for the request in seconds, default timeout is 30 seconds. Maximum is 5 minutes. |  |[default to 30]
**timeout_duration** | Option<**String**> | Timeout duration for how long to wait for the request in duration syntax. Example, `10s`. Valid units: `ns, us, ms, s, m, h`. Maximum is 5 minutes. |  |[default to 30s]
**host_timeout_duration** | Option<**String**> | Timeout duration for how long a host has time to complete processing. Default value is a bit less than the overall timeout value. It cannot be greater than the overall request timeout. Maximum is < 5 minutes. Example, `10s`. Valid units: `ns, us, ms, s, m, h`.  |  |[default to tiny bit less than overall request timeout]

### Return type

[**models::DomainPeriodMultiCommandExecuteResponseWrapper**](domain.MultiCommandExecuteResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## batch_cmd

> models::DomainPeriodMultiCommandExecuteResponseWrapper batch_cmd(body, timeout, timeout_duration, host_timeout_duration)
Batch executes a RTR read-only command across the hosts mapped to the given batch ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodBatchExecuteCommandRequest**](DomainPeriodBatchExecuteCommandRequest.md) | Use this endpoint to run these [real time response commands](https://falcon.crowdstrike.com/documentation/page/b8c1738c/real-time-response-and-network-containment#k893b7c0): - `cat` - `cd` - `clear` - `env` - `eventlog` - `filehash` - `getsid` - `help` - `history` - `ipconfig` - `ls` - `mount` - `netstat` - `ps` - `reg query`  **`base_command`** read-only command type we are going to execute, for example: `ls` or `cd`.  Refer to the RTR documentation for the full list of commands. **`batch_id`** Batch ID to execute the command on.  Received from `/real-time-response/combined/batch-init-session/v1`. **`command_string`** Full command string for the command. For example  `cd C:\\some_directory` **`optional_hosts`** List of a subset of hosts we want to run the command on.  If this list is supplied, only these hosts will receive the command. | [required] |
**timeout** | Option<**i32**> | Timeout for how long to wait for the request in seconds, default timeout is 30 seconds. Maximum is 5 minutes. |  |[default to 30]
**timeout_duration** | Option<**String**> | Timeout duration for how long to wait for the request in duration syntax. Example, `10s`. Valid units: `ns, us, ms, s, m, h`. Maximum is 5 minutes. |  |[default to 30s]
**host_timeout_duration** | Option<**String**> | Timeout duration for how long a host has time to complete processing. Default value is a bit less than the overall timeout value. It cannot be greater than the overall request timeout. Maximum is < 5 minutes. Example, `10s`. Valid units: `ns, us, ms, s, m, h`.  |  |[default to tiny bit less than overall request timeout]

### Return type

[**models::DomainPeriodMultiCommandExecuteResponseWrapper**](domain.MultiCommandExecuteResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## batch_get_cmd

> models::DomainPeriodBatchGetCommandResponse batch_get_cmd(body, timeout, timeout_duration, host_timeout_duration)
Batch executes `get` command across hosts to retrieve files. After this call is made `GET /real-time-response/combined/batch-get-command/v1` is used to query for the results.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodBatchGetCommandRequest**](DomainPeriodBatchGetCommandRequest.md) | **`batch_id`** Batch ID to execute the command on.  Received from `/real-time-response/combined/batch-init-session/v1`. **`file_path`** Full path to the file that is to be retrieved from each host in the batch. **`optional_hosts`** List of a subset of hosts we want to run the command on.  If this list is supplied, only these hosts will receive the command. | [required] |
**timeout** | Option<**i32**> | Timeout for how long to wait for the request in seconds, default timeout is 30 seconds. Maximum is 5 minutes. |  |[default to 30]
**timeout_duration** | Option<**String**> | Timeout duration for how long to wait for the request in duration syntax. Example, `10s`. Valid units: `ns, us, ms, s, m, h`. Maximum is 5 minutes. |  |[default to 30s]
**host_timeout_duration** | Option<**String**> | Timeout duration for how long a host has time to complete processing. Default value is a bit less than the overall timeout value. It cannot be greater than the overall request timeout. Maximum is < 5 minutes. Example, `10s`. Valid units: `ns, us, ms, s, m, h`.  |  |[default to tiny bit less than overall request timeout]

### Return type

[**models::DomainPeriodBatchGetCommandResponse**](domain.BatchGetCommandResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## batch_get_cmd_status

> models::DomainPeriodBatchGetCmdStatusResponse batch_get_cmd_status(batch_get_cmd_req_id, timeout, timeout_duration)
Retrieves the status of the specified batch get command.  Will return successful files when they are finished processing.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_get_cmd_req_id** | **String** | Batch Get Command Request ID received from `/real-time-response/combined/get-command/v1` | [required] |
**timeout** | Option<**i32**> | Timeout for how long to wait for the request in seconds, default timeout is 30 seconds. Maximum is 5 minutes. |  |[default to 30]
**timeout_duration** | Option<**String**> | Timeout duration for how long to wait for the request in duration syntax. Example, `10s`. Valid units: `ns, us, ms, s, m, h`. Maximum is 5 minutes. |  |[default to 30s]

### Return type

[**models::DomainPeriodBatchGetCmdStatusResponse**](domain.BatchGetCmdStatusResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## batch_init_sessions

> models::DomainPeriodBatchInitSessionResponse batch_init_sessions(body, timeout, timeout_duration, host_timeout_duration)
Batch initialize a RTR session on multiple hosts.  Before any RTR commands can be used, an active session is needed on the host.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodBatchInitSessionRequest**](DomainPeriodBatchInitSessionRequest.md) | **`host_ids`** List of host agent ID's to initialize a RTR session on. A maximum of 10000 hosts can be in a single batch session. **`existing_batch_id`** Optional batch ID. Use an existing batch ID if you want to initialize new hosts and add them to the existing batch **`queue_offline`** If we should queue this session if the host is offline.  Any commands run against an offline-queued session will be queued up and executed when the host comes online. | [required] |
**timeout** | Option<**i32**> | Timeout for how long to wait for the request in seconds, default timeout is 30 seconds. Maximum is 5 minutes. |  |[default to 30]
**timeout_duration** | Option<**String**> | Timeout duration for how long to wait for the request in duration syntax. Example, `10s`. Valid units: `ns, us, ms, s, m, h`. Maximum is 5 minutes. |  |[default to 30s]
**host_timeout_duration** | Option<**String**> | Timeout duration for how long a host has time to complete processing. Default value is a bit less than the overall timeout value. It cannot be greater than the overall request timeout. Maximum is < 5 minutes. Example, `10s`. Valid units: `ns, us, ms, s, m, h`.  |  |[default to tiny bit less than overall request timeout]

### Return type

[**models::DomainPeriodBatchInitSessionResponse**](domain.BatchInitSessionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## batch_refresh_sessions

> models::DomainPeriodBatchRefreshSessionResponse batch_refresh_sessions(body, timeout, timeout_duration)
Batch refresh a RTR session on multiple hosts. RTR sessions will expire after 10 minutes unless refreshed.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodBatchRefreshSessionRequest**](DomainPeriodBatchRefreshSessionRequest.md) | **`batch_id`** Batch ID to execute the command on.  Received from `/real-time-response/combined/batch-init-session/v1`. **`hosts_to_remove`** Hosts to remove from the batch session.  Heartbeats will no longer happen on these hosts and the sessions will expire. | [required] |
**timeout** | Option<**i32**> | Timeout for how long to wait for the request in seconds, default timeout is 30 seconds. Maximum is 5 minutes. |  |[default to 30]
**timeout_duration** | Option<**String**> | Timeout duration for how long to wait for the request in duration syntax. Example, `10s`. Valid units: `ns, us, ms, s, m, h`. Maximum is 5 minutes. |  |[default to 30s]

### Return type

[**models::DomainPeriodBatchRefreshSessionResponse**](domain.BatchRefreshSessionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_aggregate_sessions

> models::MsaPeriodAggregatesResponse r_tr_aggregate_sessions(body)
Get aggregates on session data.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) | Supported aggregations:  - `term` - `date_range`  Supported aggregation members:  **`date_ranges`** If performing a date range query specify the **`from`** and **`to`** date ranges.  These can be in common date formats like `2019-07-18` or `now` **`field`** Term you want to aggregate on.  If doing a `date_range` query, this is the date field you want to apply the date ranges to **`filter`** Optional filter criteria in the form of an FQL query. For more information about FQL queries, see our [FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). **`name`** Name of the aggregation **`size`** Size limit to apply to the queries. | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_check_active_responder_command_status

> models::DomainPeriodStatusResponseWrapper r_tr_check_active_responder_command_status(cloud_request_id, sequence_id)
Get status of an executed active-responder command on a single host.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_request_id** | **String** | Cloud Request ID of the executed command to query | [required] |
**sequence_id** | **i32** | Sequence ID that we want to retrieve. Command responses are chunked across sequences | [required] |[default to 0]

### Return type

[**models::DomainPeriodStatusResponseWrapper**](domain.StatusResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_check_command_status

> models::DomainPeriodStatusResponseWrapper r_tr_check_command_status(cloud_request_id, sequence_id)
Get status of an executed command on a single host.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_request_id** | **String** | Cloud Request ID of the executed command to query | [required] |
**sequence_id** | **i32** | Sequence ID that we want to retrieve. Command responses are chunked across sequences | [required] |[default to 0]

### Return type

[**models::DomainPeriodStatusResponseWrapper**](domain.StatusResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_delete_file

> models::MsaPeriodReplyMetaOnly r_tr_delete_file(ids, session_id)
Delete a RTR session file.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | RTR Session file id | [required] |
**session_id** | **String** | RTR Session id | [required] |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_delete_file_v2

> models::MsaPeriodReplyMetaOnly r_tr_delete_file_v2(ids, session_id)
Delete a RTR session file.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | RTR Session file id | [required] |
**session_id** | **String** | RTR Session id | [required] |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_delete_queued_session

> models::MsaspecPeriodQueryResponse r_tr_delete_queued_session(session_id, cloud_request_id)
Delete a queued session command

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | RTR Session id | [required] |
**cloud_request_id** | **String** | Cloud Request ID of the executed command to query | [required] |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_delete_session

> models::MsaPeriodReplyMetaOnly r_tr_delete_session(session_id)
Delete a session.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | RTR Session id | [required] |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_execute_active_responder_command

> models::DomainPeriodCommandExecuteResponseWrapper r_tr_execute_active_responder_command(body)
Execute an active responder command on a single host.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodCommandExecuteRequest**](DomainPeriodCommandExecuteRequest.md) | Use this endpoint to run these [real time response commands](https://falcon.crowdstrike.com/documentation/page/b8c1738c/real-time-response-and-network-containment#k893b7c0): - `cat` - `cd` - `clear` - `cp` - `encrypt` - `env` - `eventlog` - `filehash` - `get` - `getsid` - `help` - `history` - `ipconfig` - `kill` - `ls` - `map` - `memdump` - `mkdir` - `mount` - `mv` - `netstat` - `ps` - `reg query` - `reg set` - `reg delete` - `reg load` - `reg unload` - `restart` - `rm` - `runscript` - `shutdown` - `unmap` - `update history` - `update install` - `update list` - `update query` - `xmemdump` - `zip`  Required values.  The rest of the fields are unused. **`base_command`** Active-Responder command type we are going to execute, for example: `get` or `cp`.  Refer to the RTR documentation for the full list of commands. **`command_string`** Full command string for the command. For example  `get some_file.txt` **`session_id`** RTR session ID to run the command on | [required] |

### Return type

[**models::DomainPeriodCommandExecuteResponseWrapper**](domain.CommandExecuteResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_execute_command

> models::DomainPeriodCommandExecuteResponseWrapper r_tr_execute_command(body)
Execute a command on a single host.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodCommandExecuteRequest**](DomainPeriodCommandExecuteRequest.md) | Use this endpoint to run these [real time response commands](https://falcon.crowdstrike.com/documentation/page/b8c1738c/real-time-response-and-network-containment#k893b7c0): - `cat` - `cd` - `clear` - `env` - `eventlog` - `filehash` - `getsid` - `help` - `history` - `ipconfig` - `ls` - `mount` - `netstat` - `ps` - `reg query`  Required values.  The rest of the fields are unused. **`base_command`** read-only command type we are going to execute, for example: `ls` or `cd`.  Refer to the RTR documentation for the full list of commands. **`command_string`** Full command string for the command. For example  `cd C:\\some_directory` **`session_id`** RTR session ID to run the command on | [required] |

### Return type

[**models::DomainPeriodCommandExecuteResponseWrapper**](domain.CommandExecuteResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_get_extracted_file_contents

> Vec<i32> r_tr_get_extracted_file_contents(session_id, sha256, filename)
Get RTR extracted file contents for specified session and sha256.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | RTR Session id | [required] |
**sha256** | **String** | Extracted SHA256 (e.g. 'efa256a96af3b556cd3fc9d8b1cf587d72807d7805ced441e8149fc279db422b') | [required] |
**filename** | Option<**String**> | Filename to use for the archive name and the file within the archive. |  |

### Return type

**Vec<i32>**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-7z-compressed, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_init_session

> models::DomainPeriodInitResponseWrapper r_tr_init_session(body, timeout, timeout_duration)
Initialize a new session with the RTR cloud.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodInitRequest**](DomainPeriodInitRequest.md) | **`device_id`** The host agent ID to initialize the RTR session on.  RTR will retrieve an existing session for the calling user on this host **`queue_offline`** If we should queue this session if the host is offline.  Any commands run against an offline-queued session will be queued up and executed when the host comes online. | [required] |
**timeout** | Option<**i32**> | Timeout for how long to wait for the request in seconds, default timeout is 30 seconds. Maximum is 5 minutes. |  |[default to 30]
**timeout_duration** | Option<**String**> | Timeout duration for how long to wait for the request in duration syntax. Example, `10s`. Valid units: `ns, us, ms, s, m, h`. Maximum is 5 minutes. |  |[default to 30s]

### Return type

[**models::DomainPeriodInitResponseWrapper**](domain.InitResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_list_all_sessions

> models::DomainPeriodListSessionsResponseMsa r_tr_list_all_sessions(offset, limit, sort, filter)
Get a list of session_ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |
**sort** | Option<**String**> | Sort by spec. Ex: 'date_created|asc'. |  |
**filter** | Option<**String**> | Optional filter criteria in the form of an FQL query. For more information about FQL queries, see our [FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). “user_id” can accept a special value ‘@me’ which will restrict results to records with current user’s ID. |  |

### Return type

[**models::DomainPeriodListSessionsResponseMsa**](domain.ListSessionsResponseMsa.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_list_files

> models::DomainPeriodListFilesResponseWrapper r_tr_list_files(session_id)
Get a list of files for the specified RTR session.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | RTR Session id | [required] |

### Return type

[**models::DomainPeriodListFilesResponseWrapper**](domain.ListFilesResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_list_files_v2

> models::DomainPeriodListFilesV2ResponseWrapper r_tr_list_files_v2(session_id)
Get a list of files for the specified RTR session.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | RTR Session id | [required] |

### Return type

[**models::DomainPeriodListFilesV2ResponseWrapper**](domain.ListFilesV2ResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_list_queued_sessions

> models::DomainPeriodQueuedSessionResponseWrapper r_tr_list_queued_sessions(body)
Get queued session metadata by session ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) | **`ids`** List of RTR sessions to retrieve.  RTR will only return the sessions that were created by the calling user | [required] |

### Return type

[**models::DomainPeriodQueuedSessionResponseWrapper**](domain.QueuedSessionResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_list_sessions

> models::DomainPeriodSessionResponseWrapper r_tr_list_sessions(body)
Get session metadata by session id.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) | **`ids`** List of RTR sessions to retrieve.  RTR will only return the sessions that were created by the calling user | [required] |

### Return type

[**models::DomainPeriodSessionResponseWrapper**](domain.SessionResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_pulse_session

> models::DomainPeriodInitResponseWrapper r_tr_pulse_session(body)
Refresh a session timeout on a single host.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodInitRequest**](DomainPeriodInitRequest.md) | **`device_id`** The host agent ID to refresh the RTR session on.  RTR will retrieve an existing session for the calling user on this host | [required] |

### Return type

[**models::DomainPeriodInitResponseWrapper**](domain.InitResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
