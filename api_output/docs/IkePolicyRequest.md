# IkePolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**version** | **i32** | * `1` - IKEv1 * `2` - IKEv2 | 
**mode** | **String** | * `aggressive` - Aggressive * `main` - Main | 
**proposals** | Option<**Vec<i32>**> |  | [optional]
**preshared_key** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


