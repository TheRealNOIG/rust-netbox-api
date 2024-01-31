# ExportTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_types** | **Vec<String>** |  | 
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**template_code** | **String** | Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>. | 
**mime_type** | Option<**String**> | Defaults to <code>text/plain; charset=utf-8</code> | [optional]
**file_extension** | Option<**String**> | Extension to append to the rendered filename | [optional]
**as_attachment** | Option<**bool**> | Download file as attachment | [optional]
**data_source** | Option<[**crate::models::NestedDataSourceRequest**](NestedDataSourceRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


