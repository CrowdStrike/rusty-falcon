# \MsspApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_cid_group_members**](MsspApi.md#add_cid_group_members) | **POST** /mssp/entities/cid-group-members/v1 | Add new CID group member.
[**add_role**](MsspApi.md#add_role) | **POST** /mssp/entities/mssp-roles/v1 | Create a link between user group and CID group, with zero or more additional roles. The call does not replace any existing link between them. User group ID and CID group ID have to be specified in request.
[**add_user_group_members**](MsspApi.md#add_user_group_members) | **POST** /mssp/entities/user-group-members/v1 | Add new user group member. Maximum 500 members allowed per user group.
[**create_cid_groups**](MsspApi.md#create_cid_groups) | **POST** /mssp/entities/cid-groups/v1 | Create new CID groups. Name is a required field but description is an optional field. Maximum 500 CID groups allowed.
[**create_user_groups**](MsspApi.md#create_user_groups) | **POST** /mssp/entities/user-groups/v1 | Create new user groups. Name is a required field but description is an optional field. Maximum 500 user groups allowed per customer.
[**delete_cid_group_members**](MsspApi.md#delete_cid_group_members) | **DELETE** /mssp/entities/cid-group-members/v1 | Deprecated : Please use DELETE /entities/cid-group-members/v2. Delete CID group members.
[**delete_cid_group_members_v2**](MsspApi.md#delete_cid_group_members_v2) | **DELETE** /mssp/entities/cid-group-members/v2 | Delete CID group members. Prevents removal of a cid group a cid group if it is only part of one cid group.
[**delete_cid_groups**](MsspApi.md#delete_cid_groups) | **DELETE** /mssp/entities/cid-groups/v1 | Delete CID groups by ID.
[**delete_user_group_members**](MsspApi.md#delete_user_group_members) | **DELETE** /mssp/entities/user-group-members/v1 | Delete user group members entry.
[**delete_user_groups**](MsspApi.md#delete_user_groups) | **DELETE** /mssp/entities/user-groups/v1 | Delete user groups by ID.
[**deleted_roles**](MsspApi.md#deleted_roles) | **DELETE** /mssp/entities/mssp-roles/v1 | Delete links or additional roles between user groups and CID groups. User group ID and CID group ID have to be specified in request. Only specified roles are removed if specified in request payload, else association between User Group and CID group is dissolved completely (if no roles specified).
[**get_children**](MsspApi.md#get_children) | **GET** /mssp/entities/children/v1 | Get link to child customer by child CID(s)
[**get_children_v2**](MsspApi.md#get_children_v2) | **POST** /mssp/entities/children/GET/v2 | Get link to child customer by child CID(s)
[**get_cid_group_by_id**](MsspApi.md#get_cid_group_by_id) | **GET** /mssp/entities/cid-groups/v1 | Deprecated : Please use GET /mssp/entities/cid-groups/v2. Get CID groups by ID.
[**get_cid_group_by_id_v2**](MsspApi.md#get_cid_group_by_id_v2) | **GET** /mssp/entities/cid-groups/v2 | Get CID Groups by ID.
[**get_cid_group_members_by**](MsspApi.md#get_cid_group_members_by) | **GET** /mssp/entities/cid-group-members/v1 | Deprecated : Please use GET /mssp/entities/cid-group-members/v2. Get CID group members by CID group ID.
[**get_cid_group_members_by_v2**](MsspApi.md#get_cid_group_members_by_v2) | **GET** /mssp/entities/cid-group-members/v2 | Get CID group members by CID Group ID.
[**get_roles_by_id**](MsspApi.md#get_roles_by_id) | **GET** /mssp/entities/mssp-roles/v1 | Get link between user group and CID group by ID. Link ID is a string consisting of multiple components, but should be treated as opaque.
[**get_user_group_members_by_id**](MsspApi.md#get_user_group_members_by_id) | **GET** /mssp/entities/user-group-members/v1 | Deprecated : Please use GET /mssp/entities/user-group-members/v2. Get user group members by user group ID.
[**get_user_group_members_by_idv2**](MsspApi.md#get_user_group_members_by_idv2) | **GET** /mssp/entities/user-group-members/v2 | Get user group members by user group ID.
[**get_user_groups_by_id**](MsspApi.md#get_user_groups_by_id) | **GET** /mssp/entities/user-groups/v1 | Deprecated : Please use GET /entities/user-groups/v2. Get user groups by ID.
[**get_user_groups_by_idv2**](MsspApi.md#get_user_groups_by_idv2) | **GET** /mssp/entities/user-groups/v2 | Get user groups by ID.
[**query_children**](MsspApi.md#query_children) | **GET** /mssp/queries/children/v1 | Query for customers linked as children
[**query_cid_group_members**](MsspApi.md#query_cid_group_members) | **GET** /mssp/queries/cid-group-members/v1 | Query a CID groups members by associated CID.
[**query_cid_groups**](MsspApi.md#query_cid_groups) | **GET** /mssp/queries/cid-groups/v1 | Query CID groups.
[**query_roles**](MsspApi.md#query_roles) | **GET** /mssp/queries/mssp-roles/v1 | Query links between user groups and CID groups. At least one of CID group ID or user group ID should also be provided. Role ID is optional.
[**query_user_group_members**](MsspApi.md#query_user_group_members) | **GET** /mssp/queries/user-group-members/v1 | Query user group member by user UUID.
[**query_user_groups**](MsspApi.md#query_user_groups) | **GET** /mssp/queries/user-groups/v1 | Query user groups.
[**update_cid_groups**](MsspApi.md#update_cid_groups) | **PATCH** /mssp/entities/cid-groups/v1 | Update existing CID groups. CID group ID is expected for each CID group definition provided in request body. Name is a required field but description is an optional field. Empty description will override existing value. CID group member(s) remain unaffected.
[**update_user_groups**](MsspApi.md#update_user_groups) | **PATCH** /mssp/entities/user-groups/v1 | Update existing user group(s). User group ID is expected for each user group definition provided in request body. Name is a required field but description is an optional field. Empty description will override existing value. User group member(s) remain unaffected.

## add_cid_group_members

> models::DomainPeriodCidGroupMembersResponseV1 add_cid_group_members(body)
Add new CID group member.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodCidGroupMembersRequestV1**](DomainPeriodCidGroupMembersRequestV1.md) | Both 'cid_group_id' and 'cids' fields are required. | [required] |

### Return type

[**models::DomainPeriodCidGroupMembersResponseV1**](domain.CIDGroupMembersResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## add_role

> models::DomainPeriodMsspRoleResponseV1 add_role(body)
Create a link between user group and CID group, with zero or more additional roles. The call does not replace any existing link between them. User group ID and CID group ID have to be specified in request.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodMsspRoleRequestV1**](DomainPeriodMsspRoleRequestV1.md) | 'user_group_id', 'cid_group_id' and 'role_ids' fields are required. Remaining are populated by system. | [required] |

### Return type

[**models::DomainPeriodMsspRoleResponseV1**](domain.MSSPRoleResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## add_user_group_members

> models::DomainPeriodUserGroupMembersResponseV1 add_user_group_members(body)
Add new user group member. Maximum 500 members allowed per user group.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodUserGroupMembersRequestV1**](DomainPeriodUserGroupMembersRequestV1.md) | Both 'user_group_id' and 'user_uuids' fields are required. | [required] |

### Return type

[**models::DomainPeriodUserGroupMembersResponseV1**](domain.UserGroupMembersResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_cid_groups

> models::DomainPeriodCidGroupsResponseV1 create_cid_groups(body)
Create new CID groups. Name is a required field but description is an optional field. Maximum 500 CID groups allowed.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodCidGroupsRequestV1**](DomainPeriodCidGroupsRequestV1.md) | Only 'name' and/or 'description' fields are required. Remaining are assigned by the system. | [required] |

### Return type

[**models::DomainPeriodCidGroupsResponseV1**](domain.CIDGroupsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_user_groups

> models::DomainPeriodUserGroupsResponseV1 create_user_groups(body)
Create new user groups. Name is a required field but description is an optional field. Maximum 500 user groups allowed per customer.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodUserGroupsRequestV1**](DomainPeriodUserGroupsRequestV1.md) | Only 'name' and/or 'description' fields are required. Remaining are assigned by the system. | [required] |

### Return type

[**models::DomainPeriodUserGroupsResponseV1**](domain.UserGroupsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_cid_group_members

> models::DomainPeriodCidGroupMembersResponseV1 delete_cid_group_members(body)
Deprecated : Please use DELETE /entities/cid-group-members/v2. Delete CID group members.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodCidGroupMembersRequestV1**](DomainPeriodCidGroupMembersRequestV1.md) | Both 'cid_group_id' and 'cids' fields are required. | [required] |

### Return type

[**models::DomainPeriodCidGroupMembersResponseV1**](domain.CIDGroupMembersResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_cid_group_members_v2

> models::DomainPeriodCidGroupMembersResponseV1 delete_cid_group_members_v2(body)
Delete CID group members. Prevents removal of a cid group a cid group if it is only part of one cid group.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodCidGroupMembersRequestV1**](DomainPeriodCidGroupMembersRequestV1.md) | Both 'cid_group_id' and 'cids' fields are required. | [required] |

### Return type

[**models::DomainPeriodCidGroupMembersResponseV1**](domain.CIDGroupMembersResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_cid_groups

> models::MsaPeriodEntitiesResponse delete_cid_groups(cid_group_ids)
Delete CID groups by ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid_group_ids** | [**Vec<String>**](String.md) | CID group ids to delete | [required] |

### Return type

[**models::MsaPeriodEntitiesResponse**](msa.EntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_user_group_members

> models::DomainPeriodUserGroupMembersResponseV1 delete_user_group_members(body)
Delete user group members entry.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodUserGroupMembersRequestV1**](DomainPeriodUserGroupMembersRequestV1.md) | Both 'user_group_id' and 'user_uuids' fields are required. | [required] |

### Return type

[**models::DomainPeriodUserGroupMembersResponseV1**](domain.UserGroupMembersResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_user_groups

> models::MsaPeriodEntitiesResponse delete_user_groups(user_group_ids)
Delete user groups by ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_ids** | [**Vec<String>**](String.md) | User group IDs to delete | [required] |

### Return type

[**models::MsaPeriodEntitiesResponse**](msa.EntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## deleted_roles

> models::DomainPeriodMsspRoleResponseV1 deleted_roles(body)
Delete links or additional roles between user groups and CID groups. User group ID and CID group ID have to be specified in request. Only specified roles are removed if specified in request payload, else association between User Group and CID group is dissolved completely (if no roles specified).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodMsspRoleRequestV1**](DomainPeriodMsspRoleRequestV1.md) | 'user_group_id' and 'cid_group_id' fields are required. 'role_ids' field is optional. Remaining fields are ignored. | [required] |

### Return type

[**models::DomainPeriodMsspRoleResponseV1**](domain.MSSPRoleResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_children

> models::DomainPeriodChildrenResponseV1 get_children(ids)
Get link to child customer by child CID(s)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | CID of a child customer | [required] |

### Return type

[**models::DomainPeriodChildrenResponseV1**](domain.ChildrenResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_children_v2

> models::DomainPeriodChildrenResponseV1 get_children_v2(body)
Get link to child customer by child CID(s)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaspecPeriodIdsRequest**](MsaspecPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::DomainPeriodChildrenResponseV1**](domain.ChildrenResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cid_group_by_id

> models::DomainPeriodCidGroupsResponseV1 get_cid_group_by_id(cid_group_ids)
Deprecated : Please use GET /mssp/entities/cid-groups/v2. Get CID groups by ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid_group_ids** | [**Vec<String>**](String.md) | CID group IDs to be searched on | [required] |

### Return type

[**models::DomainPeriodCidGroupsResponseV1**](domain.CIDGroupsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cid_group_by_id_v2

> models::DomainPeriodCidGroupsResponseV1 get_cid_group_by_id_v2(ids)
Get CID Groups by ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | CID group IDs to search for | [required] |

### Return type

[**models::DomainPeriodCidGroupsResponseV1**](domain.CIDGroupsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cid_group_members_by

> models::DomainPeriodCidGroupMembersResponseV1 get_cid_group_members_by(cid_group_ids)
Deprecated : Please use GET /mssp/entities/cid-group-members/v2. Get CID group members by CID group ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid_group_ids** | [**Vec<String>**](String.md) | CID group IDs to search for | [required] |

### Return type

[**models::DomainPeriodCidGroupMembersResponseV1**](domain.CIDGroupMembersResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cid_group_members_by_v2

> models::DomainPeriodCidGroupMembersResponseV1 get_cid_group_members_by_v2(ids)
Get CID group members by CID Group ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | CID group IDs search for | [required] |

### Return type

[**models::DomainPeriodCidGroupMembersResponseV1**](domain.CIDGroupMembersResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_roles_by_id

> models::DomainPeriodMsspRoleResponseV1 get_roles_by_id(ids)
Get link between user group and CID group by ID. Link ID is a string consisting of multiple components, but should be treated as opaque.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Link ID is a string consisting of multiple components, but should be treated as opaque. | [required] |

### Return type

[**models::DomainPeriodMsspRoleResponseV1**](domain.MSSPRoleResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_user_group_members_by_id

> models::DomainPeriodUserGroupMembersResponseV1 get_user_group_members_by_id(user_group_ids)
Deprecated : Please use GET /mssp/entities/user-group-members/v2. Get user group members by user group ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_ids** | [**Vec<String>**](String.md) | User group IDs to search for | [required] |

### Return type

[**models::DomainPeriodUserGroupMembersResponseV1**](domain.UserGroupMembersResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_user_group_members_by_idv2

> models::DomainPeriodUserGroupMembersResponseV1 get_user_group_members_by_idv2(ids)
Get user group members by user group ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | User group IDs to search for | [required] |

### Return type

[**models::DomainPeriodUserGroupMembersResponseV1**](domain.UserGroupMembersResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_user_groups_by_id

> models::DomainPeriodUserGroupsResponseV1 get_user_groups_by_id(user_group_ids)
Deprecated : Please use GET /entities/user-groups/v2. Get user groups by ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_ids** | [**Vec<String>**](String.md) | User Group IDs to search for | [required] |

### Return type

[**models::DomainPeriodUserGroupsResponseV1**](domain.UserGroupsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_user_groups_by_idv2

> models::DomainPeriodUserGroupsResponseV1 get_user_groups_by_idv2(ids)
Get user groups by ID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | User group IDs to search for | [required] |

### Return type

[**models::DomainPeriodUserGroupsResponseV1**](domain.UserGroupsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_children

> models::MsaspecPeriodQueryResponse query_children(filter, sort, offset, limit)
Query for customers linked as children

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter using a query in Falcon Query Language (FQL). Supported filters: cid |  |
**sort** | Option<**String**> | The sort expression used to sort the results |  |[default to last_modified_timestamp|desc]
**offset** | Option<**i32**> | Starting index of overall result set from which to return ids |  |[default to 0]
**limit** | Option<**i32**> | Number of ids to return |  |[default to 10]

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_cid_group_members

> models::MsaPeriodQueryResponse query_cid_group_members(cid, sort, offset, limit)
Query a CID groups members by associated CID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | CID to lookup associated CID group ID | [required] |
**sort** | Option<**String**> | The sort expression used to sort the results |  |[default to last_modified_timestamp|desc]
**offset** | Option<**i32**> | Starting index of overall result set from which to return id |  |[default to 0]
**limit** | Option<**i32**> | Maximum number of results to return |  |[default to 10]

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_cid_groups

> models::MsaPeriodQueryResponse query_cid_groups(name, sort, offset, limit)
Query CID groups.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name to lookup groups for |  |
**sort** | Option<**String**> | The sort expression used to sort the results |  |[default to name|asc]
**offset** | Option<**i32**> | Starting index of overall result set from which to return ids |  |[default to 0]
**limit** | Option<**i32**> | Maximum number of results to return |  |[default to 10]

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_roles

> models::MsaPeriodQueryResponse query_roles(user_group_id, cid_group_id, role_id, sort, offset, limit)
Query links between user groups and CID groups. At least one of CID group ID or user group ID should also be provided. Role ID is optional.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | Option<**String**> | User group ID to fetch MSSP role for |  |
**cid_group_id** | Option<**String**> | CID group ID to fetch MSSP role for |  |
**role_id** | Option<**String**> | Role ID to fetch MSSP role for |  |
**sort** | Option<**String**> | The sort expression used to sort the results |  |[default to last_modified_timestamp|desc]
**offset** | Option<**i32**> | Starting index of overall result set from which to return ids |  |[default to 0]
**limit** | Option<**i32**> | Maximum number of results to return |  |[default to 10]

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_user_group_members

> models::MsaPeriodQueryResponse query_user_group_members(user_uuid, sort, offset, limit)
Query user group member by user UUID.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_uuid** | **String** | User UUID to lookup associated user group ID | [required] |
**sort** | Option<**String**> | The sort expression used to sort the results |  |[default to last_modified_timestamp|desc]
**offset** | Option<**i32**> | Starting index of overall result set from which to return ids |  |[default to 0]
**limit** | Option<**i32**> | Number of ids to return |  |[default to 10]

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_user_groups

> models::MsaPeriodQueryResponse query_user_groups(name, sort, offset, limit)
Query user groups.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name to lookup groups for |  |
**sort** | Option<**String**> | The sort expression used to sort the results |  |[default to name|asc]
**offset** | Option<**i32**> | Starting index of overall result set from which to return ids |  |[default to 0]
**limit** | Option<**i32**> | Maximum number of results to return |  |[default to 10]

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_cid_groups

> models::DomainPeriodCidGroupsResponseV1 update_cid_groups(body)
Update existing CID groups. CID group ID is expected for each CID group definition provided in request body. Name is a required field but description is an optional field. Empty description will override existing value. CID group member(s) remain unaffected.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodCidGroupsRequestV1**](DomainPeriodCidGroupsRequestV1.md) | 'cid_group_id' field is required to identify the CID group to update along with 'name' and/or 'description' fields to be updated. | [required] |

### Return type

[**models::DomainPeriodCidGroupsResponseV1**](domain.CIDGroupsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_user_groups

> models::DomainPeriodUserGroupsResponseV1 update_user_groups(body)
Update existing user group(s). User group ID is expected for each user group definition provided in request body. Name is a required field but description is an optional field. Empty description will override existing value. User group member(s) remain unaffected.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodUserGroupsRequestV1**](DomainPeriodUserGroupsRequestV1.md) | 'user_group_id' field is required to identify the user group to update along with 'name' and/or 'description' fields to be updated. | [required] |

### Return type

[**models::DomainPeriodUserGroupsResponseV1**](domain.UserGroupsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
