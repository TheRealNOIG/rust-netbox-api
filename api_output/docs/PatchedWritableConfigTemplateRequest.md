# PatchedWritableConfigTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**environment_params** | Option<[**serde_json::Value**](.md)> | Any <a href=\"https://jinja.palletsprojects.com/en/3.1.x/api/#jinja2.Environment\">additional parameters</a> to pass when constructing the Jinja2 environment. | [optional]
**template_code** | Option<**String**> | Jinja2 template code. | [optional]
**data_source** | Option<**i32**> | Remote data source | [optional]
**data_file** | Option<**i32**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


