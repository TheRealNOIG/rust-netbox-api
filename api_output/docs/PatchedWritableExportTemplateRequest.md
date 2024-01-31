# PatchedWritableExportTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_types** | Option<**Vec<String>**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**template_code** | Option<**String**> | Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>. | [optional]
**mime_type** | Option<**String**> | Defaults to <code>text/plain; charset=utf-8</code> | [optional]
**file_extension** | Option<**String**> | Extension to append to the rendered filename | [optional]
**as_attachment** | Option<**bool**> | Download file as attachment | [optional]
**data_source** | Option<**i32**> | Remote data source | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


