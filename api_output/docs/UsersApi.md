# \UsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_config_retrieve**](UsersApi.md#users_config_retrieve) | **GET** /api/users/config/ | 
[**users_groups_bulk_destroy**](UsersApi.md#users_groups_bulk_destroy) | **DELETE** /api/users/groups/ | 
[**users_groups_bulk_partial_update**](UsersApi.md#users_groups_bulk_partial_update) | **PATCH** /api/users/groups/ | 
[**users_groups_bulk_update**](UsersApi.md#users_groups_bulk_update) | **PUT** /api/users/groups/ | 
[**users_groups_create**](UsersApi.md#users_groups_create) | **POST** /api/users/groups/ | 
[**users_groups_destroy**](UsersApi.md#users_groups_destroy) | **DELETE** /api/users/groups/{id}/ | 
[**users_groups_list**](UsersApi.md#users_groups_list) | **GET** /api/users/groups/ | 
[**users_groups_partial_update**](UsersApi.md#users_groups_partial_update) | **PATCH** /api/users/groups/{id}/ | 
[**users_groups_retrieve**](UsersApi.md#users_groups_retrieve) | **GET** /api/users/groups/{id}/ | 
[**users_groups_update**](UsersApi.md#users_groups_update) | **PUT** /api/users/groups/{id}/ | 
[**users_permissions_bulk_destroy**](UsersApi.md#users_permissions_bulk_destroy) | **DELETE** /api/users/permissions/ | 
[**users_permissions_bulk_partial_update**](UsersApi.md#users_permissions_bulk_partial_update) | **PATCH** /api/users/permissions/ | 
[**users_permissions_bulk_update**](UsersApi.md#users_permissions_bulk_update) | **PUT** /api/users/permissions/ | 
[**users_permissions_create**](UsersApi.md#users_permissions_create) | **POST** /api/users/permissions/ | 
[**users_permissions_destroy**](UsersApi.md#users_permissions_destroy) | **DELETE** /api/users/permissions/{id}/ | 
[**users_permissions_list**](UsersApi.md#users_permissions_list) | **GET** /api/users/permissions/ | 
[**users_permissions_partial_update**](UsersApi.md#users_permissions_partial_update) | **PATCH** /api/users/permissions/{id}/ | 
[**users_permissions_retrieve**](UsersApi.md#users_permissions_retrieve) | **GET** /api/users/permissions/{id}/ | 
[**users_permissions_update**](UsersApi.md#users_permissions_update) | **PUT** /api/users/permissions/{id}/ | 
[**users_tokens_bulk_destroy**](UsersApi.md#users_tokens_bulk_destroy) | **DELETE** /api/users/tokens/ | 
[**users_tokens_bulk_partial_update**](UsersApi.md#users_tokens_bulk_partial_update) | **PATCH** /api/users/tokens/ | 
[**users_tokens_bulk_update**](UsersApi.md#users_tokens_bulk_update) | **PUT** /api/users/tokens/ | 
[**users_tokens_create**](UsersApi.md#users_tokens_create) | **POST** /api/users/tokens/ | 
[**users_tokens_destroy**](UsersApi.md#users_tokens_destroy) | **DELETE** /api/users/tokens/{id}/ | 
[**users_tokens_list**](UsersApi.md#users_tokens_list) | **GET** /api/users/tokens/ | 
[**users_tokens_partial_update**](UsersApi.md#users_tokens_partial_update) | **PATCH** /api/users/tokens/{id}/ | 
[**users_tokens_provision_create**](UsersApi.md#users_tokens_provision_create) | **POST** /api/users/tokens/provision/ | 
[**users_tokens_retrieve**](UsersApi.md#users_tokens_retrieve) | **GET** /api/users/tokens/{id}/ | 
[**users_tokens_update**](UsersApi.md#users_tokens_update) | **PUT** /api/users/tokens/{id}/ | 
[**users_users_bulk_destroy**](UsersApi.md#users_users_bulk_destroy) | **DELETE** /api/users/users/ | 
[**users_users_bulk_partial_update**](UsersApi.md#users_users_bulk_partial_update) | **PATCH** /api/users/users/ | 
[**users_users_bulk_update**](UsersApi.md#users_users_bulk_update) | **PUT** /api/users/users/ | 
[**users_users_create**](UsersApi.md#users_users_create) | **POST** /api/users/users/ | 
[**users_users_destroy**](UsersApi.md#users_users_destroy) | **DELETE** /api/users/users/{id}/ | 
[**users_users_list**](UsersApi.md#users_users_list) | **GET** /api/users/users/ | 
[**users_users_partial_update**](UsersApi.md#users_users_partial_update) | **PATCH** /api/users/users/{id}/ | 
[**users_users_retrieve**](UsersApi.md#users_users_retrieve) | **GET** /api/users/users/{id}/ | 
[**users_users_update**](UsersApi.md#users_users_update) | **PUT** /api/users/users/{id}/ | 



## users_config_retrieve

> ::std::collections::HashMap<String, serde_json::Value> users_config_retrieve()


An API endpoint via which a user can update his or her own UserConfig data (but no one else's).

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_bulk_destroy

> users_groups_bulk_destroy(group_request)


Delete a list of group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_request** | [**Vec<crate::models::GroupRequest>**](GroupRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_bulk_partial_update

> Vec<crate::models::Group> users_groups_bulk_partial_update(group_request)


Patch a list of group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_request** | [**Vec<crate::models::GroupRequest>**](GroupRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_bulk_update

> Vec<crate::models::Group> users_groups_bulk_update(group_request)


Put a list of group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_request** | [**Vec<crate::models::GroupRequest>**](GroupRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_create

> crate::models::Group users_groups_create(group_request)


Post a list of group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_request** | [**GroupRequest**](GroupRequest.md) |  | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_destroy

> users_groups_destroy(id)


Delete a group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_list

> crate::models::PaginatedGroupList users_groups_list(id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, limit, name, name__empty, name__ic, name__ie, name__iew, name__isw, name__n, name__nic, name__nie, name__niew, name__nisw, offset, ordering, q)


Get a list of group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**name** | Option<[**Vec<String>**](String.md)> |  |  |
**name__empty** | Option<**bool**> |  |  |
**name__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**name__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**name__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**name__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**name__n** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**name__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**q** | Option<**String**> | Search |  |

### Return type

[**crate::models::PaginatedGroupList**](PaginatedGroupList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_partial_update

> crate::models::Group users_groups_partial_update(id, patched_group_request)


Patch a group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group. | [required] |
**patched_group_request** | Option<[**PatchedGroupRequest**](PatchedGroupRequest.md)> |  |  |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_retrieve

> crate::models::Group users_groups_retrieve(id)


Get a group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group. | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_update

> crate::models::Group users_groups_update(id, group_request)


Put a group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group. | [required] |
**group_request** | [**GroupRequest**](GroupRequest.md) |  | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_bulk_destroy

> users_permissions_bulk_destroy(object_permission_request)


Delete a list of permission objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_permission_request** | [**Vec<crate::models::ObjectPermissionRequest>**](ObjectPermissionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_bulk_partial_update

> Vec<crate::models::ObjectPermission> users_permissions_bulk_partial_update(object_permission_request)


Patch a list of permission objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_permission_request** | [**Vec<crate::models::ObjectPermissionRequest>**](ObjectPermissionRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::ObjectPermission>**](ObjectPermission.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_bulk_update

> Vec<crate::models::ObjectPermission> users_permissions_bulk_update(object_permission_request)


Put a list of permission objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_permission_request** | [**Vec<crate::models::ObjectPermissionRequest>**](ObjectPermissionRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::ObjectPermission>**](ObjectPermission.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_create

> crate::models::ObjectPermission users_permissions_create(writable_object_permission_request)


Post a list of permission objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**writable_object_permission_request** | [**WritableObjectPermissionRequest**](WritableObjectPermissionRequest.md) |  | [required] |

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_destroy

> users_permissions_destroy(id)


Delete a permission object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this permission. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_list

> crate::models::PaginatedObjectPermissionList users_permissions_list(can_add, can_change, can_delete, can_view, description, description__empty, description__ic, description__ie, description__iew, description__isw, description__n, description__nic, description__nie, description__niew, description__nisw, enabled, group, group__n, group_id, group_id__n, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, limit, name, name__empty, name__ic, name__ie, name__iew, name__isw, name__n, name__nic, name__nie, name__niew, name__nisw, object_types, object_types__n, offset, ordering, q, user, user__n, user_id, user_id__n)


Get a list of permission objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**can_add** | Option<**bool**> |  |  |
**can_change** | Option<**bool**> |  |  |
**can_delete** | Option<**bool**> |  |  |
**can_view** | Option<**bool**> |  |  |
**description** | Option<[**Vec<String>**](String.md)> |  |  |
**description__empty** | Option<**bool**> |  |  |
**description__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**description__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**description__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**description__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**description__n** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**description__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**enabled** | Option<**bool**> |  |  |
**group** | Option<[**Vec<String>**](String.md)> | Group (name) |  |
**group__n** | Option<[**Vec<String>**](String.md)> | Group (name) |  |
**group_id** | Option<[**Vec<i32>**](i32.md)> | Group |  |
**group_id__n** | Option<[**Vec<i32>**](i32.md)> | Group |  |
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**name** | Option<[**Vec<String>**](String.md)> |  |  |
**name__empty** | Option<**bool**> |  |  |
**name__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**name__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**name__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**name__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**name__n** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**name__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**object_types** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_types__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**q** | Option<**String**> | Search |  |
**user** | Option<[**Vec<String>**](String.md)> | User (name) |  |
**user__n** | Option<[**Vec<String>**](String.md)> | User (name) |  |
**user_id** | Option<[**Vec<i32>**](i32.md)> | User |  |
**user_id__n** | Option<[**Vec<i32>**](i32.md)> | User |  |

### Return type

[**crate::models::PaginatedObjectPermissionList**](PaginatedObjectPermissionList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_partial_update

> crate::models::ObjectPermission users_permissions_partial_update(id, patched_writable_object_permission_request)


Patch a permission object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this permission. | [required] |
**patched_writable_object_permission_request** | Option<[**PatchedWritableObjectPermissionRequest**](PatchedWritableObjectPermissionRequest.md)> |  |  |

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_retrieve

> crate::models::ObjectPermission users_permissions_retrieve(id)


Get a permission object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this permission. | [required] |

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_update

> crate::models::ObjectPermission users_permissions_update(id, writable_object_permission_request)


Put a permission object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this permission. | [required] |
**writable_object_permission_request** | [**WritableObjectPermissionRequest**](WritableObjectPermissionRequest.md) |  | [required] |

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_bulk_destroy

> users_tokens_bulk_destroy(token_request)


Delete a list of token objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_request** | [**Vec<crate::models::TokenRequest>**](TokenRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_bulk_partial_update

> Vec<crate::models::Token> users_tokens_bulk_partial_update(token_request)


Patch a list of token objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_request** | [**Vec<crate::models::TokenRequest>**](TokenRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::Token>**](Token.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_bulk_update

> Vec<crate::models::Token> users_tokens_bulk_update(token_request)


Put a list of token objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_request** | [**Vec<crate::models::TokenRequest>**](TokenRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::Token>**](Token.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_create

> crate::models::Token users_tokens_create(writable_token_request)


Post a list of token objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**writable_token_request** | [**WritableTokenRequest**](WritableTokenRequest.md) |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_destroy

> users_tokens_destroy(id)


Delete a token object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this token. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_list

> crate::models::PaginatedTokenList users_tokens_list(created, created__gte, created__lte, description, description__empty, description__ic, description__ie, description__iew, description__isw, description__n, description__nic, description__nie, description__niew, description__nisw, expires, expires__gte, expires__lte, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, key, key__empty, key__ic, key__ie, key__iew, key__isw, key__n, key__nic, key__nie, key__niew, key__nisw, limit, offset, ordering, q, user, user__n, user_id, user_id__n, write_enabled)


Get a list of token objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created** | Option<**String**> |  |  |
**created__gte** | Option<**String**> |  |  |
**created__lte** | Option<**String**> |  |  |
**description** | Option<[**Vec<String>**](String.md)> |  |  |
**description__empty** | Option<**bool**> |  |  |
**description__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**description__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**description__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**description__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**description__n** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**description__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**expires** | Option<**String**> |  |  |
**expires__gte** | Option<**String**> |  |  |
**expires__lte** | Option<**String**> |  |  |
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**key** | Option<[**Vec<String>**](String.md)> |  |  |
**key__empty** | Option<**bool**> |  |  |
**key__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**key__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**key__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**key__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**key__n** | Option<[**Vec<String>**](String.md)> |  |  |
**key__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**key__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**key__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**key__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**q** | Option<**String**> | Search |  |
**user** | Option<[**Vec<String>**](String.md)> | User (name) |  |
**user__n** | Option<[**Vec<String>**](String.md)> | User (name) |  |
**user_id** | Option<[**Vec<i32>**](i32.md)> | User |  |
**user_id__n** | Option<[**Vec<i32>**](i32.md)> | User |  |
**write_enabled** | Option<**bool**> |  |  |

### Return type

[**crate::models::PaginatedTokenList**](PaginatedTokenList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_partial_update

> crate::models::Token users_tokens_partial_update(id, patched_writable_token_request)


Patch a token object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this token. | [required] |
**patched_writable_token_request** | Option<[**PatchedWritableTokenRequest**](PatchedWritableTokenRequest.md)> |  |  |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_provision_create

> crate::models::TokenProvision users_tokens_provision_create(token_provision_request)


Non-authenticated REST API endpoint via which a user may create a Token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_provision_request** | [**TokenProvisionRequest**](TokenProvisionRequest.md) |  | [required] |

### Return type

[**crate::models::TokenProvision**](TokenProvision.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_retrieve

> crate::models::Token users_tokens_retrieve(id)


Get a token object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this token. | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_update

> crate::models::Token users_tokens_update(id, writable_token_request)


Put a token object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this token. | [required] |
**writable_token_request** | [**WritableTokenRequest**](WritableTokenRequest.md) |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_bulk_destroy

> users_users_bulk_destroy(user_request)


Delete a list of user objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_request** | [**Vec<crate::models::UserRequest>**](UserRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_bulk_partial_update

> Vec<crate::models::User> users_users_bulk_partial_update(user_request)


Patch a list of user objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_request** | [**Vec<crate::models::UserRequest>**](UserRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_bulk_update

> Vec<crate::models::User> users_users_bulk_update(user_request)


Put a list of user objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_request** | [**Vec<crate::models::UserRequest>**](UserRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_create

> crate::models::User users_users_create(writable_user_request)


Post a list of user objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**writable_user_request** | [**WritableUserRequest**](WritableUserRequest.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_destroy

> users_users_destroy(id)


Delete a user object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this user. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_list

> crate::models::PaginatedUserList users_users_list(email, email__empty, email__ic, email__ie, email__iew, email__isw, email__n, email__nic, email__nie, email__niew, email__nisw, first_name, first_name__empty, first_name__ic, first_name__ie, first_name__iew, first_name__isw, first_name__n, first_name__nic, first_name__nie, first_name__niew, first_name__nisw, group, group__n, group_id, group_id__n, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, is_active, is_staff, is_superuser, last_name, last_name__empty, last_name__ic, last_name__ie, last_name__iew, last_name__isw, last_name__n, last_name__nic, last_name__nie, last_name__niew, last_name__nisw, limit, offset, ordering, q, username, username__empty, username__ic, username__ie, username__iew, username__isw, username__n, username__nic, username__nie, username__niew, username__nisw)


Get a list of user objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | Option<[**Vec<String>**](String.md)> |  |  |
**email__empty** | Option<**bool**> |  |  |
**email__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**email__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**email__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**email__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**email__n** | Option<[**Vec<String>**](String.md)> |  |  |
**email__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**email__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**email__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**email__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__empty** | Option<**bool**> |  |  |
**first_name__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__n** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**group** | Option<[**Vec<String>**](String.md)> | Group (name) |  |
**group__n** | Option<[**Vec<String>**](String.md)> | Group (name) |  |
**group_id** | Option<[**Vec<i32>**](i32.md)> | Group |  |
**group_id__n** | Option<[**Vec<i32>**](i32.md)> | Group |  |
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**is_active** | Option<**bool**> |  |  |
**is_staff** | Option<**bool**> |  |  |
**is_superuser** | Option<**bool**> |  |  |
**last_name** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__empty** | Option<**bool**> |  |  |
**last_name__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__n** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**q** | Option<**String**> | Search |  |
**username** | Option<[**Vec<String>**](String.md)> |  |  |
**username__empty** | Option<**bool**> |  |  |
**username__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**username__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**username__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**username__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**username__n** | Option<[**Vec<String>**](String.md)> |  |  |
**username__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**username__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**username__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**username__nisw** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**crate::models::PaginatedUserList**](PaginatedUserList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_partial_update

> crate::models::User users_users_partial_update(id, patched_writable_user_request)


Patch a user object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this user. | [required] |
**patched_writable_user_request** | Option<[**PatchedWritableUserRequest**](PatchedWritableUserRequest.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_retrieve

> crate::models::User users_users_retrieve(id)


Get a user object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this user. | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_update

> crate::models::User users_users_update(id, writable_user_request)


Put a user object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this user. | [required] |
**writable_user_request** | [**WritableUserRequest**](WritableUserRequest.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

