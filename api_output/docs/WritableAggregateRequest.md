# WritableAggregateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prefix** | **String** |  | 
**rir** | **i32** | Regional Internet Registry responsible for this IP space | 
**tenant** | Option<**i32**> |  | [optional]
**date_added** | Option<[**String**](string.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


