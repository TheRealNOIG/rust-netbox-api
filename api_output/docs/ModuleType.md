# ModuleType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**manufacturer** | [**crate::models::NestedManufacturer**](NestedManufacturer.md) |  | 
**model** | **String** |  | 
**part_number** | Option<**String**> | Discrete part number (optional) | [optional]
**weight** | Option<**f64**> |  | [optional]
**weight_unit** | Option<[**crate::models::DeviceTypeWeightUnit**](DeviceType_weight_unit.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


