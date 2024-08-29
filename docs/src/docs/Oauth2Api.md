# \Oauth2Api

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**oauth2_access_token**](Oauth2Api.md#oauth2_access_token) | **POST** /oauth2/token | Generate an OAuth2 access token
[**oauth2_revoke_token**](Oauth2Api.md#oauth2_revoke_token) | **POST** /oauth2/revoke | Revoke a previously issued OAuth2 access token before the end of its standard 30-minute lifespan.

## oauth2_access_token

> models::DomainPeriodAccessTokenResponseV1 oauth2_access_token(client_id, client_secret, member_cid)
Generate an OAuth2 access token

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The API client ID to authenticate your API requests. For information on generating API clients, see [API documentation inside Falcon](https://falcon.crowdstrike.com/support/documentation/1/crowdstrike-api-introduction-for-developers). | [required] |
**client_secret** | **String** | The API client secret to authenticate your API requests. For information on generating API clients, see [API documentation inside Falcon](https://falcon.crowdstrike.com/support/documentation/1/crowdstrike-api-introduction-for-developers). | [required] |
**member_cid** | Option<**String**> | For MSSP Master CIDs, optionally lock the token to act on behalf of this member CID |  |

### Return type

[**models::DomainPeriodAccessTokenResponseV1**](domain.AccessTokenResponseV1.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, text/html
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## oauth2_revoke_token

> models::MsaspecPeriodResponseFields oauth2_revoke_token(token, client_id)
Revoke a previously issued OAuth2 access token before the end of its standard 30-minute lifespan.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | The OAuth2 access token you want to revoke.  Include your API client ID and secret in basic auth format (`Authorization: basic <encoded API client ID and secret>`) in your request header. | [required] |
**client_id** | Option<**String**> | The OAuth2 client ID you are revoking the token for. |  |

### Return type

[**models::MsaspecPeriodResponseFields**](msaspec.ResponseFields.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
