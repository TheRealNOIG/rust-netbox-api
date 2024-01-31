# Prefix

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**family** | [**crate::models::AggregateFamily**](Aggregate_family.md) |  | 
**prefix** | **String** |  | 
**site** | Option<[**crate::models::NestedSite**](NestedSite.md)> |  | [optional]
**vrf** | Option<[**crate::models::NestedVrf**](NestedVRF.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**vlan** | Option<[**crate::models::NestedVlan**](NestedVLAN.md)> |  | [optional]
**status** | Option<[**crate::models::PrefixStatus**](Prefix_status.md)> |  | [optional]
**role** | Option<[**crate::models::NestedRole**](NestedRole.md)> |  | [optional]
**is_pool** | Option<**bool**> | All IP addresses within this prefix are considered usable | [optional]
**mark_utilized** | Option<**bool**> | Treat as 100% utilized | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**children** | **i32** |  | [readonly]
**_depth** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


