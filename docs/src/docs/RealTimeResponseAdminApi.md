# \RealTimeResponseAdminApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_admin_cmd**](RealTimeResponseAdminApi.md#batch_admin_cmd) | **POST** /real-time-response/combined/batch-admin-command/v1 | Batch executes a RTR administrator command across the hosts mapped to the given batch ID.
[**r_tr_check_admin_command_status**](RealTimeResponseAdminApi.md#r_tr_check_admin_command_status) | **GET** /real-time-response/entities/admin-command/v1 | Get status of an executed RTR administrator command on a single host.
[**r_tr_create_put_files**](RealTimeResponseAdminApi.md#r_tr_create_put_files) | **POST** /real-time-response/entities/put-files/v1 | Upload a new put-file to use for the RTR `put` command.
[**r_tr_create_scripts**](RealTimeResponseAdminApi.md#r_tr_create_scripts) | **POST** /real-time-response/entities/scripts/v1 | Upload a new custom-script to use for the RTR `runscript` command.
[**r_tr_delete_put_files**](RealTimeResponseAdminApi.md#r_tr_delete_put_files) | **DELETE** /real-time-response/entities/put-files/v1 | Delete a put-file based on the ID given.  Can only delete one file at a time.
[**r_tr_delete_scripts**](RealTimeResponseAdminApi.md#r_tr_delete_scripts) | **DELETE** /real-time-response/entities/scripts/v1 | Delete a custom-script based on the ID given.  Can only delete one script at a time.
[**r_tr_execute_admin_command**](RealTimeResponseAdminApi.md#r_tr_execute_admin_command) | **POST** /real-time-response/entities/admin-command/v1 | Execute a RTR administrator command on a single host.
[**r_tr_get_falcon_scripts**](RealTimeResponseAdminApi.md#r_tr_get_falcon_scripts) | **GET** /real-time-response/entities/falcon-scripts/v1 | Get Falcon scripts with metadata and content of script
[**r_tr_get_put_files**](RealTimeResponseAdminApi.md#r_tr_get_put_files) | **GET** /real-time-response/entities/put-files/v1 | Get put-files based on the ID's given. These are used for the RTR `put` command.
[**r_tr_get_put_files_v2**](RealTimeResponseAdminApi.md#r_tr_get_put_files_v2) | **GET** /real-time-response/entities/put-files/v2 | Get put-files based on the ID's given. These are used for the RTR `put` command.
[**r_tr_get_scripts**](RealTimeResponseAdminApi.md#r_tr_get_scripts) | **GET** /real-time-response/entities/scripts/v1 | Get custom-scripts based on the ID's given. These are used for the RTR `runscript` command.
[**r_tr_get_scripts_v2**](RealTimeResponseAdminApi.md#r_tr_get_scripts_v2) | **GET** /real-time-response/entities/scripts/v2 | Get custom-scripts based on the ID's given. These are used for the RTR `runscript` command.
[**r_tr_list_falcon_scripts**](RealTimeResponseAdminApi.md#r_tr_list_falcon_scripts) | **GET** /real-time-response/queries/falcon-scripts/v1 | Get a list of Falcon script IDs available to the user to run
[**r_tr_list_put_files**](RealTimeResponseAdminApi.md#r_tr_list_put_files) | **GET** /real-time-response/queries/put-files/v1 | Get a list of put-file ID's that are available to the user for the `put` command.
[**r_tr_list_scripts**](RealTimeResponseAdminApi.md#r_tr_list_scripts) | **GET** /real-time-response/queries/scripts/v1 | Get a list of custom-script ID's that are available to the user for the `runscript` command.
[**r_tr_update_scripts**](RealTimeResponseAdminApi.md#r_tr_update_scripts) | **PATCH** /real-time-response/entities/scripts/v1 | Upload a new scripts to replace an existing one.

## batch_admin_cmd

> models::DomainPeriodMultiCommandExecuteResponseWrapper batch_admin_cmd(body, timeout, timeout_duration, host_timeout_duration)
Batch executes a RTR administrator command across the hosts mapped to the given batch ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodBatchExecuteCommandRequest**](DomainPeriodBatchExecuteCommandRequest.md) | Use this endpoint to run these [real time response commands](https://falcon.crowdstrike.com/documentation/page/b8c1738c/real-time-response-and-network-containment#k893b7c0): - `cat` - `cd` - `clear` - `cp` - `encrypt` - `env` - `eventlog` - `filehash` - `get` - `getsid` - `help` - `history` - `ipconfig` - `kill` - `ls` - `map` - `memdump` - `mkdir` - `mount` - `mv` - `netstat` - `ps` - `put` - `reg query` - `reg set` - `reg delete` - `reg load` - `reg unload` - `restart` - `rm` - `run` - `runscript` - `shutdown` - `unmap` - `update history` - `update install` - `update list` - `update query` - `xmemdump` - `zip`  **`base_command`** Active-Responder command type we are going to execute, for example: `get` or `cp`.  Refer to the RTR documentation for the full list of commands. **`batch_id`** Batch ID to execute the command on.  Received from `/real-time-response/combined/batch-init-session/v1`. **`command_string`** Full command string for the command. For example  `get some_file.txt` **`optional_hosts`** List of a subset of hosts we want to run the command on.  If this list is supplied, only these hosts will receive the command. | [required] |
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

## r_tr_check_admin_command_status

> models::DomainPeriodStatusResponseWrapper r_tr_check_admin_command_status(cloud_request_id, sequence_id)
Get status of an executed RTR administrator command on a single host.

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

## r_tr_create_put_files

> models::MsaPeriodReplyMetaOnly r_tr_create_put_files(file, description, name, comments_for_audit_log)
Upload a new put-file to use for the RTR `put` command.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | put-file to upload | [required] |
**description** | **String** | File description | [required] |
**name** | Option<**String**> | File name (if different than actual file name) |  |
**comments_for_audit_log** | Option<**String**> | The audit log comment |  |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_create_scripts

> models::MsaPeriodReplyMetaOnly r_tr_create_scripts(description, permission_type, file, name, comments_for_audit_log, content, platform)
Upload a new custom-script to use for the RTR `runscript` command.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**description** | **String** | File description | [required] |
**permission_type** | **String** | Permission for the custom-script. Valid permission values:   - `private`, usable by only the user who uploaded it   - `group`, usable by all RTR Admins   - `public`, usable by all active-responders and RTR admins | [required] |[default to none]
**file** | Option<**std::path::PathBuf**> | custom-script file to upload.  These should be powershell scripts. |  |
**name** | Option<**String**> | File name (if different than actual file name) |  |
**comments_for_audit_log** | Option<**String**> | The audit log comment |  |
**content** | Option<**String**> | The script text that you want to use to upload |  |
**platform** | Option<[**Vec<String>**](String.md)> | Platforms for the file. Currently supports: windows, mac, linux, . If no platform is provided, it will default to 'windows' |  |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_delete_put_files

> models::MsaPeriodReplyMetaOnly r_tr_delete_put_files(ids)
Delete a put-file based on the ID given.  Can only delete one file at a time.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | File id | [required] |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_delete_scripts

> models::MsaPeriodReplyMetaOnly r_tr_delete_scripts(ids)
Delete a custom-script based on the ID given.  Can only delete one script at a time.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | File id | [required] |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_execute_admin_command

> models::DomainPeriodCommandExecuteResponseWrapper r_tr_execute_admin_command(body)
Execute a RTR administrator command on a single host.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodCommandExecuteRequest**](DomainPeriodCommandExecuteRequest.md) | Use this endpoint to run these [real time response commands](https://falcon.crowdstrike.com/documentation/page/b8c1738c/real-time-response-and-network-containment#k893b7c0): - `cat` - `cd` - `clear` - `cp` - `encrypt` - `env` - `eventlog` - `filehash` - `get` - `getsid` - `help` - `history` - `ipconfig` - `kill` - `ls` - `map` - `memdump` - `mkdir` - `mount` - `mv` - `netstat` - `ps` - `put` - `reg query` - `reg set` - `reg delete` - `reg load` - `reg unload` - `restart` - `rm` - `run` - `runscript` - `shutdown` - `unmap` - `update history` - `update install` - `update list` - `update query` - `xmemdump` - `zip`  Required values.  The rest of the fields are unused. **`base_command`** Active-Responder command type we are going to execute, for example: `get` or `cp`.  Refer to the RTR documentation for the full list of commands. **`command_string`** Full command string for the command. For example  `get some_file.txt` **`session_id`** RTR session ID to run the command on | [required] |

### Return type

[**models::DomainPeriodCommandExecuteResponseWrapper**](domain.CommandExecuteResponseWrapper.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_get_falcon_scripts

> models::EmpowerapiPeriodMsaFalconScriptResponse r_tr_get_falcon_scripts(ids)
Get Falcon scripts with metadata and content of script

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | IDs of the Falcon scripts you want to retrieve | [required] |

### Return type

[**models::EmpowerapiPeriodMsaFalconScriptResponse**](empowerapi.MsaFalconScriptResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_get_put_files

> models::EmpowerapiPeriodMsaPfResponseV1 r_tr_get_put_files(ids)
Get put-files based on the ID's given. These are used for the RTR `put` command.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | File IDs | [required] |

### Return type

[**models::EmpowerapiPeriodMsaPfResponseV1**](empowerapi.MsaPFResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_get_put_files_v2

> models::EmpowerapiPeriodMsaPfResponseV2 r_tr_get_put_files_v2(ids)
Get put-files based on the ID's given. These are used for the RTR `put` command.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | File IDs | [required] |

### Return type

[**models::EmpowerapiPeriodMsaPfResponseV2**](empowerapi.MsaPFResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_get_scripts

> models::EmpowerapiPeriodMsaPfResponseV1 r_tr_get_scripts(ids)
Get custom-scripts based on the ID's given. These are used for the RTR `runscript` command.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | File IDs | [required] |

### Return type

[**models::EmpowerapiPeriodMsaPfResponseV1**](empowerapi.MsaPFResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_get_scripts_v2

> models::EmpowerapiPeriodMsaPfResponseV2 r_tr_get_scripts_v2(ids)
Get custom-scripts based on the ID's given. These are used for the RTR `runscript` command.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | File IDs | [required] |

### Return type

[**models::EmpowerapiPeriodMsaPfResponseV2**](empowerapi.MsaPFResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_list_falcon_scripts

> models::EmpowerapiPeriodMsaIdListResponse r_tr_list_falcon_scripts(filter, offset, limit, sort)
Get a list of Falcon script IDs available to the user to run

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Optional filter criteria in the form of an FQL query. For more information about FQL queries, see our [FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**i32**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |
**sort** | Option<**String**> | Sort by spec. Ex: 'created_at|asc'. |  |

### Return type

[**models::EmpowerapiPeriodMsaIdListResponse**](empowerapi.MsaIDListResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_list_put_files

> models::BinservapiPeriodMsaPutFileResponse r_tr_list_put_files(filter, offset, limit, sort)
Get a list of put-file ID's that are available to the user for the `put` command.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Optional filter criteria in the form of an FQL query. For more information about FQL queries, see our [FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |
**sort** | Option<**String**> | Sort by spec. Ex: 'created_at|asc'. |  |

### Return type

[**models::BinservapiPeriodMsaPutFileResponse**](binservapi.MsaPutFileResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_list_scripts

> models::BinservapiPeriodMsaPutFileResponse r_tr_list_scripts(filter, offset, limit, sort)
Get a list of custom-script ID's that are available to the user for the `runscript` command.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Optional filter criteria in the form of an FQL query. For more information about FQL queries, see our [FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |
**sort** | Option<**String**> | Sort by spec. Ex: 'created_at|asc'. |  |

### Return type

[**models::BinservapiPeriodMsaPutFileResponse**](binservapi.MsaPutFileResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## r_tr_update_scripts

> models::MsaPeriodReplyMetaOnly r_tr_update_scripts(id, file, description, name, comments_for_audit_log, permission_type, content, platform)
Upload a new scripts to replace an existing one.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID to update | [required] |
**file** | Option<**std::path::PathBuf**> | custom-script file to upload.  These should be powershell scripts. |  |
**description** | Option<**String**> | File description |  |
**name** | Option<**String**> | File name (if different than actual file name) |  |
**comments_for_audit_log** | Option<**String**> | The audit log comment |  |
**permission_type** | Option<**String**> | Permission for the custom-script. Valid permission values:   - `private`, usable by only the user who uploaded it   - `group`, usable by all RTR Admins   - `public`, usable by all active-responders and RTR admins |  |[default to none]
**content** | Option<**String**> | The script text that you want to use to upload |  |
**platform** | Option<[**Vec<String>**](String.md)> | Platforms for the file. Currently supports: windows, mac, linux,  |  |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
