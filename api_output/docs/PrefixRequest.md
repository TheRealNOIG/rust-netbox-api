# PrefixRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prefix** | **String** |  | 
**site** | Option<[**crate::models::NestedSiteRequest**](NestedSiteRequest.md)> |  | [optional]
**vrf** | Option<[**crate::models::NestedVrfRequest**](NestedVRFRequest.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenantRequest**](NestedTenantRequest.md)> |  | [optional]
**vlan** | Option<[**crate::models::NestedVlanRequest**](NestedVLANRequest.md)> |  | [optional]
**status** | Option<**String**> | * `container` - Container * `active` - Active * `reserved` - Reserved * `deprecated` - Deprecated | [optional]
**role** | Option<[**crate::models::NestedRoleRequest**](NestedRoleRequest.md)> |  | [optional]
**is_pool** | Option<**bool**> | All IP addresses within this prefix are considered usable | [optional]
**mark_utilized** | Option<**bool**> | Treat as 100% utilized | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


