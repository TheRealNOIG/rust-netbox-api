# WritableModuleTypeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**manufacturer** | **i32** |  | 
**model** | **String** |  | 
**part_number** | Option<**String**> | Discrete part number (optional) | [optional]
**weight** | Option<**f64**> |  | [optional]
**weight_unit** | Option<**String**> | * `kg` - Kilograms * `g` - Grams * `lb` - Pounds * `oz` - Ounces | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


