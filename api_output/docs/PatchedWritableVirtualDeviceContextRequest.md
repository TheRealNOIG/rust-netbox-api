# PatchedWritableVirtualDeviceContextRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**device** | Option<**i32**> |  | [optional]
**identifier** | Option<**i32**> | Numeric identifier unique to the parent device | [optional]
**tenant** | Option<**i32**> |  | [optional]
**primary_ip4** | Option<**i32**> |  | [optional]
**primary_ip6** | Option<**i32**> |  | [optional]
**status** | Option<**String**> | * `active` - Active * `planned` - Planned * `offline` - Offline | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


