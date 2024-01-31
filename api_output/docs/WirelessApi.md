# \WirelessApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**wireless_wireless_lan_groups_bulk_destroy**](WirelessApi.md#wireless_wireless_lan_groups_bulk_destroy) | **DELETE** /api/wireless/wireless-lan-groups/ | 
[**wireless_wireless_lan_groups_bulk_partial_update**](WirelessApi.md#wireless_wireless_lan_groups_bulk_partial_update) | **PATCH** /api/wireless/wireless-lan-groups/ | 
[**wireless_wireless_lan_groups_bulk_update**](WirelessApi.md#wireless_wireless_lan_groups_bulk_update) | **PUT** /api/wireless/wireless-lan-groups/ | 
[**wireless_wireless_lan_groups_create**](WirelessApi.md#wireless_wireless_lan_groups_create) | **POST** /api/wireless/wireless-lan-groups/ | 
[**wireless_wireless_lan_groups_destroy**](WirelessApi.md#wireless_wireless_lan_groups_destroy) | **DELETE** /api/wireless/wireless-lan-groups/{id}/ | 
[**wireless_wireless_lan_groups_list**](WirelessApi.md#wireless_wireless_lan_groups_list) | **GET** /api/wireless/wireless-lan-groups/ | 
[**wireless_wireless_lan_groups_partial_update**](WirelessApi.md#wireless_wireless_lan_groups_partial_update) | **PATCH** /api/wireless/wireless-lan-groups/{id}/ | 
[**wireless_wireless_lan_groups_retrieve**](WirelessApi.md#wireless_wireless_lan_groups_retrieve) | **GET** /api/wireless/wireless-lan-groups/{id}/ | 
[**wireless_wireless_lan_groups_update**](WirelessApi.md#wireless_wireless_lan_groups_update) | **PUT** /api/wireless/wireless-lan-groups/{id}/ | 
[**wireless_wireless_lans_bulk_destroy**](WirelessApi.md#wireless_wireless_lans_bulk_destroy) | **DELETE** /api/wireless/wireless-lans/ | 
[**wireless_wireless_lans_bulk_partial_update**](WirelessApi.md#wireless_wireless_lans_bulk_partial_update) | **PATCH** /api/wireless/wireless-lans/ | 
[**wireless_wireless_lans_bulk_update**](WirelessApi.md#wireless_wireless_lans_bulk_update) | **PUT** /api/wireless/wireless-lans/ | 
[**wireless_wireless_lans_create**](WirelessApi.md#wireless_wireless_lans_create) | **POST** /api/wireless/wireless-lans/ | 
[**wireless_wireless_lans_destroy**](WirelessApi.md#wireless_wireless_lans_destroy) | **DELETE** /api/wireless/wireless-lans/{id}/ | 
[**wireless_wireless_lans_list**](WirelessApi.md#wireless_wireless_lans_list) | **GET** /api/wireless/wireless-lans/ | 
[**wireless_wireless_lans_partial_update**](WirelessApi.md#wireless_wireless_lans_partial_update) | **PATCH** /api/wireless/wireless-lans/{id}/ | 
[**wireless_wireless_lans_retrieve**](WirelessApi.md#wireless_wireless_lans_retrieve) | **GET** /api/wireless/wireless-lans/{id}/ | 
[**wireless_wireless_lans_update**](WirelessApi.md#wireless_wireless_lans_update) | **PUT** /api/wireless/wireless-lans/{id}/ | 
[**wireless_wireless_links_bulk_destroy**](WirelessApi.md#wireless_wireless_links_bulk_destroy) | **DELETE** /api/wireless/wireless-links/ | 
[**wireless_wireless_links_bulk_partial_update**](WirelessApi.md#wireless_wireless_links_bulk_partial_update) | **PATCH** /api/wireless/wireless-links/ | 
[**wireless_wireless_links_bulk_update**](WirelessApi.md#wireless_wireless_links_bulk_update) | **PUT** /api/wireless/wireless-links/ | 
[**wireless_wireless_links_create**](WirelessApi.md#wireless_wireless_links_create) | **POST** /api/wireless/wireless-links/ | 
[**wireless_wireless_links_destroy**](WirelessApi.md#wireless_wireless_links_destroy) | **DELETE** /api/wireless/wireless-links/{id}/ | 
[**wireless_wireless_links_list**](WirelessApi.md#wireless_wireless_links_list) | **GET** /api/wireless/wireless-links/ | 
[**wireless_wireless_links_partial_update**](WirelessApi.md#wireless_wireless_links_partial_update) | **PATCH** /api/wireless/wireless-links/{id}/ | 
[**wireless_wireless_links_retrieve**](WirelessApi.md#wireless_wireless_links_retrieve) | **GET** /api/wireless/wireless-links/{id}/ | 
[**wireless_wireless_links_update**](WirelessApi.md#wireless_wireless_links_update) | **PUT** /api/wireless/wireless-links/{id}/ | 



## wireless_wireless_lan_groups_bulk_destroy

> wireless_wireless_lan_groups_bulk_destroy(wireless_lan_group_request)


Delete a list of wireless LAN group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wireless_lan_group_request** | [**Vec<crate::models::WirelessLanGroupRequest>**](WirelessLANGroupRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_bulk_partial_update

> Vec<crate::models::WirelessLanGroup> wireless_wireless_lan_groups_bulk_partial_update(wireless_lan_group_request)


Patch a list of wireless LAN group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wireless_lan_group_request** | [**Vec<crate::models::WirelessLanGroupRequest>**](WirelessLANGroupRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::WirelessLanGroup>**](WirelessLANGroup.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_bulk_update

> Vec<crate::models::WirelessLanGroup> wireless_wireless_lan_groups_bulk_update(wireless_lan_group_request)


Put a list of wireless LAN group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wireless_lan_group_request** | [**Vec<crate::models::WirelessLanGroupRequest>**](WirelessLANGroupRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::WirelessLanGroup>**](WirelessLANGroup.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_create

> crate::models::WirelessLanGroup wireless_wireless_lan_groups_create(writable_wireless_lan_group_request)


Post a list of wireless LAN group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**writable_wireless_lan_group_request** | [**WritableWirelessLanGroupRequest**](WritableWirelessLanGroupRequest.md) |  | [required] |

### Return type

[**crate::models::WirelessLanGroup**](WirelessLANGroup.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_destroy

> wireless_wireless_lan_groups_destroy(id)


Delete a wireless LAN group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless LAN group. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_list

> crate::models::PaginatedWirelessLanGroupList wireless_wireless_lan_groups_list(created, created__empty, created__gt, created__gte, created__lt, created__lte, created__n, created_by_request, description, description__empty, description__ic, description__ie, description__iew, description__isw, description__n, description__nic, description__nie, description__niew, description__nisw, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, last_updated, last_updated__empty, last_updated__gt, last_updated__gte, last_updated__lt, last_updated__lte, last_updated__n, limit, modified_by_request, name, name__empty, name__ic, name__ie, name__iew, name__isw, name__n, name__nic, name__nie, name__niew, name__nisw, offset, ordering, parent, parent__n, parent_id, parent_id__n, q, slug, slug__empty, slug__ic, slug__ie, slug__iew, slug__isw, slug__n, slug__nic, slug__nie, slug__niew, slug__nisw, tag, tag__n, updated_by_request)


Get a list of wireless LAN group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created** | Option<[**Vec<String>**](String.md)> |  |  |
**created__empty** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__n** | Option<[**Vec<String>**](String.md)> |  |  |
**created_by_request** | Option<**uuid::Uuid**> |  |  |
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
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**last_updated** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__empty** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__n** | Option<[**Vec<String>**](String.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**modified_by_request** | Option<**uuid::Uuid**> |  |  |
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
**parent** | Option<[**Vec<String>**](String.md)> |  |  |
**parent__n** | Option<[**Vec<String>**](String.md)> |  |  |
**parent_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**parent_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**q** | Option<**String**> | Search |  |
**slug** | Option<[**Vec<String>**](String.md)> |  |  |
**slug__empty** | Option<**bool**> |  |  |
**slug__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**slug__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**slug__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**slug__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**slug__n** | Option<[**Vec<String>**](String.md)> |  |  |
**slug__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**slug__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**slug__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**slug__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**tag__n** | Option<[**Vec<String>**](String.md)> |  |  |
**updated_by_request** | Option<**uuid::Uuid**> |  |  |

### Return type

[**crate::models::PaginatedWirelessLanGroupList**](PaginatedWirelessLANGroupList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_partial_update

> crate::models::WirelessLanGroup wireless_wireless_lan_groups_partial_update(id, patched_writable_wireless_lan_group_request)


Patch a wireless LAN group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless LAN group. | [required] |
**patched_writable_wireless_lan_group_request** | Option<[**PatchedWritableWirelessLanGroupRequest**](PatchedWritableWirelessLanGroupRequest.md)> |  |  |

### Return type

[**crate::models::WirelessLanGroup**](WirelessLANGroup.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_retrieve

> crate::models::WirelessLanGroup wireless_wireless_lan_groups_retrieve(id)


Get a wireless LAN group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless LAN group. | [required] |

### Return type

[**crate::models::WirelessLanGroup**](WirelessLANGroup.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_update

> crate::models::WirelessLanGroup wireless_wireless_lan_groups_update(id, writable_wireless_lan_group_request)


Put a wireless LAN group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless LAN group. | [required] |
**writable_wireless_lan_group_request** | [**WritableWirelessLanGroupRequest**](WritableWirelessLanGroupRequest.md) |  | [required] |

### Return type

[**crate::models::WirelessLanGroup**](WirelessLANGroup.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_bulk_destroy

> wireless_wireless_lans_bulk_destroy(wireless_lan_request)


Delete a list of wireless LAN objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wireless_lan_request** | [**Vec<crate::models::WirelessLanRequest>**](WirelessLANRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_bulk_partial_update

> Vec<crate::models::WirelessLan> wireless_wireless_lans_bulk_partial_update(wireless_lan_request)


Patch a list of wireless LAN objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wireless_lan_request** | [**Vec<crate::models::WirelessLanRequest>**](WirelessLANRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::WirelessLan>**](WirelessLAN.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_bulk_update

> Vec<crate::models::WirelessLan> wireless_wireless_lans_bulk_update(wireless_lan_request)


Put a list of wireless LAN objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wireless_lan_request** | [**Vec<crate::models::WirelessLanRequest>**](WirelessLANRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::WirelessLan>**](WirelessLAN.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_create

> crate::models::WirelessLan wireless_wireless_lans_create(writable_wireless_lan_request)


Post a list of wireless LAN objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**writable_wireless_lan_request** | [**WritableWirelessLanRequest**](WritableWirelessLanRequest.md) |  | [required] |

### Return type

[**crate::models::WirelessLan**](WirelessLAN.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_destroy

> wireless_wireless_lans_destroy(id)


Delete a wireless LAN object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless LAN. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_list

> crate::models::PaginatedWirelessLanList wireless_wireless_lans_list(auth_cipher, auth_cipher__n, auth_psk, auth_psk__empty, auth_psk__ic, auth_psk__ie, auth_psk__iew, auth_psk__isw, auth_psk__n, auth_psk__nic, auth_psk__nie, auth_psk__niew, auth_psk__nisw, auth_type, auth_type__n, created, created__empty, created__gt, created__gte, created__lt, created__lte, created__n, created_by_request, description, description__empty, description__ic, description__ie, description__iew, description__isw, description__n, description__nic, description__nie, description__niew, description__nisw, group, group__n, group_id, group_id__n, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, last_updated, last_updated__empty, last_updated__gt, last_updated__gte, last_updated__lt, last_updated__lte, last_updated__n, limit, modified_by_request, offset, ordering, q, ssid, ssid__empty, ssid__ic, ssid__ie, ssid__iew, ssid__isw, ssid__n, ssid__nic, ssid__nie, ssid__niew, ssid__nisw, status, status__n, tag, tag__n, tenant, tenant__n, tenant_group, tenant_group__n, tenant_group_id, tenant_group_id__n, tenant_id, tenant_id__n, updated_by_request, vlan_id, vlan_id__n)


Get a list of wireless LAN objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_cipher** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_cipher__n** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__empty** | Option<**bool**> |  |  |
**auth_psk__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__n** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_type** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_type__n** | Option<[**Vec<String>**](String.md)> |  |  |
**created** | Option<[**Vec<String>**](String.md)> |  |  |
**created__empty** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__n** | Option<[**Vec<String>**](String.md)> |  |  |
**created_by_request** | Option<**uuid::Uuid**> |  |  |
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
**group** | Option<[**Vec<i32>**](i32.md)> |  |  |
**group__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**group_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**group_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**last_updated** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__empty** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__n** | Option<[**Vec<String>**](String.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**modified_by_request** | Option<**uuid::Uuid**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**q** | Option<**String**> | Search |  |
**ssid** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__empty** | Option<**bool**> |  |  |
**ssid__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__n** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**status** | Option<[**Vec<String>**](String.md)> |  |  |
**status__n** | Option<[**Vec<String>**](String.md)> |  |  |
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**tag__n** | Option<[**Vec<String>**](String.md)> |  |  |
**tenant** | Option<[**Vec<String>**](String.md)> | Tenant (slug) |  |
**tenant__n** | Option<[**Vec<String>**](String.md)> | Tenant (slug) |  |
**tenant_group** | Option<[**Vec<i32>**](i32.md)> | Tenant Group (slug) |  |
**tenant_group__n** | Option<[**Vec<i32>**](i32.md)> | Tenant Group (slug) |  |
**tenant_group_id** | Option<[**Vec<i32>**](i32.md)> | Tenant Group (ID) |  |
**tenant_group_id__n** | Option<[**Vec<i32>**](i32.md)> | Tenant Group (ID) |  |
**tenant_id** | Option<[**Vec<i32>**](i32.md)> | Tenant (ID) |  |
**tenant_id__n** | Option<[**Vec<i32>**](i32.md)> | Tenant (ID) |  |
**updated_by_request** | Option<**uuid::Uuid**> |  |  |
**vlan_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**vlan_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**crate::models::PaginatedWirelessLanList**](PaginatedWirelessLANList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_partial_update

> crate::models::WirelessLan wireless_wireless_lans_partial_update(id, patched_writable_wireless_lan_request)


Patch a wireless LAN object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless LAN. | [required] |
**patched_writable_wireless_lan_request** | Option<[**PatchedWritableWirelessLanRequest**](PatchedWritableWirelessLanRequest.md)> |  |  |

### Return type

[**crate::models::WirelessLan**](WirelessLAN.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_retrieve

> crate::models::WirelessLan wireless_wireless_lans_retrieve(id)


Get a wireless LAN object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless LAN. | [required] |

### Return type

[**crate::models::WirelessLan**](WirelessLAN.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_update

> crate::models::WirelessLan wireless_wireless_lans_update(id, writable_wireless_lan_request)


Put a wireless LAN object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless LAN. | [required] |
**writable_wireless_lan_request** | [**WritableWirelessLanRequest**](WritableWirelessLanRequest.md) |  | [required] |

### Return type

[**crate::models::WirelessLan**](WirelessLAN.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_bulk_destroy

> wireless_wireless_links_bulk_destroy(wireless_link_request)


Delete a list of wireless link objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wireless_link_request** | [**Vec<crate::models::WirelessLinkRequest>**](WirelessLinkRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_bulk_partial_update

> Vec<crate::models::WirelessLink> wireless_wireless_links_bulk_partial_update(wireless_link_request)


Patch a list of wireless link objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wireless_link_request** | [**Vec<crate::models::WirelessLinkRequest>**](WirelessLinkRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::WirelessLink>**](WirelessLink.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_bulk_update

> Vec<crate::models::WirelessLink> wireless_wireless_links_bulk_update(wireless_link_request)


Put a list of wireless link objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wireless_link_request** | [**Vec<crate::models::WirelessLinkRequest>**](WirelessLinkRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::WirelessLink>**](WirelessLink.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_create

> crate::models::WirelessLink wireless_wireless_links_create(writable_wireless_link_request)


Post a list of wireless link objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**writable_wireless_link_request** | [**WritableWirelessLinkRequest**](WritableWirelessLinkRequest.md) |  | [required] |

### Return type

[**crate::models::WirelessLink**](WirelessLink.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_destroy

> wireless_wireless_links_destroy(id)


Delete a wireless link object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless link. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_list

> crate::models::PaginatedWirelessLinkList wireless_wireless_links_list(auth_cipher, auth_cipher__n, auth_psk, auth_psk__empty, auth_psk__ic, auth_psk__ie, auth_psk__iew, auth_psk__isw, auth_psk__n, auth_psk__nic, auth_psk__nie, auth_psk__niew, auth_psk__nisw, auth_type, auth_type__n, created, created__empty, created__gt, created__gte, created__lt, created__lte, created__n, created_by_request, description, description__empty, description__ic, description__ie, description__iew, description__isw, description__n, description__nic, description__nie, description__niew, description__nisw, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, interface_a_id, interface_a_id__empty, interface_a_id__gt, interface_a_id__gte, interface_a_id__lt, interface_a_id__lte, interface_a_id__n, interface_b_id, interface_b_id__empty, interface_b_id__gt, interface_b_id__gte, interface_b_id__lt, interface_b_id__lte, interface_b_id__n, last_updated, last_updated__empty, last_updated__gt, last_updated__gte, last_updated__lt, last_updated__lte, last_updated__n, limit, modified_by_request, offset, ordering, q, ssid, ssid__empty, ssid__ic, ssid__ie, ssid__iew, ssid__isw, ssid__n, ssid__nic, ssid__nie, ssid__niew, ssid__nisw, status, status__n, tag, tag__n, tenant, tenant__n, tenant_group, tenant_group__n, tenant_group_id, tenant_group_id__n, tenant_id, tenant_id__n, updated_by_request)


Get a list of wireless link objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_cipher** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_cipher__n** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__empty** | Option<**bool**> |  |  |
**auth_psk__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__n** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_psk__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_type** | Option<[**Vec<String>**](String.md)> |  |  |
**auth_type__n** | Option<[**Vec<String>**](String.md)> |  |  |
**created** | Option<[**Vec<String>**](String.md)> |  |  |
**created__empty** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__n** | Option<[**Vec<String>**](String.md)> |  |  |
**created_by_request** | Option<**uuid::Uuid**> |  |  |
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
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_a_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_a_id__empty** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_a_id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_a_id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_a_id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_a_id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_a_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_b_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_b_id__empty** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_b_id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_b_id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_b_id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_b_id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interface_b_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**last_updated** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__empty** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__n** | Option<[**Vec<String>**](String.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**modified_by_request** | Option<**uuid::Uuid**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**q** | Option<**String**> | Search |  |
**ssid** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__empty** | Option<**bool**> |  |  |
**ssid__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__n** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**ssid__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**status** | Option<[**Vec<String>**](String.md)> |  |  |
**status__n** | Option<[**Vec<String>**](String.md)> |  |  |
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**tag__n** | Option<[**Vec<String>**](String.md)> |  |  |
**tenant** | Option<[**Vec<String>**](String.md)> | Tenant (slug) |  |
**tenant__n** | Option<[**Vec<String>**](String.md)> | Tenant (slug) |  |
**tenant_group** | Option<[**Vec<i32>**](i32.md)> | Tenant Group (slug) |  |
**tenant_group__n** | Option<[**Vec<i32>**](i32.md)> | Tenant Group (slug) |  |
**tenant_group_id** | Option<[**Vec<i32>**](i32.md)> | Tenant Group (ID) |  |
**tenant_group_id__n** | Option<[**Vec<i32>**](i32.md)> | Tenant Group (ID) |  |
**tenant_id** | Option<[**Vec<i32>**](i32.md)> | Tenant (ID) |  |
**tenant_id__n** | Option<[**Vec<i32>**](i32.md)> | Tenant (ID) |  |
**updated_by_request** | Option<**uuid::Uuid**> |  |  |

### Return type

[**crate::models::PaginatedWirelessLinkList**](PaginatedWirelessLinkList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_partial_update

> crate::models::WirelessLink wireless_wireless_links_partial_update(id, patched_writable_wireless_link_request)


Patch a wireless link object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless link. | [required] |
**patched_writable_wireless_link_request** | Option<[**PatchedWritableWirelessLinkRequest**](PatchedWritableWirelessLinkRequest.md)> |  |  |

### Return type

[**crate::models::WirelessLink**](WirelessLink.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_retrieve

> crate::models::WirelessLink wireless_wireless_links_retrieve(id)


Get a wireless link object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless link. | [required] |

### Return type

[**crate::models::WirelessLink**](WirelessLink.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_update

> crate::models::WirelessLink wireless_wireless_links_update(id, writable_wireless_link_request)


Put a wireless link object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless link. | [required] |
**writable_wireless_link_request** | [**WritableWirelessLinkRequest**](WritableWirelessLinkRequest.md) |  | [required] |

### Return type

[**crate::models::WirelessLink**](WirelessLink.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

