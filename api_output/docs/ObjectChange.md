# ObjectChange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**time** | **String** |  | [readonly]
**user** | [**crate::models::NestedUser**](NestedUser.md) |  | [readonly]
**user_name** | **String** |  | [readonly]
**request_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**action** | [**crate::models::ObjectChangeAction**](ObjectChange_action.md) |  | 
**changed_object_type** | **String** |  | [readonly]
**changed_object_id** | **i64** |  | 
**changed_object** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**prechange_data** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**postchange_data** | Option<[**serde_json::Value**](.md)> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


