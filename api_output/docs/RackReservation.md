# RackReservation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**rack** | [**crate::models::NestedRack**](NestedRack.md) |  | 
**units** | **Vec<i32>** |  | 
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**user** | [**crate::models::NestedUser**](NestedUser.md) |  | 
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**description** | **String** |  | 
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


