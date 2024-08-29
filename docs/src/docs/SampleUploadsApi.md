# \SampleUploadsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_delete_v1**](SampleUploadsApi.md#archive_delete_v1) | **DELETE** /archives/entities/archives/v1 | Delete an archive that was uploaded previously
[**archive_get_v1**](SampleUploadsApi.md#archive_get_v1) | **GET** /archives/entities/archives/v1 | Retrieves the archives upload operation statuses. Status `done` means that archive was processed successfully. Status `error` means that archive was not processed successfully.
[**archive_list_v1**](SampleUploadsApi.md#archive_list_v1) | **GET** /archives/entities/archive-files/v1 | Retrieves the archives files in chunks.
[**archive_upload_v1**](SampleUploadsApi.md#archive_upload_v1) | **POST** /archives/entities/archives/v1 | Uploads an archive and extracts files list from it. Operation is asynchronous use `/archives/entities/archives/v1` to check the status. After uploading, use `/archives/entities/extractions/v1` to copy the file to internal storage making it available for content analysis. This method is deprecated in favor of `/archives/entities/archives/v2`
[**archive_upload_v2**](SampleUploadsApi.md#archive_upload_v2) | **POST** /archives/entities/archives/v2 | Uploads an archive and extracts files list from it. Operation is asynchronous use `/archives/entities/archives/v1` to check the status. After uploading, use `/archives/entities/extractions/v1` to copy the file to internal storage making it available for content analysis.
[**delete_sample_v3**](SampleUploadsApi.md#delete_sample_v3) | **DELETE** /samples/entities/samples/v3 | Removes a sample, including file, meta and submissions from the collection
[**extraction_create_v1**](SampleUploadsApi.md#extraction_create_v1) | **POST** /archives/entities/extractions/v1 | Extracts files from an uploaded archive and copies them to internal storage making it available for content analysis.
[**extraction_get_v1**](SampleUploadsApi.md#extraction_get_v1) | **GET** /archives/entities/extractions/v1 | Retrieves the files extraction operation statuses. Status `done` means that all files were processed successfully. Status `error` means that at least one of the file could not be processed.
[**extraction_list_v1**](SampleUploadsApi.md#extraction_list_v1) | **GET** /archives/entities/extraction-files/v1 | Retrieves the files extractions in chunks. Status `done` means that all files were processed successfully. Status `error` means that at least one of the file could not be processed.
[**get_sample_v3**](SampleUploadsApi.md#get_sample_v3) | **GET** /samples/entities/samples/v3 | Retrieves the file associated with the given ID (SHA256)
[**upload_sample_v3**](SampleUploadsApi.md#upload_sample_v3) | **POST** /samples/entities/samples/v3 | Upload a file for further cloud analysis. After uploading, call the specific analysis API endpoint.

## archive_delete_v1

> archive_delete_v1(id)
Delete an archive that was uploaded previously

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The archive SHA256. | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## archive_get_v1

> models::ClientPeriodArchiveCreateResponseV1 archive_get_v1(id, include_files)
Retrieves the archives upload operation statuses. Status `done` means that archive was processed successfully. Status `error` means that archive was not processed successfully.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The archive SHA256. | [required] |
**include_files** | Option<**bool**> | If `true` includes processed archive files in response. |  |[default to false]

### Return type

[**models::ClientPeriodArchiveCreateResponseV1**](client.ArchiveCreateResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## archive_list_v1

> models::ClientPeriodArchiveListFilesResponseV1 archive_list_v1(id, limit, offset)
Retrieves the archives files in chunks.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The archive SHA256. | [required] |
**limit** | Option<**i32**> | Max number of files to retrieve. |  |[default to 100]
**offset** | Option<**String**> | Offset from where to get files. |  |

### Return type

[**models::ClientPeriodArchiveListFilesResponseV1**](client.ArchiveListFilesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## archive_upload_v1

> models::ClientPeriodArchiveCreateResponseV1 archive_upload_v1(name, body, password, is_confidential, comment)
Uploads an archive and extracts files list from it. Operation is asynchronous use `/archives/entities/archives/v1` to check the status. After uploading, use `/archives/entities/extractions/v1` to copy the file to internal storage making it available for content analysis. This method is deprecated in favor of `/archives/entities/archives/v2`

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the archive. | [required] |
**body** | [**Vec<i32>**](i32.md) | Content of the uploaded archive in binary format. For example, use `--data-binary @$FILE_PATH` when using cURL. Max file size: 100 MB.  Accepted file formats:  - Portable executables: `.zip`, `.7z`. | [required] |
**password** | Option<**String**> | Archive password. |  |
**is_confidential** | Option<**bool**> | Defines visibility of this file, either via the API or the Falcon console.  - `true`: File is only shown to users within your customer account - `false`: File can be seen by other CrowdStrike customers   Default: `true`. |  |[default to true]
**comment** | Option<**String**> | A descriptive comment to identify the file for other users. |  |

### Return type

[**models::ClientPeriodArchiveCreateResponseV1**](client.ArchiveCreateResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/octet-stream, application/zip, application/x-7z-compressed, application/x-zip-compressed
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## archive_upload_v2

> models::ClientPeriodArchiveCreateResponseV1 archive_upload_v2(file, name, password, is_confidential, comment)
Uploads an archive and extracts files list from it. Operation is asynchronous use `/archives/entities/archives/v1` to check the status. After uploading, use `/archives/entities/extractions/v1` to copy the file to internal storage making it available for content analysis.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | Content of the uploaded archive. For example, use `--form file=@$FILE_PATH;type=` when using cURL. Supported file types are `application/zip` and `application/x-7z-compressed`. | [required] |
**name** | **String** | Name of the archive. For example, use `--form name=` when using cURL. | [required] |
**password** | Option<**String**> | Archive password. For example, use `--form password=` when using cURL. |  |
**is_confidential** | Option<**bool**> | Defines visibility of this file in Falcon MalQuery, either via the API or the Falcon console. For example, use `--form is_confidential=` when using cURL.  - `true`: File is only shown to users within your customer account - `false`: File can be seen by other CrowdStrike customers   Default: `true`. |  |[default to true]
**comment** | Option<**String**> | A descriptive comment to identify the file for other users. For example, use `--form comment=` when using cURL. |  |

### Return type

[**models::ClientPeriodArchiveCreateResponseV1**](client.ArchiveCreateResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_sample_v3

> models::MsaPeriodQueryResponse delete_sample_v3(ids)
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

## extraction_create_v1

> models::ClientPeriodExtractionCreateResponseV1 extraction_create_v1(body)
Extracts files from an uploaded archive and copies them to internal storage making it available for content analysis.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ClientPeriodExtractionCreateRequestV1**](ClientPeriodExtractionCreateRequestV1.md) |  | [required] |

### Return type

[**models::ClientPeriodExtractionCreateResponseV1**](client.ExtractionCreateResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## extraction_get_v1

> models::ClientPeriodExtractionCreateResponseV1 extraction_get_v1(id, include_files)
Retrieves the files extraction operation statuses. Status `done` means that all files were processed successfully. Status `error` means that at least one of the file could not be processed.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The extraction operation ID. | [required] |
**include_files** | Option<**bool**> | If `true` includes processed archive files in response. |  |[default to false]

### Return type

[**models::ClientPeriodExtractionCreateResponseV1**](client.ExtractionCreateResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## extraction_list_v1

> models::ClientPeriodExtractionListFilesResponseV1 extraction_list_v1(id, limit, offset)
Retrieves the files extractions in chunks. Status `done` means that all files were processed successfully. Status `error` means that at least one of the file could not be processed.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The extraction operation ID. | [required] |
**limit** | Option<**i32**> | Max number of file extractions to retrieve. |  |[default to 0]
**offset** | Option<**String**> | Offset from where to get file extractions. |  |

### Return type

[**models::ClientPeriodExtractionListFilesResponseV1**](client.ExtractionListFilesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_sample_v3

> String get_sample_v3(ids, password_protected)
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

## upload_sample_v3

> models::ClientPeriodSampleMetadataResponseV2 upload_sample_v3(sample, file_name, comment, is_confidential)
Upload a file for further cloud analysis. After uploading, call the specific analysis API endpoint.

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
