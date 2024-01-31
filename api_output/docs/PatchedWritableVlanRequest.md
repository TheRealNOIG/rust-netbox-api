# PatchedWritableVlanRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**site** | Option<**i32**> | The specific site to which this VLAN is assigned (if any) | [optional]
**group** | Option<**i32**> | VLAN group (optional) | [optional]
**vid** | Option<**i32**> | Numeric VLAN ID (1-4094) | [optional]
**name** | Option<**String**> |  | [optional]
**tenant** | Option<**i32**> |  | [optional]
**status** | Option<**String**> | Operational status of this VLAN  * `active` - Active * `reserved` - Reserved * `deprecated` - Deprecated | [optional]
**role** | Option<**i32**> | The primary function of this VLAN | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


