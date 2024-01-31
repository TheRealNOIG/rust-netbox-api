# WritableTunnelTerminationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tunnel** | **i32** |  | 
**role** | Option<**String**> | * `peer` - Peer * `hub` - Hub * `spoke` - Spoke | [optional]
**termination_type** | **String** |  | 
**termination_id** | Option<**i64**> |  | [optional]
**outside_ip** | Option<**i32**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


