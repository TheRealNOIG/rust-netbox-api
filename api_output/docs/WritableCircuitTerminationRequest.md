# WritableCircuitTerminationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**circuit** | **i32** |  | 
**term_side** | **String** | * `A` - A * `Z` - Z | 
**site** | Option<**i32**> |  | [optional]
**provider_network** | Option<**i32**> |  | [optional]
**port_speed** | Option<**i32**> | Physical circuit speed | [optional]
**upstream_speed** | Option<**i32**> | Upstream speed, if different from port speed | [optional]
**xconnect_id** | Option<**String**> | ID of the local cross-connect | [optional]
**pp_info** | Option<**String**> | Patch panel ID and port number(s) | [optional]
**description** | Option<**String**> |  | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


