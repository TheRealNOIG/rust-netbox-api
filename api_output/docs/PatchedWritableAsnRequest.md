# PatchedWritableAsnRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asn** | Option<**i64**> | 16- or 32-bit autonomous system number | [optional]
**rir** | Option<**i32**> | Regional Internet Registry responsible for this AS number space | [optional]
**tenant** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


