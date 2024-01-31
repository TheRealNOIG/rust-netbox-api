# ExportTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**content_types** | **Vec<String>** |  | 
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**template_code** | **String** | Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>. | 
**mime_type** | Option<**String**> | Defaults to <code>text/plain; charset=utf-8</code> | [optional]
**file_extension** | Option<**String**> | Extension to append to the rendered filename | [optional]
**as_attachment** | Option<**bool**> | Download file as attachment | [optional]
**data_source** | Option<[**crate::models::NestedDataSource**](NestedDataSource.md)> |  | [optional]
**data_path** | **String** | Path to remote file (relative to data source root) | [readonly]
**data_file** | [**crate::models::NestedDataFile**](NestedDataFile.md) |  | [readonly]
**data_synced** | Option<**String**> |  | [readonly]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


