# InventoryItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**device** | [**crate::models::NestedDevice**](NestedDevice.md) |  | 
**parent** | Option<**i32**> |  | [optional]
**name** | **String** |  | 
**label** | Option<**String**> | Physical label | [optional]
**role** | Option<[**crate::models::NestedInventoryItemRole**](NestedInventoryItemRole.md)> |  | [optional]
**manufacturer** | Option<[**crate::models::NestedManufacturer**](NestedManufacturer.md)> |  | [optional]
**part_id** | Option<**String**> | Manufacturer-assigned part identifier | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this item | [optional]
**discovered** | Option<**bool**> | This item was automatically discovered | [optional]
**description** | Option<**String**> |  | [optional]
**component_type** | Option<**String**> |  | [optional]
**component_id** | Option<**i64**> |  | [optional]
**component** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**_depth** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


