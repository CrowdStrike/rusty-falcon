# \CustomStorageApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object**](CustomStorageApi.md#delete_object) | **DELETE** /customobjects/v1/collections/{collection_name}/objects/{object_key} | Delete the specified object
[**get_object**](CustomStorageApi.md#get_object) | **GET** /customobjects/v1/collections/{collection_name}/objects/{object_key} | Get the bytes for the specified object
[**get_object_metadata**](CustomStorageApi.md#get_object_metadata) | **GET** /customobjects/v1/collections/{collection_name}/objects/{object_key}/metadata | Get the metadata for the specified object
[**list_objects**](CustomStorageApi.md#list_objects) | **GET** /customobjects/v1/collections/{collection_name}/objects | List the object keys in the specified collection in alphabetical order
[**put_object**](CustomStorageApi.md#put_object) | **PUT** /customobjects/v1/collections/{collection_name}/objects/{object_key} | Put the specified new object at the given key or overwrite an existing object at the given key
[**search_objects**](CustomStorageApi.md#search_objects) | **POST** /customobjects/v1/collections/{collection_name}/objects | Search for objects that match the specified filter criteria (returns metadata, not actual objects)

## delete_object

> models::CustomType3191042536 delete_object(collection_name, object_key, dry_run)
Delete the specified object

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**object_key** | **String** | The object key | [required] |
**dry_run** | Option<**bool**> | If false, run the operation as normal.  If true, validate that the request *would* succeed, but don't execute it. |  |

### Return type

[**models::CustomType3191042536**](CustomType_3191042536.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_object

> std::path::PathBuf get_object(collection_name, object_key)
Get the bytes for the specified object

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**object_key** | **String** | The object key | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_object_metadata

> models::CustomType3191042536 get_object_metadata(collection_name, object_key)
Get the metadata for the specified object

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**object_key** | **String** | The object key | [required] |

### Return type

[**models::CustomType3191042536**](CustomType_3191042536.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## list_objects

> models::CustomType1255839303 list_objects(collection_name, end, limit, start)
List the object keys in the specified collection in alphabetical order

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**end** | Option<**String**> | The end key to end listing to |  |
**limit** | Option<**i32**> | The limit of results to return |  |
**start** | Option<**String**> | The start key to start listing from |  |

### Return type

[**models::CustomType1255839303**](CustomType_1255839303.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## put_object

> models::CustomType3191042536 put_object(collection_name, object_key, body, dry_run, schema_version)
Put the specified new object at the given key or overwrite an existing object at the given key

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**object_key** | **String** | The object key | [required] |
**body** | **std::path::PathBuf** |  | [required] |
**dry_run** | Option<**bool**> | If false, run the operation as normal.  If true, validate that the request *would* succeed, but don't execute it. |  |
**schema_version** | Option<**String**> | The version of the collection schema |  |

### Return type

[**models::CustomType3191042536**](CustomType_3191042536.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/octet-stream, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## search_objects

> models::CustomType3191042536 search_objects(collection_name, filter, limit, offset, sort)
Search for objects that match the specified filter criteria (returns metadata, not actual objects)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**filter** | **String** | The filter to limit the returned results. | [required] |
**limit** | Option<**i32**> | The limit of results to return |  |
**offset** | Option<**i32**> | The offset of results to return |  |
**sort** | Option<**String**> | The sort order for the returned results. |  |

### Return type

[**models::CustomType3191042536**](CustomType_3191042536.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
