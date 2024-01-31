# ConfigTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**environment_params** | Option<[**serde_json::Value**](.md)> | Any <a href=\"https://jinja.palletsprojects.com/en/3.1.x/api/#jinja2.Environment\">additional parameters</a> to pass when constructing the Jinja2 environment. | [optional]
**template_code** | **String** | Jinja2 template code. | 
**data_source** | Option<[**crate::models::NestedDataSourceRequest**](NestedDataSourceRequest.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


