# PatchedWritableEventRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_types** | Option<**Vec<String>**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**type_create** | Option<**bool**> | Triggers when a matching object is created. | [optional]
**type_update** | Option<**bool**> | Triggers when a matching object is updated. | [optional]
**type_delete** | Option<**bool**> | Triggers when a matching object is deleted. | [optional]
**type_job_start** | Option<**bool**> | Triggers when a job for a matching object is started. | [optional]
**type_job_end** | Option<**bool**> | Triggers when a job for a matching object terminates. | [optional]
**enabled** | Option<**bool**> |  | [optional]
**conditions** | Option<[**serde_json::Value**](.md)> | A set of conditions which determine whether the event will be generated. | [optional]
**action_type** | Option<**String**> | * `webhook` - Webhook * `script` - Script | [optional]
**action_object_type** | Option<**String**> |  | [optional]
**action_object_id** | Option<**i64**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


