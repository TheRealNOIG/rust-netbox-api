# PatchedWritablePrefixRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prefix** | Option<**String**> |  | [optional]
**site** | Option<**i32**> |  | [optional]
**vrf** | Option<**i32**> |  | [optional]
**tenant** | Option<**i32**> |  | [optional]
**vlan** | Option<**i32**> |  | [optional]
**status** | Option<**String**> | Operational status of this prefix  * `container` - Container * `active` - Active * `reserved` - Reserved * `deprecated` - Deprecated | [optional]
**role** | Option<**i32**> | The primary function of this prefix | [optional]
**is_pool** | Option<**bool**> | All IP addresses within this prefix are considered usable | [optional]
**mark_utilized** | Option<**bool**> | Treat as 100% utilized | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


