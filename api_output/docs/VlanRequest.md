# VlanRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**site** | Option<[**crate::models::NestedSiteRequest**](NestedSiteRequest.md)> |  | [optional]
**group** | Option<[**crate::models::NestedVlanGroupRequest**](NestedVLANGroupRequest.md)> |  | [optional]
**vid** | **i32** | Numeric VLAN ID (1-4094) | 
**name** | **String** |  | 
**tenant** | Option<[**crate::models::NestedTenantRequest**](NestedTenantRequest.md)> |  | [optional]
**status** | Option<**String**> | * `active` - Active * `reserved` - Reserved * `deprecated` - Deprecated | [optional]
**role** | Option<[**crate::models::NestedRoleRequest**](NestedRoleRequest.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


