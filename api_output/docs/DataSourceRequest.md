# DataSourceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**r#type** | **String** | * `None` - --------- * `local` - Local * `git` - Git * `amazon-s3` - Amazon S3 | 
**source_url** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**parameters** | Option<[**serde_json::Value**](.md)> |  | [optional]
**ignore_rules** | Option<**String**> | Patterns (one per line) matching files to ignore when syncing | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


