# ConfigTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**environment_params** | Option<[**serde_json::Value**](.md)> | Any <a href=\"https://jinja.palletsprojects.com/en/3.1.x/api/#jinja2.Environment\">additional parameters</a> to pass when constructing the Jinja2 environment. | [optional]
**template_code** | **String** | Jinja2 template code. | 
**data_source** | Option<[**crate::models::NestedDataSource**](NestedDataSource.md)> |  | [optional]
**data_path** | **String** | Path to remote file (relative to data source root) | [readonly]
**data_file** | Option<[**crate::models::NestedDataFile**](NestedDataFile.md)> |  | [optional]
**data_synced** | Option<**String**> |  | [readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


