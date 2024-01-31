# \CoreApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**core_data_files_list**](CoreApi.md#core_data_files_list) | **GET** /api/core/data-files/ | 
[**core_data_files_retrieve**](CoreApi.md#core_data_files_retrieve) | **GET** /api/core/data-files/{id}/ | 
[**core_data_sources_bulk_destroy**](CoreApi.md#core_data_sources_bulk_destroy) | **DELETE** /api/core/data-sources/ | 
[**core_data_sources_bulk_partial_update**](CoreApi.md#core_data_sources_bulk_partial_update) | **PATCH** /api/core/data-sources/ | 
[**core_data_sources_bulk_update**](CoreApi.md#core_data_sources_bulk_update) | **PUT** /api/core/data-sources/ | 
[**core_data_sources_create**](CoreApi.md#core_data_sources_create) | **POST** /api/core/data-sources/ | 
[**core_data_sources_destroy**](CoreApi.md#core_data_sources_destroy) | **DELETE** /api/core/data-sources/{id}/ | 
[**core_data_sources_list**](CoreApi.md#core_data_sources_list) | **GET** /api/core/data-sources/ | 
[**core_data_sources_partial_update**](CoreApi.md#core_data_sources_partial_update) | **PATCH** /api/core/data-sources/{id}/ | 
[**core_data_sources_retrieve**](CoreApi.md#core_data_sources_retrieve) | **GET** /api/core/data-sources/{id}/ | 
[**core_data_sources_sync_create**](CoreApi.md#core_data_sources_sync_create) | **POST** /api/core/data-sources/{id}/sync/ | 
[**core_data_sources_update**](CoreApi.md#core_data_sources_update) | **PUT** /api/core/data-sources/{id}/ | 
[**core_jobs_list**](CoreApi.md#core_jobs_list) | **GET** /api/core/jobs/ | 
[**core_jobs_retrieve**](CoreApi.md#core_jobs_retrieve) | **GET** /api/core/jobs/{id}/ | 



## core_data_files_list

> crate::models::PaginatedDataFileList core_data_files_list(created, created__empty, created__gt, created__gte, created__lt, created__lte, created__n, created_by_request, hash, hash__empty, hash__ic, hash__ie, hash__iew, hash__isw, hash__n, hash__nic, hash__nie, hash__niew, hash__nisw, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, last_updated, last_updated__empty, last_updated__gt, last_updated__gte, last_updated__lt, last_updated__lte, last_updated__n, limit, modified_by_request, offset, ordering, path, path__empty, path__ic, path__ie, path__iew, path__isw, path__n, path__nic, path__nie, path__niew, path__nisw, q, size, size__empty, size__gt, size__gte, size__lt, size__lte, size__n, source, source__n, source_id, source_id__n, updated_by_request)


Get a list of data file objects.

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
**hash** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__empty** | Option<**bool**> |  |  |
**hash__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__n** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
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
**path** | Option<[**Vec<String>**](String.md)> |  |  |
**path__empty** | Option<**bool**> |  |  |
**path__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**path__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**path__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**path__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**path__n** | Option<[**Vec<String>**](String.md)> |  |  |
**path__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**path__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**path__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**path__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**q** | Option<**String**> |  |  |
**size** | Option<[**Vec<i32>**](i32.md)> |  |  |
**size__empty** | Option<**bool**> |  |  |
**size__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**size__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**size__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**size__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**size__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**source** | Option<[**Vec<String>**](String.md)> | Data source (name) |  |
**source__n** | Option<[**Vec<String>**](String.md)> | Data source (name) |  |
**source_id** | Option<[**Vec<i32>**](i32.md)> | Data source (ID) |  |
**source_id__n** | Option<[**Vec<i32>**](i32.md)> | Data source (ID) |  |
**updated_by_request** | Option<**uuid::Uuid**> |  |  |

### Return type

[**crate::models::PaginatedDataFileList**](PaginatedDataFileList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_files_retrieve

> crate::models::DataFile core_data_files_retrieve(id)


Get a data file object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this data file. | [required] |

### Return type

[**crate::models::DataFile**](DataFile.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_bulk_destroy

> core_data_sources_bulk_destroy(data_source_request)


Delete a list of data source objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_source_request** | [**Vec<crate::models::DataSourceRequest>**](DataSourceRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_bulk_partial_update

> Vec<crate::models::DataSource> core_data_sources_bulk_partial_update(data_source_request)


Patch a list of data source objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_source_request** | [**Vec<crate::models::DataSourceRequest>**](DataSourceRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::DataSource>**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_bulk_update

> Vec<crate::models::DataSource> core_data_sources_bulk_update(data_source_request)


Put a list of data source objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_source_request** | [**Vec<crate::models::DataSourceRequest>**](DataSourceRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::DataSource>**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_create

> crate::models::DataSource core_data_sources_create(writable_data_source_request)


Post a list of data source objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**writable_data_source_request** | [**WritableDataSourceRequest**](WritableDataSourceRequest.md) |  | [required] |

### Return type

[**crate::models::DataSource**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_destroy

> core_data_sources_destroy(id)


Delete a data source object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this data source. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_list

> crate::models::PaginatedDataSourceList core_data_sources_list(created, created__empty, created__gt, created__gte, created__lt, created__lte, created__n, created_by_request, description, description__empty, description__ic, description__ie, description__iew, description__isw, description__n, description__nic, description__nie, description__niew, description__nisw, enabled, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, last_updated, last_updated__empty, last_updated__gt, last_updated__gte, last_updated__lt, last_updated__lte, last_updated__n, limit, modified_by_request, name, name__empty, name__ic, name__ie, name__iew, name__isw, name__n, name__nic, name__nie, name__niew, name__nisw, offset, ordering, q, status, status__n, tag, tag__n, r#type, type__n, updated_by_request)


Get a list of data source objects.

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
**enabled** | Option<**bool**> |  |  |
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
**q** | Option<**String**> | Search |  |
**status** | Option<[**Vec<String>**](String.md)> |  |  |
**status__n** | Option<[**Vec<String>**](String.md)> |  |  |
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**tag__n** | Option<[**Vec<String>**](String.md)> |  |  |
**r#type** | Option<[**Vec<String>**](String.md)> |  |  |
**type__n** | Option<[**Vec<String>**](String.md)> |  |  |
**updated_by_request** | Option<**uuid::Uuid**> |  |  |

### Return type

[**crate::models::PaginatedDataSourceList**](PaginatedDataSourceList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_partial_update

> crate::models::DataSource core_data_sources_partial_update(id, patched_writable_data_source_request)


Patch a data source object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this data source. | [required] |
**patched_writable_data_source_request** | Option<[**PatchedWritableDataSourceRequest**](PatchedWritableDataSourceRequest.md)> |  |  |

### Return type

[**crate::models::DataSource**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_retrieve

> crate::models::DataSource core_data_sources_retrieve(id)


Get a data source object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this data source. | [required] |

### Return type

[**crate::models::DataSource**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_sync_create

> crate::models::DataSource core_data_sources_sync_create(id, writable_data_source_request)


Enqueue a job to synchronize the DataSource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this data source. | [required] |
**writable_data_source_request** | [**WritableDataSourceRequest**](WritableDataSourceRequest.md) |  | [required] |

### Return type

[**crate::models::DataSource**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_update

> crate::models::DataSource core_data_sources_update(id, writable_data_source_request)


Put a data source object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this data source. | [required] |
**writable_data_source_request** | [**WritableDataSourceRequest**](WritableDataSourceRequest.md) |  | [required] |

### Return type

[**crate::models::DataSource**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_jobs_list

> crate::models::PaginatedJobList core_jobs_list(completed, completed__after, completed__before, created, created__after, created__before, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, interval, interval__empty, interval__gt, interval__gte, interval__lt, interval__lte, interval__n, limit, name, name__empty, name__ic, name__ie, name__iew, name__isw, name__n, name__nic, name__nie, name__niew, name__nisw, object_id, object_id__empty, object_id__gt, object_id__gte, object_id__lt, object_id__lte, object_id__n, object_type, object_type__n, offset, ordering, q, scheduled, scheduled__after, scheduled__before, started, started__after, started__before, status, status__n, user, user__n)


Retrieve a list of job results

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**completed** | Option<**String**> |  |  |
**completed__after** | Option<**String**> |  |  |
**completed__before** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**created__after** | Option<**String**> |  |  |
**created__before** | Option<**String**> |  |  |
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interval** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interval__empty** | Option<**bool**> |  |  |
**interval__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interval__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interval__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interval__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interval__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
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
**object_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__empty** | Option<**bool**> |  |  |
**object_id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_type** | Option<**i32**> |  |  |
**object_type__n** | Option<**i32**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**q** | Option<**String**> | Search |  |
**scheduled** | Option<**String**> |  |  |
**scheduled__after** | Option<**String**> |  |  |
**scheduled__before** | Option<**String**> |  |  |
**started** | Option<**String**> |  |  |
**started__after** | Option<**String**> |  |  |
**started__before** | Option<**String**> |  |  |
**status** | Option<[**Vec<String>**](String.md)> |  |  |
**status__n** | Option<[**Vec<String>**](String.md)> |  |  |
**user** | Option<**i32**> |  |  |
**user__n** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedJobList**](PaginatedJobList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_jobs_retrieve

> crate::models::Job core_jobs_retrieve(id)


Retrieve a list of job results

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this job. | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

