# \FalconxSandboxApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_report**](FalconxSandboxApi.md#delete_report) | **DELETE** /falconx/entities/reports/v1 | Delete report based on the report ID. Operation can be checked for success by polling for the report ID on the report-summaries endpoint.
[**delete_sample_v2**](FalconxSandboxApi.md#delete_sample_v2) | **DELETE** /samples/entities/samples/v2 | Removes a sample, including file, meta and submissions from the collection
[**get_artifacts**](FalconxSandboxApi.md#get_artifacts) | **GET** /falconx/entities/artifacts/v1 | Download IOC packs, PCAP files, memory dumps, and other analysis artifacts.
[**get_memory_dump**](FalconxSandboxApi.md#get_memory_dump) | **GET** /falconx/entities/memory-dump/v1 | Get memory dump content, as binary
[**get_memory_dump_extracted_strings**](FalconxSandboxApi.md#get_memory_dump_extracted_strings) | **GET** /falconx/entities/memory-dump/extracted-strings/v1 | Get extracted strings from a memory dump
[**get_memory_dump_hex_dump**](FalconxSandboxApi.md#get_memory_dump_hex_dump) | **GET** /falconx/entities/memory-dump/hex-dump/v1 | Get hex view of a memory dump
[**get_reports**](FalconxSandboxApi.md#get_reports) | **GET** /falconx/entities/reports/v1 | Get a full sandbox report.
[**get_sample_v2**](FalconxSandboxApi.md#get_sample_v2) | **GET** /samples/entities/samples/v2 | Retrieves the file associated with the given ID (SHA256)
[**get_submissions**](FalconxSandboxApi.md#get_submissions) | **GET** /falconx/entities/submissions/v1 | Check the status of a sandbox analysis. Time required for analysis varies but is usually less than 15 minutes.
[**get_summary_reports**](FalconxSandboxApi.md#get_summary_reports) | **GET** /falconx/entities/report-summaries/v1 | Get a short summary version of a sandbox report.
[**query_reports**](FalconxSandboxApi.md#query_reports) | **GET** /falconx/queries/reports/v1 | Find sandbox reports by providing an FQL filter and paging details. Returns a set of report IDs that match your criteria.
[**query_sample_v1**](FalconxSandboxApi.md#query_sample_v1) | **POST** /samples/queries/samples/GET/v1 | Retrieves a list with sha256 of samples that exist and customer has rights to access them, maximum number of accepted items is 200
[**query_submissions**](FalconxSandboxApi.md#query_submissions) | **GET** /falconx/queries/submissions/v1 | Find submission IDs for uploaded files by providing an FQL filter and paging details. Returns a set of submission IDs that match your criteria.
[**submit**](FalconxSandboxApi.md#submit) | **POST** /falconx/entities/submissions/v1 | Submit an uploaded file or a URL for sandbox analysis. Time required for analysis varies but is usually less than 15 minutes.
[**upload_sample_v2**](FalconxSandboxApi.md#upload_sample_v2) | **POST** /samples/entities/samples/v2 | Upload a file for sandbox analysis. After uploading, use `/falconx/entities/submissions/v1` to start analyzing the file.

## delete_report

> models::FalconxPeriodQueryResponse delete_report(ids)
Delete report based on the report ID. Operation can be checked for success by polling for the report ID on the report-summaries endpoint.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | ID of a report. | [required] |

### Return type

[**models::FalconxPeriodQueryResponse**](falconx.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_sample_v2

> models::MsaPeriodQueryResponse delete_sample_v2(ids)
Removes a sample, including file, meta and submissions from the collection

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The file SHA256. | [required] |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_artifacts

> models::MsaspecPeriodQueryResponse get_artifacts(id, name, accept_encoding)
Download IOC packs, PCAP files, memory dumps, and other analysis artifacts.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of an artifact, such as an IOC pack, PCAP file, memory dump, or actor image. Find an artifact ID in a report or summary. | [required] |
**name** | Option<**String**> | The name given to your downloaded file. |  |
**accept_encoding** | Option<**String**> | Format used to compress your downloaded file. Currently, you must provide the value `gzip`, the only valid format. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/octet-stream, application/json, text/plain, text/csv, image/png, image/jpeg, application/gzip

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_memory_dump

> models::MsaspecPeriodQueryResponse get_memory_dump(id, name, accept_encoding)
Get memory dump content, as binary

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Memory dump id | [required] |
**name** | Option<**String**> | The name given to your downloaded file. |  |
**accept_encoding** | Option<**String**> | Format used to compress your downloaded file. Currently, you must provide the value `gzip`, the only valid format. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_memory_dump_extracted_strings

> models::MsaspecPeriodQueryResponse get_memory_dump_extracted_strings(id, name, accept_encoding)
Get extracted strings from a memory dump

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Extracted strings id | [required] |
**name** | Option<**String**> | The name given to your downloaded file. |  |
**accept_encoding** | Option<**String**> | Format used to compress your downloaded file. Currently, you must provide the value `gzip`, the only valid format. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_memory_dump_hex_dump

> models::MsaspecPeriodQueryResponse get_memory_dump_hex_dump(id, name, accept_encoding)
Get hex view of a memory dump

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Hex dump id | [required] |
**name** | Option<**String**> | The name given to your downloaded file. |  |
**accept_encoding** | Option<**String**> | Format used to compress your downloaded file. Currently, you must provide the value `gzip`, the only valid format. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_reports

> models::FalconxPeriodReportV1Response get_reports(ids)
Get a full sandbox report.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | ID of a report. Find a report ID from the response when submitting a malware sample or search with `/falconx/queries/reports/v1`. | [required] |

### Return type

[**models::FalconxPeriodReportV1Response**](falconx.ReportV1Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_sample_v2

> String get_sample_v2(ids, password_protected)
Retrieves the file associated with the given ID (SHA256)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The file SHA256. | [required] |
**password_protected** | Option<**bool**> | Flag whether the sample should be zipped and password protected with pass='infected' |  |[default to false]

### Return type

**String**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_submissions

> models::FalconxPeriodSubmissionV1Response get_submissions(ids)
Check the status of a sandbox analysis. Time required for analysis varies but is usually less than 15 minutes.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | ID of a submitted malware sample. Find a submission ID from the response when submitting a malware sample or search with `/falconx/queries/submissions/v1`. | [required] |

### Return type

[**models::FalconxPeriodSubmissionV1Response**](falconx.SubmissionV1Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_summary_reports

> models::FalconxPeriodSummaryReportV1Response get_summary_reports(ids)
Get a short summary version of a sandbox report.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | ID of a summary. Find a summary ID from the response when submitting a malware sample or search with `/falconx/queries/reports/v1`. | [required] |

### Return type

[**models::FalconxPeriodSummaryReportV1Response**](falconx.SummaryReportV1Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_reports

> models::MsaspecPeriodQueryResponse query_reports(filter, offset, limit, sort)
Find sandbox reports by providing an FQL filter and paging details. Returns a set of report IDs that match your criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**String**> | The offset to start retrieving reports from. |  |
**limit** | Option<**i32**> | Maximum number of report IDs to return. Max: 5000. |  |
**sort** | Option<**String**> | Sort order: `asc` or `desc`. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_sample_v1

> models::MsaPeriodQueryResponse query_sample_v1(body)
Retrieves a list with sha256 of samples that exist and customer has rights to access them, maximum number of accepted items is 200

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ClientPeriodQuerySamplesRequest**](ClientPeriodQuerySamplesRequest.md) | Pass a list of sha256s to check if the exist. It will be returned the list of existing hashes. | [required] |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_submissions

> models::MsaspecPeriodQueryResponse query_submissions(filter, offset, limit, sort)
Find submission IDs for uploaded files by providing an FQL filter and paging details. Returns a set of submission IDs that match your criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**String**> | The offset to start retrieving submissions from. |  |
**limit** | Option<**i32**> | Maximum number of submission IDs to return. Max: 5000. |  |
**sort** | Option<**String**> | Sort order: `asc` or `desc`. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## submit

> models::FalconxPeriodSubmissionV1Response submit(body, aid)
Submit an uploaded file or a URL for sandbox analysis. Time required for analysis varies but is usually less than 15 minutes.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FalconxPeriodSubmissionParametersV1**](FalconxPeriodSubmissionParametersV1.md) | Submit either a URL or a sample SHA256 for sandbox analysis. The sample file must have been previously uploaded through `/samples/entities/samples/v2`. You must specify a JSON object that includes the `falconx.SubmissionParametersV1` key/value pairs shown below.  **`environment_id`**: Specifies the sandbox environment used for analysis. Values:  - `400`: MacOS Catalina 10.15 - `410`: MacOS Sonoma ARM 64 bit - `300`: Linux Ubuntu 16.04, 64-bit - `200`: Android (static analysis) - `160`: Windows 10, 64-bit - `140`: Windows 11, 64-bit - `110`: Windows 7, 64-bit - `100`: Windows 7, 32-bit  **`sha256`** ID of the sample, which is a SHA256 hash value. Find a sample ID from the response when uploading a malware sample or search with `/falconx/queries/submissions/v1`.The `url` parameter must be unset if `sha256` is used.  **`url`** A web page or file URL. It can be HTTP(S) or FTP. The `sha256` parameter must be unset if `url` is used.  **`action_script`** (optional): Runtime script for sandbox analysis. Values:  - `default` - `default_maxantievasion` - `default_randomfiles` - `default_randomtheme` - `default_openie`  **`command_line`** (optional): Command line script passed to the submitted file at runtime. Max length: 2048 characters  **`document_password`** (optional): Auto-filled for Adobe or Office files that prompt for a password. Max length: 32 characters  **`enable_tor`** (optional): Deprecated, please use `network_settings` instead. If `true`, sandbox analysis routes network traffic via TOR. Default: `false`.  **`network_settings`** (optional): Specifies the sandbox network_settings used for analysis. Values:  - `default`: Fully operating network - `tor`: Route network traffic via TOR - `simulated`: Simulate network traffic - `offline`: No network traffic  **`submit_name`** (optional): Name of the malware sample that's used for file type detection and analysis  **`system_date`** (optional): Set a custom date in the format `yyyy-MM-dd` for the sandbox environment  **`system_time`** (optional): Set a custom time in the format `HH:mm` for the sandbox environment. | [required] |
**aid** | Option<**String**> | Agent ID |  |

### Return type

[**models::FalconxPeriodSubmissionV1Response**](falconx.SubmissionV1Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## upload_sample_v2

> models::ClientPeriodSampleMetadataResponseV2 upload_sample_v2(sample, file_name, comment, is_confidential)
Upload a file for sandbox analysis. After uploading, use `/falconx/entities/submissions/v1` to start analyzing the file.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sample** | **std::path::PathBuf** | Content of the uploaded sample in binary format. For example, use `--data-binary @$FILE_PATH` when using cURL. Max file size: 256 MB.  Accepted file formats:  - Portable executables: `.exe`, `.scr`, `.pif`, `.dll`, `.com`, `.cpl`, etc. - Office documents: `.doc`, `.docx`, `.ppt`, `.pps`, `.pptx`, `.ppsx`, `.xls`, `.xlsx`, `.rtf`, `.pub` - PDF - APK - Executable JAR - Windows script component: `.sct` - Windows shortcut: `.lnk` - Windows help: `.chm` - HTML application: `.hta` - Windows script file: `.wsf` - Javascript: `.js` - Visual Basic: `.vbs`,  `.vbe` - Shockwave Flash: `.swf` - Perl: `.pl` - Powershell: `.ps1`, `.psd1`, `.psm1` - Scalable vector graphics: `.svg` - Python: `.py` - Linux ELF executables - Email files: MIME RFC 822 `.eml`, Outlook `.msg`. | [required] |
**file_name** | **String** | Name of the file. | [required] |
**comment** | Option<**String**> | A descriptive comment to identify the file for other users. |  |
**is_confidential** | Option<**bool**> | Defines visibility of this file in Falcon MalQuery, either via the API or the Falcon console.  - `true`: File is only shown to users within your customer account - `false`: File can be seen by other CrowdStrike customers   Default: `true`. |  |[default to true]

### Return type

[**models::ClientPeriodSampleMetadataResponseV2**](client.SampleMetadataResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data, application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
