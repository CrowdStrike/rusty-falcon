# \UserManagementApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**combined_user_roles_v1**](UserManagementApi.md#combined_user_roles_v1) | **GET** /user-management/combined/user-roles/v1 | Get User Grant(s). This endpoint lists both direct as well as flight control grants between a User and a Customer.
[**create_user**](UserManagementApi.md#create_user) | **POST** /users/entities/users/v1 | Deprecated : Please use POST /user-management/entities/users/v1. Create a new user. After creating a user, assign one or more roles with POST /user-roles/entities/user-roles/v1
[**create_user_v1**](UserManagementApi.md#create_user_v1) | **POST** /user-management/entities/users/v1 | Create a new user. After creating a user, assign one or more roles with POST '/user-management/entities/user-role-actions/v1'
[**delete_user**](UserManagementApi.md#delete_user) | **DELETE** /users/entities/users/v1 | Deprecated : Please use DELETE /user-management/entities/users/v1. Delete a user permanently
[**delete_user_v1**](UserManagementApi.md#delete_user_v1) | **DELETE** /user-management/entities/users/v1 | Delete a user permanently.
[**entities_roles_v1**](UserManagementApi.md#entities_roles_v1) | **GET** /user-management/entities/roles/v1 | Get info about a role
[**get_available_role_ids**](UserManagementApi.md#get_available_role_ids) | **GET** /user-roles/queries/user-role-ids-by-cid/v1 | Deprecated : Please use GET /user-management/queries/roles/v1. Show role IDs for all roles available in your customer account. For more information on each role, provide the role ID to `/customer/entities/roles/v1`.
[**get_roles**](UserManagementApi.md#get_roles) | **GET** /user-roles/entities/user-roles/v1 | Deprecated : Please use GET /user-management/entities/roles/v1. Get info about a role
[**get_user_role_ids**](UserManagementApi.md#get_user_role_ids) | **GET** /user-roles/queries/user-role-ids-by-user-uuid/v1 | Deprecated : Please use GET /user-management/combined/user-roles/v1. Show role IDs of roles assigned to a user. For more information on each role, provide the role ID to `/customer/entities/roles/v1`.
[**grant_user_role_ids**](UserManagementApi.md#grant_user_role_ids) | **POST** /user-roles/entities/user-roles/v1 | Deprecated : Please use POST /user-management/entities/user-role-actions/v1. Assign one or more roles to a user
[**queries_roles_v1**](UserManagementApi.md#queries_roles_v1) | **GET** /user-management/queries/roles/v1 | Show role IDs for all roles available in your customer account. For more information on each role, provide the role ID to `/user-management/entities/roles/v1`.
[**query_user_v1**](UserManagementApi.md#query_user_v1) | **GET** /user-management/queries/users/v1 | List user IDs for all users in your customer account. For more information on each user, provide the user ID to `/user-management/entities/users/GET/v1`.
[**retrieve_emails_by_cid**](UserManagementApi.md#retrieve_emails_by_cid) | **GET** /users/queries/emails-by-cid/v1 | Deprecated : Please use POST /user-management/entities/users/GET/v1. List the usernames (usually an email address) for all users in your customer account
[**retrieve_user**](UserManagementApi.md#retrieve_user) | **GET** /users/entities/users/v1 | Deprecated : Please use POST /user-management/entities/users/GET/v1. Get info about a user
[**retrieve_user_uuid**](UserManagementApi.md#retrieve_user_uuid) | **GET** /users/queries/user-uuids-by-email/v1 | Deprecated : Please use GET /user-management/queries/users/v1. Get a user's ID by providing a username (usually an email address)
[**retrieve_user_uuids_by_cid**](UserManagementApi.md#retrieve_user_uuids_by_cid) | **GET** /users/queries/user-uuids-by-cid/v1 | Deprecated : Please use GET /user-management/queries/users/v1. List user IDs for all users in your customer account. For more information on each user, provide the user ID to `/users/entities/user/v1`.
[**retrieve_users_getv1**](UserManagementApi.md#retrieve_users_getv1) | **POST** /user-management/entities/users/GET/v1 | Get info about users including their name, UID and CID by providing user UUIDs
[**revoke_user_role_ids**](UserManagementApi.md#revoke_user_role_ids) | **DELETE** /user-roles/entities/user-roles/v1 | Deprecated : Please use POST /user-management/entities/user-role-actions/v1. Revoke one or more roles from a user
[**update_user**](UserManagementApi.md#update_user) | **PATCH** /users/entities/users/v1 | Deprecated : Please use PATCH /user-management/entities/users/v1. Modify an existing user's first or last name
[**update_user_v1**](UserManagementApi.md#update_user_v1) | **PATCH** /user-management/entities/users/v1 | Modify an existing user's first or last name.
[**user_action_v1**](UserManagementApi.md#user_action_v1) | **POST** /user-management/entities/user-actions/v1 | Apply actions to one or more User. Available action names: reset_2fa, reset_password. User UUIDs can be provided in `ids` param as part of request payload.
[**user_roles_action_v1**](UserManagementApi.md#user_roles_action_v1) | **POST** /user-management/entities/user-role-actions/v1 | Grant or Revoke one or more role(s) to a user against a CID. User UUID, CID and Role ID(s) can be provided in request payload. Available Action(s) : grant, revoke

## combined_user_roles_v1

> models::FlightcontrolapiPeriodUserGrantResponse combined_user_roles_v1(user_uuid, cid, direct_only, filter, offset, limit, sort)
Get User Grant(s). This endpoint lists both direct as well as flight control grants between a User and a Customer.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_uuid** | **String** | User UUID to get available roles for. | [required] |
**cid** | Option<**String**> | Customer ID to get grants for. Empty CID would result in Role IDs for user against current CID in view. |  |
**direct_only** | Option<**bool**> | Specifies if to request direct Only role grants or all role grants between user and CID (specified in query params) |  |[default to false]
**filter** | Option<**String**> | Filter using a query in Falcon Query Language (FQL). Supported filters: role_id, role_name |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |[default to 0]
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |[default to 100]
**sort** | Option<**String**> | The property to sort by |  |[default to role_name|asc]

### Return type

[**models::FlightcontrolapiPeriodUserGrantResponse**](flightcontrolapi.userGrantResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_user

> models::ApiPeriodUserMetadataResponse create_user(body)
Deprecated : Please use POST /user-management/entities/users/v1. Create a new user. After creating a user, assign one or more roles with POST /user-roles/entities/user-roles/v1

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodUserCreateRequest**](DomainPeriodUserCreateRequest.md) | Attributes for this user. `uid` (required) is the user's email address, which is their username in Falcon.  Optional attributes:  <ul><li>`firstName`</li><li>`lastName`</li><li>`password`</li></ul>  As a best practice, we recommend omitting `password`. If single sign-on is enabled for your customer account, the `password` attribute is ignored. If single sign-on is not enabled, we send a user activation request to their email address when you create the user with no `password`. The user should use the activation email to set their own password. | [required] |

### Return type

[**models::ApiPeriodUserMetadataResponse**](api.userMetadataResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_user_v1

> models::FlightcontrolapiPeriodUserResponse create_user_v1(body, validate_only)
Create a new user. After creating a user, assign one or more roles with POST '/user-management/entities/user-role-actions/v1'

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodCreateUserRequest**](DomainPeriodCreateUserRequest.md) | Attributes for this user. `uid` (required) is the user's email address, which is their username in Falcon.  Optional attributes:  <ul><li>`firstName`</li><li>`lastName`</li><li>`password`</li></ul>  As a best practice, we recommend omitting `password`. If single sign-on is enabled for your customer account, the `password` attribute is ignored. If single sign-on is not enabled, we send a user activation request to their email address when you create the user with no `password`. The user should use the activation email to set their own password. | [required] |
**validate_only** | Option<**bool**> | Validate of user is allowed, but do not create user. |  |[default to false]

### Return type

[**models::FlightcontrolapiPeriodUserResponse**](flightcontrolapi.userResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_user

> models::MsaPeriodReplyMetaOnly delete_user(user_uuid)
Deprecated : Please use DELETE /user-management/entities/users/v1. Delete a user permanently

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_uuid** | **String** | ID of a user. Find a user's ID from `/users/entities/user/v1`. | [required] |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_user_v1

> models::MsaspecPeriodResponseFields delete_user_v1(user_uuid)
Delete a user permanently.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_uuid** | **String** | User UUID. | [required] |

### Return type

[**models::MsaspecPeriodResponseFields**](msaspec.ResponseFields.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## entities_roles_v1

> models::FlightcontrolapiPeriodGetRolesResponse entities_roles_v1(ids, cid)
Get info about a role

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | ID of a role. Find a role ID from `/user-management/queries/roles/v1`. | [required] |
**cid** | Option<**String**> | Customer ID to get available roles for. Empty CID would result in Role IDs for current CID in view. |  |

### Return type

[**models::FlightcontrolapiPeriodGetRolesResponse**](flightcontrolapi.getRolesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_available_role_ids

> models::MsaPeriodQueryResponse get_available_role_ids()
Deprecated : Please use GET /user-management/queries/roles/v1. Show role IDs for all roles available in your customer account. For more information on each role, provide the role ID to `/customer/entities/roles/v1`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_roles

> models::ApiPeriodUserRoleResponse get_roles(ids)
Deprecated : Please use GET /user-management/entities/roles/v1. Get info about a role

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | ID of a role. Find a role ID from `/customer/queries/roles/v1` or `/users/queries/roles/v1`. | [required] |

### Return type

[**models::ApiPeriodUserRoleResponse**](api.userRoleResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_user_role_ids

> models::MsaPeriodQueryResponse get_user_role_ids(user_uuid)
Deprecated : Please use GET /user-management/combined/user-roles/v1. Show role IDs of roles assigned to a user. For more information on each role, provide the role ID to `/customer/entities/roles/v1`.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_uuid** | **String** | ID of a user. Find a user's ID from `/users/entities/user/v1`. | [required] |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## grant_user_role_ids

> models::ApiPeriodUserRoleIdsResponse grant_user_role_ids(user_uuid, body)
Deprecated : Please use POST /user-management/entities/user-role-actions/v1. Assign one or more roles to a user

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_uuid** | **String** | ID of a user. Find a user's ID from `/users/entities/user/v1`. | [required] |
**body** | [**DomainPeriodRoleIds**](DomainPeriodRoleIds.md) | Role ID(s) of the role you want to assign | [required] |

### Return type

[**models::ApiPeriodUserRoleIdsResponse**](api.userRoleIDsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## queries_roles_v1

> models::MsaspecPeriodQueryResponse queries_roles_v1(cid, user_uuid, action)
Show role IDs for all roles available in your customer account. For more information on each role, provide the role ID to `/user-management/entities/roles/v1`.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | Option<**String**> | Customer ID to get available roles for. Empty CID would result in Role IDs for current CID in view. |  |
**user_uuid** | Option<**String**> | User UUID to get available roles for. Empty User UUID would returns all roles IDs available for customer. |  |
**action** | Option<**String**> | Actionable purpose of the query |  |[default to grant]

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_user_v1

> models::MsaspecPeriodQueryResponse query_user_v1(filter, offset, limit, sort)
List user IDs for all users in your customer account. For more information on each user, provide the user ID to `/user-management/entities/users/GET/v1`.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter using a query in Falcon Query Language (FQL). Supported filters: assigned_cids, cid, first_name, last_name, name, uid |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |[default to 0]
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |[default to 100]
**sort** | Option<**String**> | The property to sort by |  |[default to uid|asc]

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## retrieve_emails_by_cid

> models::MsaPeriodQueryResponse retrieve_emails_by_cid()
Deprecated : Please use POST /user-management/entities/users/GET/v1. List the usernames (usually an email address) for all users in your customer account

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## retrieve_user

> models::ApiPeriodUserMetadataResponse retrieve_user(ids)
Deprecated : Please use POST /user-management/entities/users/GET/v1. Get info about a user

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | ID of a user. Find a user's ID from `/users/entities/user/v1`. | [required] |

### Return type

[**models::ApiPeriodUserMetadataResponse**](api.userMetadataResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## retrieve_user_uuid

> models::MsaPeriodQueryResponse retrieve_user_uuid(uid)
Deprecated : Please use GET /user-management/queries/users/v1. Get a user's ID by providing a username (usually an email address)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | [**Vec<String>**](String.md) | A username. This is usually the user's email address, but may vary based on your configuration. | [required] |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## retrieve_user_uuids_by_cid

> models::MsaPeriodQueryResponse retrieve_user_uuids_by_cid()
Deprecated : Please use GET /user-management/queries/users/v1. List user IDs for all users in your customer account. For more information on each user, provide the user ID to `/users/entities/user/v1`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## retrieve_users_getv1

> models::FlightcontrolapiPeriodUserResponse retrieve_users_getv1(body)
Get info about users including their name, UID and CID by providing user UUIDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaspecPeriodIdsRequest**](MsaspecPeriodIdsRequest.md) | Maximum of 5000 User UUIDs can be specified per request. | [required] |

### Return type

[**models::FlightcontrolapiPeriodUserResponse**](flightcontrolapi.userResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## revoke_user_role_ids

> models::ApiPeriodUserRoleIdsResponse revoke_user_role_ids(user_uuid, ids)
Deprecated : Please use POST /user-management/entities/user-role-actions/v1. Revoke one or more roles from a user

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_uuid** | **String** | ID of a user. Find a user's ID from `/users/entities/user/v1`. | [required] |
**ids** | [**Vec<String>**](String.md) | One or more role IDs to revoke. Find a role's ID from `/users/queries/roles/v1`. | [required] |

### Return type

[**models::ApiPeriodUserRoleIdsResponse**](api.userRoleIDsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_user

> models::ApiPeriodUserMetadataResponse update_user(user_uuid, body)
Deprecated : Please use PATCH /user-management/entities/users/v1. Modify an existing user's first or last name

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_uuid** | **String** | ID of a user. Find a user's ID from `/users/entities/user/v1`. | [required] |
**body** | [**DomainPeriodUpdateUserFields**](DomainPeriodUpdateUserFields.md) | Attributes for this user. All attributes (shown below) are optional. | [required] |

### Return type

[**models::ApiPeriodUserMetadataResponse**](api.userMetadataResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_user_v1

> models::FlightcontrolapiPeriodUserResponse update_user_v1(user_uuid, body)
Modify an existing user's first or last name.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_uuid** | **String** | user uuid | [required] |
**body** | [**DomainPeriodUpdateUserRequest**](DomainPeriodUpdateUserRequest.md) | Both firstName and lastName have to specified. | [required] |

### Return type

[**models::FlightcontrolapiPeriodUserResponse**](flightcontrolapi.userResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## user_action_v1

> models::MsaspecPeriodResponseFields user_action_v1(body)
Apply actions to one or more User. Available action names: reset_2fa, reset_password. User UUIDs can be provided in `ids` param as part of request payload.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodUserActionRequest**](DomainPeriodUserActionRequest.md) | User UUIDs and Action Name params are required. Allowed values for Action Name param includes 'reset_2fa' and 'reset_password' | [required] |

### Return type

[**models::MsaspecPeriodResponseFields**](msaspec.ResponseFields.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## user_roles_action_v1

> models::MsaspecPeriodResponseFields user_roles_action_v1(body)
Grant or Revoke one or more role(s) to a user against a CID. User UUID, CID and Role ID(s) can be provided in request payload. Available Action(s) : grant, revoke

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodActionUserRolesRequest**](DomainPeriodActionUserRolesRequest.md) | All fields including CID, RoleID(s), User UUID and Action are required. Allowed values for Action param include 'grant' and 'revoke'. | [required] |

### Return type

[**models::MsaspecPeriodResponseFields**](msaspec.ResponseFields.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
