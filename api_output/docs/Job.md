# Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**object_type** | **String** |  | [readonly]
**object_id** | Option<**i64**> |  | [optional]
**name** | **String** |  | 
**status** | [**crate::models::JobStatus**](Job_status.md) |  | 
**created** | **String** |  | [readonly]
**scheduled** | Option<**String**> |  | [optional]
**interval** | Option<**i32**> | Recurrence interval (in minutes) | [optional]
**started** | Option<**String**> |  | [optional]
**completed** | Option<**String**> |  | [optional]
**user** | [**crate::models::NestedUser**](NestedUser.md) |  | [readonly]
**data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**error** | **String** |  | [readonly]
**job_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


