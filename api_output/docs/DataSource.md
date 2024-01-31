# DataSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**r#type** | [**crate::models::DataSourceType**](DataSource_type.md) |  | 
**source_url** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**status** | [**crate::models::DataSourceStatus**](DataSource_status.md) |  | 
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**parameters** | Option<[**serde_json::Value**](.md)> |  | [optional]
**ignore_rules** | Option<**String**> | Patterns (one per line) matching files to ignore when syncing | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**file_count** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


